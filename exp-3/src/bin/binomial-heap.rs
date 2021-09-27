use std::fs;
use std::cmp::Ordering;

#[derive(Debug)]
pub enum Error {
    DegreeMismatch,
    EmptyTree,
    AddToEmpty,
    EmptyHeap
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

    pub fn get_degree(&self) -> Option<usize> {
        match self {
            BinomialTree::Node(node) => Some(node.degree),
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

    // Fracture a binomial tree and return the root node value alongside
    pub fn fracture(self) -> Result<(T, Vec<Box<BinomialTree<T>>>), Error> {
        match self {
            BinomialTree::Node(node) => Ok((node.value, node.children)),
            BinomialTree::Empty => Err(Error::EmptyTree)
        }
    }
}

pub struct BinomialHeap<T: Ord> {
    node_list: Vec<BinomialTree<T>>,
    non_empty: usize
} 

impl<T: Ord> BinomialHeap<T> {
    pub fn new() -> Self {
        BinomialHeap {
            node_list: Vec::new(),
            non_empty: 0
        }
    }

    // Insert a tree of degree k into the heap
    pub fn insert_tree(&mut self, k: usize, tree: BinomialTree<T>) -> Result<(), Error>{
        if k == self.node_list.len() {
            self.node_list.push(tree);
            self.non_empty += 1;
            Ok(())
        } else {
            match &self.node_list[k] {
                BinomialTree::Node(_) => {
                    let mut temp_tree = std::mem::replace(&mut self.node_list[k], BinomialTree::Empty);
                    self.non_empty -= 1;
                    temp_tree = temp_tree.meld(tree)?;
                    self.insert_tree(k + 1, temp_tree)?;
                },
                BinomialTree::Empty => {
                    self.node_list[k] = tree;
                    self.non_empty += 1;
                }
            }
            return Ok(());
        }
    }
    
    // Insert an element into the heap
    pub fn insert(&mut self, value: T) -> Result<(), Error>{
        return self.insert_tree(0, BinomialTree::new(value));
    }

    // Find the position of binomial tree with min elem
    pub fn find_min_idx(&self) -> Result<usize, Error> {
        if self.non_empty == 0 {
            return Err(Error::EmptyHeap);
        }
        let mut min_idx = 0;
        for i in 0..self.node_list.len() {
            match (self.node_list[i].get_min(), self.node_list[min_idx].get_min()) {
                (_, None) => {
                    min_idx = i;
                },
                (Some(v), Some(u)) => {
                    if v < u {
                        min_idx = i;
                    }
                },
                (None, _) => {}
            }
        }
        return Ok(min_idx);
    }

    // Find an immutable reference to the minimum value in the heap
    pub fn find_min(&self) -> Option<&T> {
        match self.find_min_idx() {
            Ok(v) => self.node_list[v].get_min(),
            _ => None
        }
    }

    // Removes smallest element from heap and returns it
    pub fn extract_min(&mut self) -> Result<T, Error> {

        let min_idx = self.find_min_idx()?;
        let min_idx_tree = std::mem::replace(&mut self.node_list[min_idx], BinomialTree::Empty);
        self.non_empty -= 1;
        let (min_value, trees) = min_idx_tree.fracture()?;
        for tree_boxed in trees.into_iter() {
            let tree = *tree_boxed;
            self.insert_tree(tree.get_degree().unwrap(), tree)?;
        }
        return Ok(min_value);
    }

    pub fn meld(mut self, b: BinomialHeap<T>) -> Result<Self, Error> {
        if self.non_empty == 0 || b.non_empty == 0 {
            return Err(Error::EmptyHeap);
        }
        for tree in b.node_list.into_iter() {
            match tree {
                BinomialTree::Node(node) => {
                    self.insert_tree(node.degree, BinomialTree::Node(node))?;
                },
                BinomialTree::Empty => {}
            }
        }
        return Ok(self);
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
    let mut bheap: BinomialHeap<i32> = BinomialHeap::new();
    let mut cheap: BinomialHeap<i32> = BinomialHeap::new();
    for num in nums.iter() {
        if num%2 == 0{
            cheap.insert(*num).unwrap();
        }
        else {
            bheap.insert(*num).unwrap();
        }
    }
    println!("non empty trees a : {} b : {}", cheap.non_empty, bheap.non_empty);
    bheap = bheap.meld(cheap).unwrap();
    while bheap.non_empty > 0 {
        let temp = bheap.extract_min().unwrap();
        print!("{} ", temp);
    }
    
}