
use std::fs;
use std::cmp::Ordering;
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
    match bin_search(nums, target) {
        Some(ans) => {
            println!("Target element found at index {}", ans)
        }
        None => {
            println!("Target element not found!")
        }
    }  
}

fn bin_search(arr: &[i32], target: i32) -> Option<usize> {
    if arr.len() == 0 {
        None
    } else {
        let mut l: usize = 0;
        let mut r: usize = arr.len();
        let mut middle: usize;
        while l < r {
            middle = (l + r) / 2;
            match arr[middle].cmp(&target) {
                Ordering::Equal => return Some(middle),
                Ordering::Less => l = middle + 1,
                Ordering::Greater => r = middle
            }
        }
        None
    }
}
