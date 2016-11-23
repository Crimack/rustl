extern crate time;

use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use time::precise_time_s;

#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Comparator {
    distance: i32,
    comparisons: i32,
    prediction: i32
}

fn read_file(path: &Path) -> Vec<Vec<i32>> {
    let mut file = match File::open(path) {
            Err(why) => panic!("couldn't open {}: {}",
                        path.display(),
                        why.description()),
            Ok(file) => file,
        };

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Failed to load file contents");
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for line in file_contents.split("\n"){
        if !line.is_empty() {
            let mut row: Vec<i32> = Vec::new();
            for entry in line.split_whitespace() {
                row.push(match entry.parse::<i32>() {
                    Ok(x) => {
                        x
                    },
                    Err(e) => panic!("Target file {} has errors:\nError: {:?}", path.display(), e),
                })
            }
            matrix.push(row);
        }
    }
    matrix
}

fn write_file(path: &Path, answers: &Vec<Vec<i32>>){
    let mut file = match File::create(path) {
        Err(why) => panic!("couldn't open {}: {}",
                    path.display(),
                    why.description()),
        Ok(file) => file,
    };
    let whitespace = " ";
    let newline = "\n";

    for row in answers {
        let mut iterator = row.iter();
        loop {
            match iterator.next() {
            Some(x) => {
                    file.write(x.to_string().as_bytes());
                    file.write(&whitespace.as_bytes());
                },
            None => {
                    file.write(&newline.as_bytes());
                    break
                }
            }
        }
    }
}

fn classify_missing(target_row: &Vec<i32>, train_data: &Vec<Vec<i32>>,
                        test_data: &Vec<Vec<i32>>, k: usize) -> Vec<i32>{
    let mut answers: Vec<Comparator> = Vec::new();
    for row in train_data {
        let distance = calculate_distance(target_row, row);
        answers.push(distance);
    }
    for row in test_data {
        let distance = calculate_distance(target_row, row);
        answers.push(distance);
    }

    answers.sort();
    let mut one_counter = 0;
    let mut two_counter = 0;
    let mut pred: i32 = -1;
    for i in 0..k {
        if answers[i].prediction == 1 {
            one_counter += 1;
        } else {
            two_counter += 1;
        }

    }
    if one_counter >= two_counter {
        pred = 1;
    } else {
        pred = 2;
    }

    let mut classified: Vec<i32> = Vec::new();
    for entry in target_row {
        if *entry == 0 {
            classified.push(pred);
        } else {
            classified.push(*entry);
        }
    }
    classified

}

fn calculate_distance(target_row: &Vec<i32>, test_row: &Vec<i32>) -> Comparator {
    // distance / number comparisons
    let mut distance: i32 = 0;
    let mut comparisons: i32 = 0;
    let mut predicted: i32 = -1;
    for i in 0..target_row.len() {
        if target_row[i] == 0 {
            predicted = test_row[i];
        }
        else if test_row[i] == 0 {}
        else {
            if test_row[i] != target_row[i] { distance += 1; }
            comparisons += 1
        }
    }

    Comparator{distance: distance, comparisons: comparisons, prediction: predicted}

}

fn help() {
    println!("Usage:
kNearestNeighbour <training_data_path> <test_data_path> <output_path> <number of neighbours)
e.g. kNearestNeighbour ~/data/train.txt ~/data/test.txt ~/data/answer.txt 4")
}

fn main() {
    println!("Starting program...");
    let start_time = precise_time_s();
    let args: Vec<String> = env::args().collect();
    match args.len() {
        5 => {
            let train = PathBuf::from(&args[1]);
            let test = PathBuf::from(&args[2]);
            let answer = PathBuf::from(&args[3]);
            let k = &args[4];
            let number: usize = match k.parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    println!("Final argument is not an integer.");
                    help();
                    return;
                },
            };
            let training_data = read_file(train.as_path());
            let test_data = read_file(test.as_path());
            let mut answer_data: Vec<Vec<i32>> = Vec::new();
            println!("Starting classification...");

            for i in &test_data {
                let answer_row = classify_missing(i, &test_data, &training_data, number);
                answer_data.push(answer_row);
            }
            println!("Writing to file...");
            write_file(answer.as_path(), &answer_data);
            let end_time = precise_time_s();
            println!("Time taken: {} seconds", (end_time-start_time));

        },
        _ => help(),
    }

}
