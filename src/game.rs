use std::str::FromStr;

use super::{levels, DECCELERATION, BALL_SIZE};
use libm::{fminf, sqrtf, copysignf};
use crate::wasm4::*;

pub enum State {
    Menu,
    Playing,
    GameOver,
}

#[derive(Debug)]
pub enum Tile {
    VerticalWall,
    HorizontalWall,
    Empty,
    Player,
    Goal,
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let char = s.chars().next().unwrap();
        match char {
            '|' => Ok(Tile::VerticalWall),
            '-' => Ok(Tile::HorizontalWall),
            ' ' => Ok(Tile::Empty),
            'p' => Ok(Tile::Player),
            'x' => Ok(Tile::Goal),
            _ => {
                trace(format!("Unknown tile: {}", char));
                Err(())
            }
        }
    }
}

impl Tile {
    fn draw(&self, x: i32, y: i32, scale: u32) {
        match self {
            Tile::VerticalWall | Tile::HorizontalWall => {
                unsafe {
                    *DRAW_COLORS = 0x22;
                }
                rect(x, y, scale, scale);
            }
            Tile::Goal => {
                unsafe {
                    *DRAW_COLORS = 0x33;
                }
                oval(x, y, BALL_SIZE*scale, BALL_SIZE*scale);
            }
            _ => {}
        }
    }

    fn collision(&self, x: f32, y: f32, vel_x: f32, vel_y: f32, scale: u8) -> (f32, f32) {
        match self {
            Tile::VerticalWall => (-vel_x, vel_y),
            Tile::HorizontalWall => (vel_x, -vel_y),
            _ => (vel_x, vel_y),
        }
    }
}

#[derive(Debug)]
pub struct Level {
    pub tiles: Vec<Tile>,
    pub width: u16,
}

pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct Game {
    pub state: State,
    pub level: u16,
    pub levels: Vec<Level>,
    pub score: u16,
    pub position: Position,
    pub velocity: Position,
    pub scale: u8,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: State::Menu,
            level: 0,
            levels: levels::LEVELS
                .iter()
                .map(|level| Level {
                    tiles: level
                        .replace('\n', "")
                        .chars()
                        .filter(|c| c != &'\n')
                        .map(|c| Tile::from_str(&c.to_string()).unwrap())
                        .collect(),
                    width: level.lines().nth(1).unwrap().len() as u16,
                })
                .collect(),
            score: 0,
            position: Position { x: 5.0, y: 10.0 },
            scale: 2,
            velocity: Position { x: 0.0, y: 0.0 },
        }
    }

    pub fn update(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        let level = &self.levels[self.level as usize];
        let tile = &level.tiles[self.position.x as usize + self.position.y as usize * level.width as usize];
    
        (self.velocity.x, self.velocity.y) = tile.collision(self.position.x, self.position.y, self.velocity.x, self.velocity.y, self.scale);

        self.velocity.x *= DECCELERATION;
        self.velocity.y *= DECCELERATION;

        if self.velocity.x.abs() < 0.01 {
            self.velocity.x = 0.0;
        }
        if self.velocity.y.abs() < 0.01 {
            self.velocity.y = 0.0;
        }
    }

    pub fn draw(&mut self) {
        let level = &self.levels[self.level as usize];
        for (i, tile) in level.tiles.iter().enumerate() {
            let x = (i % level.width as usize) as i32;
            let y = level.tiles.len() as i32 / level.width as i32 - (i / level.width as usize) as i32;
            let scale = self.scale as u32;

            let x_coord = SCREEN_SIZE as i32 / 2 + x * scale as i32
                - (self.position.x * scale as f32) as i32;
            let y_coord = SCREEN_SIZE as i32 / 2
                - y * scale as i32
                + (self.position.y * scale as f32) as i32;

            tile.draw(x_coord, y_coord, scale);

            unsafe {
                *DRAW_COLORS = 0x44;
            }
            oval(
                SCREEN_SIZE as i32 / 2 - (BALL_SIZE * scale) as i32 / 2,
                SCREEN_SIZE as i32 / 2 - (BALL_SIZE * scale) as i32 / 2,
                BALL_SIZE * scale,
                BALL_SIZE * scale
            );
        }
    }
}
