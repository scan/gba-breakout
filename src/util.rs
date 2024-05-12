use agb::fixnum::{Num, num};

pub type Floating = Num<i32, 8>;

fn lerp(a: Floating, other: Floating, t: Floating) -> Floating {
    a * (num!(1.0) - t) + other * t
}
