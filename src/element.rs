use bracket_lib::prelude::*;

use crate::player::Player;
use crate::gamecore::{Camera, Render};

pub const DISPLAY_WIDTH: i32 = 60;
pub const DISPLAY_HEIGHT: i32 = 50;

/// Basic obstacle
pub struct Obstacle {
    pub x: i32,
    gap: i32,
    height: i32,
}

impl Obstacle {
    pub fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap: random.range(10, 40),
            height: i32::max(2, 20 - score),
        }
    }

    pub fn collision(&self, player: &Player) -> bool {
        let half_height = self.height / 2;
        self.x == player.x
            && (player.y > self.gap + half_height || player.y < self.gap - half_height)
    }
}

impl Render for Obstacle {
    /// Implementation of render trait for Obstacle
    fn render(&self, camera: &Camera, context: &mut BTerm) {
        let screen_x = camera.width / 2 - (camera.x - self.x);
        let half_height = self.height / 2;

        for y in 0..self.gap - half_height {
            context.set(screen_x, y, GREEN, BLACK, to_cp437('\u{2588}'));
        }
        for y in self.gap + half_height..DISPLAY_HEIGHT {
            context.set(screen_x, y, GREEN, BLACK, to_cp437('\u{2588}'));
        }
    }
}

pub struct Terrain {
    pub data: Vec<u8>,
    current: usize,
}

impl Terrain {
    pub fn new(size: usize) -> Self {
        let mut terrain: Vec<u8> = Vec::with_capacity(size);
        let mut fac = 0.0_f32;
        let step = 2.0_f32 * 3.141596_f32 / DISPLAY_WIDTH as f32;
        for _ in 0..size {
            terrain.push((6_f32 + (5.0_f32 * fac.sin())) as u8);
            fac += step;
        }
        Terrain {
            data: terrain,
            current: 0,
        }
    }

    pub fn update(&mut self) {
        self.current += 1;
        if self.current == self.data.len() {
            self.current = 0;
        }
    }

    pub fn collision(&self, player: &Player) -> bool {
        let x = player.x % self.data.len() as i32;
        player.y >= DISPLAY_HEIGHT - self.data[x as usize] as i32
    }
}

impl Render for Terrain {
    fn render(&self, _camera: &Camera, context: &mut BTerm) {
        let (right, left) = self.data.split_at(self.current);
        let mut screen_x = 0;
        for height in left.iter().chain(right.iter()) {
            for y in DISPLAY_HEIGHT - *height as i32..DISPLAY_HEIGHT {
                context.set(
                    screen_x,
                    y,
                    RGBA::from_u8(0xa3, 0x55, 0x0d, 0xff),
                    BLACK,
                    to_cp437('\u{2588}'),
                );
            }
            screen_x += 1;
            if screen_x >= DISPLAY_WIDTH {
                screen_x = 0;
            }
        }
    }
}


const POWER_UP_CHAR: [char; 9] = [
	'\u{250c}', '\u{2500}', '\u{2510}',
	'\u{2502}', '\u{2665}', '\u{2502}',
	'\u{2514}', '\u{2500}', '\u{2518}'
];


#[derive(Debug, Copy, Clone)]
pub enum Power {
	Low = 5,
	Med = 10,
	High = 15,
}

pub struct PowerUp {
	pub power: Power,
	pub x: i32,
	y: i32,
}

impl PowerUp {
	pub fn new(x: i32, y: i32, power: Power) -> Self {
		PowerUp { 
            power,
			x,
			y,
		}
	}

	 pub fn collision(&self, player: &Player) -> bool {
        player.y >= self.y-1 && player.x >= self.x-1 &&
        	player.y <= self.y+1 && player.x <= self.x+1
    }
}


impl Render for PowerUp {
    fn render(&self, camera: &Camera, context: &mut BTerm) {
    	let color = match self.power {
    		Power::Low => RGBA::from_u8(0xd5, 0x86, 0x17, 0xff),
    	    Power::Med => RGBA::from_u8(0xd6, 0xf7, 0xff, 0xff),
            Power::High => RGBA::from_u8(0xfb, 0xe6, 0x00, 0xff),
    	};
    	let screen_x = camera.width / 2 - (camera.x - self.x) - 1;
    	let screen_y = self.y - 1;
    	for y in 0..3 {
    		for x in 0..3 {
		    	context.set (
		    		screen_x + x,
		    		screen_y + y,
		    		color,
		            RGBA::from_u8(40, 168, 210, 255),
		            to_cp437(POWER_UP_CHAR[(y*3+x) as usize])
		    	);
    		}
    	}
    }
}
