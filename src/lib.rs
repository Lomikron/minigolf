#[cfg(feature = "buddy-alloc")]
mod alloc;
mod game;
mod levels;
mod wasm4;
use std::sync::Mutex;

use libm::sqrtf;
use wasm4::*;

pub const BALL_SIZE: u32 = 1;
pub const WALL_WIDTH: u32 = 2;
pub const SCALE: u8 = 4;
pub const OVERVIEW_SCALE: u8 = 2;
pub const DECCELERATION: f32 = 0.99;
pub const PUSH_FORCE: f32 = 0.013;
pub const MAX_SPEED: f32 = 2.5;

static PREVIOUS_MOUSE_BUTTON: Mutex<bool> = Mutex::new(false);
static PREVIOUS_GAMEPAD_X: Mutex<bool> = Mutex::new(false);

lazy_static::lazy_static! {
    static ref GAME: Mutex<game::Game> = Mutex::new(game::Game::new());
}

#[no_mangle]
fn start() {
    unsafe {
        *PALETTE = [0x00303b, 0xff7777, 0xffce96, 0xf1f2da];
    }

    GAME.lock().unwrap().initialize_ball();
}

#[no_mangle]
fn update() {
    let mut game = GAME.lock().unwrap();
    let mouse = unsafe { *MOUSE_BUTTONS };

    match game.state {
        game::State::Menu => {
            text("Press Space or X\n     to Start", 10, 80);
            let gamepad = unsafe { *GAMEPAD1 };
            if gamepad & BUTTON_1 == 0 && *PREVIOUS_GAMEPAD_X.lock().unwrap() {
                game.state = game::State::Playing;
                game.initialize_ball();
            }
            *PREVIOUS_GAMEPAD_X.lock().unwrap() = gamepad & BUTTON_1 != 0;
        }
        game::State::Playing => {
            let mouse_left = mouse & MOUSE_LEFT != 0;
            let mouse_right = mouse & MOUSE_RIGHT != 0;

            let mouse_x = unsafe { *MOUSE_X };
            let mouse_y = unsafe { *MOUSE_Y };

            game.update();
            game.draw();

            if mouse & MOUSE_LEFT != 0 {
                if game.is_stationary() {
                    unsafe { *DRAW_COLORS = 4 }
                } else {
                    unsafe { *DRAW_COLORS = 2 }
                }
                line(
                    mouse_x as i32,
                    mouse_y as i32,
                    SCREEN_SIZE as i32 / 2,
                    SCREEN_SIZE as i32 / 2,
                );
            } else if *PREVIOUS_MOUSE_BUTTON.lock().unwrap() != mouse_left {
                if game.is_stationary() {
                    let push_x = -(mouse_x - SCREEN_SIZE as i16 / 2) as f32 * PUSH_FORCE;
                    let push_y = (mouse_y - SCREEN_SIZE as i16 / 2) as f32 * PUSH_FORCE;
                    let speed = sqrtf(push_x * push_x + push_y * push_y);
                    if speed < MAX_SPEED {
                        game.velocity.x = push_x;
                        game.velocity.y = push_y;
                    } else {
                        game.velocity.x = push_x * MAX_SPEED / speed;
                        game.velocity.y = push_y * MAX_SPEED / speed;
                    }
                    game.score += 1;
                }
            } else if mouse_right {
                game.scale = OVERVIEW_SCALE;
            } else if !mouse_right {
                game.scale = SCALE;
            }

            *PREVIOUS_MOUSE_BUTTON.lock().unwrap() = mouse_left;
        }
        game::State::GameOver => {
            unsafe { *DRAW_COLORS = 3 }
            text("Congratulations!", 10, 50);
            text(format!("Your score is {}", game.score), 10, 70);
            unsafe { *DRAW_COLORS = 2 }
            text("Press Space or X\n   to Restart", 10, 100);
            let gamepad = unsafe { *GAMEPAD1 };
            if gamepad & BUTTON_1 != 0 {
                game.state = game::State::Playing;
                game.level = 0;
                game.score = 0;
                game.velocity.x = 0.0;
                game.velocity.y = 0.0;
                game.initialize_ball();
            }
        }
    }
}
