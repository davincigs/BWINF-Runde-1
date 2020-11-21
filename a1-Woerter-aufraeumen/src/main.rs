use std::fs::File;
use std::io::prelude::*;

const FILE_PATH: &str = "beispieldaten/raetsel1.txt";

#[derive(Debug)]
struct Example {
    cloze: Vec<String>,
    words: Vec<String>,
}

fn recursion_completion(
    example: &Example,
    mut result: &mut Vec<String>,
    mut used_words: &mut Vec<bool>,
    index: usize,
) {
    for (i, cloze_unit) in example.cloze.iter().enumerate() {
        let chars_are_numeric: Vec<bool> = cloze_unit.chars().map(|c| c == '_' ).collect();

        if chars_are_numeric.contains(&false) {
            let mut matches = Vec::new();
            for (j, word) in example.words.iter().enumerate() {
                if word.chars().count() == cloze_unit.chars().count() {
                    if !used_words[j] {
                        for (k, letter) in word.chars().enumerate() {
                            if letter == cloze_unit.chars().nth(k).unwrap() {
                                matches.push((j, word.to_string()));
                            }
                        }
                    }
                }
            }

            for word in matches.iter() {
                if matches.len() == 1 || index == 2 {
                    result[i] = word.1.to_string();
                    used_words[word.0] = true;
                } else {
                    recursion_completion(example, &mut result, &mut used_words, index + 1);
                }
            }
        }
    }
}

fn main() {
    let example = read_example(FILE_PATH);

    let mut result = vec![String::new(); example.words.len()];
    let mut used_words = vec![false; example.words.len()];

    recursion_completion(&example, &mut result, &mut used_words, 0);

    println!("{:?}", used_words);
    println!("{:?}", example.words);

    for (i, cloze_unit) in result.clone().iter().enumerate() {
        if cloze_unit.chars().count() == 0 {
            println!("{}", example.cloze[i]);
            for (j, word) in example.words.iter().enumerate() {
                if !used_words[j] {
                    if example.cloze[i].chars().count() == word.chars().count() {
                        result[i] = example.words[j].to_string();
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
