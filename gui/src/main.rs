mod window;

use fractal::{color, Mandel};
use std::time::Instant;
use window::Window;

const HEIGHT: usize = 100;
const WIDTH: usize = 100;

fn main() {
    let mut mandel = Mandel::new(-2.175, -0.9, 100, 500.);
    let mut window = Window::new(WIDTH, HEIGHT).unwrap();

    // init window
    let (width, height) = window.dimension();
    mandel.compute(&mut window.buffer, width, height);
    println!("before color: {:?}", &window.buffer);
    let max = window.buffer.iter().max().unwrap();
    color::convert_nb_to_rbg(*max, &mut window.buffer);
    println!("after color: {:?}", &window.buffer);
    window.update();

    while window.handle_event(&mut mandel) {
        let now = Instant::now();

        let (width, height) = window.dimension();
        mandel.compute(&mut window.buffer, width, height);
        color::convert_nb_to_rbg(mandel.iter, &mut window.buffer);

        println!(
            "mandelbrot {:4} for {} iter",
            now.elapsed().as_secs_f32(),
            mandel.iter
        );
        let now = Instant::now();

        window.update();

        println!("refresh {:?}", now.elapsed().as_secs_f32());
    }
}
