use crate::miniquad::conf::Icon;
use macroquad::prelude::Color;
use macroquad::color_u8;

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