use crate::constants::*;
use crate::tetrus::*;
use macroquad::prelude::*;
use macroquad::audio::*;

pub async fn play_background_music() {
    let sound = load_sound("audio/tetrus_background.wav").await.unwrap();
    play_sound(sound, BACKGROUND_SOUND_PARAMS);
}

pub enum State {
    Welcome,
    Running,
    GameOver,
}

pub struct Game {
    tetrus: Tetrus,
    time: f64,
    state: State,
}

impl Game {
    pub async fn new() -> Self {
        play_background_music().await;
        Game {
            tetrus: Tetrus::new().await,
            time: 0.0,
            state: State::Welcome,
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
        draw_text(
            format!("{:05}", self.tetrus.score).as_ref(),
            5.0,
            40.0,
            50.0,
            WHITE,
        );
    }

    fn draw_time(&mut self) {
        draw_text(
            format!("{:04}", self.time as u64).as_ref(),
            screen_width() - 100.0,
            40.0,
            50.0,
            WHITE,
        );
    }

    fn player_input(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }
        if self.tetrus.is_active() {
            if is_key_pressed(KeyCode::A) {
                self.tetrus.player_move(Movement::Left);
            } else if is_key_pressed(KeyCode::D) {
                self.tetrus.player_move(Movement::Right);
            } else if is_key_pressed(KeyCode::Space) {
                self.tetrus.player_move(Movement::Drop);
                play_sound(self.tetrus.sounds[0], SOUND_PARAMS)
            } else if is_key_pressed(KeyCode::W) {
                self.tetrus.player_move(Movement::Rotate);
                play_sound(self.tetrus.sounds[1], SOUND_PARAMS);
            }
        }
        if is_key_pressed(KeyCode::S) {
            self.tetrus.swap_tick();
        } else if is_key_released(KeyCode::S) {
            self.tetrus.swap_tick();
        }
    }

    pub async fn running(&mut self) {
        let mut last_update = get_time();
        let start_time = get_time();

        loop {
            self.time = get_time() - start_time;

            self.player_input();

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
                self.state = State::GameOver;
                return;
            }
            next_frame().await
        }
    }

    pub async fn welcome(&mut self) {
        let tetrus_size = measure_text(TETRUS_TEXT, Some(Font::default()), 100, 1.0);
        let space_size = measure_text(SPACE_TEXT, Some(Font::default()), 40, 1.0);

        draw_text(
            TETRUS_TEXT,
            screen_width() / 2.0 - tetrus_size.width / 2.0,
            screen_height() / 2.0 - tetrus_size.height / 2.0,
            100.0,
            WHITE,
        );
        draw_text(
            SPACE_TEXT,
            screen_width() / 2.0 - space_size.width / 2.0,
            screen_height() / 2.0 - space_size.height / 2.0 + tetrus_size.height / 2.0,
            40.0,
            WHITE,
        );

        if is_key_pressed(KeyCode::Space) {
            self.state = State::Running;
        } else if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }
        next_frame().await;
    }

    pub async fn game_over(&mut self) {
        let game_over_size = measure_text(GAME_OVER_TEXT, Some(Font::default()), 100, 1.0);
        let score_size = measure_text(SCORE_TEXT_PLACEHOLDER, Some(Font::default()), 40, 1.0);
        let space_size = measure_text(SPACE_TEXT, Some(Font::default()), 20, 1.0);

        draw_text(
            GAME_OVER_TEXT,
            screen_width() / 2.0 - game_over_size.width / 2.0,
            screen_height() / 2.0 - game_over_size.height / 2.0,
            100.0,
            WHITE,
        );
        draw_text(
            format!("Score: {:05}", self.tetrus.score).as_ref(),
            screen_width() / 2.0 - score_size.width / 2.0,
            screen_height() / 2.0 - score_size.height / 2.0 + game_over_size.height / 2.0,
            40.0,
            WHITE,
        );
        draw_text(
            SPACE_TEXT,
            screen_width() / 2.0 - space_size.width / 2.0,
            screen_height() / 2.0 - space_size.height / 2.0 + (game_over_size.height / 2.0) * 2.0,
            20.0,
            WHITE,
        );
        if is_key_pressed(KeyCode::Space) {
            self.state = State::Running;
            self.tetrus = Tetrus::new().await;
        } else if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }
        next_frame().await
    }

    pub async fn run(&mut self) -> bool {
        match self.state {
            State::Welcome => self.welcome().await,
            State::Running => self.running().await,
            State::GameOver => self.game_over().await,
        }
        true
    }
}
