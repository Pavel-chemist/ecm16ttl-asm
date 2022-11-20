// declaring modules used
// declaring them in main makes it possible to other modules cross-use structs and functions

mod helpers;
mod command_interpreter;

// declaring functionality of used modules
// use helpers::{read_input, number_parser};
use command_interpreter::command_interpreter;

use std::fs;
// use fltk::{app, prelude::*, *, enums};


fn main() {
    let mut command: String;
    let mut program: Vec<u16> = Vec::new();
    let mut machine_instructions: Vec<u16>;
    let file_path: String;
    let file_contents: String;
    let mut code_lines: Vec<&str> = Vec::new();


    println!("Enter the path to the file: ");
    file_path = helpers::read_input("this should be a string");

    match fs::read_to_string(file_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(buff) => {
            file_contents = buff;
            code_lines = file_contents.lines().collect();

            println!("lines: ");

            for i in 0..code_lines.len() {
                println!("{}", code_lines[i]);
            }
        }
    };

    for i in 0..code_lines.len() {
        machine_instructions = command_interpreter(code_lines[i]);
        
        if machine_instructions[0] != 0x00FF {
            for i in 0..machine_instructions.len() {
                program.push(machine_instructions[i]);
            }
        }
    }

    // println!("Enter asm commands:");

    /* loop {
        command = helpers::read_input("");
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
    } */   

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

/* 
fn open_file_dialog() /* -> Vec<String> */ {
    let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile);
    dialog.show();

    println!("{:?}", dialog.filename());

    let file_name: String = dialog.filename().into_os_string().into_string().unwrap();

    let mut string_with_newlines: String = String::new();

    for c in file_name.chars() {
        if c == '/' {
            string_with_newlines.push('\n');
        } else {
            string_with_newlines.push(c);
        }
    }
    
    if file_name.len() > 0 {
        // return read_file(&file_name);

        read_file(&file_name);
    }

    // return vec![String::from("No file chosen"); 1];
}

fn read_file(file_path: &str) /* -> Vec<String> */ {
    let mut file_buff: Vec<u8> = Vec::new();

    match fs::read(file_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(buff) => {
            file_buff = buff;

            println!("file buffer:");

            for i in 0..file_buff.len() {
                print!("{}, ", file_buff[i]);
            }
        }
    };



    // return print_file_buffer(file_buff);
}
 */
