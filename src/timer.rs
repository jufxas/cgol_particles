pub struct NameFrameNumberPair(char, u32); 

pub struct Timer {
    pub frame_rate: u32, 
    pub frame_number: u32, 
    pub time_pair: Vec<NameFrameNumberPair>,  
}   
impl Timer {
    pub fn new(frame_rate: u32, frame_number: u32) -> Self {
        Self { frame_rate, frame_number, time_pair: Vec::new() }
    }
    pub fn update_frame_number(&mut self) {
        self.frame_number += 1; 
    }
    pub fn set_time(&mut self, name: char) {
        for NameFrameNumberPair(time_pair_name,_ ) in &self.time_pair {
            if name == *time_pair_name {
                panic!("Name '{}' already taken", name); 
            }
        }
        self.time_pair.push(NameFrameNumberPair(name, self.frame_number)); 
    }
    pub fn get_time(&mut self, name: char) -> f32 {
        for NameFrameNumberPair(time_pair_name,value ) in &self.time_pair {
            if name == *time_pair_name {
                return 
                (self.frame_number as f32 - *value as f32) / (self.frame_rate as f32); 
            }
        }
        panic!("Did not find name '{}'", name); 
    }
    pub fn check_if_time_exists(&self, name: char) -> bool {
        for NameFrameNumberPair(time_pair_name,_ ) in &self.time_pair {
            if name == *time_pair_name {
                return true; 
            }
        }
        false 
    }
    pub fn remove_time(&mut self, name: char) {
        let mut index: usize = 0; 
        let mut found_name = false; 
        for NameFrameNumberPair(time_pair_name,_ ) in &self.time_pair {
            if name == *time_pair_name {
                found_name = true; 
                break; 
            }
            index += 1; 
        }
        self.time_pair.remove(index); 
        if !found_name {
            panic!("Did not find name '{}' in remove_time", name)
        }; 
    }

    #[allow(unused)]
    pub fn get_global_time(&self) -> f32 {
        self.frame_number as f32 / self.frame_rate as f32
    }
}