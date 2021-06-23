extern crate abstracto as A;
use std::f32;

mod extra;

fn render(point: A::RenderPoint) -> f32 {
    if point.xc % 3 == 0 || point.yc % 4 == 0 {
        0.8
    } else {
        (point.xv * 1.1).cos() * point.yc as f32 / 720.0
    }
}

fn main() {
    let generator = A::Generator::default()
        .set_canvas(1270, 720)
        .set_xrange(-2.0, 2.0);
    let result = generator.fill(&render);
    let palette = extra::grayscale_palette();
    A::generate_ppm("output-4.ppm", &result, &palette);
}
