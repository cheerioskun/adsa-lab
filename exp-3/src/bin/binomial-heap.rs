use std::fs;
use std::cmp::Ordering;

#[derive(Debug)]
pub enum Error {
    DegreeMismatch,
    EmptyTree,
    AddToEmpty
}


// Data Structure for a heap ordered binomial tree
// A binomial tree of degree k will have exactly 2^k nodes
// Each of the k - 1 children of the root will have degree i
pub struct Node<T: Ord> {
    value: T,
    degree: usize,
    children: Vec<Box<BinomialTree<T>>>
}
pub enum BinomialTree<T: Ord> {
    Node(Node<T>),
    Empty
}

impl<T: Ord> BinomialTree<T> {
    // Create new tree with single value
    pub fn new(val: T) -> Self {
        BinomialTree::Node(
            Node{
                value: val,
                degree: 0,
                children: Vec::new()
            }
        ) 
        
    }

    pub fn get_min(&self) -> Option<&T> {
        match self {
            BinomialTree::Node(node) => Some(&node.value),
            BinomialTree::Empty => None
        }
    }

    // No need to implement insert. Inserting into a binomial tree will not be supported...
    // Meld two binomial trees of degree k - 1 into one tree of degree k
    pub fn meld(self, b: BinomialTree<T>) -> Result<BinomialTree<T>, Error>{
        match (self, b) {
            (BinomialTree::Node(mut node_a), BinomialTree::Node(mut node_b)) => {
                if node_a.degree != node_b.degree {
                    return Err(Error::DegreeMismatch);
                }
                match node_a.value.cmp(&node_b.value) {
                    // Maintain min-heap property
                    Ordering::Less => {
                        node_a.children.push(Box::new(BinomialTree::Node(node_b)));
                        node_a.degree += 1;
                        return Ok(BinomialTree::Node(node_a));
                    },
                    _ => {
                        node_b.children.push(Box::new(BinomialTree::Node(node_a)));
                        node_b.degree += 1;
                        return Ok(BinomialTree::Node(node_b));
                    }
                }
            },
            (_, _) => Err(Error::EmptyTree)
        }
    }
}

pub struct BinomialHeap<T: Ord> {
    node_list: Vec<BinomialTree<T>>,
    n: usize
} 

impl<T: Ord> BinomialHeap<T> {
    pub fn new() -> Self {
        BinomialHeap {
            node_list: Vec::new(),
            n: 0
        }
    }
    pub fn insert_tree(&mut self, k: usize, tree: BinomialTree<T>) -> Result<(), Error>{
        if k == self.n {
            self.node_list.push(tree);
            self.n += 1;
            Ok(())
        } else {
            match &self.node_list[k] {
                BinomialTree::Node(_) => {
                    let mut temp_tree = std::mem::replace(&mut self.node_list[k], BinomialTree::Empty);
                    temp_tree = temp_tree.meld(tree)?;
                    self.insert_tree(k + 1, temp_tree)?;
                },
                BinomialTree::Empty => {
                    self.node_list[k] = tree;
                }
            }
            Ok(())
        }
    }
    
    pub fn insert(&mut self, value: T) -> Result<(), Error>{
        return self.insert_tree(0, BinomialTree::new(value));
    }

    pub fn find_min(&self, k: usize) -> Option<&T> {
        if k >= self.n {
            None
        } else {
            
            match (self.node_list[k].get_min(), self.find_min(k + 1)) {
                (Some(c_min), Some(s_min)) => {
                    match c_min.cmp(s_min) {
                        Ordering::Greater => Some(s_min),
                        _ => Some(c_min)
                    }
                },
                (None, Some(s_min)) => Some(s_min),
                (Some(c_min), None) => Some(c_min),
                _ => None
            }
        }
    }
    //pub fn extract_min(&mut self, k: usize)
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
    let mut bheap: BinomialHeap<i32> = BinomialHeap::new();
    for num in nums.iter() {
        bheap.insert(*num).unwrap();
    }
    match bheap.find_min(0) {
        Some(x) => println!("{}", x),
        None => ()
    }
    
}