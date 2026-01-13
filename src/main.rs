use std::io;
use std::io::{Read};

fn main() {
    let mut string = String::new();
    io::stdin().read_to_string(&mut string).expect("Couldn't read from stdin."); 

    let binding = string.trim();

    let mut count = match binding {
        "" => 0,
        _ => 1,
    };

    let chars = binding.split("").collect::<Vec<&str>>();
    println!("{:?}", chars.clone());

    for ch in chars.clone() {
        match ch {
           "\n" => { count = count + 1; }
            _ => { count = count + 0; }
        }    
    }

    let binding = string.as_str().replace("\n", " ");
    let binding = binding.trim();
    let words = binding.split(" ").collect::<Vec<&str>>();
    
    println!("     {:?}L {:?}W {:?}Ch", count, words.len(), chars.len() - 1 );
}
