use std::collections::HashMap;

pub fn _hash_maps() {
    let mut person: HashMap<&str, i32>;
    person = HashMap::new();
    person.insert("Mehmet", 28);
    person.insert("Ahmet", 30);
    person.insert("Ali", 25);
    println!("Mehmet's age: {}", person["Mehmet"]);
    // This panics
    // println!("Ahmet's age: {}", person.get("None").unwrap());
}
