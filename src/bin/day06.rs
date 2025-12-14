use std::fs;

fn compute_numbers(digits: &Vec<Vec<char>>, operation: char, mut cols: (u32, u32)) -> u64 {
    let mut numbers = Vec::new();

    for col in (cols.0..cols.1).rev() {
        let mut number = String::new();

        for row in digits {
            number.push(*row.get(col as usize).unwrap_or(&' '));
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
    let mut max_len = 0;
    for line in contents.lines() {
        let d: Vec<char> = line.chars().collect();

        max_len = max_len.max(d.len());
        digits.push(d);
    }

    /* Pad rows */
    for row in &mut digits {
        while row.len() <= max_len {
            row.push(' ');
        }
    }

    let mut operations = digits.pop().unwrap();
    operations.push('s');

    for d in &digits {
        println!("{:?}", d);
    }
    println!("{:?}", operations);

    let mut i = 0;
    let mut total = 0;
    let mut breaker = false;
    loop {
        let op = operations[i];

        println!("op: {}", op);

        let mut cols = (i as u32, 0);

        for n in i + 1..operations.len() {
            if operations[n] != ' ' {
                cols.1 = (n - 1) as u32;
                i = n;

                if operations[n] == 's' {
                    breaker = true;
                }
                break;
            }
        }

        let ret = compute_numbers(&digits, op, cols);
        println!("Result: {}", ret);
        total += ret;

        if breaker {
            break;
        }
    }

    println!("Total: {}", total);
}