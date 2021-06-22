#![allow(dead_code, unused_variables)]
extern crate abstracto as A;
use std::fs::File;
use std::io::{BufWriter, Write, BufReader, BufRead};

fn xor_shift(mut init: u32) -> impl Iterator<Item = u32> {
    std::iter::repeat_with(move || {
        init ^= init << 13;
        init ^= init >> 17;
        init ^= init << 5;
        init
    })
}

pub fn generate_palette() -> A::Palette {
    let mut map = A::Palette::new();
    for (i, c) in xor_shift(1).take(256).enumerate() {
        let green: u8 = (c % 255) as u8;
        let red = ((i + green as usize) / 2) as u8;
        let blue = (256 - red as u16) as u8;
        map.insert(i as u8, A::Color::rgb(red, green, blue));
    }
    map
}

pub fn grayscale_palette() -> A::Palette {
    let mut map = A::Palette::new();
    for (i, c) in xor_shift(1).take(256).enumerate() {
        let v = (256 - c % 255) as u8;
        map.insert(i as u8, A::Color::rgb(v, v, v));
    }
    map
}

pub fn read_palette(file: &str) -> A::Palette {
    let f = BufReader::new(File::open(file).unwrap());
    let mut result = A::Palette::new();

    for (index, line) in f.lines().skip(3).enumerate() {
        match line {
            Ok(line) => {
                let items: Vec<u8> = line.split(" ").map(|x| x.parse().unwrap()).collect();
                result.insert(index as u8, A::Color::rgb(items[0], items[1], items[2]));
            }
            Err(_) => {},
        }
    }

    result
}

pub fn write_palette(file: &str, palette: &A::Palette) {
    let mut f = BufWriter::new(File::create(file).unwrap());
    let _ = write!(f, "P3\n16 16\n255\n");
    for (_, color) in palette {
        let _ = write!(f, "{} {} {}\n", color.r, color.g, color.b);
    }
}
