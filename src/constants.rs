use crate::miniquad::conf::Icon;
use macroquad::prelude::*;

use crate::icons::{ICON_SMALL, ICON_MEDIUM, ICON_BIG};

pub const ICON: Icon = Icon {
    small: ICON_SMALL,
    medium: ICON_MEDIUM,
    big: ICON_BIG,
};

pub const GRID_WIDTH: usize = 10;
pub const GRID_HEIGHT: usize = 24;
pub const DISPLAY_PADDING: f32 = 100.0;
pub const DISPLAY_WIDTH: f32 = 600.0;
pub const DISPLAY_HEIGHT: f32 = 1000.0;
pub const BLOCK_SIZE: f32 = 40.0;

pub const YELLOW: Color = color_u8!(0xfa, 0xff, 0x00, 0xff); //faff00
pub const CYAN: Color = color_u8!(0x00, 0xe4, 0xff, 0xff); //00e4ff
pub const RED: Color = color_u8!(0xf6, 0x00, 0x00, 0xff); //f60000
pub const GREEN: Color = color_u8!(0x69, 0xb6, 0x25, 0xff); //69b625
pub const ORANGE: Color = color_u8!(0xff, 0x8d, 0x00, 0xff); //ff8d00
pub const PINK: Color = color_u8!(0xff, 0x51, 0xbc, 0xff); //ff51bc
pub const PURPLE: Color = color_u8!(0x9f, 0x00, 0x96, 0xff); //9f0096
pub const _WHITE: Color = color_u8!(0xff, 0xff, 0xff, 0xff); //ffffff
pub const BLACK: Color = color_u8!(0x00, 0x00, 0x00, 0xff); //000000

pub const IBLOCK: [(usize, usize); 4] = [(5, 0), (5, 1), (5, 2), (5, 3)];
pub const JBLOCK: [(usize, usize); 4] = [(5, 1), (5, 2), (4, 3), (5, 3)];
pub const LBLOCK: [(usize, usize); 4] = [(4, 1), (4, 2), (4, 3), (5, 3)];
pub const OBLOCK: [(usize, usize); 4] = [(4, 2), (5, 2), (4, 3), (5, 3)];
pub const SBLOCK: [(usize, usize); 4] = [(4, 1), (4, 2), (5, 2), (5, 3)];
pub const TBLOCK: [(usize, usize); 4] = [(4, 1), (4, 2), (5, 2), (4, 3)];
pub const ZBLOCK: [(usize, usize); 4] = [(5, 1), (4, 2), (5, 2), (4, 3)];

pub const IORIGIN: (usize, usize) = (5, 2);
pub const JORIGIN: (usize, usize) = (5, 2);
pub const LORIGIN: (usize, usize) = (4, 2);
pub const OORIGIN: (usize, usize) = (4, 3);
pub const SORIGIN: (usize, usize) = (5, 2);
pub const TORIGIN: (usize, usize) = (4, 2);
pub const ZORIGIN: (usize, usize) = (5, 2);

pub const TETRUS_TEXT: &str = "Tetrus";
pub const SPACE_TEXT: &str = "Press Space";
pub const GAME_OVER_TEXT: &str = "Game Over";
pub const SCORE_TEXT_PLACEHOLDER: &str = "Score: 00000";