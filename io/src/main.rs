mod repeater;
mod text_reader;

fn main() {
    repeater::echo();
    text_reader::read("C:/Users/Chris/Documents/repos/rustl/io/text/alice.txt")
}
