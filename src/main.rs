use macroquad::{
    audio::{self, PlaySoundParams, Sound},
    prelude::*,
};
use macroquad::ui::{hash, root_ui, Skin};
use std::time::Duration;

mod helper;
// use helper::*;

mod assets;
use assets::*;

mod init_objects;
use init_objects::*;

enum GameState {
    MainMenu,
    Playing,
    Paused,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        fullscreen: true,
        window_height: 1080,
        window_width: 1920,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::MainMenu;
    set_fullscreen(true);
    // next_frame().await;

    set_pc_assets_folder("assets");

    let Assets { score, hit } = load_assets().await;

    let Objects {
        mut ball,
        mut bar1,
        mut bar2,
        midfield,
        mut v,
    } = init_objects();

    let mut scoring_message = String::new();

    let mut paused = false;
    let mut pause_duration = Duration::new(0, 0);

    loop {
        clear_background(BLACK);

        match game_state {
            GameState::Playing => {
                if paused {
                    draw_text(
                        &scoring_message,
                        screen_width() / 2.0 - 210.0,
                        120.0,
                        55.0,
                        RED,
                    );
                    pause_duration = if pause_duration > Duration::from_millis(16) {
                        pause_duration - Duration::from_millis(16)
                    } else {
                        paused = false;
                        Duration::new(0, 0)
                    };
                } else {
                    ball.custom_draw_circle();
                    scoring_message = ball.bounce(
                        &bar1,
                        &bar2,
                        &mut v,
                        &mut paused,
                        &mut pause_duration,
                        &score,
                        &hit,
                    );
                }
        
                bar1.custom_draw_rect();
                bar2.custom_draw_rect();
                midfield.custom_draw_rect();
        
                if is_key_down(KeyCode::F) && bar1.y <= screen_height() - bar1.h {
                    bar1.y += 3.0;
                };
                if is_key_down(KeyCode::R) && bar1.y >= 0.0 {
                    bar1.y += -3.0;
                };
        
                if is_key_down(KeyCode::Escape) {
                    break;
                };
        
                if is_key_down(KeyCode::Semicolon) && bar2.y <= screen_height() - bar2.h {
                    bar2.y += 3.0;
                };
                if is_key_down(KeyCode::P) && bar2.y >= 0.0 {
                    bar2.y += -3.0;
                };

                if is_key_pressed(KeyCode::Space){
                    game_state=GameState::Paused;
                }
            } 
            GameState::Paused => {
                if is_key_pressed(KeyCode::Space){
                    game_state=GameState::Playing;
                }
                let text = "Paused";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    RED,
                );
                bar1.custom_draw_rect();
                bar2.custom_draw_rect();
                midfield.custom_draw_rect();
                ball.custom_draw_circle();
            }
            GameState::MainMenu => {
                let text = "Main Menu";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 4.0,
                    50.0,
                    RED,
                );
            }
        }
        

        next_frame().await
    }
}
