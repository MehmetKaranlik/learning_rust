use std::fmt::Display;

pub fn _basic_function() {
    println!("Hello from basic function");
}

pub fn _function_with_parameters(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

pub fn _function_with_return_value(x: i32, y: i32) -> i32 {
    x + y
}

pub fn _function_with_multiple_returns(x: i32, y: i32) -> (i32, i32) {
    let first_value = x + y;
    let second_value = x - y;
    return (first_value, second_value);
}

pub fn _generic_function<T: Display>(x: T) {
    println!("Generic function with parameter: {}", x);
}

pub fn _generic_function_with_multiple_generic_types<T: Display, U: Display>(x: T, y: U) {
    println!("Generic function with multiple types: {} {}", x, y);
}

pub fn _function_with_closure<F>(closure: F)
where
    F: Fn(),
{
    closure();
}

pub fn _function_with_closure_and_parameters<F>(closure: F)
where
    F: Fn(i32, i32),
{
    closure(1, 2);
}

pub fn _function_mutating_parameters(x: &mut i32) {
    *x += 1;
}
