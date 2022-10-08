pub fn run() {
    greeting("Bonjour", "mamon");

    // Bind function values to variables
    let get_sum = add(21, 5);
    println!("Sum: {}", get_sum);

    // Closures
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3; // Can use outside variables as well with closures
    println!("Clousure sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, Enchante", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2 // We do not use a semicolon because i32 is what we want to return
}
