// Tuples group together values of different types
// Max 12 elements

pub fn run() {
    let person: (&str, &str, u8) = ("Abuela", "Hermosa", 143);

    println!(
        "Mi {} y Mi {} tienos pagar {} boleto",
        person.0, person.1, person.2
    );
}
