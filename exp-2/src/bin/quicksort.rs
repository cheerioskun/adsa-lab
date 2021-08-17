use std::fs;
use rand::Rng;
fn swap(arr: &mut Vec<i32>, i: usize, j: usize) {
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}
fn choose_pivot(n: usize, s: &str) -> usize{
    match s {
        "first" => {0}
        "last" => {n - 1}
        "random" => {
            rand::thread_rng().gen_range(0..n)
        }
        _ => {0}
    }
}

// Sort between l..=r
fn quicksort(arr: &mut Vec<i32>, l: usize, r: usize) {
    if l == r {
        // Array is of size 1
        return;
    }
    let n = r - l + 1;
    let (mut i, mut j) = (l, r);
    // Choose pivot ans swap with the last elem
    let pivot = l + choose_pivot(n, "random");
    swap(arr, pivot, r);
    // Now start bringing i and j closer together
    // while maintaining invariant that
    // all elems before i are <= pivot and all elems
    // after j are > pivot
    while i < j{
        while i < j && arr[i] <= arr[r]{ 
            i += 1;
        }
        // i points to elem that is greater than pivot
        while j > i && arr[j] > arr[r]{ 
            j -= 1;
        }
        // j points to elem that is smaller than or equal to pivot
        if i != j { 
            swap(arr, i, j);
        }
    }
    // Now quicksort the halves formed
    if l != i {
        quicksort(arr, l, i - 1);
    }
    if r != i {
        quicksort(arr, i + 1, r);
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
    let n = nums.len();
    quicksort(&mut nums, 0, n - 1);
    print!("Sorted array is : ");
    for item in nums.iter() {
        print!("{} ", item);
    }
}