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

            let mut angles: u32 = 0;
            let mut last_point = z;
            z = z * z + c;
            let mut antepenultimate_point = z;

            while ((z * z).re <= 4.0) && i < self.iter {
                z = z * z + c;

                let angle = (z - last_point).atan() - (antepenultimate_point - last_point).atan();
                angles = angles.saturating_add(angle.to_polar().1 as u32);
                antepenultimate_point = last_point;
                last_point = z;

                i += 1;
            }

            *val = angles / i;
        })
    }
}
