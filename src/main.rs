use macroquad::{audio::{self, PlaySoundParams, Sound}, prelude::*};
use std::time::{Duration};

struct MyCircle {
    x: f32,
    y: f32,
    r: f32,
    clr: Color,
}

struct MyRectangle {
    x: f32,
    y: f32, 
    w: f32,
    h: f32,
    clr: Color
}

struct Velocity {
    x: f32,
    y: f32
}

fn custom_draw_circle(circle: &MyCircle) -> () {
    let r_au_carré = (circle.r).powf(2.0);
    for x in (circle.x - circle.r) as i32..=(circle.x + circle.r) as i32 {
        for y in (circle.y - circle.r) as i32..=(circle.y + circle.r) as i32 {
            let distance_au_carré = ((x as f32 - circle.x).powf(2.0)) + ((y as f32 - circle.y).powf(2.0));
            if distance_au_carré <= r_au_carré {
                draw_rectangle(x as f32, y as f32, 1.0, 1.0, circle.clr);
            }
        }
    }
}

fn custom_draw_rect(rect: &MyRectangle) -> () {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, rect.clr);
}

fn bounce(circle: &mut MyCircle, bar1: &MyRectangle, bar2: &MyRectangle, velocity: &mut Velocity, game_state_paused: &mut bool, pause_duration: &mut Duration, sound: &Sound, hit: &Sound) -> String {

    let mut scoring_message = String::new(); 

    let touching_bar1_from_sides =  circle.x-circle.r <= bar1.x+bar1.w && circle.y-circle.r <= bar1.y+bar1.h && circle.y+circle.r >= bar1.y;
    let touching_bar2_from_sides =  circle.x+circle.r >= bar2.x && circle.y-circle.r <= bar2.y+bar2.h && circle.y+circle.r >= bar2.y;
    
    //top/bottom collision is broken and im not tryna deal with that ✌️💔
    let touching_bar_from_tops = circle.x-circle.r < bar1.x+bar1.w && (circle.y-circle.r == bar1.y+bar1.h || circle.y+circle.r == bar1.y); 

    if circle.y - circle.r <= 0.0 || circle.y + circle.r >= screen_height() || touching_bar_from_tops {
        velocity.y = -velocity.y;
    }
    if touching_bar1_from_sides || touching_bar2_from_sides {
        velocity.x = -velocity.x; 
        velocity.x *= 1.1;
        velocity.y *= 1.1; 
        audio::play_sound_once(hit);

    }

    circle.x += velocity.x;
    circle.y += velocity.y;
    velocity.x += 0.001;
    velocity.y += 0.001; 

    if circle.x - circle.r <= 0.0 {
        audio::play_sound_once(sound);
        scoring_message = reset_ball_place(circle, velocity, false, game_state_paused, pause_duration);

    };

    if circle.x + circle.r >= screen_width() {
        audio::play_sound_once(sound);
        scoring_message = reset_ball_place(circle, velocity, true, game_state_paused, pause_duration);
    };

    return scoring_message;
}

fn reset_ball_place(circle: &mut MyCircle, velocity: &mut Velocity,  side: bool, game_state_paused: &mut bool, pause_duration: &mut Duration) -> String {
    circle.x = screen_width()/2.0;
    circle.y = screen_height()/2.0;

    *game_state_paused = true;
    *pause_duration = Duration::new(3, 0);

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

#[macroquad::main("BasicShapes")]
    async fn main() { 
        set_fullscreen(true);
        // next_frame().await;

        set_pc_assets_folder("assets");
        let score = audio::load_sound("score.wav").await.unwrap();
        let hit = audio::load_sound("hit.wav").await.unwrap();

        let mut scoring_message = String::new();

        let mut ball =MyCircle{
            x: screen_width()/2.0,
            y: screen_height()/2.0,
            r: 15.0,
            clr: WHITE
        };

        let mut bar1 = MyRectangle{
            x: -5.0, // temporary fix for top collision
            y: screen_height()/2.0,
            w: 10.0,
            h: 150.0,
            clr: WHITE 
        };

        let mut bar2 = MyRectangle{
            x: screen_width() ,
            y: screen_height()/2.0,
            w: 10.0,
            h: 150.0,
            clr: WHITE 
        };

        let mut v = Velocity{
            x: -2.0,
            y: 2.0 
        };
        

        let mut paused = false;
        let mut pause_duration = Duration::new(0, 0);

        loop{
        clear_background(BLACK);
        bar2.x = screen_width() - bar2.w + 5.0;

        if paused {
                draw_text(&scoring_message, screen_width()/2.0-210.0, 120.0, 55.0, RED);
                pause_duration = if pause_duration > Duration::from_millis(16) {
                pause_duration - Duration::from_millis(16) 
            } else {
                paused = false;
                Duration::new(0, 0)
            };
        } else {
            custom_draw_circle(&ball);    
            scoring_message = bounce(&mut ball, &bar1, &bar2, &mut v, &mut paused, &mut pause_duration, &score, &hit);
        }
            
        custom_draw_rect(&bar1);
        custom_draw_rect(&bar2);

        if is_key_down(KeyCode::F) && bar1.y <= screen_height()-bar1.h {
            bar1.y += 3.0;
        };
        if is_key_down(KeyCode::R) && bar1.y >= 0.0{
            bar1.y += -3.0;
        };

        if is_key_down(KeyCode::Escape){
            break;
        };

        if is_key_down(KeyCode::Semicolon) && bar2.y <= screen_height()-bar2.h {
            bar2.y += 3.0;
        };
        if is_key_down(KeyCode::P) && bar2.y >= 0.0{
            bar2.y += -3.0;
        };

        next_frame().await
}
}
