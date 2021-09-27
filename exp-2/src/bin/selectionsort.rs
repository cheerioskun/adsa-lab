use std::fs;


fn swap(arr: &mut [i32], i: usize, j: usize) {
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

fn selectionsort(arr: &mut [i32]) {
    // Size > 1
    if arr.len() > 1 {
        let mut mindex = 0;
        let mut i = 1;
        while i < arr.len() {
            if arr[i] < arr[mindex] {
                mindex = i;
            }
            i += 1;
        }
        swap(arr, 0, mindex);
        selectionsort(&mut arr[1..]);
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
    selectionsort(&mut nums[..]);
    print!("Sorted array is : ");
    for item in nums.iter() {
        print!("{} ", item);
    }
}