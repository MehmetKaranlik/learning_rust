
pub fn compound_variables() { 

//      Strings
//       - Fixed length strings (&str) aka String slices
//       - Growable strings (String)
    let x :&str= "Fixed length string ";
    let mut y :String= String::from("Growable string ");
    y.push(char::from('!'));
    println!("{} {}", x, y);
    y.pop();
    println!("{} {}", x, y);
    y.push_str("some string");
    println!("{} {}", x, y);
    
   /* println!(
        "Basic String Methods,
        is_empty: {},
        contains: {},
        replace: {},
        Bytes: {:?},
        split: {:?},
        trim: {},
        to_lowercase: {},
        to_uppercase: {},
        len: {}
        Capacity: {}
        ",
        y.is_empty(), -> Returns [Boolean] value if string is empty or not
        y.contains("string"), -> Returns [Boolean] value if string contains the given string or not
        y.replace("string", "stringgg"),  -> Replaces the given string with the given string
        y.bytes(), -> Converts the string to bytes
        y.split(" "), -> Splits the string with the given string
        y.trim(), -> Removes the white spaces from the string
        y.to_lowercase(), -> Converts the string to lowercase
        y.to_uppercase(), -> Converts the string to uppercase
        y.len(), -> Returns the length of the string
        y.capacity(), -> Returns the capacity of the string
    ); */

 

    
}