// Arrays - Fixed list whwere elements are the same data types

// Using std mem
use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    // Re-assigning a value
    numbers[2] = 20;

    // Get single val
    println!("Single value: {}", numbers[2]);

    // Get length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("This array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slices
    let slice: &[i32] = &numbers[0..2]; // Only gives 0-2 values in an array
    println!("Slice: {:?}", slice);
}
