use agb::{
    display::object::{OamManaged, Object},
    fixnum::{Vector2D, Rect},
};

use crate::resources;

const BALL_WIDTH: i32 = 16;
const BALL_HEIGHT: i32 = 16;

pub struct Ball<'a> {
    pos: Vector2D<i32>,
    velocity: Vector2D<i32>,
    sprite: Object<'a>,
}

impl<'a> Ball<'a> {
    pub fn new(oam: &'a OamManaged) -> Self {
        let mut sprite = oam.object_sprite(resources::SPRITE_BALL.sprite(0));
        sprite.show();

        Self {
            pos: Vector2D::new(
                agb::display::WIDTH / 2 - BALL_WIDTH / 2,
                agb::display::HEIGHT / 2 - BALL_HEIGHT / 2,
            ),
            velocity: Vector2D::new(1, 1),
            sprite,
        }
    }

    pub fn update(&mut self) {
        self.pos += self.velocity;

        if (self.pos.x + BALL_WIDTH) >= agb::display::WIDTH {
            self.velocity.x = -self.velocity.x;
            self.pos.x = agb::display::WIDTH - BALL_WIDTH;
        } else if self.pos.x <= 0 {
            self.velocity.x = -self.velocity.x;
            self.pos.x = 0;
        }

        if (self.pos.y + BALL_HEIGHT) >= agb::display::HEIGHT {
            self.velocity.y = -self.velocity.y;
            self.pos.y = agb::display::HEIGHT - BALL_HEIGHT;
        } else if self.pos.y <= 0 {
            self.velocity.y = -self.velocity.y;
            self.pos.y = 0;
        }

        self.sprite.set_position(self.pos);
    }

    pub fn get_rect(&self) -> Rect<i32> {
        Rect::new(self.pos, Vector2D::new(BALL_WIDTH, BALL_HEIGHT))
    }
}
