use std::fs::File;

struct Counter<'a> {
    word: &'a str,
    count: i32
}

pub fn read(path : &str) {
    let mut words : Vec<&Counter> = vec![];
    let mut f = try!(File::open(path));
    let word_string = String::new();
    try!(f.read_to_string(&mut word_string));
    for word in word_string.split_whitespace() {
        println!("{}", word);
    }

}
