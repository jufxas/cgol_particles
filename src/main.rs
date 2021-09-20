extern crate sfml;

use sfml::graphics::*;
use sfml::window::*;
use sfml::system::*;

mod particle;      mod timer;      mod ui_and_settings;      mod navbar;    mod textbox; 
use particle::*;   use timer::*;   use ui_and_settings::*;   use navbar::*; use textbox::*; 


const WIDTH: u32 = 1200; 
const HEIGHT: u32 = 1000;    // default 1200, 1000 
const FRAME_RATE: u32 = 30; 

/*
    TODO: 
    Working pause and play button | UI and all that that allows customization | Game restart 
 */


struct GameHandler<'a> {
    window: RenderWindow, 
    mouse_held: bool, 
    particle_holder: Vec<Particle<'a>>, 
    counter: i32, 
    timer: Timer, 
    navbar_ui: NavbarUI<'a>, 
    game_state: GameStates, 
    settings: Settings,
    particle_gen_config: ParticleGenerationConfig, 
    text_box_sys: TextBoxSystem<'a>, 
} 
impl GameHandler<'_> {
    fn update(&mut self) {
        if self.game_state == GameStates::On { self.run_game() }
    }
    fn run_game(&mut self) {
        // insert game logic, event listeners, etc. 
        // Key::UP.is_pressed
        // mouse::Button::LEFT.is_pressed()
        // RenderWindow::mouse_position(&self.window).x
        

        let mut particle_index_to_remove: Vec<usize> = Vec::new();  
        // checking for rules   
        if !self.timer.check_if_time_exists('c') {
            self.timer.set_time('c'); 
        } else {

            if self.timer.get_time('c') >= self.settings.check_rules_after_n_seconds {

                for i in 0..self.particle_holder.len() {
                    self.particle_holder[i].update_hit_box(); 

                    // ghost particle check (dead to alive)
                    let r = self.particle_holder[i].get_particle_radius(); 

                    // 0 = left, 1 = right, 2 = up, 3 = down 
                    let mut ghost_particles_holder_lrud: [GhostParticle; 4] = [
                        GhostParticle {
                            particle: Particle::new(self.particle_holder[i].get_charge(), self.particle_holder[i].get_particle_shape_ref().position() + Vector2f::new(-r*2.0,0.0)), 
                            can_exist: true, neighbors: 0 
                        }, 
                        GhostParticle {
                            particle: Particle::new(self.particle_holder[i].get_charge(), self.particle_holder[i].get_particle_shape_ref().position() + Vector2f::new(r*2.0,0.0)), 
                            can_exist: true, neighbors: 0 
                        }, 
                        GhostParticle {
                            particle: Particle::new(self.particle_holder[i].get_charge(), self.particle_holder[i].get_particle_shape_ref().position() + Vector2f::new(0.0,-r*2.0)), 
                            can_exist: true, neighbors: 0 
                        }, 
                        GhostParticle {
                            particle: Particle::new(self.particle_holder[i].get_charge(), self.particle_holder[i].get_particle_shape_ref().position() + Vector2f::new(0.0,r*2.0)), 
                            can_exist: true, neighbors: 0 
                        }
                    ]; 
                    for i in 0..4 { ghost_particles_holder_lrud[i].particle.update_hit_box() }


                    let mut neighbors: i32 = 0;                     
                    for x in 0..self.particle_holder.len() {
                        if i == x { continue; }
                        self.particle_holder[x].update_hit_box(); 

                        let left_top = self.particle_holder[i].get_hit_box_shape_ref().position(); 
                        let right_bottom = self.particle_holder[i].get_hit_box_shape_ref().position() + self.particle_holder[i].get_hit_box_shape_ref().size(); 

                        let other_left_top = self.particle_holder[x].get_hit_box_shape_ref().position(); 
                        let other_right_bottom = self.particle_holder[x].get_hit_box_shape_ref().position() + self.particle_holder[x].get_hit_box_shape_ref().size(); 
                        
                        if (
                            (left_top.x <= other_left_top.x && other_left_top.x<=  right_bottom.x) ||
                            (left_top.x <= other_right_bottom.x && other_right_bottom.x <=  right_bottom.x)
                        ) 
                        && 
                        (
                            (left_top.y <= other_left_top.y && other_left_top.y <=  right_bottom.y) ||
                            (left_top.y <= other_right_bottom.y && other_right_bottom.y <=  right_bottom.y)
                        ) 
                        {
                            neighbors += 1; 
                        }
                        
                        for y in 0 as usize..4 {
                            if ghost_particles_holder_lrud[y].can_exist {
                                if ghost_particles_holder_lrud[y].particle.check_if_this_and_another_particle_intersect(self.particle_holder[x].get_particle_shape_ref()) ||
                                (ghost_particles_holder_lrud[y].particle.get_particle_shape_ref().position().x <= ghost_particles_holder_lrud[y].particle.get_particle_radius() ||

                                ghost_particles_holder_lrud[y].particle.get_particle_shape_ref().position().x >= WIDTH as f32 - ghost_particles_holder_lrud[y].particle.get_particle_radius()  ) ||

                                (ghost_particles_holder_lrud[y].particle.get_particle_shape_ref().position().y <= ghost_particles_holder_lrud[y].particle.get_particle_radius()  ||

                                ghost_particles_holder_lrud[y].particle.get_particle_shape_ref().position().y >= HEIGHT as f32 - ghost_particles_holder_lrud[y].particle.get_particle_radius() ) 
                                
                                {
                                    ghost_particles_holder_lrud[y].can_exist = false; 
                                } else {
                                    let ghost_left_top = ghost_particles_holder_lrud[y].particle.get_particle_shape_ref().position(); 
                                    let ghost_right_bottom = ghost_particles_holder_lrud[y].particle.get_particle_shape_ref().position() + ghost_particles_holder_lrud[y].particle.get_hit_box_shape_ref().size(); 

                                    if (
                                        (other_left_top.x <= ghost_left_top.x && ghost_left_top.x <= other_right_bottom.x) ||
                                        (other_left_top.x <= ghost_right_bottom.x && ghost_right_bottom.x <= other_right_bottom.x)
                                    ) && 
                                    (
                                        (other_left_top.y <= ghost_left_top.y && ghost_left_top.y <= other_right_bottom.y) || 
                                        (other_left_top.y <= ghost_right_bottom.y && ghost_right_bottom.y <= other_right_bottom.y)
                                    )
                                     {
                                        ghost_particles_holder_lrud[y].neighbors += 1; 
                                    }
                                }
                            }
                        }
                        
                    }


                    // impl cgol rules 
                    if neighbors < self.settings.dies_on_fewer_than_n_neighbors || neighbors > self.settings.dies_on_more_than_n_neighbors {
                        // n < 2, n > 3    
                        particle_index_to_remove.push(i); 
                    }

                    for z in 0 as usize..4 {
                        if ghost_particles_holder_lrud[z].can_exist {
                            if self.settings.dead_to_alive_on_n_neighbors.contains(&ghost_particles_holder_lrud[z].neighbors) {
                                // ! 'dead to alive on n' can be changed here to 'dead to alive on n or more' with changing == to >= 

                                self.particle_holder.push(Particle::new(ghost_particles_holder_lrud[z].particle.get_charge(), ghost_particles_holder_lrud[z].particle.get_particle_shape_ref().position())); 
                            }

                            break; 
                        }
                    }

                }

                self.timer.remove_time('c')
            }
        }

        // removing particles 
        for i in 0..particle_index_to_remove.len() {
            self.particle_holder.remove(particle_index_to_remove[i]); 
            for x in 0..particle_index_to_remove.len() {
                if particle_index_to_remove[x] != 0 { particle_index_to_remove[x] -= 1; } 
            }
        }

        // mouse inputs 
        if mouse::Button::LEFT.is_pressed() {
            if !self.mouse_held {
                for i in 0..self.text_box_sys.text_box_holder.len() {
                    if self.text_box_sys.text_box_holder[i].mouse_clicked_in_box(RenderWindow::mouse_position(&self.window) )
                        {
                            self.text_box_sys.text_box_holder[i].open_text_stream(); 
                        }   else {
                            self.text_box_sys.text_box_holder[i].close_text_stream(); 
                        }
                    }
                self.mouse_held = true; 
            }
        } else {
            self.mouse_held = false; 
        }


        let mut num_of_particles_not_moved: i32 = 0; 
        for i in 0..self.particle_holder.len() {
            if self.particle_holder[i].get_charge() == 0.0 { continue; }

            // attraction to particles 

            let mut move_x:f32 = 0.0; 
            let mut move_y:f32 = 0.0; 

            let opposite_charge = if self.particle_holder[i].get_charge() > 0.0 { -0.0000001  } else { 0.0000001 };
            let mut moved = false; 
            let mut particle_index_taken: Vec<i32> = Vec::new(); 
            
            for x in 0..self.particle_holder.len() {
                // c = 1.0 Oc = -0.01 (LF neg charges).   if -0.01 > 0 ... else 
                // r = [-1,0,1]   -1 < -0.1   0 < -0.1   1 < -0.1
                
                // c = -1.0 Oc = 0.1 (LF pos charges)  if 0.1 > 0.0 -> 
                // r = [-1,0,1]  -1 > 1.1  0 > 0.1   1 > 0.1  
                if !self.settings.allow_particle_attraction && self.settings.use_clumping_prevention { break; }

                let statement = if opposite_charge > 0.0 {
                    self.particle_holder[x].get_charge() > opposite_charge && !particle_index_taken.contains(&(x as i32))
                } else {
                    self.particle_holder[x].get_charge() < opposite_charge && !particle_index_taken.contains(&(x as i32))
                }; 

                if statement {
                    let dx = self.particle_holder[x].get_particle_shape_ref().position().x 
                    - self.particle_holder[i].get_particle_shape_ref().position().x; 


                    if dx > 0.0 {
                        move_x = self.settings.random_move_magnitude * self.particle_holder[i].get_speed()
                    } else if dx < 0.0 {
                        move_x = -self.settings.random_move_magnitude * self.particle_holder[i].get_speed()
                    } 

                    let dy = self.particle_holder[i].get_particle_shape_ref().position().y 
                    - self.particle_holder[x].get_particle_shape_ref().position().y; 

                    if dy > 0.0 {
                        move_y = -self.settings.random_move_magnitude * self.particle_holder[i].get_speed()
                    } else if dy < 0.0 {
                        move_y = self.settings.random_move_magnitude * self.particle_holder[i].get_speed()
                    } 

                    // add some more random so it isn't completely static 
                    let pos_neg_x = (random_num(self.counter as i32) % 2) as u8;      // 0 or 1
                    let pos_neg_y = (random_num((self.counter + 1) as i32) % 2) as u8; 
    
                    move_x += if pos_neg_x == 1 {
                        (self.settings.random_move_magnitude * self.particle_holder[i].get_speed()) * 2.0 
                    } else {
                        (-self.settings.random_move_magnitude * self.particle_holder[i].get_speed()) * 2.0
                    }; 
                    move_y += if pos_neg_y == 1 {
                        (self.settings.random_move_magnitude * self.particle_holder[i].get_speed()) * 2.0
                    } else {
                        (-self.settings.random_move_magnitude * self.particle_holder[i].get_speed()) * 2.0
                    }; 

                    self.particle_holder[i].move_(Vector2f::new(move_x, move_y)); 
                    particle_index_taken.push(x as i32); 
                    moved = true; 
                    break; 
                }
            }
            if !moved {
                let pos_neg_x = (random_num(self.counter as i32) % 2) as u8;      // 0 or 1
                let pos_neg_y = (random_num((self.counter + 1) as i32) % 2) as u8; 

                // ! EDIT    idk it just seems better like this
                // let zero_or_one_x:f32 = if random_num(self.counter + 2 as i32) % 2 == 0 { 1.0 } else { 0.0 };
                // let zero_or_one_y:f32 = if random_num(self.counter + 3 as i32) % 2 == 0 { 1.0 } else { 0.0 };
                let zero_or_one_x:f32 = 1.0;
                let zero_or_one_y:f32 = 1.0;


                move_x = if pos_neg_x == 1 {
                    self.settings.random_move_magnitude * self.particle_holder[i].get_speed() * zero_or_one_x
                } else {
                    -self.settings.random_move_magnitude * self.particle_holder[i].get_speed() * zero_or_one_x
                }; 
                move_y = if pos_neg_y == 1 {
                    self.settings.random_move_magnitude * self.particle_holder[i].get_speed() * zero_or_one_y
                } else {
                    -self.settings.random_move_magnitude * self.particle_holder[i].get_speed() * zero_or_one_y 
                }; 

                self.particle_holder[i].move_(Vector2f::new(move_x, move_y)); 

            }
        

            let mut intersects_another_particle = false; 
            for x in 0..self.particle_holder.len() {
                if i == x  { continue; }
                if self.particle_holder[i].check_if_this_and_another_particle_intersect(self.particle_holder[x].get_particle_shape_ref()) {
                    intersects_another_particle = true; 
                    break; 
                }
            }
            
            if intersects_another_particle || 
            (self.particle_holder[i].get_particle_shape_ref().position().x <= self.particle_holder[i].get_particle_radius() ||
            self.particle_holder[i].get_particle_shape_ref().position().x >= WIDTH as f32 - self.particle_holder[i].get_particle_radius()  ) ||
            (self.particle_holder[i].get_particle_shape_ref().position().y <= self.particle_holder[i].get_particle_radius()  ||
            self.particle_holder[i].get_particle_shape_ref().position().y >= HEIGHT as f32 - self.particle_holder[i].get_particle_radius() ) 
            {
                self.particle_holder[i].move_(Vector2f::new(-move_x, -move_y)); 
                num_of_particles_not_moved += 1; 
            }
            
            self.counter += 1; 
        }

        if self.counter > 10000 { self.counter = 0 ; }
        if (num_of_particles_not_moved as f32) / (self.particle_holder.len() as f32) >= 0.7 && self.settings.use_clumping_prevention {
            self.settings.allow_particle_attraction = false; 
            self.settings.random_move_magnitude *= -1.0;
            
            if !self.timer.check_if_time_exists('a') { self.timer.set_time('a') }
        } 

        if self.timer.check_if_time_exists('a') {
            if self.timer.get_time('a') >= 60.0 {   
                self.settings.allow_particle_attraction = true; 
                self.settings.random_move_magnitude *= -1.0; 
                self.timer.remove_time('a'); 
            }
        }

        // hit_box 
        if self.settings.draw_hitbox {
            for i in 0..self.particle_holder.len() {
                self.particle_holder[i].update_hit_box(); 
            }
        }

        

    }
    fn on_init_game(&mut self) {
        self.mouse_held = false; 
        self.window.set_vertical_sync_enabled(true); 
        self.window.set_framerate_limit(FRAME_RATE); 
        self.navbar_ui.build_navbar(); 
        self.update_particle_holder(); 

        // textbox 
        self.text_box_sys.push_new_text_box("test");
        self.text_box_sys.text_box_holder[0].construct_text_box(Vector2f::new(100.0, 100.0), 1000.0, 50.0);
    }
    fn update_particle_holder(&mut self) {
        self.particle_holder = Particle::return_particles(
            self.particle_gen_config.number_of_particles,
            self.particle_gen_config.particle_charge, 
            self.particle_gen_config.use_random_charges, 
            self.particle_gen_config.random_charge_range, 
             WIDTH as i32, HEIGHT as i32, 
             self.particle_gen_config.ensure_no_particle_overlapping,
             self.particle_gen_config.make_rand_charge_into_int,
            self.particle_gen_config.choose_particle_charge_from_set_list, 
            self.particle_gen_config.particle_charge_set_list.clone()
        ); 
    }
    fn render(&mut self) {
        // clear screen 
        self.window.clear(Color::rgb(245, 66, 191));

        // draw objects, i.e, window.draw(&obj); 
        // then display         

        for obj in &self.particle_holder {
            self.window.draw(obj.get_particle_shape_ref()); 

            for part in obj.get_sign_ref() {
                self.window.draw(part); 
            }
            if self.settings.draw_hitbox {
                self.window.draw(obj.get_hit_box_shape_ref()); 
            }
        }

        // navbar render 
        if self.game_state == GameStates::Paused {
            self.window.draw(self.navbar_ui.get_play_button_ref()  );
        } else if self.game_state == GameStates::On {
            let pause_button = self.navbar_ui.get_pause_button_ref(); 
            self.window.draw(&pause_button[0]  );
            self.window.draw(&pause_button[1]  );
        }
        self.window.draw(self.navbar_ui.get_settings_button_square()  );
        self.window.draw(self.navbar_ui.get_settings_button_text()  );

        // text box render 
        for i in 0..self.text_box_sys.text_box_holder.len() {
            self.window.draw(self.text_box_sys.text_box_holder[i].get_text_box_ref());
            self.window.draw(self.text_box_sys.text_box_holder[i].get_text_channel_ref());
        }
        

        self.window.display();
        self.timer.update_frame_number(); 
    } 
    fn receive_char(&mut self, unicode: char)  { 
        // disallowed: 8  (delete), 10 (enter)  
        let utf8 = unicode.to_string().as_bytes()[0] ; 
        if utf8 == 10 { return; }

        for i in 0..self.text_box_sys.text_box_holder.len() {
            if self.text_box_sys.text_box_holder[i].get_text_channel_state() {
                self.text_box_sys.text_box_holder[i].send_char_text_stream(unicode);
            }
        }
    }
    
}

fn main() {
    let font = &*Font::from_file("./src/fonts/RobotoMono-VariableFont_wght.ttf").unwrap(); 
    
    let mut game = GameHandler {
        window: RenderWindow::new(
            (WIDTH, HEIGHT),
        "C G O L Particles",
        Style::CLOSE , 
        &Default::default(),
        ), 
        mouse_held: false, 
        particle_holder: Vec::new(), 
        counter: 0, 
        timer: Timer::new(FRAME_RATE, 0), 
        navbar_ui: NavbarUI { 
            play_button: None, 
            pause_button: None, 
            settings_button_square: None, 
            settings_button_font: font, settings_button_text: None
        },
        game_state: GameStates::On, 
        settings: Settings {
             dies_on_fewer_than_n_neighbors: 2,
             dies_on_more_than_n_neighbors: 5,
             dead_to_alive_on_n_neighbors: vec![2,3,4],
             check_rules_after_n_seconds: 0.1, 
             random_move_magnitude: 3.0,
             allow_particle_attraction: true,
             use_clumping_prevention: true, // if true, program will check if 70% of the particles don't move on a frame, for 60s they move randomly 
             draw_hitbox: false, 
             circle_scale_factor: 40.0, 
             hit_box_scale_factor: 1.0 
        }, 
        particle_gen_config: ParticleGenerationConfig {
            number_of_particles: 30, 
            particle_charge: 1.0, 
            use_random_charges: true, 
            random_charge_range: [-1.0, 3.0],   // [-1.0, 3.0]  nice default
            ensure_no_particle_overlapping: true, 
            make_rand_charge_into_int: true, 
            choose_particle_charge_from_set_list: true, 
            particle_charge_set_list: vec![0.0],  // vec![-1.0, 0.0, 1.0] nice default 
        }, 
        text_box_sys: TextBoxSystem { font, text_box_holder: Vec::new() }
    };

    game.on_init_game(); 
    loop {

        game.update(); 

        while let Some(ev) = game.window.poll_event() {
            match ev {
                Event::Closed => {
                    game.window.close();
                    return ;
                },
                Event::TextEntered  { unicode } => {
                    game.receive_char(unicode);
                },  
                _ => {},
            }
        }

        // drawing
        game.render(); 
    }
}
