// Variables hold primitive data or references to data
// Variables are immutable by default
// To make them mutable we have to use mut keyword
// Rust is a block-scoped language

pub fn run() {
    // Creating variables using let
    let nom = "nom nom";

    let mut age = 18;
    println!("Mon nom est {} et j'ai {} ans", nom, age);

    age = 45;

    println!("Mon nom est {} et j'ai {} ans", nom, age);

    // Using const
    const ID: &str = "007";
    println!("My ID is {} but I'm no James Bond", ID);

    // Assigning multiple variables
    let (nom, age) = ("nom nom", 45);
    println!("Mon nom est {} et j'ai {} ans", nom, age);
}
