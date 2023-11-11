#[allow(dead_code)]
///-------------------------------
///         as_ref() and partial move in option and result
///
fn partial_move() {
    // let some_string = Some("Hello".to_owned());
    // match some_string {
    //     Some(s) => println!("{}", s), // some_string inner value is moved to s
    //     None => (),
    // }
    //
    //  println!("{:?}", some_string); // This will not compile because some_string is moved to s

    let some_string = Some("Hello".to_owned());
    match some_string {
        Some(ref s) => println!("{}", s), // some_string inner value is moved to s
        None => (),
    }

    println!("{:?}", some_string); // This will compile because match is using as_ref() which borrows the value

    //-----------------------------------
    // This will not compile because &some_string is &Option<String> and not &String
    //try_me(&some_string);
    // To borrow inner value value sending option enum to try_me() we need to use as_ref() which borrows the value
    try_me(some_string.as_ref());


}


fn try_me(string : Option<&String>) {
    match string {
        Some(s) => println!("{}", s),
        None => (),
    }
}
