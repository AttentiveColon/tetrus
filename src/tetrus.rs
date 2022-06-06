use std::fmt::Debug;

use crate::constants::GRID_WIDTH;
use crate::constants::GRID_HEIGHT;

use macroquad::color_u8;
use macroquad::prelude::Color;

const YELLOW: Color = color_u8!(0xfa, 0xff, 0x00, 0xff); //faff00
const CYAN: Color = color_u8!(0x00, 0xe4, 0xff, 0xff); //00e4ff
const RED: Color = color_u8!(0xf6, 0x00, 0x00, 0xff); //f60000
const GREEN: Color = color_u8!(0x69, 0xb6, 0x25, 0xff); //69b625
const ORANGE: Color = color_u8!(0xff, 0x8d, 0x00, 0xff); //ff8d00
const PINK: Color = color_u8!(0xff, 0x51, 0xbc, 0xff); //ff51bc
const PURPLE: Color = color_u8!(0x9f, 0x00, 0x96, 0xff); //9f0096
const _WHITE: Color = color_u8!(0xff, 0xff, 0xff, 0xff); //ffffff
const BLACK: Color = color_u8!(0x00, 0x00, 0x00, 0xff); //000000

#[derive(Clone, Copy)]
pub struct Block {
    pub populated: bool,
    pub locked: bool,
    pub color: Color,
}

impl Block {
    fn new(color: Color) -> Block {
        Block { populated: true, locked: false, color: color }
    }
}

impl Default for Block {
    fn default() -> Self {
        Block {
            populated: false,
            locked: false,
            color: BLACK
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

    pub fn create_block(&mut self, p1: usize, p2: usize, p3: usize, p4: usize, color: Color) {
        self.grid[p1] = Block::new(color);
        self.grid[p2] = Block::new(color);
        self.grid[p3] = Block::new(color);
        self.grid[p4] = Block::new(color);
    }

    pub fn spawn_i(&mut self) {
        self.create_block(5, 15, 25, 35, CYAN);
    }

    pub fn spawn_j(&mut self) {
        self.create_block(15, 25, 34, 35, PINK);
    }

    pub fn spawn_l(&mut self) {
        self.create_block(14, 24, 34, 35, ORANGE);
    }

    pub fn spawn_o(&mut self) {
        self.create_block(24, 25, 34, 35, YELLOW);
    }

    pub fn spawn_s(&mut self) {
        self.create_block(14, 24, 25, 35, RED);
    }

    pub fn spawn_t(&mut self) {
        self.create_block(14, 24, 25, 34, PURPLE);
    }

    pub fn spawn_z(&mut self) {
        self.create_block(15, 24, 25, 34, GREEN);
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
                result += "----------\n";
            }
            if self.grid[i].populated {
                result += "█";
            } else {
                result += "░";
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