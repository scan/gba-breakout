use agb::{
    display::object::{Graphics, Tag}, include_aseprite
};

const SPRITES: &Graphics = include_aseprite!("gfx/sprites.aseprite");

pub const SPRITE_PADDLE_END: &Tag = &SPRITES.tags().get("paddle end");
pub const SPRITE_PADDLE_MID: &Tag = &SPRITES.tags().get("paddle mid");

pub const SPRITE_BALL: &Tag = &SPRITES.tags().get("ball");
