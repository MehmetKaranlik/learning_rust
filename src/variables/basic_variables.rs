

pub fn variables() {

    // Variables can be type annotated.

    let _logical: bool = true;

    let _a_float: f64 = 1.0;  // Regular annotation

    let _an_integer = 5i32; // Suffix annotation

    // Or a default will be used.

    let _default_float = 3.0; // `f64`
    
    let _default_integer = 7; // `i32`

    // A type can also be inferred from context

    let mut _inferred_type = 12; // Type i64 is inferred from another line

    _inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.

    let mut _mutable = 12; // Mutable `i32`

    _mutable = 21; 

    let _immutable = 12;

    // immutable = 21; --> This will give a compile-time error

    let n = 1;
    let n2 = 12.1;
    // Rust will automatically convert n to float
    // But this conversion only applied for this equation only, n is still an integer
    let n3 = n+n2 as i32;
    println!("n3 is {}", n3);



}