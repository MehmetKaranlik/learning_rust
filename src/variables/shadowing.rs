

pub fn shadowing () { 
   // This is shadowing  while shadowing type of variable can be changed
    let n4: f32 = 12.1;
    println!("n4 is {}", n4);
    let mut n4 : f32 = 13.0;
    println!("n4 is {}", n4);
    n4 = 23.0;
    println!("n4 is {}", n4);
    let mut n4: i32 = 46;
    // n4 = "";  --> compile timer error 
    println!("n4 is {}", n4);
    // This will not give compile time error 
    n4 = 24;
    println!("n4 is {}", n4);
}   