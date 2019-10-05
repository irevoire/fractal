mod window;

pub struct Coord {
    pub x: f64,
    pub y: f64,
}

pub struct Update {
    pub coord: Coord,
    pub iter: usize,
    pub zoom: f64,
}

pub use window::Window;
