pub fn ownership_references_in_functions() { 
    let stack_num = 32;
    let mut heap_vec = vec![1,2,3];

    stack_function(stack_num);
    println!("Stack variable: {}", stack_num);
    //heap_function(heap_vec);
    // println!("Heap variable: {:?}", heap_vec); // This will not compile because heap_vec is moved to the heap_function
    heap_referencing_function(&mut heap_vec);

}



fn stack_function(mut var:i32) { // <- var is a copy of original variable, so updating does not effect the original one
    var += 10;
    println!("Stack function: {}", var);
}

fn heap_function(mut var:Vec<i32>) { // <- var is a moved owner of original variable, so it invalidates the original one
    var.push(4);
    println!("Heap function: {:?}", var);
}

fn heap_referencing_function(var:&mut Vec<i32>) { // <- var is a reference to the original variable, so updating effects the original one
    var.push(4);
    println!("Heap function: {:?}", var);
}