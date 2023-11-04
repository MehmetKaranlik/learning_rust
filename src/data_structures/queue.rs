#![allow(unused)]

use std::rc:: { Rc};

struct Queue<T> {
    data: Vec<T>,
    head: Rc<T>,
    length: usize,
}

impl<T: Copy> Queue<T> {
    pub fn enqueue(&mut self, data: T) {
        self.head = Rc::new(data);
        self.data.push(*self.head);
        self.length += 1;
    }

    pub fn dequeue(&mut self) {
        if self.length == 0 {
            return;
        }
        self.data.remove(self.data.len() - 1);
        match self.data.last() {
            Some(&data) => self.head = Rc::new(data),
            None => (),
        }
        self.length -= 1;
    }

    pub fn peek(&self) -> Option<T> {
        if self.length == 0 {
            return None;
        }
        match self.data.last() {
            Some(&data) => Some(data),
            None => None,
        }
    }
}