use ::rand::Rng;
use macroquad::prelude::scene::clear;
use macroquad::prelude::*;

use constants::{
    ACTIVE_GRID_OFFSET, BLOCK_SIZE, DISPLAY_HEIGHT, DISPLAY_PADDING, DISPLAY_WIDTH, ICON,
};
use tetrus::Tetrus;
use tetrus::Direction;

mod constants;
mod icons;
mod tetrus;

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

fn draw(tetrus: &Tetrus) {
    for block in &tetrus.active {
        if block.position.y > 3 {
            draw_rectangle(
                (block.position.x as f32 * BLOCK_SIZE) + DISPLAY_PADDING,
                (block.position.y as f32 * BLOCK_SIZE) + DISPLAY_PADDING - (4.0 * BLOCK_SIZE),
                BLOCK_SIZE,
                BLOCK_SIZE,
                block.color,
            )
        }
    }
    for block in &tetrus.inactive {
        if block.position.y > 3 {
            draw_rectangle(
                (block.position.x as f32 * BLOCK_SIZE) + DISPLAY_PADDING,
                (block.position.y as f32 * BLOCK_SIZE) + DISPLAY_PADDING - (4.0 * BLOCK_SIZE),
                BLOCK_SIZE,
                BLOCK_SIZE,
                block.color,
            )
        }
    }

    for i in 0..11 {
        draw_line(
            (i as f32 * BLOCK_SIZE) + DISPLAY_PADDING,
            DISPLAY_PADDING,
            (i as f32 * BLOCK_SIZE) + DISPLAY_PADDING,
            DISPLAY_HEIGHT - DISPLAY_PADDING,
            1.0,
            WHITE,
        );
    }
    for i in 0..21 {
        draw_line(
            DISPLAY_PADDING,
            (i as f32 * BLOCK_SIZE) + DISPLAY_PADDING,
            DISPLAY_WIDTH - DISPLAY_PADDING,
            (i as f32 * BLOCK_SIZE) + DISPLAY_PADDING,
            1.0,
            WHITE,
        );
    }
}

fn player_input(tetrus: &mut Tetrus) {
    if is_key_pressed(KeyCode::Escape) {
        std::process::exit(0);
    } else if is_key_pressed(KeyCode::A) {
        tetrus.player_move(Direction::Left);
    } else if is_key_pressed(KeyCode::D) {
        tetrus.player_move(Direction::Right);
    } else if is_key_pressed(KeyCode::Space) && tetrus.is_active() {
        tetrus.player_move(Direction::Drop);
    }
}

async fn run() -> bool {
    let mut last_update = get_time();
    let tick = 0.2;
    let mut tetrus = Tetrus::new();

    loop {
        player_input(&mut tetrus);
        if get_time() - last_update > tick {
            last_update = get_time();
            if !tetrus.is_active() {
                tetrus.spawn_block();
            } else {
                tetrus.update_active()
            }
            
            println!("origin: {:?}", tetrus.origin);
            
        }
        draw(&tetrus);
        if tetrus.is_game_over() {
            return false;
        } else {
            clear();
            next_frame().await
        }
    }
}

#[macroquad::main(get_mq_conf)]
async fn main() {
    while run().await {}
}
