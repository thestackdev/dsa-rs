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

    fn insert_at_index(&mut self, index: usize, value: T) {
        if index == 0 {
            return self.push_front(value);
        }

        if self.head.is_none() && index > 0 {
            return;
        }

        let mut current = self.head.as_mut().unwrap();

        for _ in 0..(index - 1) {
            match current.next.as_mut() {
                Some(node) => current = node,
                None => return,
            }
        }

        let new_node = Box::new(Node::new(value, current.next.take()));
        current.next = Some(new_node);
    }

    fn remove_at_index(&mut self, index: usize) -> Option<T> {
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

    fn peek_back(&self) -> Option<&T> {
        let mut current = self.head.as_ref()?;

        while current.next.is_some() {
            current = current.next.as_ref()?;
        }

        Some(&current.value)
    }

    fn contains(&self, value: T) -> bool
    where
        T: PartialEq,
    {
        let mut current = &self.head;

        while let Some(node) = current {
            if node.value == value {
                return true;
            }
            current = &node.next;
        }

        false
    }

    fn reverse(&mut self) {
        let mut current = self.head.take();
        let mut prev = None;

        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = next;
        }

        self.head = prev;
    }
}
