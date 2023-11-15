pub mod channels;
mod async_await;

use std::time::Duration;

#[allow(dead_code)]


pub fn spawning() {
    println!("From main thread");
    std::thread::spawn(||{
        println!("From thread");
    });
}

#[allow(dead_code)]
pub fn sleep() {
    std::thread::sleep(Duration::from_secs(1));
}

#[allow(dead_code)]
pub fn ownership_in_multithreading(){
    let v = vec![1,2,3];
    for e in v {
        std::thread::spawn(move || {
            println!("Here's a vector: {:?}", e);
        });
    }
}