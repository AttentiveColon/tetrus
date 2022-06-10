use ::rand::Rng;
use macroquad::prelude::scene::clear;
use macroquad::prelude::*;

use constants::{BLOCK_SIZE, DISPLAY_HEIGHT, DISPLAY_PADDING, DISPLAY_WIDTH, ICON};
use tetrus::{BlockType, Tetrus};

mod icons;
mod tetrus;
mod constants;

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

fn get_xy(pos: usize) -> (f32, f32) {
    let x = (pos % 10) as f32 * BLOCK_SIZE + DISPLAY_PADDING;
    let y = (pos / 10) as f32 * BLOCK_SIZE + DISPLAY_PADDING;

    (x, y)
}

fn draw(tetrus: &Tetrus) {
    draw_blocks(tetrus);
    draw_grid();
}

fn draw_blocks(tetrus: &Tetrus) {
    for i in 40..tetrus.grid.len() {
        let (x, y) = get_xy(i - 40);
        let color = tetrus.grid[i].color;
        draw_rectangle(x, y, BLOCK_SIZE, BLOCK_SIZE, color);
    }
}

fn draw_grid() {
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
    if is_key_pressed(KeyCode::A) {
        tetrus.player_move_left();
    } else if is_key_pressed(KeyCode::D) {
        tetrus.player_move_right();
    } else if is_key_pressed(KeyCode::Escape) {
        std::process::exit(0);
    } else if is_key_pressed(KeyCode::O) {
        let num = ::rand::thread_rng().gen_range(1..7);
        match num {
            0 => tetrus.spawn_block(BlockType::I),
            1 => tetrus.spawn_block(BlockType::J),
            2 => tetrus.spawn_block(BlockType::L),
            3 => tetrus.spawn_block(BlockType::O),
            4 => tetrus.spawn_block(BlockType::S),
            5 => tetrus.spawn_block(BlockType::T),
            6 => tetrus.spawn_block(BlockType::Z),
            _ => panic!(),
        }
    }
}

#[macroquad::main(get_mq_conf)]
async fn main() {
    let mut last_update = get_time();
    let tick = 0.2;
    let mut tetrus = Tetrus::new();

    loop {
        player_input(&mut tetrus);

        if get_time() - last_update > tick {
            last_update = get_time();
            tetrus.check_collision();
            tetrus.move_unlocked();
            //println!("{:?}", tetrus);
        }

        draw(&tetrus);

        
        if tetrus.is_game_over() {
            println!("GAME OVER");
            break;
        }
        clear();
        next_frame().await
    }
}
