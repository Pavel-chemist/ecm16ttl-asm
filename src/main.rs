use std::fs;
// declaring modules used
// declaring them in main makes it possible to other modules cross-use structs and functions

mod helpers;
mod structs;
mod enums;
mod lut;
mod preprocessor;
mod encoder;

// declaring functionality of used modules
// use helpers::{read_input, number_parser};
use encoder::encode;

use crate::enums::LineType;

fn main() {
    let file_path: String;
    let file_contents: String;
    let mut labels_table: Vec<structs::Label> = Vec::new();
    let mut code_listing: Vec<structs::Code> = Vec::new();

    file_path = String::from("test/test.asm");
    
    match fs::read_to_string(file_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(buff) => {
            file_contents = buff;

            // first pass
            code_listing = preprocessor::first_read(file_contents.lines().collect(), &mut labels_table);

            // second pass
            encode(&mut code_listing, &labels_table);
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
        let mut address: String;
        let mut machine_code: String;

        match code_listing[i].line_type {
            LineType::Untyped => {
                address = String::from("");
                machine_code = String::from("");    
            },
            _ => {
                address = format!("{:04X}", code_listing[i].address);

                if code_listing[i].machine_code.len() != 0 {
                    machine_code = format!("{:04X}", code_listing[i].machine_code[0]);
                } else {
                    machine_code = String::from("0000");    
                }    
            }
        }

        println!("{:04}\t  {}\t{}\t  {}", code_listing[i].line_number, address, machine_code, code_listing[i].original_line);

        if code_listing[i].machine_code.len() > 1 {
            for j in 1..code_listing[i].machine_code.len() {
                machine_code = format!("{:04X}", code_listing[i].machine_code[j]);
                address = format!("{:04X}", (code_listing[i].address + (j as i32) * 2));

                println!("{}\t  {}\t{}\t  {}", "", address, machine_code, "");
            }
        }
    }
}
