/*
           Memory is divided into 4 parts on allocated to a program

           -----------------Heap-----------------

           -----------------Stack----------------
           Information regarding stack trace
           Local variables & primitive type variables.

           -----------------Code-----------------
           Programs instructions and code are stored in this section of memory

           -----------------Static/Global-----------------
           These variable are stored in the static memory of the program
           These variables are valid for the entire program
           These variables are accessible from anywhere in the program
*/

fn _memory() {
    _stack();
    _heap();
}

fn _heap() {
    // Heap is the memory which is used to store data which is not known at compile time
    // Heap is used to store data which is created at runtime && data which is mutable in nature
    // Heap is less organized than stack, things are not added in sequential order.
    // When you put data on heap, u request certain amount of space.
    // OS finds empty spot in heap that is big enough, marks it as being in use, and returns a pointer

    let s1 = String::from("Hello");
    let s2 = s1; // <- moving owner ship, its not copied its reference is moved to s2 and s1 is invalidated at this point
    //let s3 = &s2; // <- s3 is a reference to s2, no new memory for s3 is allocated
    let _s4 = s2.clone(); // <- s4 is a clone of s2, s2 is still valid at this point, another piece of memory is allocated for s4

    // Rust does have special method for memory management called drop
    // Drop is called when a variable goes out of scope
}

fn _stack() {
    // All the data stored in stack must have a known fixed size at compile time
    // When the program executes some amount memory from stack is allocated for execution of main function
    // All the local variables and arguments to different functions stored in this stack memory
    // Size of stack frame for a method or function is calculated at compile time.

    // Every step below adds allocated memory to stack step by step
    let (x, y) = (5, 6);
    let sum = _square_sum(x, y);
    println!("Sum of squares of {} and {} is {}", x, y, sum);

    // When the all functions above are executed the stack frame is removed from the stack
    // And if the memory goes beyond the allocated memory then stack overflow error occurs
    // One common case for stack overflow is declaring variables in infinite loop
}

fn _square_sum(x: i32, y: i32) -> i32 {
    let x_squared = _square(x);
    let y_squared = _square(y);
    x_squared + y_squared
}

fn _square(x: i32) -> i32 {
    x * x
}
