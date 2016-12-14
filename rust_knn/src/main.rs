extern crate time;

use std::cmp::Ordering;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use time::precise_time_s;

#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd)]
struct Comparator {
    // Comparisons not made is a sorting hack, so that I don't have to impl Ord myself
    distance: i32,
    comparisons_not_made: i32,
    prediction: i32
}

impl Ord for Comparator {
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.distance, &self.comparisons_not_made).cmp(&(&other.distance, &other.comparisons_not_made))
    }
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
                        test_data: &Vec<Vec<i32>>, k: usize) -> (Vec<i32>, i32) {
    let mut answers: Vec<Comparator> = Vec::new();
    for row in train_data {
        let distance = calculate_distance(target_row, row);
        answers.push(distance);
    }

    for row in test_data {
        let distance = calculate_distance(target_row, row);
        // -1 indicates no prediction could be made 
        if distance.prediction != -1 { answers.push(distance); }
    }

    answers.sort();

    let mut one_counter: i32 = 0;
    let mut two_counter: i32 = 0;
    for i in 0..k {
        if answers[i].prediction == 1 {
            one_counter += 1;
        } else if answers[i].prediction == 2 {
            two_counter += 1;
        }
    }

    let pred = if one_counter > two_counter {1} else {2};

    let mut classified: Vec<i32> = Vec::new();
    for entry in target_row {
        if *entry == 0 {
            classified.push(pred);
        } else {
            classified.push(*entry);
        }
    }
    (classified, pred)

}

fn calculate_distance(target_row: &Vec<i32>, test_row: &Vec<i32>) -> Comparator {
    // distance / number comparisons
    let mut distance: i32 = 0;
    let mut comparisons_not_made: i32 = 0;
    let mut predicted: i32 = -1;
    for i in 0..target_row.len() {
        if test_row[i] == 0 { comparisons_not_made += 1 }
        else {
            if target_row[i] == 0 { predicted = test_row[i]; } 
            else if test_row[i] != target_row[i] { distance += 1; }
        }
    }

    Comparator{distance: distance, comparisons_not_made: comparisons_not_made, prediction: predicted}

}

fn arg_to_vec(test_data: &str) -> Vec<i32> {
    let mut arg_as_vec: Vec<i32> = Vec::new();
    for entry in test_data.chars() {
        arg_as_vec.push(entry.to_digit(10).unwrap() as i32);
    }
    arg_as_vec
}

fn help() {
    println!("Usage:
kNearestNeighbour <training_data_path> <test_data_path> <output_path> <number of neighbours)
e.g. kNearestNeighbour ~/data/train.txt ~/data/test.txt ~/data/answer.txt 4

Test Usage:
kNearestNeighbour <example train row> <example test row>
e.g. kNearestNeighbour 1102112112 211212221
This will return the distance between the first and second arguments")
}

fn main() {
    println!("Starting program...");
    let start_time = precise_time_s();
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {
            let train = &args[1];
            let test = &args[2];
            let train_vec = arg_to_vec(train);
            let test_vec = arg_to_vec(test);
            let distance = calculate_distance(&train_vec, &test_vec);
            println!("{:?}", distance);
        }
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
            let mut ones = 0;
            let mut twos = 0;

            for i in &test_data {
                let (answer_row, prediction) = classify_missing(i, &training_data, &test_data, number);
                answer_data.push(answer_row);
                if prediction == 1 { ones += 1 } else { twos += 1 };
            }
            println!("Writing to file...");
            write_file(answer.as_path(), &answer_data);
            let end_time = precise_time_s();
            println!("Time taken: {} seconds", (end_time-start_time));
            println!("Number of 1s predicted: {}", ones);
            println!("Number of 2s predicted: {}", twos);

        },
        _ => help(),
    }

}
