/// POINTERS(Reference): A variable that holds address of specific memory location.
/// By using that address we can access the data stored in that memory location.
///
/// SMART POINTERS: A data structure that containts a pointer, also additional metada and capabilities.
/// There are many different kinds of smart pointers in rust.
/// The most common ones are: Box, Rc, RefCell, Arc, Mutex, Cell, Cow, etc.


pub fn smart_pointers() {
    boxes();
}


/// Box: A smart pointer variable itself is stored on the stack,
/// but the data it points to is stored on the heap.
fn boxes() {
    let single_box = Box::new(0.625);
    println!("Single box: {}", single_box);
    /// Even thou box's value and variable x is same;
    /// box's value is stored on the heap, but x is stored on the stack.
    /// Because x is a primitive type and it's size is known at compile time.
    let x = 0.625;
    println!("Size of x in bytes: {}", std::mem::size_of_val(&x)); // 8 bytes
    println!("Size of single_box in bytes: {}", std::mem::size_of_val(&single_box)); // 8 bytes
}