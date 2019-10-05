use minifb::{Key, KeyRepeat};
use rayon::prelude::*;
use std::{thread, time};

/// The buffer should be filled with the number of iteration used to compute the fractal
/// When a point is found in the fractal it should be inserted with a value of u32::MAX
pub struct Window {
    window: minifb::Window,
    width: usize,
    height: usize,
    pub buffer: Vec<u32>,
}

impl Window {
    pub fn new(title: &str, width: usize, height: usize) -> Result<Self, String> {
        let window = minifb::Window::new(
            title,
            width,
            height,
            minifb::WindowOptions {
                resize: false, // TODO allow resize
                scale: minifb::Scale::X1,
                ..minifb::WindowOptions::default()
            },
        );
        if let Err(e) = window {
            return Err(format!("Unable to create window {}", e));
        };

        Ok(Window {
            // if the window creation fail we exit everything
            window: window.unwrap(),
            width,
            height,
            buffer: vec![0; width * height],
        })
    }

    pub fn draw(&mut self, update: &crate::Update) {
        self.buffer.par_iter_mut().for_each(|val| {
            if *val == std::u32::MAX {
                *val = 0x0000_0000;
            } else {
                *val = hue_to_rgb(
                    *val as f32 * (360.0 / update.iter as f32),
                    1.0,
                    *val as f32 / update.iter as f32,
                );
            }
        });
        self.window
            .update_with_buffer(&self.buffer)
            .unwrap_or_else(|e| log::error!("Window update failed: {}", e));
    }

    /// Update the mandel struct with the fetched event
    /// The user want to exit if this function return false
    pub fn handle_event(&mut self, mandel: &mut crate::Update) -> bool {
        let mut update = false;

        while !update {
            self.window.update(); // needed in order to fetch the new events

            if !self.window.is_open() {
                return false;
            }
            if self.window.is_key_down(Key::Escape) {
                return false;
            }

            update |= self.handle_event_key(mandel);
            thread::sleep(time::Duration::from_millis(10));
        }
        update
    }

    fn handle_event_key(&self, mandel: &mut crate::Update) -> bool {
        let mut update = false;
        self.window.get_keys_pressed(KeyRepeat::Yes).map(|keys| {
            for t in keys {
                match t {
                    Key::W | Key::Z | Key::Up => {
                        mandel.coord.y -= 100.0 / mandel.zoom;
                        update = true;
                    }
                    Key::S | Key::Down => {
                        mandel.coord.y += 100.0 / mandel.zoom;
                        update = true;
                    }
                    Key::A | Key::Q | Key::Left => {
                        mandel.coord.x -= 100.0 / mandel.zoom;
                        update = true;
                    }
                    Key::D | Key::Right => {
                        mandel.coord.x += 100.0 / mandel.zoom;
                        update = true;
                    }
                    Key::Space => {
                        mandel.coord.x += self.width as f64 * 0.25 / mandel.zoom;
                        mandel.coord.y += self.height as f64 * 0.25 / mandel.zoom;
                        mandel.zoom *= 2.0;
                        update = true;
                    }
                    Key::X => {
                        mandel.zoom /= 2.0;
                        mandel.coord.x -= self.width as f64 * 0.25 / mandel.zoom;
                        mandel.coord.y -= self.height as f64 * 0.25 / mandel.zoom;
                        update = true;
                    }
                    Key::I => {
                        mandel.iter += 3;
                        update = true;
                    }
                    Key::U => {
                        mandel.iter -= 3;
                        update = true;
                    }
                    _ => (),
                }
            }
        });
        update
    }

    /// return the width of the window
    pub fn width(&self) -> usize {
        self.width
    }

    /// return the height of the window
    pub fn height(&self) -> usize {
        self.height
    }
}

pub fn hue_to_rgb(hue: f32, saturation: f32, value: f32) -> u32 {
    let c: f32 = saturation * value;
    let x: f32 = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs()) as f32;
    let m: f32 = value - c;
    let (r, g, b) = match hue as u32 {
        0..=60 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        300..=359 => (c, 0.0, x),
        _ => return 0,
    };
    let (r, g, b) = ((r + m) * 255.0, (g + m) * 255.0, (b + m) * 255.0);
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}
