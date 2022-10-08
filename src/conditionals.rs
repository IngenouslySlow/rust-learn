// Conditionals -- Used to check the condition

pub fn run() {
    let age = 18;
    let check_id: bool = false;
    let knows_person_of_age = true;

    // If - else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Allez-y!! Vous souhaitez prenez une biere?");
    } else if age < 21 && check_id {
        println!("Putain!! Que-ce-que t'as fait ici!!");
    } else {
        println!("Montrez votre ID");
    }

    // Shorthand -- There's no ternary in rust
    let is_of_age = if age >= 21 { true } else { false };
    println!("{}", is_of_age);
}
