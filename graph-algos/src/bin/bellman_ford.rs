use std::fs;


fn bellman_ford(edgeList: &Vec<((usize, usize), i32)>, n: usize, s: usize) -> Vec<i32> {
    let inf = 0x3f3f3f3f;
    let mut dist = vec![inf; n];
    dist[s] = 0;
    for _ in 0..n {
        // Relax all edges, considering it to be a directed graph
        for ((u, v), w) in edgeList {
            if dist[*u] + w < dist[*v] {
                dist[*v] = dist[*u] + w;
            }
        }
    }
    return dist;
}


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
    
    let N = nums[0] as usize;
    let M = nums[1] as usize;
    let mut edgeList = Vec::new();
    // Taking in graph as an adjacency list
    for i in 0..M as usize{
        let u = nums[3*i + 2] as usize;
        let v = nums[3*i + 3] as usize;
        let w = nums[3*i + 4];
        edgeList.push(((u, v), w));
    }
    // Assuming source to be 0 node
    let distances = bellman_ford(&edgeList, N, 0);
    for i in 0..N {
        println!("Distance to {} is {}", i, distances[i]);
    }
}