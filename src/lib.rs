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

static PREVIOUS_MOUSE_BUTTON: Mutex<u8> = Mutex::new(0);

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
    let mouse = unsafe { *MOUSE_BUTTONS };
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
    } else if *PREVIOUS_MOUSE_BUTTON.lock().unwrap() != mouse {
        GAME.lock().unwrap().velocity.x = -(mouse_x - SCREEN_SIZE as i16 / 2) as f32 / 80.0;
        GAME.lock().unwrap().velocity.y = (mouse_y - SCREEN_SIZE as i16 / 2) as f32 / 80.0;
    }

    *PREVIOUS_MOUSE_BUTTON.lock().unwrap() = mouse;
}
