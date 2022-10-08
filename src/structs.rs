// Structs - Used to create custom data types

// Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct TupleColor(u8, u8, u8);

// Struct with methods / impl
struct Person {
    first_name: String,
    second_name: String,
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: (first.to_string()),
            second_name: (last.to_string()),
        }
    }

    // get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.second_name)
    }

    // Set second name
    fn set_first_name(&mut self, first: &str) {
        self.first_name = first.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.second_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 220;

    println!("Traditional Color: ({},{},{})", c.red, c.green, c.blue);

    let mut tc = TupleColor(255, 0, 0);
    tc.0 = 200;
    println!("Tuple Color: {},{},{}", tc.0, tc.1, tc.2);

    // Using Person Struct
    let mut p = Person::new("John", "Doe");
    p.set_first_name("June");
    println!("{:?}", p.full_name());
    println!("Person Tuple {:?}", p.to_tuple());
}
