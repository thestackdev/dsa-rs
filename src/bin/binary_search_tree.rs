use std::cmp::*;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Ord> TreeNode<T> {
    fn insert(&mut self, value: T) {
        match self.value.cmp(&value) {
            Ordering::Less => {
                if let Some(ref mut right) = self.right {
                    right.insert(value);
                } else {
                    self.right = Some(Box::new(TreeNode {
                        value,
                        left: None,
                        right: None,
                    }))
                }
            }
            Ordering::Greater => {
                if let Some(ref mut left) = self.left {
                    left.insert(value);
                } else {
                    self.left = Some(Box::new(TreeNode {
                        value,
                        left: None,
                        right: None,
                    }))
                }
            }
            Ordering::Equal => {}
        }
    }

    fn search_node(&self, value: T) -> bool {
        match self.value.cmp(&value) {
            Ordering::Less => {
                if let Some(ref right) = self.right {
                    right.search_node(value)
                } else {
                    false
                }
            }
            Ordering::Greater => {
                if let Some(ref left) = self.left {
                    left.search_node(value)
                } else {
                    false
                }
            }
            Ordering::Equal => true,
        }
    }

    fn min_node(&self) -> Option<&T> {
        if let Some(ref left) = self.left {
            left.min_node()
        } else {
            Some(&self.value)
        }
    }

    fn max_node(&self) -> Option<&T> {
        if let Some(ref right) = self.right {
            right.max_node()
        } else {
            Some(&self.value)
        }
    }
}

struct BinarySearchTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: Ord> BinarySearchTree<T> {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&mut self, value: T) {
        if let Some(ref mut node) = self.root {
            node.insert(value);
        } else {
            self.root = Some(Box::new(TreeNode {
                value,
                left: None,
                right: None,
            }))
        }
    }

    fn search(&self, value: T) -> bool {
        if let Some(ref node) = self.root {
            node.search_node(value)
        } else {
            false
        }
    }

    fn min(&self) -> Option<&T> {
        if let Some(ref node) = self.root {
            node.min_node()
        } else {
            None
        }
    }

    fn max(&self) -> Option<&T> {
        if let Some(ref node) = self.root {
            node.max_node()
        } else {
            None
        }
    }
}

fn main() {
    let mut bst = BinarySearchTree::new();

    bst.insert(100);
    bst.insert(200);
    bst.insert(600);

    println!("Searching for 300 {}", bst.search(300));

    bst.insert(300);
    bst.insert(400);
    bst.insert(500);
    bst.insert(50);

    println!("Searching for 300 {}", bst.search(300));
    println!("Searching for 700 {}", bst.search(700));

    if let Some(min_node) = bst.min() {
        println!("Min Node is {}", min_node);
    }
    if let Some(max_node) = bst.max() {
        println!("Max Node is {}", max_node);
    }
}
