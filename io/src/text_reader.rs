use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

struct Counter<'a> {
    word: &'a str,
    count: i32
}

pub fn read(path : &str) {
    let mut words : Vec<Counter> = vec![];
    let rpath = Path::new(path);
    let display = rpath.display();

    // totally ripped out of rust by example
    let mut file = match File::open(&rpath) {
            Err(why) => panic!("couldn't open {}: {}", display,
                                                       why.description()),
            Ok(file) => file,
        };

    let mut word_string = String::new();
    file.read_to_string(&mut word_string);
    for word in word_string.split_whitespace() {
        let word_collected = words.iter()
                             .find(|x| x.word == word);
        match word_collected {
            Some(x) => x.count += 1,
            None => words.push(make_counter(word)),
        }
    }

}

fn make_counter(word : &str) -> Counter {
    let new_count = Counter { word:word , count : 1};
    new_count
}
