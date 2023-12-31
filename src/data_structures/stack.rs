#![allow(unused)]
struct StackNode<T: Copy> {
    data: T,
    prev: Option<Box<StackNode<T>>>,

}


struct Stack<T: Copy> {
    head: Option<Box<StackNode<T>>>,
    length: usize,
}

impl<T: Copy> Stack<T> {
    fn push(&mut self, data: T) {
        let node = StackNode {
            data: data,
            prev: self.head.take(),
        };
        self.head = Some(Box::new(node));
        self.length += 1;
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(node) => {
                self.head = node.prev;
                self.length -= 1;
                Some(node.data)
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<T> {
        match self.head.as_ref() {
            Some(node) => Some(node.data.clone()),
            None => None,
        }
    }
}