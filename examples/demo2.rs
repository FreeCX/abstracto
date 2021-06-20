extern crate abstracto as A;
use std::f32;

mod extra;

fn render(point: &A::RenderPoint) -> f32 {
    let xshift = 0.3 * f32::consts::PI;
    let yshift = 0.0;
    let a = 0.5;
    let b = 9.0 / 32.0;
    (point.xv * a * point.yv.cos() + xshift).cos() + (point.yv * b * point.xv.sin() + yshift).sin()
}

fn main() {
    let generator = A::Generator::default()
        .set_canvas(1280, 720)
        .set_xrange(-f32::consts::PI, f32::consts::PI)
        .set_yrange(-f32::consts::PI, f32::consts::PI);
    let result = generator.fill(&render);
    let palette = extra::generate_palette();
    A::generate_ppm("output-2.ppm", &result, &palette);
}
