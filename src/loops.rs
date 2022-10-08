// Loops - Used to iterate until a condition is set

pub fn run() {
    let mut count = 0;

    // Infinite loops
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 10 {
            break;
        }
    }

    // While loop (Fizz buzz)
    while count <= 50 {
        if count % 15 == 0 {
            println!("Fizzbuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }

        // Increment
        count += 1;
    }

    // For range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("Fizzbuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }
}
