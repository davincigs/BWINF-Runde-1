use std::fs::File;
use std::io::prelude::*;
use rand::Rng;

const EXAMPLE_PATH: &str = "beispieldaten/wichteln1.txt";

#[derive(Debug)]
struct Example {
    num_of_gifts: usize,
    preferences: Vec<(usize, usize, usize)>
}

fn generate_random_assignment(n: usize) -> Vec<usize> {
    let mut used: Vec<usize> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..n {
        let mut found = false;
        while !found {
            let rand: usize = rng.gen_range(1, n + 1);
            if !used.contains(&rand) {
                used.push(rand);
                found = true;
            }
        }
    }

    used
}

fn main() {

    let example = read_example(EXAMPLE_PATH);
    

    let mut result = 0;
    let mut assignment;

    while !(result == 10) {
        let a = generate_random_assignment(example.num_of_gifts);
        let mut primary = 0;
        let mut secondary = 0;
        let mut tertiary = 0;

        for (i, gift) in a.iter().enumerate() {
            if gift == &example.preferences[i].0 {
                primary += 1;
            } else if gift == &example.preferences[i].1 {
                secondary += 1;
            } else if gift == &example.preferences[i].2 {
                tertiary += 1;
            }
        }

        assignment = a;
        result = primary + secondary + tertiary;
    }
}

fn read_example(path: &str) -> Example {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let num_of_gifts = lines[0].parse::<usize>().unwrap();
    let mut preferences = Vec::new();

    for i in 1..lines.len() {
        let preference: Vec<usize> = lines[i].split(" ").map(|l| l.parse::<usize>().unwrap()).collect();
        preferences.push((preference[0], preference[1], preference[2]));
    }

    Example {
        preferences,
        num_of_gifts,
    }
}
