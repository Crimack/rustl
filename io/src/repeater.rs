use std::io;

pub fn echo() {
    println!("Enter something");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("You suck at this");
    println!("You entered: {}", input);

    for word in input.split_whitespace() {
        println!("{}", word)
    }
}
