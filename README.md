Fractal
=======

A small engine I made to draw the
[mandelbrot set](https://github.com/irevoire/mandelbrust) and the
[julia set](https://github.com/irevoire/rulia).


Usage
=====

Include
-------

Since this engine is really specific I don’t plan to do a release on crate.io.
If you want to use it you can include it in your `cargo.toml` as follow:
```toml
fractal = { git = "https://github.com/irevoire/fractal" }
```

In your code
------------

This engine provide a minimal interface to easily write your code:
First you should initialize a `Window`.
```rust
fractal::Window::new(title: &str, width: usize, height: usize) -> Result<Self, String>
```

Then you can either check for input provided by the user with `handle_event`.
Or you can draw the fractal.
To be able to draw anything you’ll first need to fill the buffer contained in the
`Window` struct, it is a vector of the size `width * height`.
When you identified a point that is a part of the fractal you must write
`std::u32::MAX` in the buffer. All the other values will draw different color.
Usually you should write the number of iteration you’ve done before knowing this
pixel was not a part of the fractal.

Example
=======

For example of utilization of this crate check theses two repositories:
- [mandelbrot set](https://github.com/irevoire/mandelbrust)
- [julia set](https://github.com/irevoire/rulia)

