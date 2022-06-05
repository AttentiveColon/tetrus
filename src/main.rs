use core::time;
use std::default;
use std::fmt::{write, Debug};
use std::thread::sleep;
use std::time::Duration;

use macroquad::prelude::*;
mod icon;
use icon::ICON;

pub const GRID_WIDTH: usize = 10;
pub const GRID_HEIGHT: usize = 20;
pub const DISPLAY_PADDING: u32 = 100;

fn get_mq_conf() -> macroquad::prelude::Conf {
    macroquad::prelude::Conf {
        window_title: String::from("Tetrus"),
        window_height: 50, //1000,
        window_width: 50,  //600,
        fullscreen: false,
        window_resizable: false,
        icon: Some(ICON),
        ..Default::default()
    }
}

#[derive(Clone, Copy)]
pub struct Block {
    pub populated: bool,
    pub locked: bool,
    //color?
    //position?
}

impl Default for Block {
    fn default() -> Self {
        Block {
            populated: false,
            locked: false,
        }
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.populated {
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
        Tetrus {
            grid: [Block::default(); GRID_WIDTH * GRID_HEIGHT],
        }
    }

    pub fn create_active(&mut self, pos: usize) {
        self.grid[pos] = Block {
            populated: true,
            locked: false,
        }
    }

    pub fn spawn_i(&mut self) {
        self.create_active(5);
        self.create_active(15);
        self.create_active(25);
        self.create_active(35);
    }

    pub fn spawn_j(&mut self) {
        self.create_active(15);
        self.create_active(25);
        self.create_active(34);
        self.create_active(35);
    }

    pub fn spawn_l(&mut self) {
        self.create_active(14);
        self.create_active(24);
        self.create_active(34);
        self.create_active(35);
    }

    pub fn spawn_o(&mut self) {
        self.create_active(24);
        self.create_active(25);
        self.create_active(34);
        self.create_active(35);
    }

    pub fn spawn_s(&mut self) {
        self.create_active(14);
        self.create_active(24);
        self.create_active(25);
        self.create_active(35);
    }

    pub fn spawn_t(&mut self) {
        self.create_active(14);
        self.create_active(24);
        self.create_active(25);
        self.create_active(34);
    }

    pub fn spawn_z(&mut self) {
        self.create_active(15);
        self.create_active(24);
        self.create_active(25);
        self.create_active(34);
    }

    pub fn lock_grid(&mut self) {
        for i in 0..(GRID_WIDTH * GRID_HEIGHT) {
            if self.grid[i].populated {
                self.grid[i].locked = true;
            }
        }
    }

    pub fn check_collision(&mut self) {
        for i in 0..(GRID_WIDTH * GRID_HEIGHT) {
            if self.grid[i].populated && !self.grid[i].locked {
                if i > (GRID_WIDTH * GRID_HEIGHT) - GRID_WIDTH {
                    self.lock_grid();
                    
                    return ();
                }
                else if self.grid[i + 10].populated && self.grid[i + 10].locked {
                    self.lock_grid();
                    
                    return ();
                }
            }
        }
    }

    pub fn move_unlocked(&mut self) {
        for i in (0..(GRID_WIDTH * GRID_HEIGHT)).rev() {
            if self.grid[i].populated && !self.grid[i].locked {
                self.grid[i + 10] = self.grid[i];
                self.grid[i] = Block::default();
            }
        }
    }

    pub fn is_game_over(&self) -> bool {
        for i in 0..(GRID_WIDTH * 4) {
            
            if self.grid[i].locked {
                println!("Work");
                return true
            }
        }
        false
    }
}

impl Debug for Tetrus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        let mut accum = 0;
        result += "Next Step: \n";
        for i in 0..GRID_WIDTH * GRID_HEIGHT {
            if i == 40 {
                result += "- - - - - - - - - - \n";
            }
            if self.grid[i].populated {
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
    tetrus.check_collision();
    tetrus.move_unlocked();
    println!("{:?}", tetrus);
}

#[macroquad::main(get_mq_conf)]
async fn main() {
    let mut tetrus = Tetrus::new();

    while is_running() {
        //testing
        if is_key_pressed(KeyCode::O) {
            tetrus.spawn_i();
        }
        step(Duration::from_millis(200), &mut tetrus);
        if tetrus.is_game_over() {
            println!("HELLO");
            std::process::exit(0);
        }
        next_frame().await
    }
}
