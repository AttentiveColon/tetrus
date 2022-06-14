use macroquad::prelude::*;
use crate::constants::*;
use crate::tetrus::*;

pub struct Game {
    tetrus: Tetrus,
    time: f64,
    _is_running: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            tetrus: Tetrus::new(),
            time: 0.0,
            _is_running: true,
        }
    }
    
    fn draw_board(&mut self) {
        for block in &self.tetrus.active {
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
        for block in &self.tetrus.inactive {
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

    fn draw_score(&mut self) {
        draw_text(format!("{}", self.tetrus.score).as_ref(), 5.0, 40.0, 50.0, WHITE);
    }

    fn draw_time(&mut self) {
        draw_text(format!("{}", self.time as u64).as_ref(), screen_width() - 80.0, 40.0, 50.0, WHITE);
    }

    fn player_input(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        } else if is_key_pressed(KeyCode::A) {
            self.tetrus.player_move(Movement::Left);
        } else if is_key_pressed(KeyCode::D) {
            self.tetrus.player_move(Movement::Right);
        } else if is_key_pressed(KeyCode::Space) {
            self.tetrus.player_move(Movement::Drop);
        } else if is_key_pressed(KeyCode::S) {
            self.tetrus.player_move(Movement::Rotate);
        }
    }
    
    pub async fn run(&mut self) -> bool {
        let mut last_update = get_time();
        let start_time = get_time();
    
        loop {
            self.time = get_time() - start_time;
            if self.tetrus.is_active() {
                self.player_input();
            }
            if get_time() - last_update > self.tetrus.tick {
                last_update = get_time();
                if !self.tetrus.is_active() {
                    self.tetrus.spawn_block();
                } else {
                    self.tetrus.update_active();
                }
            }
            self.draw_board();
            self.draw_score();
            self.draw_time();
            if self.tetrus.is_game_over() {
                return false;
            } else {
                next_frame().await
            }
        }
    }
}