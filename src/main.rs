use core::time;
use std::default;
use std::time::Duration;
use std::thread::sleep;
use std::fmt::Debug;

use macroquad::prelude::*;
mod icon;
use icon::ICON;

pub const GRID_WIDTH: usize = 10;
pub const GRID_HEIGHT: usize = 20;
pub const DISPLAY_PADDING: u32 = 100;

fn get_mq_conf() -> macroquad::prelude::Conf {
    macroquad::prelude::Conf {
        window_title: String::from("Tetrus"),
        window_height: 1000,
        window_width: 600,
        fullscreen: false,
        window_resizable: false,
        icon: Some(ICON),
        ..Default::default()
    }
}

#[derive(Clone, Copy)]
pub struct Block {
    pub locked: bool,
    //color?
    //position?
}

impl Default for Block {
    fn default() -> Self {
        Block { locked: false }
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.locked {
            write!(f, "1")
        } else {
            write!(f, "0")
        }
    }
}


pub struct Tetrus {
    pub grid: [Block; GRID_WIDTH * GRID_HEIGHT], //top 4 rows are for spawning new blocks off screen, if any block gets locked about row 20 the game ends
}


impl Tetrus {
    pub fn new() -> Self {
        Tetrus { grid: [Block::default();  GRID_WIDTH * GRID_HEIGHT] }
    }
}

impl Debug for Tetrus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        let mut accum = 0;
        result += "Next Step: \n";
        for i in 0..GRID_WIDTH * GRID_HEIGHT {
            if self.grid[i].locked {
                result += "1 ";
            } else {
                result += "0 ";
            }
            accum += 1;
            if accum == 10 {
                result += "\n";
                accum = 0;
            }
        }
        write!(f, "{result}")
    }
}

fn is_running() -> bool {
    if is_key_pressed(KeyCode::Escape) {
        false
    } else {
        true
    }
}



fn step(time_step: Duration, tetrus: &mut Tetrus) {
    sleep(time_step);
    println!("{:?}", tetrus);
    
}

#[macroquad::main(get_mq_conf)]
async fn main() {
    let mut tetrus = Tetrus::new();

    while is_running() {
        step(Duration::from_secs(1), &mut tetrus);
        next_frame().await
    }
}
