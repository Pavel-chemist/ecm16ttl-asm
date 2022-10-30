// declaring modules used
// declaring them in main makes it possible to other modules cross-use structs and functions

mod helpers;
mod command_interpreter;

// declaring functionality of used modules
use helpers::read_input;
use command_interpreter::command_interpreter;

fn main() {
    let command: String;
    let machine_instructions: Vec<u16>;

    println!("Enter the asm command:");

    command = read_input("");

    machine_instructions = command_interpreter(&command);

    show_code(machine_instructions);
}

fn show_code(machine_instruction: Vec<u16>) {

    for i in 0..machine_instruction.len() {
        println!(
            "word {}: {:04b} {:04b} {:04b} {:04b} | {:04X}",
            i+1,
            machine_instruction[i] >> 12,
            (machine_instruction[i] >> 8) & 0xF,
            (machine_instruction[i] >> 4) & 0xF,
            machine_instruction[i] & 0xF,
            machine_instruction[i],
        );
    }
}



