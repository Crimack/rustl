use std::env::current_dir;
mod repeater;
mod text_reader;

fn main() {
    repeater::echo();
    let current = current_dir();
    match current {
        Ok(mut current) => {
            current.push("text/");
            current.push("alice.txt");
            println!("Current: {:?}", current);
            text_reader::read(&current);
        },
        Err(e) => println!("No current dir: Error: {:?}", e),
    }
}
