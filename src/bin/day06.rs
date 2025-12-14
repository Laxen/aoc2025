use std::fs;

fn compute_numbers(digits: &Vec<Vec<char>>, operation: char, mut cols: (u32, u32)) -> u64 {
    let mut numbers = Vec::new();

    for row in digits {
        let mut number = String::new();

        if cols.1 == 0 {
            cols.1 = row.len() as u32
        };

        for i in cols.0..cols.1 {
            number.push(row[i as usize]);
        }

        numbers.push(number.trim().parse::<u64>().unwrap());
    }

    match operation {
        '+' => numbers.iter().sum(),
        '*' => numbers.iter().product(),
        _ => panic!("Unsupported operation"),
    }
}

fn main() {
    let contents =
        fs::read_to_string("inputs/day06/input.txt").expect("Failed to read example.txt");

    let mut digits = Vec::new();
    for line in contents.lines() {
        let d: Vec<char> = line.chars().collect();
        digits.push(d);
    }

    let operations = digits.pop().unwrap();

    for d in &digits {
        println!("{:?}", d);
    }
    println!("{:?}", operations);

    let mut i = 0;
    let mut total = 0;
    while i < operations.len() {
        let op = operations[i];

        println!("op: {}", op);

        let mut cols = (i as u32, 0);

        if i + 1 >= operations.len() {
            cols.1 = 0;
        } else {
            for n in i + 1..operations.len() {
                if operations[n] != ' ' {
                    cols.1 = n as u32;
                    i = n;
                    break;
                }
            }
        }

        let ret = compute_numbers(&digits, op, cols);
        println!("Result: {}", ret);
        total += ret;

        if cols.1 == 0 {
            break;
        }
    }

    println!("Total: {}", total);
}
