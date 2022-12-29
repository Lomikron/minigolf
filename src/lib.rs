#[cfg(feature = "buddy-alloc")]
mod alloc;
mod game;
mod levels;
mod wasm4;
use std::sync::Mutex;

use wasm4::*;

pub const BALL_SIZE: u32 = 2;
pub const WALL_WIDTH: u32 = 2;
pub const DECCELERATION: f32 = 0.99;

static PREVIOUS_MOUSE_BUTTON: Mutex<bool> = Mutex::new(false);

lazy_static::lazy_static! {
    static ref GAME: Mutex<game::Game> = Mutex::new(game::Game::new());
}

#[no_mangle]
fn start() {
    unsafe {
        *PALETTE = [0x00303b, 0xff7777, 0xffce96, 0xf1f2da];
    }
}

#[no_mangle]
fn update() {
    let mut mouse = unsafe { *MOUSE_BUTTONS };
    let mouse_left = mouse & MOUSE_LEFT != 0;
    let mouse_right = mouse & MOUSE_RIGHT != 0;

    let mouse_x = unsafe { *MOUSE_X };
    let mouse_y = unsafe { *MOUSE_Y };

    GAME.lock().unwrap().update();
    GAME.lock().unwrap().draw();

    if mouse & MOUSE_LEFT != 0 {
        unsafe { *DRAW_COLORS = 4 }
        line(
            mouse_x as i32,
            mouse_y as i32,
            SCREEN_SIZE as i32 / 2,
            SCREEN_SIZE as i32 / 2,
        );
    } else if *PREVIOUS_MOUSE_BUTTON.lock().unwrap() != mouse_left {
        GAME.lock().unwrap().velocity.x = -(mouse_x - SCREEN_SIZE as i16 / 2) as f32 / 80.0;
        GAME.lock().unwrap().velocity.y = (mouse_y - SCREEN_SIZE as i16 / 2) as f32 / 80.0;
    } else if mouse_right {
        GAME.lock().unwrap().scale = 1;
    } else if !mouse_right {
        GAME.lock().unwrap().scale = 2;
    }

    *PREVIOUS_MOUSE_BUTTON.lock().unwrap() = mouse_left;
}
