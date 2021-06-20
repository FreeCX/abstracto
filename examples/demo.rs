extern crate abstracto as A;
use std::f32;

fn xor_shift(mut init: u32) -> impl Iterator<Item = u32> {
    std::iter::repeat_with(move || {
        init ^= init << 13;
        init ^= init >> 17;
        init ^= init << 5;
        init
    })
}

fn render(point: &A::RenderPoint) -> f32 {
    let xshift = f32::consts::PI * 0.5;
    let yshift = f32::consts::PI * 0.5;
    let a = 0.3;
    let b = 0.3;
    (point.xv * a + xshift).cos() + (point.yv * b + yshift).sin()
}

fn generate_palette() -> A::Palette {
    let mut map = A::Palette::new();
    for (i, c) in xor_shift(1).take(255).enumerate() {
        let green: u8 = (c % 255) as u8;
        let red = ((i + green as usize) / 2) as u8;
        let blue = (256 - red as u16) as u8;
        map.insert(i as u8, A::Color::new(red, green, blue));
    }
    map
}

fn main() {
    let generator = A::Generator::default()
        .set_width(500)
        .set_height(300)
        .set_xrange(-f32::consts::PI, f32::consts::PI)
        .set_yrange(-f32::consts::PI, f32::consts::PI);
    let result = generator.fill(&render);
    let palette = generate_palette();
    A::generate_ppm("output.ppm", &result, &palette);
}
