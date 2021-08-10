use std::fs;


/*
    Implementation for Knuth-Morris-Pratt Algorithm.
    fn kmp takes in a text string and a pattern string
    and returns a list of offsets where the text matches
    the pattern
*/
fn preprocess(p: &Vec<char>) -> Vec<usize> {
    // Prefix stores the max value k such that
    // Pₖ ⊏ Pᵢ and Pₖ ⊐ Pᵢ
    
    let mut prefix = vec![0, p.len() - 1];
    for i in 1..p.len() {
        let mut k = prefix[i - 1];
        while k > 0 && p[i] != p[k] {
            k = prefix[k - 1];
        }
        if p[i] == p[k] {
            prefix[i] = k + 1;
        } else {
            prefix[i] = 0;
        }
        print!("{}", prefix[i]);
    }
    prefix
}
fn kmp(t: &str, p: &str) -> Option<Vec<usize>> {
    if p.len() == 0 {
        None
    } else {
        let p = p.chars().collect();
        let t: Vec<char> = t.chars().collect();
        // preprocess the pattern to form the prefix function π
        let prefix = preprocess(&p);
        let mut offsets = Vec::new();
        // Matching phase. j is pointing at the next char to match in pattern
        // i points to next char to match in text
        let mut j = 0; 
        for i in 0..t.len() {
            if j == p.len() {
                offsets.push(i - j);
                j = prefix[j - 1];
            }
            if p[j] == t[i] {
                j += 1;
            } else {
                while j > 0 && p[j] != t[i] {
                    j = prefix[j - 1];
                }
                if p[j] == t[i] {
                    j += 1;
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
    println!("t: {}, p: {}", text, pattern);
    let offsets = match kmp(&text, &pattern) {
        Some(x) => x,
        None => vec![0;0]
    };
    for offset in offsets.iter() {
        println!("Pattern found at index: {}", offset);
    }
}