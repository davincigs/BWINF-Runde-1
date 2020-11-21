use std::fs::File;
use std::io::prelude::*;

const EXAMPLE_PATH: &str = "beispieldaten/puzzle0.txt";

#[derive(Debug)]
struct Example {
    num_of_types: usize,
    num_of_parts: usize,
    pieces: Vec<(isize, isize, isize)>,
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

    let num_of_types = lines[0].parse::<usize>().unwrap();
    let num_of_parts = lines[1].parse::<usize>().unwrap();
    let mut pieces = Vec::new();

    for i in 2..lines.len() {
        let figures: Vec<isize> = lines[i].split(" ").map(|l| l.parse::<isize>().unwrap()).collect();
        pieces.push((figures[0], figures[1], figures[2]));
    }

    Example {
        num_of_types,
        num_of_parts,
        pieces,
    }
}
