use std::{collections::HashMap, fs};

fn euclidean_distance(a: &Vec<i32>, b: &Vec<i32>) -> f64 {
    let dx = (a[0] - b[0]) as f64;
    let dy = (a[1] - b[1]) as f64;
    let dz = (a[2] - b[2]) as f64;

    return (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt();
}

fn main() {
    let contents =
        fs::read_to_string("inputs/day08/input.txt").expect("Failed to read example.txt");

    let coords: Vec<Vec<i32>> = contents
        .lines()
        .map(|line| line.split(",").map(|n| n.parse::<i32>().unwrap()).collect())
        .collect();

    let mut distances: Vec<(f64, (usize, usize))> = Vec::new();
    for a in 0..coords.len() {
        for b in (a + 1)..coords.len() {
            let dist = euclidean_distance(&coords[a], &coords[b]);
            distances.push((dist, (a, b)));
        }
    }

    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut circuits: HashMap<&Vec<i32>, usize> = HashMap::new();
    for i in 0..coords.len() {
        circuits.insert(&coords[i], i);
    }

    for (_, (a, b)) in distances {
        let a_c = *circuits.get(&coords[a]).unwrap();
        let b_c = *circuits.get(&coords[b]).unwrap();

        let mut all_same = true;
        for (coord, circuit) in circuits.iter_mut() {
            if *circuit == b_c {
                *circuit = a_c;
            }

            if *circuit != a_c {
                all_same = false;
            }
        }

        if all_same {
            println!("{:?} and {:?} are the last to connect.", coords[a], coords[b]);
            break;
        }
    }

    let mut circuit_count: HashMap<usize, usize> = HashMap::new();
    for (_, circuit) in circuits {
        *circuit_count.entry(circuit).or_insert(0) += 1;
    }

    let mut counts = circuit_count.iter().map(|(k, v)| *v).collect::<Vec<usize>>();
    counts.sort();
    counts.reverse();

    let mut result = 1;
    for i in 0..3 {
        result *= counts[i];
    }

    println!("Result: {}", result);
}
