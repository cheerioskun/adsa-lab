use std::fs;


/*
    Implementation for naive string matching
*/
fn matches(t: &str, p: &str) -> Option<Vec<usize>> {
    if p.len() == 0 {
        None
    } else {
        let p: Vec<char> = p.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let mut offsets = Vec::new();
        // Matching phase
        for i in 0..=(t.len() - p.len()) {
            for j in 0..=p.len() {
                if j == p.len() {
                    offsets.push(i);
                } else {
                    if t[i + j] != p[j] {
                        break;
                    }
                }

            }  
        }
        Some(offsets)
    }
}

fn main() {
    // Read input file
    let filename = "./data/input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    // Parse 
    let input: Vec<&str> = contents.split_whitespace()
                                .collect();
    let text = input[0];
    let pattern = input[1];
    let offsets = match matches(&text, &pattern) {
        Some(x) => x,
        None => vec![0;0]
    };
    for offset in offsets.iter() {
        println!("Pattern found at index: {}", offset);
    }
}