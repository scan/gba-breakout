use agb::fixnum::{num, Num};

pub type Floating = Num<i32, 8>;

pub fn lerp(a: Floating, other: Floating, t: Floating) -> Floating {
    a * (num!(1.0) - t) + other * t
}
