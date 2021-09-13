#![warn(clippy::all, clippy::pedantic)]

mod gamecore;
mod element;
mod player; 

use rand::Rng;
use bracket_lib::prelude::*;
use gamecore::{Render, Camera};
use crate::element::{Obstacle, Terrain, Power, PowerUp};
use crate::player::Player;

const FRAME_DURATION: f32 = 75.0;
const FLAP_DURATION: f32 = FRAME_DURATION * 2.0;

enum GameState {
    Menu,
    Playing,
    End,
}

struct State {
    camera: Camera,
    player: Player,
    obstacle: Obstacle,
    terrain: Terrain,
    new_obstacle: bool,
    frame_time: f32,
    back_frame_time: f32,
    state: GameState,
    score: i32,
    last_flap: f32,
    power_up: Vec<PowerUp>,
    power_up_idx: usize,
    power_remaining: bool,
}

impl State {
    fn new() -> Self {
        let camera = Camera::new(
            element::DISPLAY_WIDTH / 2,
            element::DISPLAY_HEIGHT / 2,
            element::DISPLAY_WIDTH,
            element::DISPLAY_HEIGHT,
        );

        State {
            camera,
            player: Player::new(15, element::DISPLAY_HEIGHT / 3),
            obstacle: Obstacle::new(element::DISPLAY_WIDTH, 0),
            terrain: Terrain::new(element::DISPLAY_WIDTH as usize),
            new_obstacle: true,
            frame_time: 0.0,
            back_frame_time: 0.0,
            state: GameState::Menu,
            score: 0,
            last_flap: 0_f32,
            power_up: State::init_power_up(),
            power_up_idx: 0_usize,
            power_remaining: true,
        }
    }

    fn init_power_up() -> Vec<PowerUp> {
        let mut power_up = Vec::with_capacity(30);

        let mut rng = rand::thread_rng();
        // loop through 
        for x in 0..45 {
            for i in 1..=10 {
                let fac = i as f32/ 10.0;
                if rng.gen::<f32>() < fac {
                    let y = rng.gen_range(5..30);
                    let power = match rng.gen_range(1..4) {
                        1 => Power::Low,
                        2 => Power::Med,
                        _ => Power::High,
                    };
                    power_up.push(PowerUp::new(x * 30 + 17 + i, y, power));
                    break;
                }
            }
        }
        power_up
    }

    fn menu_state(&mut self, context: &mut BTerm) {
        context.cls();
        context.print_centered(5, "Welcome to Flappy!");
        context.print_centered(6, "Fly East, collect power up, and avoid the obstacles!");
        context.print_centered(8, "(P)lay");
        context.print_centered(9, "(Q)uit");

        if let Some(key) = context.key {
            match key {
                VirtualKeyCode::P => {
                    self.restart();
                }
                VirtualKeyCode::Q => context.quitting = true,
                _ => {}
            }
        }
    }

    fn play_state(&mut self, context: &mut BTerm) {
        context.cls_bg(RGBA::from_u8(40, 168, 210, 255));
        context.print(1, 1, format!("POWER: {}", self.player.power));
        context.print(1, 2, format!("SCORE: {}", self.score));

        self.frame_time += context.frame_time_ms;
        self.back_frame_time += context.frame_time_ms;
        self.last_flap += context.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.update();
            self.camera.update();
            self.terrain.update();
        }
        // check the background frame movement
        if self.last_flap > FLAP_DURATION && self.player.flap {
            self.player.flap = false;
        }

        if let Some(VirtualKeyCode::Space) = context.key {
            if self.power_remaining {
                if self.player.flap().is_none() {
                    self.power_remaining = false;
                }
                self.last_flap = 0.0;                
            }
        }

        if self.player.x > self.obstacle.x && self.new_obstacle {
            self.score += 1;
            self.new_obstacle = false;
        }

        if self.player.y > element::DISPLAY_HEIGHT
            || self.obstacle.collision(&self.player)
            || self.terrain.collision(&self.player)
        {
            self.state = GameState::End;
        }

        for (idx, power) in self.power_up.iter().skip(self.power_up_idx).enumerate() {
            if power.x < self.camera.left() {
                self.power_up_idx += 1;
            }
            if power.collision(&self.player) {
                self.player.power_up(power.power as u32);
                self.power_up_idx = self.power_up_idx + idx + 1;
            }
        }

        if self.obstacle.x <= self.camera.left() {
            self.obstacle = Obstacle::new(self.obstacle.x + self.camera.width, self.score);
            self.new_obstacle = true;
        }
        if self.obstacle.x > self.camera.left() && self.obstacle.x < self.camera.right() {
            self.obstacle.render(&self.camera, context);
        }
        self.terrain.render(&self.camera, context);
        self.player.render(&self.camera, context);
        for p in &self.power_up {
            if p.x >= self.camera.left() && p.x <= self.camera.right() {
                p.render(&self.camera, context);
            }
        }
    }

    fn end_state(&mut self, context: &mut BTerm) {
        context.cls();
        context.print_centered(5, "You have perished!");
        context.print_centered(8, "(R)estart");
        context.print_centered(11, "(Q)uit");
        if let Some(key) = context.key {
            match key {
                VirtualKeyCode::R => {
                    self.restart();
                }
                VirtualKeyCode::Q => context.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(15, element::DISPLAY_HEIGHT / 3);
        self.state = GameState::Playing;
        self.frame_time = 0.0;
        self.score = 0;
        self.obstacle = Obstacle::new(element::DISPLAY_WIDTH, self.score);
        self.terrain = Terrain::new(element::DISPLAY_WIDTH as usize);
        self.power_up  = State::init_power_up();
        self.power_up_idx = 0_usize;
        self.power_remaining = true;        
        self.camera = Camera::new(
            element::DISPLAY_WIDTH / 2,
            element::DISPLAY_HEIGHT / 2,
            element::DISPLAY_WIDTH,
            element::DISPLAY_HEIGHT,
        );
    }
}

impl bracket_lib::prelude::GameState for State {
    fn tick(&mut self, context: &mut BTerm) {
        match self.state {
            GameState::Menu => {
                self.menu_state(context);
            }
            GameState::Playing => {
                self.play_state(context);
            }
            GameState::End => {
                self.end_state(context);
            }
        }
    }
}

fn main() -> BError {
    let context = bracket_lib::prelude::BTermBuilder::simple::<i32>(60, 50).unwrap();
    let context = context.with_title("Flappy").build()?;

    main_loop(context, State::new())
}
