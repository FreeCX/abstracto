extern crate abstracto as A;
use std::f32;

mod extra;

fn render(point: &A::RenderPoint) -> f32 {
    let xshift = f32::consts::PI * 0.5;
    let yshift = f32::consts::PI * 0.5;
    let a = 0.3;
    let b = 0.3;
    (point.xv * a + xshift).cos() + (point.yv * b + yshift).sin()
}

fn main() {
    let generator = A::Generator::default()
        .set_canvas(500, 300)
        .set_xrange(-f32::consts::PI, f32::consts::PI)
        .set_yrange(-f32::consts::PI, f32::consts::PI);
    let result = generator.fill(&render);
    let palette = extra::generate_palette();
    A::generate_ppm("output-1.ppm", &result, &palette);
}
