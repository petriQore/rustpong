use macroquad::audio::Sound;
use macroquad::audio;

pub struct Assets {
    pub score: Sound,
    pub hit: Sound,
}



pub async fn load_assets() -> Assets {
    let score = audio::load_sound("score.wav").await.unwrap();
    let hit = audio::load_sound("hit.wav").await.unwrap();


    Assets { score, hit }
}
