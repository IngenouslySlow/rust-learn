// Enums are types which have a few definite values

enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // Perform action depending on info
    match m {
        Movement::Up => println!("Avatar Moving Up"),
        Movement::Down => println!("Avatar Moving Down"),
        Movement::Left => println!("Avatar Moving Left"),
        Movement::Right => println!("Avatar Moving Right"),
    }
}

pub fn run() {
    let avatar1 = Movement::Left;
    let _avatar2 = Movement::Up; // Using _ before a variable name, compiler doesn't complain about variable being not used.
    let _avatar3 = Movement::Right;
    let _avatar4 = Movement::Down;

    move_avatar(avatar1);
}
