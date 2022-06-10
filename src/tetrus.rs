use std::fmt::Debug;

use macroquad::prelude::Color;

use crate::constants::{GRID_HEIGHT, GRID_WIDTH};
use crate::constants::{YELLOW, CYAN, RED, GREEN, ORANGE, PINK, PURPLE, BLACK};

#[derive(Clone, Copy)]
pub struct Block {
    pub is_populated: bool,
    pub is_locked: bool,
    pub color: Color,
}

impl Block {
    fn new(color: Color) -> Block {
        Block {
            is_populated: true,
            is_locked: false,
            color,
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        Block {
            is_populated: false,
            is_locked: false,
            color: BLACK,
        }
    }
}

pub enum BlockType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
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

    pub fn spawn_block(&mut self, block: BlockType) {
        match block {
            BlockType::I => self.create_block(5, 15, 25, 35, CYAN),
            BlockType::J => self.create_block(15, 25, 34, 35, PINK),
            BlockType::L => self.create_block(14, 24, 34, 35, ORANGE),
            BlockType::O => self.create_block(24, 25, 34, 35, YELLOW),
            BlockType::S => self.create_block(14, 24, 25, 35, RED),
            BlockType::T => self.create_block(14, 24, 25, 34, PURPLE),
            BlockType::Z => self.create_block(15, 24, 25, 34, GREEN),
        }
    }

    pub fn lock_grid(&mut self) {
        for i in 0..(GRID_WIDTH * GRID_HEIGHT) {
            if self.grid[i].is_populated {
                self.grid[i].is_locked = true;
            }
        }
    }

    pub fn check_collision(&mut self) {
        for i in (0..(GRID_WIDTH * GRID_HEIGHT)).rev() {
            if self.grid[i].is_populated && !self.grid[i].is_locked {
                if i > (GRID_WIDTH * GRID_HEIGHT) - GRID_WIDTH
                    || (i < self.grid.len() - 11
                        && self.grid[i + 10].is_locked
                        && self.grid[i + 10].is_populated)
                {
                    self.lock_grid();

                    return;
                }
            }
        }
    }

    pub fn move_unlocked(&mut self) {
        for i in (0..(GRID_WIDTH * GRID_HEIGHT)).rev() {
            if self.grid[i].is_populated && !self.grid[i].is_locked {
                self.grid[i + 10] = self.grid[i];
                self.grid[i] = Block::default();
            }
        }
    }

    pub fn is_game_over(&self) -> bool {
        for i in 0..(GRID_WIDTH * 4) {
            if self.grid[i].is_locked {
                return true;
            }
        }
        false
    }

    fn get_active_blocks(&self) -> Vec<usize> {
        let mut t_vec = Vec::new();
        for i in 0..(GRID_WIDTH * GRID_HEIGHT) {
            if self.grid[i].is_populated && !self.grid[i].is_locked {
                t_vec.push(i);
            }
        }

        t_vec
    }

    fn is_colliding_left(&self, blocks: Vec<usize>) -> bool {
        println!("{:?}", blocks);
        for i in blocks {
            if self.grid[i - 1].is_populated && i % 10 != 1 {
                println!("returned true from collision");
                return true;
            }
        }
        println!("returned false from collision");
        false
    }

    pub fn player_move_left(&mut self) {
        if self.is_colliding_left(self.get_active_blocks()) {
            for i in self.get_active_blocks() {
                if i != self.grid.len() - 1 {
                    self.grid[i - 1] = self.grid[i];
                    self.grid[i] = Block::default();
                }
            }
            println!("Moved left");
        }
    }

    fn is_colliding_right(&self, blocks: Vec<usize>) -> bool {
        println!("{:?}", blocks);
        for i in blocks {
            if i != self.grid.len() - 1 && self.grid[i + 1].is_populated && i % 10 != 8 {
                println!("returned true from collision");
                return true;
            }
        }
        println!("returned false from collision");
        false
    }

    pub fn player_move_right(&mut self) {
        if self.is_colliding_right(self.get_active_blocks()) {
            for i in self.get_active_blocks().into_iter().rev() {
                if i != self.grid.len() - 1 {
                    self.grid[i + 1] = self.grid[i];
                    self.grid[i] = Block::default();
                }
            }
        }
        println!("Moved right");
    }
}

//-----------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------

impl Debug for Tetrus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        let mut accum = 0;
        result += "Next Step: \n";
        for i in 0..GRID_WIDTH * GRID_HEIGHT {
            if i == 40 {
                result += "----------\n";
            }
            if self.grid[i].is_populated {
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

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_populated {
            write!(f, "1")
        } else {
            write!(f, "0")
        }
    }
}