use std::fmt::{self, Display};

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T, next: Option<Box<Node<T>>>) -> Self {
        Self { value, next }
    }
}

impl<T: Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: fmt::Display> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
    }

    pub fn display(&self) {
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = node.next.as_ref();
        }
        println!("None")
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let current = self.head.take()?;
        self.head = current.next;

        Some(current.value)
    }

    pub fn push_back(&mut self, value: T) {
        if self.head.is_none() {
            return self.push_front(value);
        }

        let new_node = Box::new(Node::new(value, None));
        let mut current = self.head.as_mut().unwrap();

        while current.next.is_some() {
            current = current.next.as_mut().unwrap();
        }

        current.next = Some(new_node)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.head.as_ref()?.next.is_none() {
            return self.pop_front();
        }

        let mut current = self.head.as_mut()?;

        while current.next.as_ref()?.next.is_some() {
            current = current.next.as_mut()?;
        }

        current.next.take().map(|node| node.value)
    }

    pub fn len(&self) -> usize {
        let mut current = self.head.as_ref();
        let mut count: usize = 0;

        while let Some(node) = current {
            count += 1;
            current = node.next.as_ref();
        }

        count
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn peek_back(&self) -> Option<&T> {
        let mut current = self.head.as_ref()?;

        while let Some(node) = &current.next {
            current = node;
        }

        Some(&current.value)
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn clear(&mut self) {
        self.head.take();
    }

    pub fn insert_at_index(&mut self, index: usize, value: T) {
        if index == 0 {
            return self.push_front(value);
        }

        let mut current = self.head.as_mut();

        for _ in 0..(index - 1) {
            match current {
                None => return,
                Some(node) => {
                    current = node.next.as_mut();
                }
            }
        }

        if let Some(node) = current {
            let new_node = Box::new(Node::new(value, node.next.take()));
            node.next = Some(new_node)
        }
    }

    pub fn get_value(&self, index: usize) -> Option<&T> {
        let mut current = self.head.as_ref()?;

        for _ in 0..(index) {
            current = current.next.as_ref()?;
        }

        Some(&current.value)
    }

    pub fn remove_at_index(&mut self, index: usize) -> Option<T> {
        if index == 0 {
            return self.pop_front();
        }

        let mut current = self.head.as_mut()?;

        for _ in 0..(index - 1) {
            current = current.next.as_mut()?;
        }

        let removed_node = current.next.take()?;

        current.next = removed_node.next;
        Some(removed_node.value)
    }

    pub fn contains(&self, value: T) -> bool
    where
        T: PartialEq,
    {
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            if node.value == value {
                return true;
            }

            current = node.next.as_ref();
        }

        false
    }

    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.take();

        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = next;
        }

        self.head = prev;
    }
}
