fn shadowing () { 
   // This is shadowing  while shadowing type of variable can be changed
    let n4: f32 = 12.1;
    let mut n4 : f32 = 13;
    let n4 = 23;
    let mut n4 = 46;
    n4 = "";
    
    // This will not give compile time error 
    n4 = 24;

    println!("n4 is {}", n4);
}