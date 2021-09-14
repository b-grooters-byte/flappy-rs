use bracket_lib::prelude::*;
use bracket_lib::prelude::YELLOW;
use crate::gamecore::{Camera, Render};

const PLAYER_INIT_POWER: u32 = 20;

/// Flappy bird player
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub velocity: f32,
    pub power: u32,
    pub flap: bool,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
            power: PLAYER_INIT_POWER,
            flap: false,
        }
    }

    pub fn update(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }

    pub fn flap(&mut self) -> Option<u32>{
        self.velocity -= 1.5;
        self.flap = true;
        if let Some(rem) = self.power.checked_sub(1) {
        	self.power = rem;
        	return Some(rem);
        }
        None
    }

    pub fn power_up(&mut self, power: u32) -> u32 {
    	self.power += power;
    	self.power
    }
}

impl Render for Player {
    fn render(&self, camera: &Camera, context: &mut BTerm) {
        if self.flap {
            context.set(
                camera.width / 2 - (camera.x - self.x),
                self.y + 1,
                YELLOW,
                RGBA::from_u8(40, 168, 210, 255),
                to_cp437('\u{25BC}'),
            );
        } else {
            context.set(
                camera.width / 2 - (camera.x - self.x),
                self.y,
                YELLOW,
                RGBA::from_u8(40, 168, 210, 255),
                to_cp437('\u{25B2}'),
            );
        }
    }
}

