extern crate abstracto as A;
use std::f32;

mod extra;

const W_WIDTH: u32 = 1280;
const W_HEIGHT: u32 = 720;
const MAX_ITER: u32 = 30;
const X_MIN: f32 = -2.2;
const X_MAX: f32 = 1.0;
const Y_MIN: f32 = -1.2;
const Y_MAX: f32 = 1.2;

fn render(point: A::RenderPoint) -> f32 {
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut iteration = 0;

    while x.powi(2) + y.powi(2) < 4.0 && iteration < MAX_ITER {
        let xt = x.powi(2) - y.powi(2) + point.xv;
        y = 2.0 * x * y + point.yv;
        x = xt;
        iteration += 1;
    }

    iteration as f32
}

fn main() {
    let generator = A::Generator::default()
        .set_canvas(W_WIDTH, W_HEIGHT)
        .set_xrange(X_MIN, X_MAX)
        .set_yrange(Y_MIN, Y_MAX);
    let result = generator.fill(&render);
    let palette = extra::read_palette("./palettes/random.ppm");
    A::generate_ppm("mandelbrot_set.ppm", &result, &palette);
}
