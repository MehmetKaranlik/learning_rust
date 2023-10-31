/// Result enum simply a wrapper around Ok and Err
/// Which is returned by caller.
/// Result enum is used to handle errors.
fn result_enum() {
    let result = divide(10, 2);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("Cannot divide by zero"));
    }
    return Ok(a / b);
}