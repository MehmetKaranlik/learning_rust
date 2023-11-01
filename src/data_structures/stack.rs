fn _new_stack(capacity: usize) -> Vec<i32> {
    return Vec::with_capacity(capacity);
}

fn _push(stack: &mut Vec<i32>, value: i32) {
    stack.push(value);
}

fn _pop(stack: &mut Vec<i32>) -> Option<i32> {
    return stack.pop();
}

pub fn _stack() {
    let mut stack = _new_stack(3);
    _push(&mut stack, 1);
    _push(&mut stack, 2);
    _push(&mut stack, 3);
    _push(&mut stack, 4);

    println!("Stack: {:?}", stack);

    let value = _pop(&mut stack).unwrap();
    println!("Popped value: {:?}", value);
}
