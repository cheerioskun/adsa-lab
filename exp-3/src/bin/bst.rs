
use std::Ordering;

pub enum BST<T: Ord> {
    Node {
        val: T,
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

    // Insert a new element into our BST. Only unique values.
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
                    left: BST::Empty,
                    right: BST::Empty
                }
            }
        }
    }

    // Search for an elem. Returns bool (if it is present)...
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


fn main() {
    // Read input file
    let filename = "./data/input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    // Parse 
    let nums: Vec<&str> = contents.split_whitespace()
                                .collect();
    let mut nums: Vec<i32> = nums.iter()
                            .map(|s| s.trim().parse().unwrap())
                            .collect();
    insertionsort(&mut nums[..]);