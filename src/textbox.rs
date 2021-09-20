use sfml::graphics::{RectangleShape, Font, Transformable, Shape, Color, Text};
use sfml::system::{Vector2f, Vector2i};

const DELETE_CHAR_CODE: u8 = 8; 

// monospace fonts recommended  
// ur gonna have to modify main.rs to get text boxes to work; does not work out the box 

// Steps: 
/*
    Make a font reference defined in the main fn 
         let font = &*Font::from_file("./src/fonts/RobotoMono-VariableFont_wght.ttf").unwrap(); 
    Add text_box_sys into GameHandler 
        text_box_sys: TextBoxSystem { font, text_box_holder: Vec::new() }
    Add the following code to the update method in GameHandler or wherever mouse inputs are handled 
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

    Add the following code to the render method (text/text box typically is last to render)
    for i in 0..self.text_box_sys.text_box_holder.len() {
            self.window.draw(self.text_box_sys.text_box_holder[i].get_text_box_ref());
            self.window.draw(self.text_box_sys.text_box_holder[i].get_text_channel_ref());
        }
    
    Add the following method into the impl GameHandler  
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

    On the poll events handler 'match ev' statement, add the following code 
    Event::TextEntered  { unicode } => {
                    game.receive_char(unicode);
                },  

    game is GameHandler instance 


    On the init game method, you can add a text box   Here's a sample one 
        self.text_box_sys.push_new_text_box("test");
        self.text_box_sys.text_box_holder[0].construct_text_box(Vector2f::new(100.0, 100.0), 1000.0, 50.0);
*/

pub struct TextBoxSystem<'a> {
    pub font: &'a Font, 
    pub text_box_holder: Vec<TextBox<'a>>, 
} 
impl TextBoxSystem<'_> {
    pub fn push_new_text_box(&mut self, name: &str) {
        let name = name.to_string(); 
        self.text_box_holder.push(TextBox{
            rectangle_box: RectangleShape::new(), text_channel_state: false, name,
            text_channel: Text::new("", self.font, 0), max_chars_in_one_row: 0,
        })
    }
    pub fn get_text_box_index_by_name(&self, name: &str) -> usize {
        let name = name.to_string(); 
        for i in 0..self.text_box_holder.len() {
            if self.text_box_holder[i].name == name {
                return i; 
            }
        }
        panic!("Did not find text box with name '{}'", name); 
    } 
   
}

pub struct TextBox<'a> {
     rectangle_box: RectangleShape<'a>, 
     text_channel_state: bool,    // true -> accept text inputs, false -> don't 
     text_channel: Text<'a>, 
     name: String, // ! not sure if it should be used, impl here so there's easy way to identify each text box
     max_chars_in_one_row: u32, 
    
}

impl TextBox<'_> {
    pub fn construct_text_box(&mut self, position: Vector2f, width: f32, height: f32) {
        self.rectangle_box.set_position(position);
        self.rectangle_box.set_size(Vector2f::new(width, height));
        self.rectangle_box.set_outline_thickness(3.0);
        self.rectangle_box.set_outline_color(Color::BLACK); 

        self.text_channel.set_position(position - Vector2f::new(0.0, height / 10.0));
        self.text_channel.set_fill_color(Color::BLACK); 
        self.text_channel.set_character_size( (height - 0.2*(height)) as u32 );


        self.max_chars_in_one_row = (((width / (self.text_channel.character_size() as f32)) * (20.0/12.5)).round()) as u32;    // works surprisingly well 
    }
    pub fn get_text_box_ref(&self) -> &RectangleShape { &self.rectangle_box }
    pub fn get_text_channel_ref(&self) -> &Text { &self.text_channel }
    pub fn get_text_channel_state(&self) -> bool { self.text_channel_state }

    pub fn mouse_clicked_in_box(&self, mouse_pos: Vector2i) -> bool {
        let mouse_pos_2f = Vector2f::new(mouse_pos.x as f32, mouse_pos.y as f32);
        self.rectangle_box.global_bounds().contains(mouse_pos_2f)
    }
    
    pub fn open_text_stream(&mut self)  { 
        self.text_channel_state = true; 
        self.rectangle_box.set_outline_color(Color::rgb(52, 113, 235));
    }
    pub fn close_text_stream(&mut self) { 
        self.text_channel_state = false; 
        self.rectangle_box.set_outline_color(Color::BLACK); 
    } 

    pub fn send_char_text_stream(&mut self, text: char) {
        if !self.text_channel_state { panic!("Disallowed access to text channel as it is closed") }

        if text.to_string().as_bytes()[0] == DELETE_CHAR_CODE  { 
            let text_in_box_as_string = self.text_channel.string().to_rust_string(); 

            if text_in_box_as_string.len() == 0 { return; }
            let last = text_in_box_as_string.len() - 1; 

            let new_text = (&text_in_box_as_string.as_str()[0..last]).to_string();
            self.text_channel.set_string(new_text.as_str());

            if self.rectangle_box.outline_color() != Color::rgb(52, 113, 235)  {
                self.rectangle_box.set_outline_color(Color::rgb(52, 113, 235))
            };

            return; 
        }

        if self.text_channel.string().to_rust_string().len() as u32 == self.max_chars_in_one_row { 
            self.rectangle_box.set_outline_color(Color::RED);
            return; 
        } else {
            self.rectangle_box.set_outline_color(Color::rgb(52, 113, 235));
        }

        let new_text = self.text_channel.string().to_rust_string() + text.to_string().as_str(); 
        self.text_channel.set_string(new_text.as_str()); 
        
    }
}
