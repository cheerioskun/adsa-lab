
use std::cmp::Ordering;
use std::cmp;
use std::fs;

pub enum BST<T: Ord> {
    Node {
        value: T,
        left: Box<BST<T>>,
        right: Box<BST<T>>
    },
    Empty
}

impl<T: Ord> BST<T> {
    // Create an empty BST
    pub fn new() -> Self {
        BST::Empty
    }

    // Inserts a new element into our BST. Only unique values.
    pub fn insert(&mut self, new_val: T) {
        match self {
            BST::Node {
                ref value,
                ref mut left,
                ref mut right,
            } => {
                match new_val.cmp(value) {
                    Ordering::Less => left.insert(new_val),
                    Ordering::Greater => right.insert(new_val),
                    Ordering::Equal => return
                }
            },
            BST::Empty => {
                *self = BST::Node {
                    value: new_val, 
                    left: Box::new(BST::Empty),
                    right: Box::new(BST::Empty)
                }
            }
        }
    }

    // Searches for an elem. Returns bool (if it is present)...
    pub fn find(&self, val: T) -> bool {
        match self {
            BST::Node {
                ref value,
                ref left,
                ref right
            } => {
                match val.cmp(value) {
                    Ordering::Equal => true,
                    Ordering::Less => left.find(val),
                    Ordering::Greater => right.find(val)
                }
            },
            BST::Empty => false
        }
    }

}

// Returns minimum value in the BST
fn find_min<T: Ord>(root: &BST<T>) -> Option<&T> {
    match root {
        BST::Node {
            ref value,
            ref left,
            right: _
        } => {
            match find_min(left) {
                Some(v) => Some(v),
                None => Some(value)
            }
        },
        BST::Empty => None
    }
}

// Returns the number of nodes on the longest path from root to a leaf
fn find_height<T: Ord>(root: &BST<T>) -> usize {
    match root {
        BST::Node {
            value: _,
            ref left,
            ref right
        } => {
            1 + cmp::max(find_height(left), find_height(right))
        }
        BST::Empty => 0
    }
}

fn main() {
    // Read input file
    let filename = "./data/input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    // Parse 
    let nums: Vec<&str> = contents.split_whitespace()
                                .collect();
    let nums: Vec<i32> = nums.iter()
                            .map(|s| s.trim().parse().unwrap())
                            .collect();
    let mut bst: BST<i32> = BST::new();
    for num in nums.iter() {
        bst.insert(*num);
    }
    println!("Height of BST is {}", find_height(&bst));
    println!("Minimum element is {}",find_min(&bst).unwrap());
}