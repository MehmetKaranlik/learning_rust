
use std::num::ParseIntError;
#[allow(dead_code)]

/// Result enum simply a wrapper around Ok and Err
/// Which is returned by caller.
/// Result enum is used to handle errors.
fn _result_enum() {
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

#[allow(dead_code)]
pub fn question_mark() {
    let result = parse_str("10");
    println!("Result: {:?}", result);
    let result2 = parse_str("10a");
    println!("Result: {:?}", result2);
}

fn parse_str(input : &str) -> Result<i32,ParseIntError> {
    let integer =  input.parse::<i32>()?;
    println!("Integer: {}", integer);
    return Ok(integer);

}