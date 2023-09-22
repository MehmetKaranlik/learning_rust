unsafe fn option_enum() {
    let mut value: Option<String> = None;
    value = Some(String::from("hello"));
    match value {
        Some(ref str) => println!("Value: {}", str),
        None => println!("No value"),
    }

    ///Returns wrapped value and panics if there is no value
    let output = value.clone().unwrap();

    ///Returns wrapped value and returns default if there is no value
    value.clone().unwrap_or(String::from("default value"));
    /// Returns wrapped value and returns default if there is no value, but default value is calculated lazily
    value.clone().unwrap_or_else(|| String::from("default value"));
    /// Returns wrapped value and its unsafe to call this method if there is no value
    /// If [Option::None] is return, this method will return undefined behavior
    /// Its only usable in unsafe blocks and functions
    value.clone().unwrap_unchecked();
}