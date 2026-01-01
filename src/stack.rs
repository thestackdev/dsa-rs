use std::fmt::Display;

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T, next: Option<Box<Node<T>>>) -> Self {
        Self { value, next }
    }
}

pub struct Stack<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self { head: None }
    }
}

impl<T> Stack<T> {
    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        let removed_node = self.head.take()?;
        self.head = removed_node.next;

        Some(removed_node.value)
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut count: usize = 0;
        let mut current = &self.head;

        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }

        count
    }

    pub fn display(&self)
    where
        T: Display,
    {
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            print!("{} <- ", node.value);
            current = node.next.as_ref();
        }
    }
}
