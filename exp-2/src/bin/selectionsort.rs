use std::fs;

fn merge(a: &[i32], b: &[i32], c: &mut Vec<i32>) {
    // Check if we have enough size in C to fill elems
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while k < a.len() + b.len() {
        if j >= b.len() || (i < a.len() && a[i] < b[j]) {
            c.push(a[i]);
            i += 1;
            k += 1;
        } else {
            c.push(b[j]);
            j += 1;
            k += 1;
        }
    }
}

fn mergesort(arr: &mut Vec<i32>, l: usize, r: usize) {
    if l == r {
        return;
    }
    let middle = (l + r) / 2;
    mergesort(arr, l, middle);
    mergesort(arr, middle + 1, r);
    let mut aux = Vec::new();
    merge(&arr[l..=middle], &arr[(middle + 1)..=r], &mut aux);
    for i in l..=r {
        arr[i] = aux[i - l];
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
    mergesort(&mut nums, 0, n - 1);
    print!("Sorted array is : ");
    for item in nums.iter() {
        print!("{} ", item);
    }
}