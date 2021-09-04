extern crate abstracto as A;
use std::f32;

const CONSTS: &[(f32, f32, f32)] = &[(0.5, 2.5, 10.0), (0.1, 1.0, 4.0), (0.05, 0.8, 2.0), (0.03, 0.7, 1.0)];
const W: f32 = 1.2;

fn render(point: A::RenderPoint) -> f32 {
    let valid = |v, r, size| v >= r - size && v <= r + size;
    let p = |n| point.xv.abs().powf(n) + point.yv.abs().powf(n);
    if CONSTS.iter().map(|&(s, r, n)| valid(p(n), r, s)).any(|x| x) {
        1.0
    } else {
        0.0
    }
}

fn bw_palette() -> A::Palette {
    let mut map = A::Palette::new();
    map.insert(0, A::Color::rgb(0, 0, 0));
    map.insert(1, A::Color::rgb(255, 255, 255));
    map
}

fn main() {
    let generator = A::Generator::default().set_canvas(500, 500).set_xrange(-W, W).set_yrange(-W, W);
    let result = generator.fill(&render);
    let palette = bw_palette();
    A::generate_ppm("squircle.ppm", &result, &palette);
}
