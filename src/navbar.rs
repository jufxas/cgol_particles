use sfml::graphics::{RectangleShape, Color, VertexArray, PrimitiveType, Font, Text, Shape, Transformable, RenderTarget, RenderWindow};
use sfml::system::{Vector2f, Vector2i}; 

use crate::ui_and_settings::*;  
pub struct NavbarUI<'a>                        
{
    pub play_button: Option<VertexArray>,                    
    pub pause_button: Option<[RectangleShape<'a>; 2]>, 
    pub settings_button_square: Option<RectangleShape<'a>>, 
    pub settings_button_font: &'a Font, 
    pub settings_button_text: Option<Text<'a>>, 
    pub pause_button_hit_box: Option<RectangleShape<'a>>, 
}

impl NavbarUI<'_> {
    pub fn get_play_button_ref(&self) -> &VertexArray { self.play_button.as_ref().unwrap()  }
    pub fn get_pause_button_ref(&self) -> &[RectangleShape; 2] { &self.pause_button.as_ref().unwrap() }
    pub fn get_settings_button_square_ref(&self) -> &RectangleShape { &self.settings_button_square.as_ref().unwrap() }
    pub fn get_settings_button_text_ref(&self) -> &Text { &self.settings_button_text.as_ref().unwrap()  }

    pub fn build_navbar(&mut self)  {
        let offset = Vector2f::new(10.0, 10.0); 
        let size = 25.0;   // default 25 

        self.play_button = Some(VertexArray::new(PrimitiveType::TRIANGLES, 4)); 

        self.play_button.as_mut().unwrap()[0].position = offset; 
        self.play_button.as_mut().unwrap()[1].position = offset + Vector2f::new(size, size); 
        self.play_button.as_mut().unwrap()[2].position = offset + Vector2f::new(0.0, 2.0*size); 

        self.play_button.as_mut().unwrap()[0].color = Color::rgb(66, 255, 0);
        self.play_button.as_mut().unwrap()[1].color = Color::rgb(66, 255, 0);
        self.play_button.as_mut().unwrap()[2].color = Color::rgb(66, 255, 0);
        self.play_button.as_mut().unwrap()[3] = self.play_button.as_mut().unwrap()[0]; 

        
        self.pause_button = Some([RectangleShape::new(), RectangleShape::new()]);

        self.pause_button.as_mut().unwrap()[0].set_fill_color(Color::BLACK); 
        self.pause_button.as_mut().unwrap()[0].set_position(offset); 
        self.pause_button.as_mut().unwrap()[0].set_size(Vector2f::new(size/2.0, size*2.0));

        self.pause_button.as_mut().unwrap()[1].set_fill_color(Color::BLACK); 
        self.pause_button.as_mut().unwrap()[1].set_position(offset + Vector2f::new(size, 0.0) ); 
        self.pause_button.as_mut().unwrap()[1].set_size(Vector2f::new(size/2.0, size*2.0));

        self.pause_button_hit_box = Some(RectangleShape::new());

        self.pause_button_hit_box.as_mut().unwrap().set_position(offset);
        self.pause_button_hit_box.as_mut().unwrap().set_size(Vector2f::new(size * 2.0, size * 2.0));   
        self.pause_button_hit_box.as_mut().unwrap().set_fill_color(Color::TRANSPARENT);


        self.settings_button_square = Some(RectangleShape::new());

        self.settings_button_square.as_mut().unwrap().set_size(Vector2f::new(size * 2.0, size * 2.0)); 
        self.settings_button_square.as_mut().unwrap().set_outline_color(Color::BLACK);
        self.settings_button_square.as_mut().unwrap().set_outline_thickness(5.0);
        self.settings_button_square.as_mut().unwrap().set_fill_color(Color::rgb(0, 163, 255));
        self.settings_button_square.as_mut().unwrap().set_position(offset + Vector2f::new(offset.x +(size * 2.0), 0.0)); 


        self.settings_button_text = Some(Text::new("SETTINGS", self.settings_button_font, ((15 as f32/25 as f32)*size) as u32 ));
        self.settings_button_text.as_mut().unwrap().set_outline_thickness(0.7);
        self.settings_button_text.as_mut().unwrap().set_outline_color(Color::WHITE);
        self.settings_button_text.as_mut().unwrap().set_fill_color(Color::WHITE);

        let origin = Vector2f::new(self.settings_button_text.as_ref().unwrap().global_bounds().width / 2.0, self.settings_button_text.as_ref().unwrap().global_bounds().height / 2.0); 
        self.settings_button_text.as_mut().unwrap().set_origin(origin); 
        self.settings_button_text.as_mut().unwrap().set_rotation(45.0);
        self.settings_button_text.as_mut().unwrap().set_position(self.settings_button_square.as_ref().unwrap().position() + Vector2f::new(size, size)); 


    }
    pub fn pause_button_clicked(&self, mouse_position: Vector2i) -> bool {
        let mouse_position = Vector2f::new(mouse_position.x as f32, mouse_position.y as f32);
        self.pause_button_hit_box.as_ref().unwrap().global_bounds().contains(mouse_position)
    }
    pub fn settings_button_clicked(&self, mouse_position: Vector2i) -> bool {
        let mouse_position = Vector2f::new(mouse_position.x as f32, mouse_position.y as f32);
        self.settings_button_square.as_ref().unwrap().global_bounds().contains(mouse_position)
    }
    pub fn handle_drawing(&mut self, game_state: &GameStates, render_window: &mut RenderWindow ) {
        if game_state == &GameStates::On {
            render_window.draw(&self.pause_button.as_ref().unwrap()[0]);
            render_window.draw(&self.pause_button.as_ref().unwrap()[1]);
        } else if game_state == &GameStates::Paused {
            render_window.draw(self.get_play_button_ref());
        }
        render_window.draw(self.get_settings_button_square_ref());
        render_window.draw(self.get_settings_button_text_ref());
    }
}
