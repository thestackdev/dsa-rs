use std::cmp::*;
use std::collections::VecDeque;
use std::fmt;

#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Debug)]
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: fmt::Display> fmt::Display for TreeNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T: Ord + fmt::Display> TreeNode<T> {
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

    fn height(&self) -> i32 {
        let left_height = self.left.as_ref().map_or(-1, |node| node.height());
        let right_height = self.right.as_ref().map_or(-1, |node| node.height());

        1 + max(left_height, right_height)
    }

    fn inorder(&self) {
        if let Some(left) = &self.left {
            left.inorder();
        }
        print!("{} ", self);
        if let Some(right) = &self.right {
            right.inorder();
        }
    }

    fn preorder(&self) {
        print!("{} ", self);
        if let Some(left) = &self.left {
            left.preorder();
        }
        if let Some(right) = &self.right {
            right.preorder();
        }
    }

    fn postorder(&self) {
        if let Some(left) = &self.left {
            left.postorder();
        }
        if let Some(right) = &self.right {
            right.postorder();
        }
        print!("{} ", self);
    }

    fn search_node(&self, value: &T) -> bool {
        match self.value.cmp(value) {
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

impl<T: Ord + Clone + fmt::Display> BinarySearchTree<T> {
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

    fn level_order(&self) {
        if let Some(ref root) = self.root {
            let mut queue = VecDeque::new();

            queue.push_back(root);

            while let Some(node) = queue.pop_front() {
                print!("{} ", node.value);

                if let Some(ref left) = node.left {
                    queue.push_back(left);
                }
                if let Some(ref right) = node.right {
                    queue.push_back(right);
                }
            }
        }
    }

    fn search(&self, value: &T) -> bool {
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

    fn delete(&mut self, value: &T) -> bool {
        let mut deleted = false;
        self.root = Self::delete_node(self.root.take(), value, &mut deleted);
        deleted
    }

    fn delete_node(
        node: Option<Box<TreeNode<T>>>,
        value: &T,
        deleted: &mut bool,
    ) -> Option<Box<TreeNode<T>>> {
        match node {
            None => None,
            Some(mut n) => match value.cmp(&n.value) {
                Ordering::Less => {
                    n.left = Self::delete_node(n.left.take(), value, deleted);
                    Some(n)
                }
                Ordering::Greater => {
                    n.right = Self::delete_node(n.right.take(), value, deleted);
                    Some(n)
                }
                Ordering::Equal => {
                    *deleted = true;

                    if n.left.is_none() && n.right.is_none() {
                        return None;
                    }

                    if n.left.is_none() {
                        return n.right.take();
                    }

                    if n.right.is_none() {
                        return n.left.take();
                    }

                    if let Some(min_value) = Self::find_min(&n.right) {
                        n.value = min_value;
                        n.right = Self::delete_node(n.right.take(), value, &mut false)
                    }

                    Some(n)
                }
            },
        }
    }

    fn find_min(node: &Option<Box<TreeNode<T>>>) -> Option<T> {
        node.as_ref().and_then(|n| {
            if n.left.is_none() {
                Some(n.value.clone())
            } else {
                Self::find_min(&n.left)
            }
        })
    }

    fn inorder(&self) {
        println!("In Order: ");
        if let Some(ref node) = self.root {
            node.inorder();
        }
        println!();
    }

    fn preorder(&self) {
        println!("Pre Order: ");
        if let Some(ref node) = self.root {
            node.preorder();
        }
        println!();
    }

    fn postorder(&self) {
        println!("Post Order: ");
        if let Some(ref node) = self.root {
            node.postorder();
        }
        println!();
    }

    fn height(&self) -> i32 {
        self.root.as_ref().map_or(-1, |node| node.height())
    }
}

fn main() {
    let mut bst = BinarySearchTree::new();

    bst.insert(100);
    bst.insert(200);
    bst.insert(600);

    println!("Searching for 300: {}", bst.search(&300));

    bst.insert(300);
    bst.insert(400);
    bst.insert(500);
    bst.insert(50);

    println!("Searching for 300: {}", bst.search(&300));
    println!("Searching for 700: {}", bst.search(&700));

    if let Some(min_node) = bst.min() {
        println!("Min Node is {}", min_node);
    }
    if let Some(max_node) = bst.max() {
        println!("Max Node is {}", max_node);
    }

    println!("\nDeleting 300: {}", bst.delete(&300));
    println!("Searching for 300 after delete: {}", bst.search(&300));

    bst.inorder();
    bst.preorder();
    bst.postorder();

    println!("Height of the tree is {}", bst.height());

    println!("Level Order");
    bst.level_order();
}
