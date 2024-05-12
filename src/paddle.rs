use alloc::vec::Vec;

use agb::{
    display::object::{OamManaged, Object},
    fixnum::{num, Rect, Vector2D},
};

use crate::{
    collidable::Collidable,
    resources,
    util::{lerp, Floating},
};

const PADDLE_HEIGHT: i32 = 16;
const PADDLE_SEGMENT_WIDTH: i32 = 16;

const PADDLE_MIN_POSITION: i32 = 0;

const PADDLE_SPEED: f32 = 2.0;

pub struct Paddle<'a> {
    pos_x: Floating,
    segments: usize,
    sprites: Vec<Object<'a>>,
}

impl<'a> Paddle<'a> {
    pub fn new(oam: &'a OamManaged, segments: usize) -> Self {
        let mut sprites = Vec::with_capacity(segments + 2);
        sprites.push(oam.object_sprite(resources::SPRITE_PADDLE_END.sprite(0)));
        for _ in 0..segments {
            sprites.push(oam.object_sprite(resources::SPRITE_PADDLE_MID.sprite(0)));
        }
        sprites.push(oam.object_sprite(resources::SPRITE_PADDLE_END.sprite(0)));
        sprites.last_mut().map(|s| s.set_hflip(true));

        for sprite in sprites.iter_mut() {
            sprite
                .set_y((agb::display::HEIGHT - PADDLE_HEIGHT) as u16)
                .show();
        }

        let pos_x =
            (agb::display::WIDTH / 2 - (PADDLE_SEGMENT_WIDTH * (segments as i32 + 2) / 2)).into();

        Self {
            pos_x,
            segments,
            sprites,
        }
    }

    fn max_position(&self) -> i32 {
        agb::display::WIDTH - (PADDLE_SEGMENT_WIDTH * ((self.segments as i32) + 2))
    }

    pub fn update(&mut self, input: &agb::input::ButtonController) {
        let direction = input.x_tri() as i32;
        self.pos_x += lerp(
            num!(0.0),
            Floating::from(direction) * Floating::from_f32(PADDLE_SPEED),
            num!(0.5),
        );
        self.pos_x = self
            .pos_x
            .clamp(PADDLE_MIN_POSITION.into(), self.max_position().into());

        let trunc_pos = self.pos_x.trunc() as u16;
        for (sprite, i) in self.sprites.iter_mut().zip(0..) {
            sprite.set_x(trunc_pos + ((i * PADDLE_SEGMENT_WIDTH) as u16));
        }
    }
}

impl<'a> Collidable for Paddle<'a> {
    fn rect(&self) -> Rect<i32> {
        Rect::new(
            Vector2D::new(self.pos_x.trunc(), agb::display::HEIGHT - PADDLE_HEIGHT),
            Vector2D::new(
                PADDLE_SEGMENT_WIDTH * (self.segments as i32 + 2),
                PADDLE_HEIGHT,
            ),
        )
    }
}

impl<'a> Drop for Paddle<'a> {
    fn drop(&mut self) {
        for sprite in self.sprites.iter_mut() {
            sprite.hide();
        }
    }
}
