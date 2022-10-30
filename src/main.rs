// declaring modules used
// declaring them in main makes it possible to other modules cross-use structs and functions

mod helpers;
mod command_interpreter;

// declaring functionality of used modules
use helpers::read_input;
use command_interpreter::command_interpreter;

fn main() {
    let mut command: String;
    let mut program: Vec<u16> = Vec::new();
    let mut machine_instructions: Vec<u16>;

    println!("Enter asm commands:");

    loop {
        command = read_input("");
        if command.len() > 0 {
            machine_instructions = command_interpreter(&command);
        
            if machine_instructions[0] != 0x00FF {
                for i in 0..machine_instructions.len() {
                    program.push(machine_instructions[i]);
                }
            }
        } else {
            break;
        }
    }   

    show_code(program);
}

fn show_code(instructions: Vec<u16>) {

    for i in 0..instructions.len() {
        println!(
            "word {}: {:04b} {:04b} {:04b} {:04b} | {:04X}",
            i+1,
            instructions[i] >> 12,
            (instructions[i] >> 8) & 0xF,
            (instructions[i] >> 4) & 0xF,
            instructions[i] & 0xF,
            instructions[i],
        );
    }
}



