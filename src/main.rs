use std::fs;
// declaring modules used
// declaring them in main makes it possible to other modules cross-use structs and functions

mod helpers;
mod structs;
mod enums;
mod lut;
mod preprocessor;
mod command_interpreter;

// declaring functionality of used modules
// use helpers::{read_input, number_parser};
use command_interpreter::interpret;




fn main() {
    let mut command: String;
    let mut program: Vec<u16> = Vec::new();
    let mut machine_instructions: Vec<u16>;
    let file_path: String;
    let file_contents: String;
    // let mut code_lines: Vec<&str> = Vec::new();
    let mut labels_table: Vec<structs::Label> = Vec::new();
    let mut code_listing: Vec<structs::Code> = Vec::new();

    // println!("Use file? y/n:");
    // command = helpers::read_input("");
    command = String::from("y");

    if &command == "y" {
        // println!("Enter the path to the file: ");
        // file_path = helpers::read_input("this should be a string");
        file_path = String::from("test/test.asm");
    
        match fs::read_to_string(file_path) {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(buff) => {
                file_contents = buff;
    
                code_listing = preprocessor::first_read(file_contents.lines().collect(), &mut labels_table);
            }
        };

        println!("\n--------------------------------------------------------------------------------\n");
        println!("Found labels:\n");

        for i in 0..labels_table.len() {
            println!("label: \"{}\", value: 0x{:04X} ({})", labels_table[i].label, labels_table[i].address, labels_table[i].address);

        }

        println!("\n--------------------------------------------------------------------------------\n");
        println!("Full listing:\n");
        println!("line\taddress\tword\t  original line\n");

        for i in 0..code_listing.len() {
            let address: String;
            let machine_code: String;

            if code_listing[i].is_instruction || code_listing[i].is_variable {
                address = format!("{:04X}", code_listing[i].address);

                if code_listing[i].machine_code.len() != 0 {
                    machine_code = format!("{:04X}", code_listing[i].machine_code[0]);
                } else {
                    machine_code = String::from("0000");    
                }
            } else {
                address = String::from("");
                machine_code = String::from("");
            }

            println!("{:04}\t  {}\t{}\t  {}", code_listing[i].line_number, address, machine_code, code_listing[i].original_line);
        }
        

        
    } else {
        println!("Enter asm commands:");

        loop {
            command = helpers::read_input("");
            if command.len() > 0 {
    
                machine_instructions = interpret(&command);
            
                if machine_instructions[0] != 0x00FF {
                    for i in 0..machine_instructions.len() {
                        program.push(machine_instructions[i]);
                    }
                }
            } else {
                break;
            }
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

    println!("\nv2.0 raw");

    for i in 0..instructions.len() {
        println!("{:04X}", instructions[i]);
    }
}
