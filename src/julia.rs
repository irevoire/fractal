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

    #[allow(non_upper_case_globals)]
    /// Compute mandelbrot and write the number of iteration in each cells of the slice.
    pub fn compute(&self, window: &mut [u32], width: usize, _height: usize) {
        // TODO
        let x1 = self.pos.x;
        let y1 = self.pos.y;

        window.par_iter_mut().enumerate().for_each(|(index, val)| {
            let x = index % width;
            let y = index / width;

            const c: Complex<f64> = Complex::new(-0.7, 0.27015);

            let mut z = Complex::new(x as f64 / self.zoom + x1, y as f64 / self.zoom + y1);

            let mut i = 0;

            while ((z * z).re <= 4.0) && i < self.iter {
                z = z * z + c;
                i += 1;
            }

            *val = i;
        })
    }
}
