use std::collections::VecDeque;

/// A last in, first out structure, i.e. a stack.
#[derive(Debug)]
pub struct Lifo<T>(Vec<T>);

/// A first in, first out structure, i.e. a queue.
#[derive(Debug)]
pub struct Fifo<T>(VecDeque<T>);

#[allow(dead_code)]
impl<T> Lifo<T> {
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, item: T) {
        self.0.push(item)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }
}

impl<T> From<T> for Lifo<T> {
    fn from(value: T) -> Self {
        let mut set = Lifo::new();
        set.push(value);
        set
    }
}

#[allow(dead_code)]
impl<T> Fifo<T> {
    pub const fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn push(&mut self, item: T) {
        self.0.push_back(item)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

impl<T> From<T> for Fifo<T> {
    fn from(value: T) -> Self {
        let mut set = Fifo::new();
        set.push(value);
        set
    }
}
