extern crate abstracto as A;

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
    for (i, c) in xor_shift(1).take(255).enumerate() {
        let green: u8 = (c % 255) as u8;
        let red = ((i + green as usize) / 2) as u8;
        let blue = (256 - red as u16) as u8;
        map.insert(i as u8, A::Color::new(red, green, blue));
    }
    map
}
