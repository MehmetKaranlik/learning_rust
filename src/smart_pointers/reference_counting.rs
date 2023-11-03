/// RC: Reference Counting Smart Pointer
/// Rc<T> enables multiple ownership of the same data.
/// It keeps track of the number of references to a value which determines whether or not a value is still in use.
/// If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

use std::rc::Rc;

pub fn rc() {
    /// This one wont compile because multiple ownershit is restricted for Rust
    //let x = String::from("Hello");
    //let y = x;
    //let z = x;

    /// This one will compile because Rc<T> enables multiple ownership of the same data.
    /// Even thou rc is cloned with Rc::clone() method, it doesn't copy the data
    let x = Rc::new(String::from("Hello"));
    println!("Reference count {:?}", Rc::strong_count(&x)); // -> 1
    let y = Rc::clone(&x);
    println!("Reference count {:?}", Rc::strong_count(&x)); // -> 2
    let z = Rc::clone(&x);
    println!("Reference count {:?}", Rc::strong_count(&x)); // -> 3
}

// After scope ends since there is no more valid ref for x it will be dropped
// Along with y, z and the data destroyed , memory space freed.


/// We can also define weak references over same value,
/// Main difference between weak and strong references is that weak references doesn't increment the reference count.
/// So if we drop the strong reference, weak reference will be invalid.
/// We can check if weak reference is valid by using Rc::weak_count() method.
pub fn rc_weak() {
    let x = Rc::new(String::from("Hello"));
    let y = Rc::downgrade(&x);
    println!("Reference count {:?}", Rc::strong_count(&x)); // -> 1
    println!("Weak reference count {:?}", Rc::weak_count(&x)); // -> 1
    let z = Rc::clone(&x);
    println!("Reference count {:?}", Rc::strong_count(&x)); // -> 2
}