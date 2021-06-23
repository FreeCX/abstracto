use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufWriter, Write};

pub type ColorIndex = u8;
pub type Palette = BTreeMap<ColorIndex, Color>;
pub type RenderFunc = dyn Fn(RenderPoint) -> f32;

#[derive(Default)]
pub struct Generator {
    x_left: f32,
    x_right: f32,
    y_left: f32,
    y_right: f32,
    width: u32,
    height: u32,
}

pub struct RenderResult {
    raw: Vec<f32>,
    width: u32,
    height: u32,
}

#[derive(Copy, Clone)]
pub struct RenderPoint {
    pub xv: f32,
    pub yv: f32,
    pub xc: u32,
    pub yc: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Generator {
    pub fn set_canvas(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn set_xrange(mut self, left: f32, right: f32) -> Self {
        assert!(
            left <= right,
            "The left ({}) border should be less or equal than the right ({})!",
            left,
            right
        );
        self.x_left = left;
        self.x_right = right;
        self
    }

    pub fn set_yrange(mut self, left: f32, right: f32) -> Self {
        assert!(
            left <= right,
            "The left ({}) border should be less or equal than the right ({})!",
            left,
            right
        );
        self.y_left = left;
        self.y_right = right;
        self
    }

    pub fn fill(&self, func: &RenderFunc) -> RenderResult {
        let iterations = self.width * self.height;
        let mut result = RenderResult {
            raw: vec![0.0; iterations as usize],
            width: self.width,
            height: self.height,
        };
        let mut minv: Option<f32> = None;
        let mut maxv: Option<f32> = None;

        let x_step = (self.x_right - self.x_left) / self.width as f32;
        let y_step = (self.y_right - self.y_left) / self.height as f32;

        // inplace calculating
        for (index, value) in result.raw.iter_mut().enumerate() {
            let ix = index as u32 % self.width;
            let iy = index as u32 / self.width;
            *value = func(RenderPoint {
                xv: self.x_left as f32 + ix as f32 * x_step,
                yv: self.y_left as f32 + iy as f32 * y_step,
                xc: ix,
                yc: iy,
            });

            // find min & max
            minv = match minv {
                Some(v) => Some(value.min(v)),
                None => Some(*value)
            };
            maxv = match maxv {
                Some(v) => Some(value.max(v)),
                None => Some(*value)
            };
        }

        // no data - no remap
        if result.raw.len() == 0 {
            return result;
        }

        let minv = minv.unwrap().abs();
        let maxv = maxv.unwrap().abs();
        let range = minv + maxv;

        // remap values to [0, 1]
        for value in result.raw.iter_mut() {
            *value = (*value + minv) / range;
        }

        result
    }
}

impl Color {
    pub fn rgb<T: Into<u8>>(r: T, g: T, b: T) -> Color {
        Color {
            r: r.into(),
            g: g.into(),
            b: b.into(),
        }
    }
}

pub fn generate_ppm(output: &str, render: &RenderResult, palette: &Palette) {
    // i don't know exact header size
    let header_hint_size = 20;
    // allocate image buffer
    let mut buffer = Vec::with_capacity(3 * render.width as usize * render.height as usize + header_hint_size);
    let default = Color::rgb(0, 0, 0);

    // fill buffer
    let _ = write!(buffer, "P6\n{} {}\n255\n", render.width, render.height);
    for point in &render.raw {
        let index = ((palette.len() - 1) as f32 * (*point)) as ColorIndex;
        let color = palette.get(&index).unwrap_or(&default);
        buffer.extend_from_slice(&[color.r, color.g, color.b]);
    }

    // write to file
    let mut f = BufWriter::new(File::create(output).unwrap());
    let _ = f.write(&buffer);
}
