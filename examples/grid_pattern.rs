extern crate abstracto as A;
use std::f32;

mod extra;

fn render(point: A::RenderPoint) -> f32 {
    if point.xc % 16 == 0 && point.yc % 16 == 0 {
        0.0
    } else if (point.xc * point.yc) % 16 == 0 {
        0.5
    } else {
        1.0
    }
}

fn main() {
    let generator = A::Generator::default()
        .set_canvas(257, 257)
        .set_xrange(0.0, 1.0);
    let result = generator.fill(&render);
    let palette = extra::special_palette();
    A::generate_ppm("grid_pattern.ppm", &result, &palette);
}
