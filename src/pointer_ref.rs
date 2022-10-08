// Reference pointers - Point to a resource in a memory

pub fn run() {
    // Primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

    /*  With non-primitives, Vectors for ex, if you assign variable to a piece of data, the first variable will no longer hold that value.
    You'll need to use a reference (&) to point to the resource. */
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1; // If we don't use & here vec1's value will no longer be available.

    println!("Vectors: {:?}", (&vec1, vec2));
}
