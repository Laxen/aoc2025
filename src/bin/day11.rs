use std::{collections::HashMap, fs};

fn dfs(connections: &HashMap<&str, Vec<&str>>, node: &str) -> u32 {
    if node == "out" {
        return 1;
    }

    let mut n_connections = 0;
    for connection in connections.get(node).unwrap() {
        n_connections += dfs(connections, connection);
    }

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

    let ret = dfs(&connections, "you");
    println!("{}", ret);
}