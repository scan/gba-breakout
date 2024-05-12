#![no_std]
#![no_main]
#![feature(allocator_api)]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;

mod ball;
mod paddle;
mod resources;
mod collidable;
mod util;

use agb::interrupt::VBlank;
use ball::Ball;
use paddle::Paddle;

pub fn main(mut gba: agb::Gba) -> ! {
    let mut input = agb::input::ButtonController::new();
    let vblank = VBlank::get();
    let oam = gba.display.object.get_managed();

    let mut paddle = Paddle::new(&oam, 3);
    let mut ball = Ball::new(&oam);

    loop {
        paddle.update(&input);
        ball.update();
        ball.bounce_object(&paddle);

        vblank.wait_for_vblank();
        oam.commit();
        input.update();
    }
}
