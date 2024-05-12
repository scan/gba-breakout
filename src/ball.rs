use agb::{
    display::object::{OamManaged, Object},
    fixnum::{num, Rect, Vector2D},
};

use crate::{collidable::Collidable, resources, util::Floating};

const BALL_WIDTH: i32 = 16;
const BALL_HEIGHT: i32 = 16;

pub struct Ball<'a> {
    pos: Vector2D<Floating>,
    velocity: Vector2D<Floating>,
    sprite: Object<'a>,
}

impl<'a> Ball<'a> {
    pub fn new(oam: &'a OamManaged) -> Self {
        let mut sprite = oam.object_sprite(resources::SPRITE_BALL.sprite(0));
        sprite.show();

        Self {
            pos: Vector2D::new(
                (agb::display::WIDTH / 2 - BALL_WIDTH / 2).into(),
                (agb::display::HEIGHT / 2 - BALL_HEIGHT / 2).into(),
            ),
            velocity: Vector2D::new(num!(1.0), num!(1.0)),
            sprite,
        }
    }

    pub fn update(&mut self) {
        self.pos += self.velocity;

        if (self.pos.x + BALL_WIDTH) >= agb::display::WIDTH.into() {
            self.velocity.x = -self.velocity.x;
            self.pos.x = (agb::display::WIDTH - BALL_WIDTH).into();
        } else if self.pos.x <= num!(0.0) {
            self.velocity.x = -self.velocity.x;
            self.pos.x = num!(0.0);
        }

        if self.pos.y <= num!(0.0) {
            self.velocity.y = -self.velocity.y;
            self.pos.y = num!(0.0);
        }

        self.sprite.set_position(self.pos.trunc());
    }

    pub fn bounce_object<T: Collidable>(&mut self, other: &T) {
        if !self.collides(other) {
            return;
        }

        let ball_center = self.center();
        let rect_center = other.center();

        let dx = ball_center.x - rect_center.x;
        let dy = ball_center.y - rect_center.y;

        let (mut x, mut y) = (self.velocity.x, self.velocity.y);

        if dx.abs() > dy.abs() {
            x = -x;
        } else {
            y = -y;
        }

        self.velocity = Vector2D::new(x, y);
    }
}

impl<'a> Collidable for Ball<'a> {
    fn rect(&self) -> Rect<i32> {
        Rect::new(self.pos.trunc(), Vector2D::new(BALL_WIDTH, BALL_HEIGHT))
    }
}
