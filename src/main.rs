use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, Skin};
use std::time::Duration;

mod helper;
// use helper::*;

mod assets;
use assets::*;

mod init_objects;
use init_objects::*;

const WINNING_SCORE: i32 = 3;
enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameWon,
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

    let Assets {
        score,
        hit,
        window_background,
        button_background,
        button_clicked_background,
    } = load_assets().await;

    let window_style = root_ui()
        .style_builder()
        .background(window_background)
        .background_margin(RectOffset::new(32.0, 76.0, 44.0, 20.0))
        .margin(RectOffset::new(0.0, -40.0, 0.0, 0.0))
        .build();

    let button_style = root_ui()
        .style_builder()
        .background(button_background)
        .background_clicked(button_clicked_background)
        .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
        .margin(RectOffset::new(16.0, 0.0, -8.0, -8.0))
        // .font(&font)
        // .unwrap()
        .text_color(WHITE)
        .font_size(64)
        .build();

    let label_style = root_ui()
        .style_builder()
        // .font(&font)
        // .unwrap()
        .text_color(WHITE)
        .font_size(28)
        .build();

    let ui_skin = Skin {
        window_style,
        button_style,
        label_style,
        ..root_ui().default_skin()
    };
    root_ui().push_skin(&ui_skin);
    let window_size = vec2(370.0, 500.0);

    let Objects {
        mut ball,
        mut bar1,
        mut bar2,
        midfield,
        mut v,
    } = init_objects();

    let mut scoring_message = String::new();

    let mut refreshing = false;
    let mut pause_duration = Duration::new(0, 0);

    let mut player1_score = 0;
    let mut player2_score = 0;

    loop {
        clear_background(BLACK);

        match game_state {
            GameState::Playing => {
                if refreshing {
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
                        refreshing = false;
                        Duration::new(0, 0)
                    };
                } else {
                    ball.custom_draw_circle();
                    scoring_message = ball.bounce(
                        &bar1,
                        &bar2,
                        &mut v,
                        &mut refreshing,
                        &mut pause_duration,
                        &score,
                        &hit,
                        &mut player1_score,
                        &mut player2_score,
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

                if is_key_down(KeyCode::Semicolon) && bar2.y <= screen_height() - bar2.h {
                    bar2.y += 3.0;
                };
                if is_key_down(KeyCode::P) && bar2.y >= 0.0 {
                    bar2.y += -3.0;
                };

                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Paused;
                }

                if player1_score == WINNING_SCORE || player2_score == WINNING_SCORE {
                    game_state = GameState::GameWon;
                }
            }
            GameState::Paused => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
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
                if !refreshing {
                    ball.custom_draw_circle();
                }
            }
            GameState::MainMenu => {
                root_ui().window(
                    hash!(),
                    vec2(
                        screen_width() / 2.0 - window_size.x / 2.0,
                        screen_height() / 2.0 - window_size.y / 2.0,
                    ),
                    window_size,
                    |ui| {
                        ui.label(vec2(115.0, -34.0), "Pong");
                        if ui.button(vec2(65.0, 25.0), "Play") {
                            game_state = GameState::Playing;
                        }
                        if ui.button(vec2(65.0, 125.0), "Quit") {
                            std::process::exit(0);
                        }
                        ui.label(vec2(65.0, 210.0), "Player1 (Left):");
                        ui.label(vec2(65.0, 230.0), "up:R / down:F ");

                        ui.label(vec2(65.0, 280.0), "Player2 (Right):");
                        ui.label(vec2(65.0, 300.0), "up:P / down:M ");

                        ui.label(vec2(65.0, 350.0), "Pause: Space");
                        ui.label(vec2(65.0, 370.0), "Exit: Escape");
                    },
                );
            }
            GameState::GameWon => {
                root_ui().window(
                    hash!(),
                    vec2(
                        screen_width() / 2.0 - window_size.x / 2.0,
                        screen_height() / 2.0 - window_size.y / 2.0,
                    ),
                    window_size,
                    |ui| {
                        ui.label(vec2(115.0, -34.0), "Game Over!");
                        if ui.button(vec2(35.0, 100.0), "Replay") {
                            game_state = GameState::Playing;
                            player1_score = 0;
                            player2_score = 0;
                            refreshing = false;
                        }
                        if ui.button(vec2(35.0, 200.0), "Quit") {
                            std::process::exit(0);
                        }

                        let winner: &str;

                        if player1_score > player2_score {
                            winner = "Player1";
                        } else {
                            winner = "Player2";
                        }
                        ui.label(vec2(35.0, 35.0), &[winner, " is the winner!"].concat());
                    },
                );
            }
        }

        draw_text(
            &player1_score.to_string(),
            screen_width() / 4.0,
            screen_height() / 9.0,
            60.0,
            LIGHTGRAY,
        );
        draw_text(
            &player2_score.to_string(),
            3.0 * screen_width() / 4.0,
            screen_height() / 9.0,
            60.0,
            LIGHTGRAY,
        );

        if is_key_down(KeyCode::Escape) {
            break;
        };

        next_frame().await
    }
}
