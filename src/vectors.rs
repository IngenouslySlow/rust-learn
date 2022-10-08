// Vectors are not fixed like arrays but has the same methods mostly

// Using std mem
use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    // Re-assigning a value
    numbers[2] = 20;

    // Add on to the vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    // Get single val
    println!("Single value: {}", numbers[2]);

    // Get length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("This array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slices
    let slice: &[i32] = &numbers[0..2]; // Only gives 0-2 values in an array
    println!("Slice: {:?}", slice);

    // Loop through the vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    // Print the vector
    println!("Numbers vec after mut: {:?}", numbers);
}
