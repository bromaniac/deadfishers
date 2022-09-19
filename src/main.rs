use std::io;
use std::io::Read;

use deadfishers::interpret;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin.");

    interpret(&input, &mut std::io::stdout());
}