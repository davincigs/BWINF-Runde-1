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
    let example = read_example(EXAMPLE_PATH);
    println!("{:?}", example);
}

fn read_example(path: &str) -> Example {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let num_of_competitors = lines[0].parse::<usize>().unwrap();
    let mut strengths = Vec::new();

    for i in 1..num_of_competitors + 1 {
        let strength = lines[i].parse::<usize>().unwrap();
        strengths.push(strength);
    }

    Example {
        num_of_competitors,
        strengths,
    }
}
