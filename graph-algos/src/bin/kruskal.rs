use std::{fs, vec};

#[derive(Debug)]
pub enum Error {
    NoSuchElement
}
#[derive(Debug, Clone, Copy)]
struct Edge {
    u: usize,
    v: usize,
    w: i32
}


struct DSU {
    n: usize,
    p: Vec<usize>,
    sz: Vec<usize>
}

impl DSU {
    pub fn new(n: usize) -> Self {
        return DSU { n: n, p: (0..n).collect(), sz: vec![1usize; n] };
    }

    pub fn root(&mut self, i: usize) -> Result<usize, Error> {
        if i >= self.n { 
            return Err(Error::NoSuchElement);
        }
        let mut temp = i;
        while self.p[temp] != temp {
            temp = self.p[temp];
        }
        let mut x = i;
        while self.p[x] != x {
            let t = x;
            x = self.p[x];
            self.p[t] = temp;
        }
        return Ok(temp);
    }

    pub fn connect(&mut self, u: usize, v: usize) -> Result<(), Error>{
        let uroot = self.root(u)?;
        let vroot = self.root(v)?;
        if uroot != vroot {
            if self.sz[uroot] < self.sz[vroot] {
                self.p[uroot] = vroot;
                self.sz[vroot] += self.sz[uroot];
            } else {
                self.p[vroot] = uroot;
                self.sz[uroot] += self.sz[vroot];
            }
        }
        return Ok(());
    }

    pub fn is_connected(&mut self, u: usize, v: usize) -> Result<bool, Error> {
        let uroot = self.root(u)?;
        let vroot = self.root(v)?;
        Ok(uroot == vroot)
    }
}

fn kruskal(edgeList: &Vec<Edge>, n: usize) -> Result<Vec<Edge>, Error> {
    let mut edges_copy = edgeList.clone();
    edges_copy.sort_by(|a, b| a.w.cmp(&b.w));
    let mut mst_edges = Vec::new();
    let mut dsu = DSU::new(n);
    for edge in edges_copy {
        if !dsu.is_connected(edge.u, edge.v)? {
            dsu.connect(edge.u, edge.v)?;
            mst_edges.push(edge);
        }
    }
    return Ok(mst_edges);
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
        edgeList.push(Edge { u: u, v: v, w: w });
    }
    // Assuming source to be 0 node
    match kruskal(&edgeList, N) {
        Ok(mst_edges) => {
            let mut min_cost = 0;
            for edge in mst_edges.iter() {
                min_cost += edge.w;
            }
            println!("Minimum cost is {}", min_cost);
            println!("{:?}", mst_edges);
        },
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    
}