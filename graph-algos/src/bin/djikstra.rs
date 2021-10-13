use std::fs;
use priority_queue::DoublePriorityQueue;

fn dijkstra(adjList: &Vec<Vec<(usize, i32)>>) -> Vec<i32> {
    let inf = 0x3f3f3f3f;
    let N = adjList.len();
    let mut visited: Vec<bool> = vec![false; N];
    let mut dist: Vec<i32> = vec![inf; N];
    let mut pq:DoublePriorityQueue<usize, i32> = DoublePriorityQueue::with_capacity(N);
    for i in 1..N as usize {
        pq.push(i, inf);
    }
    pq.push(0, 0);
    while !pq.is_empty() {
        let (u, d) = pq.pop_min().unwrap();
        visited[u] = true;
        dist[u] = d;
        for (v, w) in adjList[u].iter() {
            if visited[*v] {
                continue;
            }
            pq.push_decrease(*v, d + w);
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
    let mut adjList: Vec<Vec<(usize, i32)>> = Vec::with_capacity(N);
    for i in 0..N {
        adjList.push(Vec::new());
    }
    for i in 0..M as usize{
        let u = nums[3*i + 2] as usize;
        let v = nums[3*i + 3] as usize;
        let w = nums[3*i + 4];
        adjList[u].push((v, w));
        adjList[v].push((u, w));
    }
    // Assuming source to be 0 node
    let distances = dijkstra(&adjList);
    for i in 0..N {
        println!("Distance to {} is {}", i, distances[i]);
    }
}