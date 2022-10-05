pub fn run() {
    // basic console printing
    println!("Beunos Dias Mi Abuela!");

    // Basic Formatting
    println!("{} {} esta aqui", "Mi", "gato");

    // Positional arguments
    println!("{0} {1} esta aqui. {0} tienes {2}", "El", "gato", "pagar");

    // Named Arguments
    println!("{qui} nom est {question}", qui = "votre", question = "quoi");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
