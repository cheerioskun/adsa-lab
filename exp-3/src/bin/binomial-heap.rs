use std::fs;
use std::cmp::Ordering;


pub enum Error {
    kDegreeMismatch,
    kEmptyTree
}
// Data Structure for a heap ordered binomial tree
// A binomial tree of degree k will have exactly 2^k nodes
// Each of the k - 1 children of the root will have degree i
pub enum BinomialTree<T: Ord> {
    Node {
        value: T,
        degree: usize,
        children: Vec<Box<BinomialTree<T>>>
    },
    Empty
}

impl<T: Ord> BinomialTree<T> {
    // Create new empty tree
    pub fn new() -> BinomialTree<T> {
        BinomialTree::Empty
    }

    // Create new tree with single value
    pub fn new(val: T) -> BinomialTree<T> {
        BinomialTree::Node { 
            value: val,
            degree: 0,
            children: Vec::new()
        }
    }

    // No need to implement insert. Inserting into a binomial tree will not be supported...

    // Meld two binomial trees of degree k - 1 into one tree of degree k
    // Consumes both tree objects and returns another tree as a result
    pub fn meld(mut self, b: mut BinomialTree<T>) -> Result<self, Error> {
        match self {
            BinomialTree::Node{
                ref value_a: value,
                mut ref degree_a: degree,
                mut ref children_a: children
            } => {
                match b {
                    BinomialTree::Node{
                        ref value_b: value,
                        mut ref degree_b: degree
                        mut ref children_b: children
                    } => {
                        if degree_a != degree_b{
                            Err(Error::kDegreeMismatch)
                        } else {
                            match value_a.cmp(value_b) {
                                Ordering::Less => {
                                    children_a.push(Box::new(b));
                                    degree_a += 1;
                                    Ok(self)
                                },
                                (_) => {
                                    children_b.push(Box::new(self));
                                    degree_b += 1;
                                    Ok(b)
                                }
                            }
                        }
                    },
                    BinomialTree::Empty => Err(Error::kEmptyTree)
                }
            },
            BinomialTree::Empty => Err(Error::kEmptyTree)
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
    let nums: Vec<i32> = nums.iter()
                            .map(|s| s.trim().parse().unwrap())
                            .collect();
    let mut bst: BST<i32> = BST::new();
    for num in nums.iter() {
        bst.insert(*num);
    }
}