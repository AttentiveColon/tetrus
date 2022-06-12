use std::fmt::Debug;

use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use macroquad::prelude::Color;

//use crate::constants::{GRID_HEIGHT, GRID_WIDTH};
use crate::constants::{YELLOW, CYAN, RED, GREEN, ORANGE, PINK, PURPLE, BLACK};
use crate::constants::{IBLOCK, JBLOCK, LBLOCK, OBLOCK, SBLOCK, TBLOCK, ZBLOCK};

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    fn new(pos: (usize, usize)) -> Self {
        Position { x: pos.0, y: pos.1, }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub position: Position,
    pub color: Color,
}

impl Block {
    fn new(position: Position, color: Color) -> Block {
        Block {
            position,
            color,
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        Block {
            position: Position::default(),
            color: BLACK,
        }
    }
}

pub struct Tetrus {
    pub active: Vec<Block>,
    pub inactive: Vec<Block>,
    pub rng: ThreadRng,
}

impl Tetrus {
    pub fn new() -> Self {
        Tetrus {
            active: Vec::new(),
            inactive: Vec::new(),
            rng: thread_rng(),
        }
    }

    pub fn create_block(&mut self, color: Color, blocks: [(usize, usize); 4]) {
        for block in blocks {
            let position = Position::new(block);
            self.active.push(Block {
                position,
                color,
            } )
        }
    }

    pub fn spawn_block(&mut self) {
        match self.rng.gen_range(0..7) {
            0 => self.create_block(CYAN, IBLOCK),
            1 => self.create_block(PINK, JBLOCK),
            2 => self.create_block(ORANGE, LBLOCK),
            3 => self.create_block(YELLOW, OBLOCK),
            4 => self.create_block(RED, SBLOCK),
            5 => self.create_block(PURPLE, TBLOCK),
            6 => self.create_block(GREEN, ZBLOCK),
            _ => panic!("Invalid range generated: tetrus.spawn_block()"),
        }
    }

    pub fn is_active(&self) -> bool {
        if self.active.is_empty() {
            return false
        }
        true
    }

    pub fn change_status(&mut self) {
        while !self.active.is_empty() {
            self.inactive.push(self.active.pop().unwrap());
        }
    }

    pub fn down_collision(&mut self) -> bool {
        for block in &self.active {
            if block.position.y >= 23 {
                return true;
            } 
            for col_block in &self.inactive {
                if block.position == Position::new((col_block.position.x, col_block.position.y - 1)) {
                    return true;
                }
            }
        }
        false
    }
    
    pub fn left_collision(&mut self) -> bool {
        for block in &self.active {
            if block.position.x == 0 {
                return true;
            }
            for col_block in &self.inactive {
                if block.position == Position::new((col_block.position.x + 1, col_block.position.y)) {
                    return true;
                }
            }
        }
        false
    }

    pub fn right_collision(&mut self) -> bool {
        for block in &self.active {
            if block.position.x == 9 {
                return true;
            }
            for col_block in &self.inactive {
                if block.position == Position::new((col_block.position.x.saturating_sub(1), col_block.position.y)) {
                    return true;
                }
            }
        }
        false
    }

    pub fn move_active(&mut self) {
        for mut block in &mut self.active {
            block.position.y += 1;
        }
    }

    pub fn check_clear(&mut self) -> bool {
        for i in 0..24 {
            let count = self.inactive.iter().filter(|n| n.position.y == i).count();
            println!("Count for row {}: {}", i, count);
            if count == 10 {
                let mut temp_inactive: Vec<Block> = self.inactive.iter().cloned().filter(|n| n.position.y != i).collect();
                for val in &mut temp_inactive {
                    if val.position.y < i {
                        val.position.y += 1;
                    }
                }
                self.inactive = temp_inactive;
            }
        }
        false
    }

    pub fn update_active(&mut self) {
        if !self.down_collision() {
            self.move_active();
        } else {
            self.change_status();
            self.check_clear();
        }
    }

    pub fn move_left(&mut self) {
        if !self.left_collision() {
            for mut block in &mut self.active {
                block.position.x -= 1;
            }
        }
    }

    pub fn move_right(&mut self) {
        if !self.right_collision() {
            for mut block in &mut self.active {
                block.position.x += 1;
            }
        }
    }

    pub fn drop_block(&mut self) {
        while !self.down_collision() {
            self.move_active();
        }
    }

    pub fn is_game_over(&mut self) -> bool {
        for block in &self.inactive {
            if block.position.y <= 3 {
                return true;
            }
        }
        false
    }

    
}

//-----------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------

