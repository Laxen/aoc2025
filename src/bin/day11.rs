use std::{collections::HashMap, fs};

fn dfs(connections: &HashMap<&str, Vec<&str>>, node: &str, mut dac: bool, mut fft: bool, memo: &mut HashMap<(String, bool, bool), u64>) -> u64 {
    if node == "out" {
        if dac && fft {
            return 1;
        } else {
            return 0;
        }
    }

    if let Some(m) = memo.get(&(node.to_string(), dac, fft)) {
        return *m;
    }

    if !dac && node == "dac" {
        dac = true;
    }

    if !fft && node == "fft" {
        fft = true;
    }

    let mut n_connections = 0;
    for connection in connections.get(node).unwrap() {
        n_connections += dfs(connections, connection, dac, fft, memo);
    }

    memo.insert((node.to_string(), dac, fft), n_connections);
    return n_connections;
}

fn main() {
    let contents =
        fs::read_to_string("inputs/day11/input.txt").expect("Failed to read input.txt");

    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        connections.insert(parts[0], parts[1].trim().split(' ').collect());
    }

    let mut memo = HashMap::new();
    let ret = dfs(&connections, "svr", false, false, &mut memo);
    println!("{}", ret);
}