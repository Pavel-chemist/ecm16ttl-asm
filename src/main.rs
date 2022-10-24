// declaring modules used
// declaring them in main makes it possible to other modules cross-use structs and functions

mod helpers;

// declaring functionality of used modules
use helpers::{read_input};

fn main() {
    let command: String;

    println!("Enter the asm command:");

    command = read_input("");

    println!("The command entered: {}", command);
}
