// 格式化打印的其他格式通过实现对应的trait实现，比如：std::fmt::Binary对应{:b}，std::fmt::Octal对应{:o}，根据参数对应
// Display默认对应的是{}，也可以在逻辑内部实现参数对应
// 参照：https://doc.rust-lang.org/std/fmt/#formatting-traits 来查看格式化对应的trait

use std::fmt::{Display, Formatter};

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' }; // 计算纬度的后缀
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' }; // 计算经度的后缀

        write!(f, "{}: {:.3}°{}, {:.3}°{}", &self.name, self.lat, lat_c, self.lon, lon_c)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let rgb = self.red as u32 * 65536 + self.green as u32 * 256 + self.blue as u32; // 需要显示类型提升，Rust不做隐式转换
        write!(f, "RGB ({}, {}, {}) 0x{:0>width$x}", self.red, self.green, self.blue, rgb, width=6)
    }
}

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ] {
        println!("{}", city);
    }

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ] {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{:?}", color);
    }

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ] {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{}", color);
    }
}