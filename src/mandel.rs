use num::Complex;
use rayon::prelude::*;

pub struct Pos {
    pub x: f64,
    pub y: f64,
}

pub struct Mandel {
    pub pos: Pos,
    pub iter: u32,
    pub zoom: f64,
}

impl Mandel {
    pub fn new(x: f64, y: f64, iter: u32, zoom: f64) -> Self {
        Mandel {
            pos: Pos { x, y },
            iter,
            zoom,
        }
    }

    /// Compute mandelbrot and write the number of iteration in each cells of the slice.
    pub fn compute(&self, window: &mut [u32], width: usize, _height: usize) {
        let x1 = self.pos.x;
        let y1 = self.pos.y;

        window.par_iter_mut().enumerate().for_each(|(index, val)| {
            let x = index % width;
            let x = x as f64 / self.zoom + x1;
            let y = index / width;
            let y = y as f64 / self.zoom + y1;

            let c = Complex::new(x, y);
            let mut z = Complex::new(x, y);
            let mut i = 0;

            let mut distance: u32 = 0;
            let mut last_point = z.clone();

            while ((z * z).re <= 4.0) && i < self.iter {
                z = z * z + c;

                distance = distance.saturating_add((z + last_point).norm() as u32);
                last_point = z;

                i += 1;
            }

            *val = distance;

            /*
            if i == self.iter {
                // println!("inside mandelbrot with a distance of {}", distance);
                *val = distance;
            } else {
                *val = i;
            }
            */
        })
    }
}
