fn basicVariables() {

    // Variables can be type annotated.

    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation

    let an_integer = 5i32; // Suffix annotation

    // Or a default will be used.

    let default_float = 3.0; // `f64`
    
    let default_integer = 7; // `i32`

    // A type can also be inferred from context

    let mut inferred_type = 12; // Type i64 is inferred from another line

    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.

    let mut mutable = 12; // Mutable `i32`

    mutable = 21; 

    let unmutable = 12;

    // This will give a compile-time error
    // unmutable = 21;

    let n = 1;
    let n2 = 12.1;
    // Rust will automatically convert n to float
    // But this conversion only applied for this equation only, n is still an integer
    let n3 = n+n2;



}