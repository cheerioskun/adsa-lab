use std::fs;

fn swap(arr: &mut Vec<i32>, i: usize, j: usize) {
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

// Sort between l..=r
fn quicksort(arr: &mut Vec<i32>, l: usize, r: usize) {
    if r - l + 1 == 1 {
        return;
    }
    let (mut i, mut j) = (l, r - 1);
    // Choose pivot
    let pivot = r;
    // Now start bringing i and j closer together
    // while maintaining invariant that
    // all elems before i are <= pivot and all elems
    // after j are > pivot
    while i < j{
        if arr[i] > arr[pivot] {
            swap(arr, i, j);
            j -= 1;
        } else if i < r{
            i += 1;
        }
        if arr[j] <= arr[pivot] {
            swap(arr, i, j);
            i += 1;
        } else if j > 1{
            j -= 1;
        }
    }
    println!("i: {} j: {}", i, j);
    swap(arr, j + 1 as usize, pivot);
    // Now quicksort the halves formed
    quicksort(arr, l as usize, j as usize);
    quicksort(arr, j + 2 as usize, r as usize);
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
    let n = nums.len();
    quicksort(&mut nums, 0, n - 1);
    for i in nums.iter() {
        print!("{} ", i);
    }
    println!();
}