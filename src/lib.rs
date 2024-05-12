#![no_std]
#![no_main]
#![feature(allocator_api)]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;

mod ball;
mod collidable;
mod paddle;
mod resources;
mod state;
mod util;

use agb::{input::Button, interrupt::VBlank};
use ball::Ball;
use paddle::Paddle;
use state::GameState;

pub fn main(mut gba: agb::Gba) -> ! {
    let mut input = agb::input::ButtonController::new();
    let vblank = VBlank::get();
    let oam = gba.display.object.get_managed();

    let mut paddle = Paddle::new(&oam, 1);
    let mut ball = Ball::new(&oam);

    let mut current_state = GameState::default();

    loop {
        paddle.update(&input);
        ball.update();

        match current_state {
            GameState::Start => {
                if input.is_just_pressed(Button::A) {
                    current_state = GameState::Running;
                    ball.start();
                }
            }
            GameState::Running => {
                if ball.out_of_bounds() {
                    current_state = GameState::GameOver;
                }

                ball.bounce_object(&paddle);
            }
            GameState::GameOver => {
                ball.reset();
                current_state = GameState::Start;
            }
        }

        vblank.wait_for_vblank();
        oam.commit();
        input.update();
    }
}
