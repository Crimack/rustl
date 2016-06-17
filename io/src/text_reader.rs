use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read(path : &Path) {
    let mut input = String::new();
    let mut words = HashMap::new();
    let display = path.display();

    // totally ripped out of rust by example
    let mut file = match File::open(path) {
            Err(why) => panic!("couldn't open {}: {}", display,
                                                       why.description()),
            Ok(file) => file,
        };

    file.read_to_string(&mut input).expect("Failed to load file contents");
    for word in input.split_whitespace() {
        println!("{}", word);
        let word_collected = words.entry(word).or_insert(0);
        *word_collected += 1;
    }

    println!("{:#?}", words);
}
