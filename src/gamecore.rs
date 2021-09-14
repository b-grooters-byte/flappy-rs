use bracket_lib::prelude::BTerm;


/// Game components must implement render
pub trait Render {
    fn render(&self, camera: &Camera, context: &mut BTerm);
}

#[derive(Debug, Copy, Clone)]
/// Game camera
pub struct Camera {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    height: i32,
}

impl Camera {
    /// Creates a new game camera.
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Camera {
            x,
            y,
            width,
            height,
        }
    }

    /// update advances the camera X position
    pub fn update(&mut self) {
        self.x += 1;
    }

    /// Gets the maximum X position for field of view of the camera.
    pub fn left(&self) -> i32 {
        self.x - self.width / 2
    }

    /// Gets the minimum X position for field of view of the camera.
    pub fn right(&self) -> i32 {
        self.x + self.width / 2
    }
}

