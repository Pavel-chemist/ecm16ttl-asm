use std::{fs, path};
// declaring modules used
// declaring them in main makes it possible to other modules cross-use structs and functions

mod helpers;
mod structs;
mod enums;
mod lut;
mod preprocessor;
mod encoder;

// declaring functionality of used modules
use helpers::read_input;
use encoder::encode;
use structs::{Code, Label};

use crate::enums::LineType;

fn main() {
    let file_path_in: String;
    let file_contents: String;
    let mut labels_table: Vec<Label> = Vec::new();
    let mut code_listing: Vec<Code> = Vec::new();
    let mut err: bool = false;

    println!("Enter path to .asm file:");

    file_path_in = read_input("couldn't read input");
    
    match fs::read_to_string(&file_path_in) {
        Err(why) => {
            println!("! {:?}", why.kind());
            err = true;
        },
        Ok(buff) => {
            file_contents = buff;

            // first pass
            code_listing = preprocessor::first_read(file_contents.lines().collect(), &mut labels_table, &mut err);

            // second pass
            encode(&mut code_listing, &labels_table, &mut err);
        }
    };

    if !err {
        save_files(&code_listing, &labels_table, &file_path_in);
    }
}

fn save_files(code_listing: &Vec<Code>, labels_table: &Vec<Label>, file_path: &str) {
    let path_root_parts: Vec<&str> = file_path.split(".").collect();
    let path_root: String = String::from(path_root_parts[0]);
    
    let mut path_to_list: String = path_root.clone();
    path_to_list.push_str(".list");

    let mut path_to_hex: String = path_root.clone();
    path_to_hex.push_str(".hex");

    let path = path::Path::new(&path_to_list);
    let display = path.display();

    let mut file = match fs::File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match std::io::Write::write_all(&mut file, collate_listing(code_listing, labels_table).as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    let path = path::Path::new(&path_to_hex);
    let display = path.display();

    let mut file = match fs::File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match std::io::Write::write_all(&mut file, output_hex(code_listing).as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

fn collate_listing(code_listing: &Vec<Code>, labels_table: &Vec<Label>) -> String {
    let mut list_output: String = String::new();
    
    list_output.push_str("\n--------------------------------------------------------------------------------\n");
    list_output.push_str(&format!("Found labels:\n\n{:20}{:16}{:16}", "label:", "hex value:", "decimal value:\n"));

    for i in 0..labels_table.len() {
        list_output.push_str(&format!("\n{:20}0x{:08X}{:6}{}", labels_table[i].label, labels_table[i].address, "", labels_table[i].address));
    }

    list_output.push_str("\n--------------------------------------------------------------------------------\n");
    list_output.push_str("Full listing:\n");
    list_output.push_str(&format!("\n{:8}{:12}{:8}{}",  "line:", "address:", "word:", "original line:\n"));

    for i in 0..code_listing.len() {
        let mut address: String;
        let mut machine_code: String;

        match code_listing[i].line_type {
            LineType::Untyped => {
                address = String::from("");
                machine_code = String::from("");    
            },
            _ => {
                address = format!("{:04X} {:04X}", (code_listing[i].address >> 16), code_listing[i].address);

                if code_listing[i].machine_code.len() != 0 {
                    machine_code = format!("{:04X}", code_listing[i].machine_code[0]);
                } else {
                    machine_code = String::from("0000");    
                }    
            }
        }

        list_output.push_str(&format!("\n{:5}{:3}{:12}{:8}{}", code_listing[i].line_number, "", address, machine_code, code_listing[i].original_line));

        if code_listing[i].machine_code.len() > 1 {
            for j in 1..code_listing[i].machine_code.len() {
                machine_code = format!("{:04X}", code_listing[i].machine_code[j]);
                address = format!("{:04X} {:04X}", ((code_listing[i].address + (j as i32) * 2) >> 16), (code_listing[i].address + (j as i32) * 2));

                list_output.push_str(&format!("\n{:8}{:12}{:8}", "", address, machine_code));
            }
        }
    }

    return list_output;
}

fn output_hex(code_listing: &Vec<Code>) -> String {
    let mut hex_output: String = String::new();

    hex_output.push_str("v2.0 raw\n");
    for i in 0..code_listing.len() {
        for j in 0..code_listing[i].machine_code.len() {
            hex_output.push_str(&format!("{:04X}\n", code_listing[i].machine_code[j]));
        }
    }

    return hex_output;
}