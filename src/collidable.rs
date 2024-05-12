use agb::fixnum::{Rect, Vector2D};

pub trait Collidable {
    fn rect(&self) -> Rect<i32>;

    fn center(&self) -> Vector2D<i32> {
        let rect = self.rect();
        rect.position + (rect.size / 2)
    }

    fn collides<T: Collidable>(&self, other: &T) -> bool {
        self.rect().touches(other.rect())
    }
}

impl Collidable for Rect<i32> {
    fn rect(&self) -> Rect<i32> {
        *self
    }
}
