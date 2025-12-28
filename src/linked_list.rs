struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T, next: Option<Box<Node<T>>>) -> Self {
        Self { value, next }
    }
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    fn push_back(&mut self, value: T) {
        let new_node = Box::new(Node::new(value, None));

        match &mut self.head {
            None => self.head = Some(new_node),
            Some(current) => {
                let mut current = current;
                while current.next.is_some() {
                    current = current.next.as_mut().unwrap();
                }
                current.next = Some(new_node);
            }
        }
    }

    fn pop_back(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        if self.head.as_ref().unwrap().next.is_none() {
            return self.pop_front();
        }

        let mut current = self.head.as_mut().unwrap();

        while current.next.as_ref().unwrap().next.is_some() {
            current = current.next.as_mut().unwrap();
        }

        current.next.take().map(|node| node.value)
    }

    fn len(&self) -> usize {
        let mut count: usize = 0;
        let mut current = &self.head;

        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }

        count
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn clear(&mut self) {
        self.head.take();
    }

    fn get_value(&self, index: usize) -> Option<&T> {
        let mut current = &self.head;

        for _ in 0..index {
            match current {
                Some(node) => {
                    current = &node.next;
                }
                None => return None,
            }
        }
        current.as_ref().map(|node| &node.value)
    }
}

