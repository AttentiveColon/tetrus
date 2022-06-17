use macroquad::prelude::*;
use constants::*;
use game::*;

mod game;
mod tetrus;
mod sounds;
mod constants;
mod icons;

fn get_mq_conf() -> macroquad::prelude::Conf {
    macroquad::prelude::Conf {
        window_title: String::from("Tetrus"),
        window_height: DISPLAY_HEIGHT as i32,
        window_width: DISPLAY_WIDTH as i32,
        fullscreen: false,
        window_resizable: false,
        icon: Some(ICON),
        ..Default::default()
    }
}

#[macroquad::main(get_mq_conf)]
async fn main() {
    let mut game = Game::new().await;
    while game.run().await {}
}
