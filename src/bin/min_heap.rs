struct MinHeap {
    heap: Vec<i32>,
}

impl MinHeap {
    fn new() -> Self {
        Self { heap: Vec::new() }
    }

    fn size(&self) -> usize {
        self.heap.len()
    }

    fn parent(current: usize) -> usize {
        (current - 1) / 2
    }

    fn left_child(current: usize) -> usize {
        2 * current + 1
    }

    fn right_child(current: usize) -> usize {
        2 * current + 2
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);
    }

    fn peek(&self) -> Option<&i32> {
        self.heap.first()
    }

    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    fn push(&mut self, value: i32) {
        self.heap.push(value);

        let mut current = self.size() - 1;

        while current > 0 {
            let parent = Self::parent(current);

            if self.heap[current] >= self.heap[parent] {
                break;
            }

            self.swap(parent, current);
            current = parent;
        }
    }

    fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        if self.size() == 1 {
            return self.heap.pop();
        }

        let min = self.heap[0];

        self.heap[0] = self.heap.pop().unwrap();

        self.heapify_down(0);

        Some(min)
    }

    fn heapify_down(&mut self, mut index: usize) {
        loop {
            let left = Self::left_child(index);
            let right = Self::right_child(index);

            let mut current = index;

            if self.size() > left && self.heap[current] > self.heap[left] {
                current = left;
            }

            if self.size() > right && self.heap[current] > self.heap[right] {
                current = right;
            }

            if current == index {
                break;
            }

            self.swap(current, index);
            index = current;
        }
    }
}

fn main() {
    let mut heap = MinHeap::new();

    heap.push(5);
    heap.push(3);
    heap.push(7);
    heap.push(1);

    println!("Heap: {:?}", heap.heap);
    println!("Min: {:?}", heap.peek());

    println!("Popped: {:?}", heap.pop());
    println!("Heap: {:?}", heap.heap);

    heap.push(-1);
    println!("Heap: {:?}", heap.heap);

    println!("Popped: {:?}", heap.pop());
    println!("Heap: {:?}", heap.heap);
}
