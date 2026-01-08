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

#[derive(Default)]
struct Queue<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: Display> Queue<T> {
    fn push(&mut self, value: T) {
        let new_node = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        let mut current = self.head.as_mut()?;

        while current.next.as_mut()?.next.is_some() {
            current = current.next.as_mut()?;
        }

        current.next.take().map(|node| node.value)
    }

    fn display(&self) {
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = node.next.as_ref();
        }

        println!("None");
    }
}

fn main() {
    let mut queue = Queue::default();

    queue.push(10);
    queue.push(20);
    queue.push(30);

    queue.display();

    queue.pop();

    queue.display();
}

