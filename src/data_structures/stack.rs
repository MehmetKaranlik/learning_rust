fn new_stack(capaticty:usize) -> Vec<i32> { 
    return Vec::with_capacity(capaticty);
}


fn push(stack: &mut Vec<i32>, value: i32) { 
    stack.push(value);
}

fn pop(stack: &mut Vec<i32>) -> Option<i32> { 
   return  stack.pop()
}

pub fn stack() { 
    let mut stack = new_stack(3);
    push(&mut stack, 1);
    push(&mut stack, 2);
    push(&mut stack, 3);
    push(&mut stack, 4);

    println!("Stack: {:?}", stack);

    let value = pop(&mut stack).unwrap();
    println!("Popped value: {:?}", value);
}