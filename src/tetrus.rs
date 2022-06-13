use macroquad::prelude::Color;
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};

//use crate::constants::{GRID_HEIGHT, GRID_WIDTH};
use crate::constants::{
    BLACK, CYAN, GREEN, GRID_HEIGHT, GRID_WIDTH, ORANGE, PINK, PURPLE, RED, YELLOW,
};
use crate::constants::{IBLOCK, JBLOCK, LBLOCK, OBLOCK, SBLOCK, TBLOCK, ZBLOCK};
use crate::constants::{IORIGIN, JORIGIN, LORIGIN, OORIGIN, SORIGIN, TORIGIN, ZORIGIN};

pub enum Movement {
    Left,
    Right,
    Drop,
    Rotate,
}

pub enum Collision {
    Left,
    Right,
    Down,
}

#[derive(Clone, PartialEq)]
pub enum BlockType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

#[derive(Default, Clone, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    fn new(pos: (usize, usize)) -> Self {
        Position { x: pos.0, y: pos.1 }
    }
}

#[derive(Clone)]
pub struct Block {
    pub position: Position,
    pub color: Color,
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
    pub origin: Position,
    pub block_id: BlockType,
    pub rng: ThreadRng,
}

impl Tetrus {
    pub fn new() -> Self {
        Tetrus {
            active: Vec::new(),
            inactive: Vec::new(),
            origin: Position::new((0, 0)),
            block_id: BlockType::I,
            rng: thread_rng(),
        }
    }

    pub fn create_block(&mut self, color: Color, blocks: [(usize, usize); 4], id: BlockType) {
        for block in blocks {
            let position = Position::new(block);
            self.active.push(Block { position, color })
        }
        self.block_id = id.clone();
        match id {
            BlockType::I => self.origin = Position::new(IORIGIN),
            BlockType::J => self.origin = Position::new(JORIGIN),
            BlockType::L => self.origin = Position::new(LORIGIN),
            BlockType::O => self.origin = Position::new(OORIGIN),
            BlockType::S => self.origin = Position::new(SORIGIN),
            BlockType::T => self.origin = Position::new(TORIGIN),
            BlockType::Z => self.origin = Position::new(ZORIGIN),
        }
    }

    pub fn spawn_block(&mut self) {
        match self.rng.gen_range(0..7) {
            0 => self.create_block(CYAN, IBLOCK, BlockType::I),
            1 => self.create_block(PINK, JBLOCK, BlockType::J),
            2 => self.create_block(ORANGE, LBLOCK, BlockType::L),
            3 => self.create_block(YELLOW, OBLOCK, BlockType::O),
            4 => self.create_block(RED, SBLOCK, BlockType::S),
            5 => self.create_block(PURPLE, TBLOCK, BlockType::T),
            6 => self.create_block(GREEN, ZBLOCK, BlockType::Z),
            _ => panic!("Invalid range generated: tetrus.spawn_block()"),
        }
    }

    pub fn is_active(&self) -> bool {
        if self.active.is_empty() {
            return false;
        }
        true
    }

    pub fn change_status(&mut self) {
        while !self.active.is_empty() {
            self.inactive.push(self.active.pop().unwrap());
        }
    }

    pub fn check_collision(&mut self, collision: Collision) -> bool {
        match collision {
            Collision::Left => {
                for block in &self.active {
                    if block.position.x == 0 {
                        return true;
                    }
                    for col_block in &self.inactive {
                        if block.position
                            == Position::new((col_block.position.x + 1, col_block.position.y))
                        {
                            return true;
                        }
                    }
                }
                false
            }
            Collision::Right => {
                for block in &self.active {
                    if block.position.x == 9 {
                        return true;
                    }
                    for col_block in &self.inactive {
                        if block.position
                            == Position::new((
                                col_block.position.x.saturating_sub(1),
                                col_block.position.y,
                            ))
                        {
                            return true;
                        }
                    }
                }
                false
            }
            Collision::Down => {
                for block in &self.active {
                    if block.position.y >= 23 {
                        return true;
                    }
                    for col_block in &self.inactive {
                        if block.position
                            == Position::new((col_block.position.x, col_block.position.y - 1))
                        {
                            return true;
                        }
                    }
                }
                false
            }
        }
    }

    pub fn move_active(&mut self) {
        for mut block in &mut self.active {
            block.position.y += 1;
        }
        self.origin.y += 1;
    }

    pub fn check_clear(&mut self) -> bool {
        for i in 4..GRID_HEIGHT {
            let count = self.inactive.iter().filter(|n| n.position.y == i).count();
            if count == GRID_WIDTH {
                let mut temp_inactive: Vec<Block> = self
                    .inactive
                    .iter()
                    .cloned()
                    .filter(|n| n.position.y != i)
                    .collect();
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
        if !self.check_collision(Collision::Down) {
            self.move_active();
        } else {
            self.change_status();
            self.check_clear();
        }
    }

    pub fn rotate_block(&mut self) {
        let mut active_rotated: Vec<Block> = Vec::new();
        for block in &mut self.active {
            let offset_x: i32 = self.origin.x as i32 - block.position.x as i32;
            let offset_y: i32 = self.origin.y as i32 - block.position.y as i32;
            let x1 = offset_y;
            let y1 = -offset_x;

            let x = self.origin.x as i32 + x1;
            let y = self.origin.y as i32 + y1;

            active_rotated.push(Block {
                position: Position::new((x as usize, y as usize)),
                color: block.color,
            })
        }
        for temp in &active_rotated {
            if temp.position.x >= GRID_WIDTH || temp.position.y >= GRID_HEIGHT {
                return;
            }
            for block in &self.inactive {
                if block.position == temp.position {
                    return;
                }
            }
        }
        self.active = active_rotated;
    }

    pub fn player_move(&mut self, direction: Movement) {
        match direction {
            Movement::Left => {
                if !self.check_collision(Collision::Left) {
                    for mut block in &mut self.active {
                        block.position.x -= 1;
                    }
                    self.origin.x -= 1;
                }
            }
            Movement::Right => {
                if !self.check_collision(Collision::Right) {
                    for mut block in &mut self.active {
                        block.position.x += 1;
                    }
                    self.origin.x += 1;
                }
            }
            Movement::Drop => {
                while !self.check_collision(Collision::Down) {
                    self.move_active();
                }
            }
            Movement::Rotate => {
                if self.block_id == BlockType::O {
                    return;
                } else {
                    self.rotate_block();
                }
            }
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
