/*
    This program implements a heap and finds the 
    kth minimum and kth maximum by creating a min heap
    and a max heap using the same code by passing 
    operator as argument
*/
use std::fs;
use std::cmp::Ordering;



fn make_heap(arr: &mut Vec<i32>) {
    let n = arr.len();
    // Scheme: 0-based indexing on the array means
    //                  n
    //          2n + 1     2n + 2
    // Assume the array to be an unmade heap. Start at the last node that is a parent.
    // (n - 1) is the last node. if n - 1 % 2 then n - 1 = 2(P + 1), P = (n - 1)/2 - 1 = (n - 3)/2
    // if n - 1 % 2 == 1, then n - 1 = 2P + 1 => P = (n - 2)/2 
    // For every right child there is a left sibling with the same parent. 
    // Why not find the parent of the left child always. This will simplify formula to 
    // p(n) = (n - 1)/2
    let parent = | n: usize | (n - 1)/2;
    let left = |n: usize| 2*n + 1;
    let right = |n: usize| 2*n + 2;
    let mut i = parent(n - 1) as i32;
    while i >= 0 {
        let (mut best, l, r) = (i as usize, left(i as usize), right(i as usize));
        if l < n {

            match arr[best].cmp(&arr[l]) {
                // best is bigger than l
                Ordering::Greater => { best = l; },
                _ => {}
            };
        }
        if r < n {
            match arr[best].cmp(&arr[r]) {
                Ordering::Greater => { best = r; },
                _ => {}
            };
        }
        let temp = arr[best];
        arr[best] = arr[i as usize];
        arr[i as usize] = temp;
        if best != i as usize {
            percolate_down(arr, best);
        }
        i = i - 1;
    }
}

fn percolate_down(arr: &mut Vec<i32>, i: usize) {
    let left = |n: usize| 2*n + 1;
    let right = |n: usize| 2*n + 2;
    if i < arr.len() {
        let (mut best, l, r) = (i as usize, left(i as usize), right(i as usize));
        if l < arr.len() {
            match arr[best].cmp(&arr[l]) {
                // best is bigger than l
                Ordering::Greater => { best = l; },
                _ => {}
            };
        }
        if r < arr.len() {
            match arr[best].cmp(&arr[r]) {
                Ordering::Greater => { best = r; },
                _ => {}
            };
        }
        let temp = arr[best];
        arr[best] = arr[i as usize];
        arr[i as usize] = temp;
        if best != i {
            percolate_down(arr, best);
        }
        
    }
}

fn get_min(arr: &mut Vec<i32>) -> Option<i32>{
    if arr.len() == 0 {
        return None;
    }
    // Min is at the top of the heap
    let n = arr.len();
    let temp = arr[n - 1];
    arr[n - 1] = arr[0];
    arr[0] = temp;
    let min = arr.pop().unwrap();
    percolate_down(arr, 0);
    return Some(min);
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
    let k = nums.pop().unwrap();
    make_heap(&mut nums);
    // make_heap works correctly
    for i in 0..k {
        println!("minimum #{}: {}", i + 1, get_min(&mut nums).unwrap());
    }

}