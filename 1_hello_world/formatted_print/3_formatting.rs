use std::fmt::{self, Display, Formatter};

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

/*

Below outputs like this:
RGB (128, 255, 90) 0x80FF5A
RGB (0, 3, 254) 0x0003FE
RGB (0, 0, 0) 0x000000

*/

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "RGB ({}, {}, {}) 0x{:02X}{:02X}{:02X}",
            self.red, self.green, self.blue, self.red, self.green, self.blue
        )
    }
}

fn main() {
    // nice lil tricks
    // let foo: i64 = 3735928559;
    // let dead_beef = format!("0x{:X}", foo);
    // println!("{}", format!("0x{:X}", foo));

    for city in [
        City {
            name: "Madrid",
            lat: 53.23323,
            lon: -3.23121313,
        },
        City {
            name: "Oslo",
            lat: 27.23112,
            lon: -1.212333,
        },
        City {
            name: "Vancouver",
            lat: 39.1231,
            lon: -19.11233,
        },
    ]
    .iter()
    {
        println!("{}", *city);
    }

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        println!("{}", *color);
    }
}
