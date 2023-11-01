pub fn _compound_variables() {
    _strings();
    _tuples();
    _arrays();
    _slices();
}

//noinspection ALL,SpellCheckingInspection
fn _strings() {
    //      Strings
    //       - Fixed length strings (&str) aka String slices
    //       - Growable strings (String)
    // Declarations
    let x: &str = "Fixed length string ";
    let mut y: String = String::from("Growable string ");
    let _empty_string = String::new();
    // Adding to string
    y.push(char::from('!'));

    // Removes the last character
    y.pop();

    // Concatenating to string
    y.push_str("some string");
    println!("{} {}", x, y);

    let _string1 = String::from("Hello, ");
    let _string2 = String::from("world!");
    let _string3 = format!("{} {}", _string1, _string2);
    println!("{}", _string3);

    println!(
        "Basic String Methods,
        is_empty: {:#?},
        contains: {:#?},
        replace: {:#?},
        Bytes: {:#?},
        split: {:#?},
        trim: {:#?},
        to_lowercase: {:#?},
        to_uppercase: {:#?},
        len: {:#?}
        Capacity: {:#?}
        ",
        y.is_empty(),         //-> Returns [Boolean] value if string is empty or not
        y.contains("string"), //-> Returns [Boolean] value if string contains the given string or not
        y.replace("string", "stringgg"), //-> Replaces the given string with the given string
        y.bytes(),            // -> // Converts the string to bytes
        y.split(" "),         //->  Splits the string with the given string
        y.trim(),             //->  Removes the white spaces from the string
        y.to_lowercase(),     // -> //  Converts the string to lowercase
        y.to_uppercase(),     // -> Converts the string to uppercase
        y.len(),              // -> Returns the length of the string
        y.capacity(),         // -> Returns the capacity of the string
    );
}

fn _tuples() {
    // Tuples
    // Tuples are fixed length
    // Tuples can contain different types
    // Tuples can be used as function arguments and as return values
    let _some_tuple = (1, "hello");
    println!("{} {}", _some_tuple.0, _some_tuple.1);
    // Another to print
    println!("{:?}", _some_tuple);

    // Assigning tuple to another variable , compiler will automatically assign the type and position
    let _some_tuple2 = _some_tuple;

    // There is also nested tuples which i will never ever use
    let _nested_tuple = ((1, 2), _some_tuple);

    // Empty tuple
    let _empty_tuple = ();
}

fn _arrays() {
    // Arrays
    // Arrays can contain only same types
    // Arrays are stored in stack but they are just meta data that stores length,capacity and pointer to the heap

    // Declaration
    // Compiler automatically infers the type and length of the array
    let _some_array = [1, 2, 3, 4, 5];
    // Explicit declaration
    let _some_array2: [i32; 5] = [1, 2, 3, 4, 5];

    // Accessing array elements

    println!("{}", _some_array[0]);

    // Iteration over an array
    _some_array.iter().for_each(|x| println!("{}", x));

    // Updating an array
    let mut _some_array3 = [1, 2, 3, 4, 5];
    _some_array3[0] = 10;

    // Common methods of arrays
    println!(
        "Common methods of arrays,
        first: {:#?},
        last: {:#?},
        split_first: {:#?},
        split_last: {:#?},
        len: {:#?},
        is_empty: {:#?},
        contains: {:#?},
        ",
        _some_array3.first(),       // -> Returns the first element of the array
        _some_array3.last(),        // -> Returns the last element of the array
        _some_array3.split_first(), // -> Returns the first element of the array as Some and the rest as None
        _some_array3.split_last(), // -> Returns the last element of the array as Some and the rest as None
        _some_array3.len(),        // -> Returns the length of the array
        _some_array3.is_empty(),   // -> Returns [Boolean] value if array is empty or not
        _some_array3.contains(&1), // -> Returns [Boolean] value if array contains the given element or not
    );

    // Byte checking
    println!(
        "The array occupying {} bytes",
        std::mem::size_of_val(&_some_array3)
    );
}

// I LOVE SLICES ❤️
// Slices are subset of elements in the array
// But they are not copy of the elements in the array
// They are just a pointer to the elements in the array
fn _slices() {
    let mut _some_array = [1, 2, 3, 4, 5];
    // Slice declaration
    // Compiler automatically infers the type and length of the slice
    // And this declaration is exclusive
    let _slice = &_some_array[0..2];
    // This is inclusive
    let _slice2 = &_some_array[0..=2];
    println!("{:?}", _slice);
}
