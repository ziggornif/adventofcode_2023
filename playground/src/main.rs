// cas à gérer :
// le char est un chiffre ?
// -> l'ajouter à la chaîne
// -> digit = true
// -> first = true
// le char est une lettre ?
// -> est-ce que c'est la première de la section ? (first = true)
// ----> OUI - est-ce que c'est le début d'un chiffre ?
// ------> OUI - stocker dans une variable worded numeric
// ------> NON - est-ce que la chaîne worded est toujours le début d'un chiffre ?
// ---------> OUI - continuer
// ---------> NON - on push dans la chaîne et on passe first à true et digit à false
// ----> NON - on push dans la chaîne et on passe first à true et digit à false

use std::collections::HashMap;

// Est-ce que j'ai un mot dans mon buffer ?
fn parse(input: &str) -> String {
    let mut result = String::new();
    let mut digit = true;
    let mut worded_number = String::new();
    for c in input.chars() {
        if c.is_numeric() {
            if worded_number.len() > 0 {
                let num = word_to_number(&worded_number);
                // on a trouvé un chiffre en lettres, on push et on clear
                if num > 0 {
                    result.push_str(&num.to_string());
                    worded_number.clear();
                    digit = true;
                }
            }

            result.push(c);
            digit = true;
        } else {
            // première lettre d'un "mot"
            if digit {
                if start_of_worded_number(&c.to_string()) {
                    worded_number.push(c);
                    digit = false;
                } else {
                    // ce n'est pas le début d'un mot, on push dans le résult
                    result.push(c);
                    digit = true;
                }
            } else {
                let num = word_to_number(&worded_number);
                // on a trouvé un chiffre en lettres, on push et on clear
                if num > 0 {
                    result.push_str(&num.to_string());
                    worded_number.clear();
                    digit = true;
                } else {
                    worded_number.push(c);
                    if !start_of_worded_number(&worded_number) {
                        result.push_str(&worded_number);
                        worded_number.clear();
                        digit = true;
                    } else {
                        let num = word_to_number(&worded_number);
                        // on a trouvé un chiffre en lettres, on push et on clear
                        if num > 0 {
                            result.push_str(&num.to_string());
                            worded_number.clear();
                            digit = true;
                        } else {
                            digit = false;
                        }
                    }
                }
            }
        }
    }

    if worded_number.len() > 0 {
        let num = word_to_number(&worded_number);
        // on a trouvé un chiffre en lettres, on push et on clear
        if num > 0 {
            result.push_str(&num.to_string());
            worded_number.clear();
        }
    }

    result
}

fn start_of_worded_number(word: &str) -> bool {
    let valid_starters = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for &starter in &valid_starters {
        if starter.starts_with(word) {
            return true;
        }
    }

    false
}

fn parse_line(input: &str) -> String {
    let mut first_word = String::new();
    let mut last_word = String::new();
    let mut current_word = String::new();
    let mut is_digit = false;

    for c in input.chars() {
        if c.is_alphabetic() {
            if is_digit {
                if first_word.is_empty() {
                    first_word = current_word.clone();
                }
                last_word = current_word.clone();
                current_word.clear();
                is_digit = false;
            }
            current_word.push(c);
        } else if c.is_numeric() {
            if !is_digit {
                current_word.clear();
                is_digit = true;
            }
            current_word.push(c);
        } else {
            current_word.push(c);
            is_digit = false;
        }
    }

    if !current_word.is_empty() {
        if first_word.is_empty() {
            first_word = current_word.clone();
        }
        last_word = current_word;
    }

    let result = format!(
        "{}{}",
        word_to_number(&first_word),
        word_to_number(&last_word)
    );
    if result == "00" {
        input.to_string()
    } else {
        result
    }
}

fn dummy_parser(input: &str) -> String {
    let mut result = String::new();
    let mut temp = String::new();
    for c in input.chars() {
        temp.push(c);
        temp = replace_worded_numbers(temp);
        println!("{}", temp);
    }
    result = temp;
    result
}

fn replace_worded_numbers(input: String) -> String {
    let numbers = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut result = input.clone();
    for (key, val) in numbers {
        result = result.replace(key, &*val.to_string());
    }

    result
}


fn word_to_number(word: &str) -> u32 {
    match word.to_lowercase().as_str() {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0, // Default to 0 if the word is not recognized
    }
}

fn main() {
    let lines = [
        // "two1nine",
        "eightwothree",
        // "abcone2threexyz",
        // "xtwone3four",
        // "4nineeightseven2",
        // "zoneight234",
        // "7pqrstsixteen",
    ];

    for line in &lines {
        // let result = parse(line);
        let result = dummy_parser(line);
        println!("{} => {}", line, result);
    }
}
