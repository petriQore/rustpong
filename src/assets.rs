use macroquad::audio::Sound;
use macroquad::audio;
use macroquad::texture::{load_image, Image};

pub struct Assets {
    pub score: Sound,
    pub hit: Sound,
    pub window_background: Image, 
    pub button_background: Image, 
    pub button_clicked_background: Image
}



pub async fn load_assets() -> Assets {
    let score = audio::load_sound("score.wav").await.unwrap();
    let hit = audio::load_sound("hit.wav").await.unwrap();
    let window_background = load_image("window_background.png").await.unwrap();
    let button_background = load_image("button_background.png").await.unwrap();
    let button_clicked_background = load_image("button_clicked_background.png").await.unwrap();

    Assets { score, hit , window_background, button_background, button_clicked_background}
}
