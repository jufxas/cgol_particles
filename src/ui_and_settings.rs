// include manual placement of particles 
// reset game button 

// play button + pause button on the left + a gear icon 
use sfml::graphics::{RectangleShape, Color, Font, Text, TextStyle};

#[derive(PartialEq)]
pub enum GameStates {
    Paused, 
    On, 
    Settings, 
}
pub struct Settings {
    pub dies_on_fewer_than_n_neighbors: i32,
    pub dies_on_more_than_n_neighbors: i32,
    pub dead_to_alive_on_n_neighbors: Vec<i32>,
    pub check_rules_after_n_seconds: f32, 
    pub random_move_magnitude: f32,
    pub allow_particle_attraction: bool,
    pub use_clumping_prevention: bool,
    pub draw_hitbox: bool, 
    // have to convert the constants into variables 
    pub circle_scale_factor: f32, 
    pub hit_box_scale_factor: f32 
}
pub struct ParticleGenerationConfig {
    pub number_of_particles: i32, 
    pub particle_charge: f32, 
    pub use_random_charges: bool, 
    pub random_charge_range: [f32; 2], 
    pub ensure_no_particle_overlapping: bool, 
    pub make_rand_charge_into_int: bool, 
    pub choose_particle_charge_from_set_list: bool, 
    pub particle_charge_set_list: Vec<f32>

    // Use random charges paired w/ random_charge range > Particle Charge Adherence 
    // choose_particle_charge_from_set_list adherence > random_charge_range > particle_charge 
}

pub struct SettingsUI<'a> {
    screen_width: f32,     // w,h needed so it scales appropriately 
    screen_height: f32, 
    font: Font, 
    rectangle_holder: Vec<RectangleShape<'a>>, 
    text_holder: Vec<Text<'a>>, 
}
impl SettingsUI<'_> {
    pub fn new(screen_width: f32, screen_height: f32, font: Font) -> Self {
        Self {
            screen_width, 
            screen_height, 
            font, 
            rectangle_holder: Vec::new(), 
            text_holder: Vec::new(), 
        }
    }
    pub fn build_ui(&mut self) {   // to be ran once 

    }

    
    pub fn get_rectangle_holder_ref(&self) -> &Vec<RectangleShape<'_>> { &self.rectangle_holder }
    pub fn get_text_holder_ref(&self) -> &Vec<Text<'_>> { &self.text_holder }
    
}