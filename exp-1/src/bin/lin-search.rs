
use std::fs;

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
    let target = nums[nums.len() - 1];
    let nums = &nums[0..nums.len() - 1];
    match lin_search(nums, target) {
        Some(ans) => {
            println!("Target element found at index {}", ans)
        }
        None => {
            println!("Target element not found!")
        }
    }   
}


fn lin_search(arr: &[i32], target: i32) -> Option<usize> {
    if arr.len() == 0 {
        None
    } else {
        for idx in 0..arr.len() {
            if arr[idx] == target { return Some(idx) }
        }
        None
    }
}
