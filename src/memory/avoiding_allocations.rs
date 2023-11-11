use std::mem::swap;
#[allow(dead_code)]
use std::mem::take;

#[derive(Debug)]
enum Customer {
    New { name: String },
    Loyal { name: String },
    Rich { name: String },
}

impl Customer {
    fn promote(&mut self) {
        *self = match self {
            Customer::New { name } => Customer::Loyal { name: take(name) },
            Customer::Loyal { name } => Customer::Rich { name: take(name) },
            Customer::Rich { .. } => return,
        }
    }

}


pub fn take_operation() {
    let mut customer = Customer::New { name: "John".to_owned() };
    let mut customer2 = Customer::Loyal { name: "Doe".to_owned() };
    customer.promote();
    customer2.promote();
    println!("{:?}", customer);
    println!("{:?}", customer2);
}

pub fn swap_operation() {
    let mut customer = Customer::New { name: "John".to_owned() };
    let mut customer2 = Customer::Loyal { name: "Doe".to_owned() };
    println!("{:?}", customer);
    println!("{:?}", customer2);
    swap(&mut customer, &mut customer2);
    println!("{:?}", customer);
}