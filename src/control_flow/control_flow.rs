//noinspection ALL,RsConstantConditionIf
pub fn _control_flow() {
    // Simple if as usual in every language

    if true {
        println!("This is true");
    } else if false {
        println!("This is false");
    } else {
    }

    // Simple loop
    let mut i = 0;
    loop {
        println!("Loop: {}", i);
        i += 1;
        if i == 1000 {
            break;
        }
    }

    // while loop as usual in every language

    let mut i = 0;
    while i < 10 {
        println!("While loop: {}", i);
        i += 1;
    }

    // for loop as usual in every language

    for i in 0..10 {
        println!("For loop: {}", i);
    }

    // for loop with continue and break

    for i in 0..10 {
        if i == 5 {
            continue;
        }
        if i == 8 {
            break;
        }
        println!("For loop with continue and break: {}", i);
    }

    // for loop with steps

    for i in (0..10).step_by(2) {
        println!("For loop with steps: {}", i);
    }

    // for loop with reverse

    for i in (0..10).rev() {
        println!("For loop with reverse: {}", i);
    }

    let vec = vec!["a", "b", "c", "d", "e"];

    // for loop with iterator
    for element in vec {
        println!("For loop with vector: {}", element);
    }

    let mut mutable_vec = vec!["a", "b", "c", "d", "e"];

    // for loop with mutable iterator
    // Changing values of the vector without transferring ownership [BLAZING FAST]
    for element in &mut mutable_vec {
        *element = "f";
        println!("For loop with mutable vector: {}", element);
    }

    // match statement as usual in every language (switch statement)

    let x = 5;
    match x {
        1 => println!("x is 1"),
        2 => println!("x is 2"),
        3 => println!("x is 3"),
        4 => println!("x is 4"),
        5 => println!("x is 5"),
        _ => println!("x is something else"),
    }
}
