#![allow(dead_code)]

use std::sync::mpsc as sync;
use std::thread;

pub fn channels() {
    let (tx, rx) = sync::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        match tx.send(val) {
            Ok(..) => println!("Sent!"),
            Err(e) => println!("Error: {}", e),
        }
    });


    match rx.recv() {
        Ok(msg) => println!("Received: {}", msg),
        Err(e) => println!("Error: {}", e),
    }
}

