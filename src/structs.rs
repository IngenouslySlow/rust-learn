// Structs - Used to create custom data types

// Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
Struct TupleColor!(u8, u8, u8);

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    println!("Traditional Color: ({},{},{})", c.red, c.green, c.blue);

    let mut tc = TupleColor(255, 0, 0);
    println!("Traditional Color: {},{},{}", tc.0, tc.1, tc.2);
}
