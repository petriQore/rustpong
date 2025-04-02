use macroquad::prelude::*;
use macroquad::audio::{self, Sound};
use std::time::Duration;

pub struct MyCircle {
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub clr: Color,
   
}

pub struct MyRectangle {
    pub x: f32,
    pub y: f32, 
    pub w: f32,
    pub h: f32,
    pub clr: Color
}

pub struct Velocity {
    pub x: f32,
    pub y: f32
}

impl MyCircle{
    pub fn custom_draw_circle(&self) -> () {
        let r_au_carré = (self.r).powf(2.0);
        for x in (self.x - self.r) as i32..=(self.x + self.r) as i32 {
            for y in (self.y - self.r) as i32..=(self.y + self.r) as i32 {
                let distance_au_carré = ((x as f32 - self.x).powf(2.0)) + ((y as f32 - self.y).powf(2.0));
                if distance_au_carré <= r_au_carré {
                    draw_rectangle(x as f32, y as f32, 1.0, 1.0, self.clr);
                }
            }
        }
    }

    pub fn reset_ball_place(&mut self, velocity: &mut Velocity,  side: bool, game_state_refreshing: &mut bool, refresh_duration: &mut Duration) -> String {
        self.x = screen_width()/2.0;
        self.y = screen_height()/2.0;
    
        *game_state_refreshing = true;
        *refresh_duration = Duration::new(3, 0);
        
        if side {
            velocity.x = 2.0;
            velocity.y = 2.0;
            return "Player 1 scored!".to_string();
    
            
        } else {
            velocity.x = -2.0;
            velocity.y = -2.0;
            return "Player 2 scored!".to_string();
    
        }
    }

    pub fn bounce(&mut self, bar1: &MyRectangle, bar2: &MyRectangle, velocity: &mut Velocity, game_state_refreshing: &mut bool, refresh_duration: &mut Duration, sound: &Sound, hit: &Sound, player1_score: &mut i32, player2_score: &mut i32) -> String {
        // println!("{}",velocity.x);
        // println!("{}",velocity.y);
        let mut scoring_message = String::new(); 
    
        let touching_bar1_from_sides =  self.x-self.r <= bar1.x+bar1.w && self.y-self.r <= bar1.y+bar1.h && self.y+self.r >= bar1.y;
        let touching_bar2_from_sides =  self.x+self.r >= bar2.x && self.y-self.r <= bar2.y+bar2.h && self.y+self.r >= bar2.y;
        
        //top/bottom collision is broken and im not tryna deal with that ✌️💔
        let touching_bar_from_tops = self.x-self.r < bar1.x+bar1.w && (self.y-self.r == bar1.y+bar1.h || self.y+self.r == bar1.y); 
    
        if self.y - self.r <= 0.0 || self.y + self.r >= screen_height() || touching_bar_from_tops {
            if velocity.x.abs() < 10.0 || velocity.y.abs() < 10.0 {
                velocity.x *= 1.03;
                velocity.y *= 1.03; 
            } 
            velocity.y = -velocity.y;
            audio::play_sound_once(hit);
    
        }
        if touching_bar1_from_sides || touching_bar2_from_sides {
            velocity.x = -velocity.x; 
            if velocity.x.abs() < 10.0 || velocity.y.abs() < 10.0 {
                velocity.x *= 1.03;
                velocity.y *= 1.03; 
            }    
            audio::play_sound_once(hit);
    
        }
    
        self.x += velocity.x;
        self.y += velocity.y;
        // velocity.x += 0.001;
        // velocity.y += 0.001; 
    
        if self.x - self.r <= 0.0 {
            audio::play_sound_once(sound);
            scoring_message = self.reset_ball_place( velocity, false, game_state_refreshing, refresh_duration);
            *player2_score += 1;
    
        };
    
        if self.x + self.r >= screen_width() {
            audio::play_sound_once(sound);
            scoring_message = self.reset_ball_place(velocity, true, game_state_refreshing, refresh_duration);
            *player1_score += 1;
        };
    
        return scoring_message;
    }
}    


impl MyRectangle{
        pub fn custom_draw_rect(&self) -> () {
        draw_rectangle(self.x, self.y, self.w, self.h, self.clr);
    }
} 