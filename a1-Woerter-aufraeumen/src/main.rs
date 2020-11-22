use std::fs::File;
use std::io::prelude::*;

const FILE_PATH: &str = "beispieldaten/raetsel1.txt";

#[derive(Debug)]
struct Example {
    cloze: Vec<String>,
    words: Vec<String>,
}

fn calculate_qualities(example: &Example) -> Vec<(usize, String)>{
    let mut words = vec![(0, String::new()); example.words.len()];

    for (i, word) in example.words.iter().enumerate() {
        let mut matches = 0;
        for cloze_unit in example.cloze.iter() {
            if cloze_unit.chars().count() == word.chars().count() {
                for (j, letter) in word.chars().enumerate() {
                    if letter == cloze_unit.chars().nth(j).unwrap() {
                        matches += 1;
                    }
                }
            }
        }
        if matches == 0 {
            words[i] = (100, word.to_string());
        } else {
            words[i] = (matches, word.to_string());
        }
    }
    words.sort_by(|a, b| a.0.cmp(&b.0));

    words
}

fn main() {
    let example = read_example(FILE_PATH);

    let mut result = vec![String::new(); example.words.len()];
    let mut gaps_closed = vec![false; example.words.len()];

    let words = calculate_qualities(&example);
    println!("{:?}", words);

    for word in words.iter() {
        for (i, cloze_unit) in example.cloze.iter().enumerate() {
            if !gaps_closed[i] {
                if cloze_unit.chars().count() == word.1.chars().count() {
                    if word.0 == 100 {
                        result[i] = word.1.to_string();
                        gaps_closed[i] = true;
                    } else {
                        for (j, letter) in word.1.chars().enumerate() {
                            if letter == cloze_unit.chars().nth(j).unwrap() {
                                result[i] = word.1.to_string();
                                gaps_closed[i] = true;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{:?}", result);
}

fn read_example(file_path: &str) -> Example {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let mut cloze = String::new();
    let mut extensions: Vec<String> = Vec::new();

    for character in lines[0].to_string().chars() {
        if character.is_alphabetic() || character == '_' || character.is_whitespace() {
            cloze.push(character);
        } else {
            extensions.push(character.to_string());
        }
    }

    let words: Vec<String> = lines[1]
        .to_string()
        .split(" ")
        .map(|s| s.to_string())
        .collect();

    let cloze = cloze.split(" ").map(|s| s.to_string()).collect();

    Example { cloze, words }
}
