struct Node<T: Clone> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Node {
            data: self.data.clone(),
            next: self.next.clone(),
        }
    }
}

struct _LList<T: Clone> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
    length: i32,
}

impl<T: Clone> _LList<T> {
    pub fn _new() -> Self {
        _LList { head: None, tail: None, length: 0 }
    }
    pub fn _push(&mut self, data: T) {
        let node = Box::new(Node {
            next: None,
            data,
        });
        self.head.as_mut().unwrap().next = Some(node);
        self.length += 1;
    }
    pub fn _pop(&mut self) -> T {
        let current = self.head.take();
        return current.unwrap().data;
    }
    pub fn _peek(&self) -> Option<&T> {
        match self.head.as_ref() {
            Some(node) => Some(&node.data),
            None => None,
        }
    }
}


