use std::fs;
use z3::ast::{Ast, Int};
use z3::{Config, Context, Optimize};

struct Machine {
    lights: Vec<u32>,
    buttons: Vec<Vec<u32>>,
    joltage: Vec<usize>,
}

fn solve_machine(buttons: &[Vec<u32>], targets: &[usize]) -> Option<i64> {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let opt = Optimize::new(&ctx);

    // Create variables for button presses (one per button)
    let button_presses: Vec<Int> = (0..buttons.len())
        .map(|i| Int::new_const(&ctx, format!("button_{}", i)))
        .collect();

    // All button presses must be non-negative
    for press in &button_presses {
        opt.assert(&press.ge(&Int::from_i64(&ctx, 0)));
    }

    // For each counter (joltage requirement), the sum of button presses
    // that affect it must equal the target value
    for (counter_idx, &target) in targets.iter().enumerate() {
        let mut sum = Int::from_i64(&ctx, 0);

        for (button_idx, button) in buttons.iter().enumerate() {
            if button.contains(&(counter_idx as u32)) {
                sum = sum + &button_presses[button_idx];
            }
        }

        opt.assert(&sum._eq(&Int::from_i64(&ctx, target as i64)));
    }

    // Minimize the total number of button presses
    let total: Int = button_presses
        .iter()
        .fold(Int::from_i64(&ctx, 0), |acc, press| acc + press);

    opt.minimize(&total);

    // Solve and extract the result
    if opt.check(&[]) == z3::SatResult::Sat {
        let model = opt.get_model().unwrap();
        let result = model.eval(&total).unwrap().as_i64().unwrap();
        Some(result)
    } else {
        None
    }
}

fn main() {
    let contents =
        fs::read_to_string("inputs/day10/input.txt").expect("Failed to read input.txt");

    let mut total_presses = 0;

    for line in contents.lines() {
        let mut parts: Vec<&str> = line.split(' ').collect();

        let lights_str = parts.remove(0).to_string();
        let mut lights = Vec::new();
        for c in lights_str.chars() {
            if c == '[' || c == ']' {
                continue;
            }

            if c == '#' {
                lights.push(1);
            } else {
                lights.push(0);
            }
        }

        let joltage_str = parts.pop().unwrap().to_string();
        let mut joltage: Vec<usize> = Vec::new();
        joltage_str
            .trim_matches(|c| c == '{' || c == '}')
            .split(",")
            .for_each(|s| {
                joltage.push(s.parse().unwrap());
            });

        let mut buttons = Vec::new();
        for button in parts {
            buttons.push(
                button
                    .trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .map(|i| i.parse().unwrap())
                    .collect(),
            );
        }

        let machine = Machine {
            lights,
            buttons,
            joltage,
        };

        if let Some(presses) = solve_machine(&machine.buttons, &machine.joltage) {
            println!("Machine requires {} presses", presses);
            total_presses += presses;
        } else {
            println!("No solution found for machine");
        }
    }

    println!("Total button presses required: {}", total_presses);
}
