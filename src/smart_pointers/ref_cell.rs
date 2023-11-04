/// RefCell<T> is a type that gives us mutability in an immutable context
/// RefCell<T> is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that

pub fn ref_cell() {
    use std::cell::RefCell;

    let x = RefCell::new(42);
    let y = x.borrow_mut();
    // let z = x.borrow_mut(); // This will give a runtime error
    println!("y is {}", y);

    let x = RefCell::new(42);
    let y = x.borrow();
    // As long as not mutable u can borrow multiple times
    let z = x.borrow();

    println!("y is {} , z is {}", y, z);
}