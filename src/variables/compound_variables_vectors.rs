pub fn _vectors() {
    // Vectors
    // Very similar to arrays but can grow and shrink in size
    // Vectors size value is demanded by compiler at compile time

    // Declaration
    // Compiler automatically infers the type and length of the vector
    let _some_vector = vec![1, 2, 3, 4, 5];

    // Explicit declaration
    let _some_vector2: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Accessing vector elements
    println!("{}", _some_vector[0]);

    // Iteration over a vector

    _some_vector.iter().for_each(|x| println!("{}", x));

    // Updating a vector
    let mut _some_vector3 = vec![1, 2, 3, 4, 5];
    _some_vector3[0] = 10;

    // Creating vector that contains N elements with value X
    let _some_vector4 = vec![0; 10];

    // Common methods of vectors

    println!(
        "Common methods of vectors,
        first: {:#?},
        last: {:#?},
        split_first: {:#?},
        split_last: {:#?},
        len: {:#?},
        is_empty: {:#?},
        contains: {:#?},
        ",
        _some_vector3.first(), // -> Returns the first element of the vector
        _some_vector3.last(),  // -> Returns the last element of the vector
        _some_vector3.split_first(), // -> Returns the first element of the vector as Some and the rest as None
        _some_vector3.split_last(), // -> Returns the last element of the vector as Some and the rest as None
        _some_vector3.len(),        // -> Returns the length of the vector
        _some_vector3.is_empty(),   // -> Returns [Boolean] value if vector is empty or not
        _some_vector3.contains(&1), // -> Returns [Boolean] value if vector contains the given element or not
    );

    // Pushing and popping elements to/from a vector

    // Pushing
    _some_vector3.push(6);

    // Popping
    _some_vector3.pop();

    // Slicing vectors

    // Slicing
    let _slice = &_some_vector3[0..3];

    // Slicing from start to end
    let _slice2 = &_some_vector3[..];
}
