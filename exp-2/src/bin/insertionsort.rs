use std::fs;

fn insertionsort(arr: &mut [i32]) {
    // Size > 1
    if arr.len() > 1 {
        let n = arr.len();
        insertionsort(&mut arr[0..n - 1]);
        let mut j = n - 1;
        let elem = arr[j];
        while j > 0 && arr[j - 1] > elem {
            arr[j] = arr[j - 1];
            j -= 1;
        } 
        arr[j] = elem;
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
    print!("Sorted array is : ");
    for item in nums.iter() {
        print!("{} ", item);
    }
}