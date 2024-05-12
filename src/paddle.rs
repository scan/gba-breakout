use agb::display::object::{OamManaged, Object};
use alloc::vec::Vec;

use crate::resources;

const PADDLE_HEIGHT: u16 = 16;
const PADDLE_SEGMENT_WIDTH: u16 = 16;

const PADDLE_MIN_POSITION: u16 = 0;

fn paddle_max_position(paddle_segments: usize) -> u16 {
    (agb::display::WIDTH as u16) - (PADDLE_SEGMENT_WIDTH * ((paddle_segments as u16) + 2))
}

pub struct Paddle<'a> {
    pos_x: u16,
    segments: usize,
    sprites: Vec<Object<'a>>,
}

impl<'a> Paddle<'a> {
    pub fn new(oam: &'a OamManaged, segments: usize) -> Self {
        let mut sprites = Vec::with_capacity(segments + 2);
        sprites.push(oam.object_sprite(resources::SPRITE_PADDLE_END.sprite(0)));
        sprites.push(oam.object_sprite(resources::SPRITE_PADDLE_MID.sprite(0)));
        sprites.push(oam.object_sprite(resources::SPRITE_PADDLE_END.sprite(0)));
        sprites[2].set_hflip(true);

        for sprite in sprites.iter_mut() {
            sprite
                .set_y(agb::display::HEIGHT as u16 - PADDLE_HEIGHT)
                .show();
        }

        let pos_x =
            agb::display::WIDTH as u16 / 2 - (PADDLE_SEGMENT_WIDTH * (segments as u16 + 2) / 2);

        Self {
            pos_x,
            segments,
            sprites,
        }
    }

    pub fn update(&mut self, input: &agb::input::ButtonController) {
        let new_pos_x = (self.pos_x as i32 + (input.x_tri() as i32)).clamp(
            PADDLE_MIN_POSITION as i32,
            paddle_max_position(self.segments) as i32,
        );

        self.pos_x = new_pos_x as u16;

        for (sprite, i) in self.sprites.iter_mut().zip(0..) {
            sprite.set_x((self.pos_x) + (i * PADDLE_SEGMENT_WIDTH));
        }
    }
}
