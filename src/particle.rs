use sfml::graphics::{CircleShape, Shape, Color, Transformable, RectangleShape}; 
use sfml::system::Vector2f; 
use std::time::SystemTime; 

const CIRCLE_SCALE_FACTOR: f32 = 20.0;  // default 40 
const HIT_BOX_SCALE_FACTOR: f32 = 1.0;  // default 1.0    0.5 to have it fit exactly around circle 

#[derive(Debug)]
pub struct CircleMiddleAndRadius {
    pub middle: Vector2f, 
    pub radius: f32, 
}

pub struct GhostParticle<'a> {
    pub particle: Particle<'a>, 
    pub can_exist: bool, 
    pub neighbors: i32, 
}


#[derive(Debug)]
pub struct Particle<'a> {
    charge: f32, 
    // charge will determine if it's proton (+), electron (-), or neutron ( when charge = 0.0)
    // abs value of charge also influences size (inverse proportion) and size will have inv prop to speed
    // particle charge determines what it is attracted to 
    // hit box of particle should be proportional to size 
    size: f32,  // size same as radius 
    speed: f32, 
    circle: CircleShape<'a>, 
    sign: Vec<RectangleShape<'a>>, // +, -, none 
    hit_box: RectangleShape<'a>, 
}

impl Particle<'_> {
    pub fn new(charge: f32, position: Vector2f) -> Self {
        let mut size: f32 = (1.0 / charge.abs()) * CIRCLE_SCALE_FACTOR; 
        let speed: f32 = charge.abs() / 2.0; 
        
        if charge == 0.0 {
            size = CIRCLE_SCALE_FACTOR; 
        } 

        if size > CIRCLE_SCALE_FACTOR * 4.0 {
            size = CIRCLE_SCALE_FACTOR * 4.0; 
        }

        let mut circle = CircleShape::new(size, 30); 
        circle.set_outline_thickness(1.0);  
        circle.set_outline_color(Color::BLACK); 
        circle.set_origin(Vector2f::new(
            circle.global_bounds().width / 2.0, circle.global_bounds().height / 2.0 
        )); 
        circle.set_position(position); 

        let mut sign: Vec<RectangleShape> = Vec::new(); 
        let mut fill_color: Option<Color> = None; // fill_color WILL be replaced so .unwrap is fine to use as there will never be an error 
        
        if charge < 0.0 { // neg
            fill_color = Some(Color::rgb(235, 64, 52)); 

            let mut minus_sign = RectangleShape::new(); 
            let width = size * (2.0/3.0); 
            let height = size / 10.0; 
            minus_sign.set_size(Vector2f::new( width, height)); 
            minus_sign.set_fill_color(Color::BLACK); 
            minus_sign.set_origin(Vector2f::new(minus_sign.global_bounds().width / 2.0, minus_sign.global_bounds().height / 2.0)); 
            minus_sign.set_position(circle.position()); 
            sign.push(minus_sign); 


        } else if charge == 0.0 {
            fill_color = Some(Color::rgb(1,223,190)); 
        } else { // pos 
            fill_color = Some(Color::rgb(249,211,31)); 

            let mut minus_sign = RectangleShape::new(); 
            let width = size * (2.0/3.0); 
            let height = size / 10.0; 
            minus_sign.set_size(Vector2f::new( width, height)); 
            minus_sign.set_fill_color(Color::BLACK); 
            minus_sign.set_origin(Vector2f::new(minus_sign.global_bounds().width / 2.0, minus_sign.global_bounds().height / 2.0)); 
            minus_sign.set_position(circle.position()); 

            let mut plus_sign = minus_sign.clone(); 
            plus_sign.set_rotation(90.0); 
            // minus_sign is the horizontal part of the +, plus_sign is the vertical part of the + 


            sign.push(minus_sign); 
            sign.push(plus_sign); 
        }


        circle.set_fill_color(fill_color.unwrap()); 

        let mut hit_box = RectangleShape::new(); 
        hit_box.set_outline_thickness(1.0); 
        hit_box.set_fill_color(Color::TRANSPARENT); 
        hit_box.set_outline_color(Color::WHITE); 

        Self {
            charge, 
            size, 
            speed, 
            circle, 
            sign, 
            hit_box, 
        } 
    }
    pub fn radius_from_charge(charge: f32) -> f32 {
        let mut size: f32 = (1.0 / charge.abs()) * CIRCLE_SCALE_FACTOR; 
        
        if charge == 0.0 {
            size = CIRCLE_SCALE_FACTOR; 
        } 

        if size >= 150.0 {
            size = 150.0; 
        }

        size 
    }
    
    pub fn get_particle_shape_ref(&self) -> &CircleShape { &self.circle }
    pub fn get_hit_box_shape_ref(&self) -> &RectangleShape { &self.hit_box }
    pub fn get_sign_ref(&self) -> &Vec<RectangleShape> { &self.sign }
    pub fn get_speed(&self) -> f32 { self.speed } 
    pub fn get_charge(&self) -> f32 { self.charge }
    pub fn get_particle_radius(&self) -> f32 { self.circle.radius() }

    pub fn move_(&mut self, offset: Vector2f) {
        self.circle.move_(offset); 
        for i in 0..self.sign.len() {
            self.sign[i].move_(offset); 
        }
    }   
    pub fn set_position(&mut self, offset: Vector2f) {
        self.circle.set_position(offset); 
        for i in 0..self.sign.len() {
            self.sign[i].set_position(offset); 
        }
    }
    pub fn check_if_this_and_another_particle_intersect(&self, other_particle_circle_shape_ref: &CircleShape) -> bool {
        let distance = (
            ( other_particle_circle_shape_ref.position().x - self.circle.position().x ).powi(2) + 
            ( other_particle_circle_shape_ref.position().y - self.circle.position().y ).powi(2)
        ).sqrt();   // everyone's favorite theorem 

        distance <= self.circle.radius() + other_particle_circle_shape_ref.radius()
    }
    pub fn update_hit_box(&mut self) {  // we only want to calculate the hit box points when it's time to check for circle intersections. this function would not exist if we updated the points on .move_, however it's unneeded to do it what would basically be every frame 


        self.hit_box.set_position(Vector2f::new(self.circle.position().x, self.circle.position().y)); 
        self.hit_box.set_size(Vector2f::new(self.circle.radius() * 4.0 * HIT_BOX_SCALE_FACTOR, self.circle.radius() * 4.0 * HIT_BOX_SCALE_FACTOR));
        self.hit_box.move_(Vector2f::new(-self.hit_box.size().x/2.0, -self.hit_box.size().y/2.0 ));
    }
    pub fn return_particles(number_of_particles: i32, mut particle_charge: f32, use_random_charges: bool, random_charge_range: [f32; 2], screen_width: i32, screen_height: i32, ensure_no_particle_overlapping: bool, make_rand_charge_range_into_int: bool, choose_particle_charge_from_set_list: bool, particle_charge_set_list: Vec<f32>, ) -> Vec<Particle<'static>> 
    {
        let mut particle_holder: Vec<Particle> = Vec::new(); 

        let mut particle_ranges_occupied: Vec<CircleMiddleAndRadius> = Vec::new(); // this is for when we have ensure_no_particle_overlapping 

        let mut x_pos: f32 = 0.0; 
        let mut y_pos: f32 = 0.0; 

        for i in 0..number_of_particles {
            if choose_particle_charge_from_set_list {
                if particle_charge_set_list.len() == 0 {panic!("particle_charge_set_list cannot equal 0")} 

                if particle_charge_set_list.len() == 1 {
                    particle_charge = particle_charge_set_list[0]; 
                } else {
                    particle_charge = particle_charge_set_list[(random_num(i + 6) % (particle_charge_set_list.len()) as u128) as usize]; 
                }; 
            }
            else if use_random_charges {
                if random_charge_range[0] > random_charge_range[1] {
                    panic!("Cannot construct range where lower bound '{}' is greater than upper bound '{}'", random_charge_range[0], random_charge_range[1]); 
                }

                // println!("{}", type_of(&random_charge_range[0]));    f64 or i32 
                if make_rand_charge_range_into_int {
                    particle_charge = rand_from_range(
                        [random_charge_range[0] as i32, random_charge_range[1] as i32], i) as f32; 
                } else {
                    particle_charge = rand_float_from_range([random_charge_range[0], random_charge_range[1]], i);
                } 

            }// note to past self to future self: don't put an else statement for if use_random_charge == false,, it will already be configured

            if ensure_no_particle_overlapping {
                let mut counter = 0; 
                // ! This runs a ton of times for more and more particles and may cause in stack overflow if particles are not scaled appropriately with screen width/height 
                // let a normal particle to screen ratio be represented as p:s. if u make some pNew > p with s being the same, expect stack overflows  

                loop {
                    let mut unique_spot = true; 
                    let radius = Self::radius_from_charge(particle_charge); 

                    x_pos = radius + (random_num(counter) % (screen_width - 2*radius as i32) as u128) as f32; 
                    y_pos = radius + (random_num(counter +1) % (screen_height - 2*radius as i32) as u128) as f32;


                    let d = Self::new(particle_charge, Vector2f::new(x_pos,y_pos)); 

                    for i in 0..particle_ranges_occupied.len() { 
                        
                        let this_particle_range = CircleMiddleAndRadius {  
                            middle: d.get_particle_shape_ref().position(),
                            radius: d.get_particle_shape_ref().radius(), 
                        }; 
                        // it is just better to see what values sfml gives them than to approximate what they might be (used to do x_pos + radius, y_pos + radius which kept on producing bugs)

                        if 
                        (
                            (particle_ranges_occupied[i].middle.x - this_particle_range.middle.x ).powi(2) + 
                            (particle_ranges_occupied[i].middle.y - this_particle_range.middle.y ).powi(2)
                        ).sqrt() < particle_ranges_occupied[i].radius + this_particle_range.radius
                        {
                            unique_spot = false; 
                            break; 
                        } 
                        counter += 1; 
                    }
                    counter += 1; 
                    if unique_spot {
                        let d = Self::new(particle_charge, Vector2f::new(x_pos,y_pos)); 
                        particle_ranges_occupied.push(CircleMiddleAndRadius {  
                            middle: d.get_particle_shape_ref().position(),
                            radius: d.get_particle_shape_ref().radius(), 
                        }); 
                        
                        break; 
                    }
                }
            } else {
                x_pos = (random_num(i+4) % screen_width as u128) as f32; 
                y_pos = (random_num(i+5) % screen_height as u128) as f32; 
            }


            particle_holder.push(Particle::new(particle_charge , Vector2f::new(x_pos, y_pos))); 
        }

        particle_holder
    }
}    

// revised        by a lot 
pub fn random_num(seed: i32) -> u128 {
    let sys_time = SystemTime::now(); 
    match sys_time.duration_since(SystemTime::UNIX_EPOCH)  {
        Ok(n) => {
            let increment = 405; 
            let multiplier = 291; 
            let modulus = 112; 

            (( ((multiplier * seed + increment)%modulus)) as u128) * n.as_micros()
        }, 
        Err(_) => panic!("Calling time failed"), 
    }
}

pub fn rand_from_range(range: [i32; 2], seed: i32) -> i128 {
    if range[1]-range[0] == 1 {
        (range[0] + (random_num(seed)%2) as i32) as i128
    } else {
        (range[0] + (random_num(seed)%((range[1]-range[0]) as u128) + random_num(seed+1)%2) as i32) as i128
    }
}

pub fn random_float(seed: i32) -> f32 {  // random float from [0,1)
    let sys_time = SystemTime::now(); 
    match sys_time.duration_since(SystemTime::UNIX_EPOCH)  {
        Ok(n) => {
            let increment = 405; 
            let multiplier = 291; 
            let modulus = 112; 

            let result = (( ((multiplier * seed + increment)%modulus)) as u128) * n.as_micros(); 

            match ("0.".to_string() + result.to_string().as_ref()).parse()  {
                Ok(v) => v, 
                Err(v) => panic!("Error for random_float string parse to f32 {}", v)
            }
            
        }, 
        Err(_) => panic!("Calling time failed"), 
    }
}

pub fn rand_float_from_range(range: [f32; 2], seed: i32) -> f32 {
    range[0] + random_float(seed)*(range[1]-range[0])
}