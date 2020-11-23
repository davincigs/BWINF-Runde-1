use std::fs::File;
use std::io::prelude::*;
use rand::Rng;

const EXAMPLE_PATH: &str = "beispieldaten/spielstaerken1.txt";

#[derive(Debug)]
struct Example {
    num_of_competitors: usize,
    strengths: Vec<usize>,
}

fn main() {
    let mut example = read_example(EXAMPLE_PATH);
    println!("{:?}", example);

    for i in 0..=example.strengths.len() {
        for j in i..example.strengths.len() {
            if competition(example.strengths[i], example.strengths[j]) {
                example.strengths[i] = example.strengths[i] + 1;
            } else {
                example.strengths[j] = example.strengths[j] + 1;
            }
        }
    }

    println!("{:?}", example);
}

fn competition(ps1: usize, ps2: usize) -> bool {
    let sum = (ps1 + ps2) as f32;    
    let mut rng = rand::thread_rng();
    let mut rand = rng.gen::<f32>() * sum;

    while rand == ps1 as f32 {
        rand = rng.gen::<f32>() * sum
    }

    if rand < ps1 as f32 {
        return true;
    }
    
    false
}

fn read_example(path: &str) -> Example {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let num_of_competitors = lines[0].parse::<usize>().unwrap();
    let mut strengths = Vec::new();

    for i in 1..=num_of_competitors {
        let strength = lines[i].parse::<usize>().unwrap();
        strengths.push(strength);
    }

    Example {
        num_of_competitors,
        strengths,
    }
}
