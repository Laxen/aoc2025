use std::{collections::VecDeque, fs};

struct Machine {
    lights: Vec<u32>,
    buttons: Vec<Vec<u32>>,
    joltage: String,
}

fn push_button(machine: &Machine, button: usize, lights: &Vec<u32>) -> Vec<u32> {
    let mut ret = lights.clone();

    for b in &machine.buttons[button] {
        let index = *b as usize;
        ret[index] = 1 - ret[index];
    }

    ret
}

fn search(machine: &Machine) -> u32 {
    let mut queue = VecDeque::new();

    for button in 0..machine.buttons.len() {
        /* (lights, presses, button) */
        queue.push_back((vec![0; machine.lights.len()], 0, button));
    }

    while let Some(state) = queue.pop_front() {
        let lights = &state.0;
        let presses = state.1 + 1;
        let button = state.2;

        let new_lights = push_button(machine, button, lights);
        // println!("{:?} -> {:?} button {:?}", lights, new_lights, machine.buttons[button]);

        if *new_lights == machine.lights {
            return presses;
        }

        for b in 0..machine.buttons.len() {
            if b == button {
                /* Useless to push the same button again */
                continue;
            }

            /* (lights, presses, button) */
            queue.push_back((new_lights.clone(), presses, b));
        }
    }

    return 9999;
}

fn main() {
    let contents =
        fs::read_to_string("inputs/day10/input.txt").expect("Failed to read example.txt");

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

        let joltage = parts.pop().unwrap().to_string();

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
            joltage
        };

        println!("{:?}, {:?}, {}", machine.lights, machine.buttons, machine.joltage);

        let res = search(&machine);
        println!("{}", res);
        total_presses += res;
    }

    println!("Total presses: {}", total_presses);
}