// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify own string data

pub fn run() {
    // Immutable string
    let hello = "Bonjour";

    // Growable string
    let mut bonjour = String::from("Hello ");

    // Get length of a string -- Works for both immutable and growable type
    println!("Length: {}", hello.len());

    // Push method -- Only used to push char
    // bonjour.push("C"); // Works because we are pushing a single character
    // bonjour.push("Ca"); // Gives an error because we are pushing multi characters

    // Push_str -- Used to push strings
    bonjour.push_str("Ca va bien?");
    println!("{}", bonjour);

    // Capacity in bytes
    println!("Capacity: {}", bonjour.capacity());

    // check if string is empty
    println!("Is Empty: {}", bonjour.is_empty());

    // Check if string contains a word
    println!("Contains world: {}", bonjour.contains("world"));

    // Replace
    println!("{}", bonjour.replace("Ca va bien?", "World"));

    // Loop through string by whitespace
    for word in bonjour.split_whitespace() {
        println!("{}", word);
    }

    // Create a string with capacity
    let mut my_str = String::with_capacity(10);
    my_str.push('a');
    my_str.push('b');
    println!("Capacity: {}", my_str.capacity());

    // Assertion testing
    assert_eq!(2, my_str.len()); // Only shows error if it fails
}
