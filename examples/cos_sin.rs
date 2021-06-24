extern crate abstracto as A;
use std::f32;

mod extra;

fn render(point: A::RenderPoint) -> f32 {
    let xshift = f32::consts::PI * 0.5;
    let yshift = 0.0;
    let a = 0.5;
    let b = 9.0 / 32.0;
    (point.xv * a + xshift).cos() + (point.yv * b + yshift).sin()
}

fn main() {
    let generator = A::Generator::default()
        .set_canvas(1280, 720)
        .set_xrange(-f32::consts::PI, f32::consts::PI)
        .set_yrange(-f32::consts::PI, f32::consts::PI);
    let result = generator.fill(&render);
    let palette = extra::generate_palette();
    // extra.write_paletter("./palettes/random.ppm");
    A::generate_ppm("cos_sin.ppm", &result, &palette);
}
