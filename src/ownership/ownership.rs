
pub fn ownership() {
    /* ------------------------------------------
                 Rust Ownership
                    - Each value in Rust has a variable that's called its owner.
                    - There can only be one owner at a time.
                    - When the owner goes out of scope, the value will be dropped.

                    Owner 
                        - Owner is the variable that holds the value.
                        - Owner is the only variable that can access the value.
                        - When the owner goes out of scope, the value will be dropped.
                        - Owner is always valid.

                    Copying 
                        - Carrying value of one variable to another variable.
                        - After copying, both variables will be valid.
                        - For primitive types, copying is done automatically.

                    Moving     
                        - Carrying pointer of one variable to another variable.
                        - After moving, the previous variable will be invalidated.
                        - For non-primitive types, moving is done automatically.
       
     ------------------------------------------ */

    // Note : Primitive and Non-Primitive types are not data types, they are variable types!
    // Primitive types cannot be empty and does have fixed size
    // Non-Primitive types can be empty and does not have fixed size, can grow and shrink
    // Since primitives does have fixed size and cant grow and shrink storing and handling them relatively easy
    // Non - primitive types need to computationally calculate the size of data so its costly to recreate and store them.
    // That's why Rust does not copy non-primitive types to the new variable, it moves them to the new variable
    // Primitive types include : integers,floats,bool,arrays,chars and others.
    // Non-primitive types include : String, Vectors, Hashmaps and others.

    primitive_types();
    non_primitive_types();
}




fn primitive_types() { 
    let x = 5; // x is the owner of the value 5
    let y = x; // y is the owner of the value 5
    println!("x: {}, y: {}", x, y);
    // x and y are correctly compiled because primitive types are copied to the new variable

}


fn non_primitive_types() { 
    let s1 = String::from("hello"); // s1 is the owner of the value "hello"
    //let s2 = s1; // s2 is the owner of the value "hello"
    // println!("s1: {}, s2: {}", s1, s2); // This will not compile because non primitive types are moved to the new variable 
    // So we cannot use s1 while owner of the particular value is s2
    let s2 = &s1;
    println!("s1: {}, s2: {}", s1, s2); // This will compile because s1 is borrowed to s2
    // Borrowing means simply just creating a reference to the value, not moving it to the new variable3

    /*
    s2 = s1 --> moving ownership
    s2 = &s1 --> borrowing ownership (creating reference to original memory address)
     */

    let vec1 = vec![1,2,3,4,5];
    // Creating new value from the old value is called cloning
    // Cloning is done by calling clone() method
    let vec2 = vec1.clone();
    // Now they are 2 different variables
    // And they are both owners of the value with different memory addresses
    

}