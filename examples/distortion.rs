extern crate abstracto as A;
use std::f32;

mod extra;

fn render(point: A::RenderPoint) -> f32 {
    let aspect = ((point.xc + 2) as f32).ln() / ((point.yc + 2) as f32).ln();
    aspect * point.yv.ln() * point.xv.cos() + point.xv.ln() * point.yv.sin()
}

fn main() {
    let generator = A::Generator::default()
        .set_canvas(1280, 720)
        .set_xrange(1.0, 2.0)
        .set_yrange(1.0, 2.0);
    let result = generator.fill(&render);
    let palette = extra::read_palette("./palettes/random.ppm");
    A::generate_ppm("distortion.ppm", &result, &palette);
}
