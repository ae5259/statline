use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Couldn't read from stdin.");

    let lines = input.lines().count();
    let words = input.split_whitespace().count();
    let chars = input.chars().count();

    println!("     {lines}L {words}W {chars}Ch");
}
