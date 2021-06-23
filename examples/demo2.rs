extern crate abstracto as A;
use std::f32;

mod extra;

fn render(point: A::RenderPoint) -> f32 {
    let xshift = f32::consts::PI;
    let yshift = 0.0;
    let a = 2.5 * 0.5;
    let b = 2.5 * 9.0 / 32.0;
    let x = point.xc as f32 / 2000.0;
    let y = point.yc as f32 / 1000.0;
    (point.xv * a * y as f32 + xshift).cos() + (point.yv * b * x + yshift).sin()
}

fn main() {
    let generator = A::Generator::default()
        .set_canvas(1280, 720)
        .set_xrange(-f32::consts::PI, f32::consts::PI)
        .set_yrange(-f32::consts::PI, f32::consts::PI);
    let result = generator.fill(&render);
    let palette = extra::read_palette("./palettes/pastels.ppm");
    A::generate_ppm("output-2.ppm", &result, &palette);
}
