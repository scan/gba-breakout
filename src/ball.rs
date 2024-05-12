use agb::{
    display::object::{OamManaged, Object},
    fixnum::{num, Rect, Vector2D},
    rng,
};

use crate::{collidable::Collidable, resources, util::Floating};

const BALL_WIDTH: i32 = 16;
const BALL_HEIGHT: i32 = 16;

pub struct Ball<'a> {
    pos: Vector2D<Floating>,
    velocity: Vector2D<Floating>,
    sprite: Object<'a>,
    has_touched: bool,
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
            velocity: Vector2D::new(num!(0.0), num!(0.0)),
            sprite,
            has_touched: false,
        }
    }

    pub fn start(&mut self) {
        let x = Floating::from(rng::gen()) / Floating::from(i32::MAX);
        let y = Floating::from(rng::gen()) / Floating::from(i32::MAX);
        self.velocity = Vector2D::new(x, y).fast_normalise();
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
            self.has_touched = false;
            return;
        }

        // Only bounce once per collision
        if self.has_touched {
            return;
        }

        self.has_touched = true;

        let ball_center = self.center();
        let rect_center = other.center();
        let rect_size: Vector2D<Floating> = other.rect().size.into();

        let dist: Vector2D<Floating> = (ball_center - rect_center).into();
        let (mut x, mut y) = (self.velocity.x, self.velocity.y);

        if dist.x.abs() > dist.y.abs() {
            let bounce_angle =
                (dist.y / (rect_size.y / 2)) * num!(0.25) * num!(3.14159265358979323846);
            x = Floating::cos(bounce_angle);
        } else {
            let bounce_angle =
                (dist.x / (rect_size.x / 2)) * num!(0.25) * num!(3.14159265358979323846);
            y = Floating::sin(bounce_angle);
        }

        self.velocity = Vector2D::new(x, y).normalise();
    }

    pub fn out_of_bounds(&self) -> bool {
        self.pos.y >= agb::display::HEIGHT.into()
    }

    pub(crate) fn reset(&mut self) {
        self.pos = Vector2D::new(
            (agb::display::WIDTH / 2 - BALL_WIDTH / 2).into(),
            (agb::display::HEIGHT / 2 - BALL_HEIGHT / 2).into(),
        );
        self.velocity = Vector2D::new(num!(0.0), num!(0.0));
    }
}

impl<'a> Collidable for Ball<'a> {
    fn rect(&self) -> Rect<i32> {
        Rect::new(self.pos.trunc(), Vector2D::new(BALL_WIDTH, BALL_HEIGHT))
    }
}
