use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;

const EXAMPLE_PATH: &str = "beispieldaten/spielstaerken1.txt";
const NUM_OF_RUNS: usize = 1000;

#[derive(Debug)]
struct Example {
    num_of_players: usize,
    strengths: Vec<usize>,
}

fn best_player_index(wins: &Vec<usize>) -> usize {
    wins.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index)
        .unwrap()
}

fn main() {
    let example = read_example(EXAMPLE_PATH);
    let mut league_winners = vec![0; example.num_of_players];

    for _ in 0..NUM_OF_RUNS {
        league_winners[league_system(&example.strengths)] += 1;
    }

    println!("{:?}", best_player_index(&league_winners));
}

fn read_example(path: &str) -> Example {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let num_of_players = lines[0].parse::<usize>().unwrap();
    let mut strengths = Vec::new();

    for i in 1..=num_of_players {
        let strength = lines[i].parse::<usize>().unwrap();
        strengths.push(strength);
    }

    Example {
        num_of_players,
        strengths,
    }
}

fn confrontation(h: usize, g: usize) -> bool {
    let mut rng = rand::thread_rng();

    if h == g {
        return rng.gen::<bool>();
    }

    let sum = h + g;
    let rand = rng.gen_range(1, sum + 1);

    if rand <= h {
        return true;
    }
    false
}

fn league_system(strengths: &Vec<usize>) -> usize {
    let mut wins: Vec<usize> = vec![0; strengths.len()];

    for i in 0..strengths.len() {
        for j in i + 1..strengths.len() {
            if confrontation(strengths[i], strengths[j]) {
                wins[i] += 1;
            } else {
                wins[j] += 1;
            }
        }
    }

    return wins.iter().max().unwrap().clone();
}

fn knockout_system(strengths: Vec<usize>) -> usize {
    0
}
