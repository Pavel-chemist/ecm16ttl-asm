use std::collections::HashMap;

use crate::enums::LineType;
use crate::structs::{Code, Label, InstrDescr};
use crate::lut::get_instr_table;

pub fn parse_text_line(
    line_number: usize,
    code_line: Vec<&str>,
    code_length: usize,
    prev_address: i32,
    labels_table: &mut Vec<Label>,
    listing_line: &mut Code,
    err: &mut bool,
) -> i32 {
    //push found labels to labels table
    //if found code, update listing_line
    let instr_lut: HashMap<&str, InstrDescr> = get_instr_table();

    let mut new_address: i32 = prev_address;
    let mut instr_index: usize = 0;

    if code_line[0].chars().last().unwrap() == ':' {
        //found label
        instr_index = 1;

        let mut new_found_label: String = String::new();

        for i in 0..(code_line[0].len()-1) {
                new_found_label.push(code_line[0].chars().nth(i).unwrap());
        }

        labels_table.push(Label::new(new_found_label, prev_address + (prev_address & 0x1)));
    }

    // here, the prefix Aliases should be checked
    if instr_index < code_length {
        match instr_lut.get(code_line[instr_index]) {
            Some(instr) => {
                let num_code_parts: usize = code_length - instr_index - 1;
                let instr_name: &str = code_line[instr_index];

                if num_code_parts == instr.args {
                    new_address = prev_address + instr.bytes;
                    
                    listing_line.address = prev_address;
                    listing_line.line_type = LineType::Instruction;
                    listing_line.code_parts.push(String::from(instr_name));

                    for i in 0..instr.args {
                        listing_line.code_parts.push(String::from(code_line[instr_index + 1 + i]));
                    }



                } else {
                    println!("\nText segment ERROR\nOn line {}\nInstruction {} should have {} arguments, got {}.", line_number, instr_name, instr.args, num_code_parts);

                    *err = true;
                }
            },
            None => {
                //here, all other aliaces are to be checked                      

                println!("\nText segment ERROR\nOn line {}\n{} is not a valid instruction.", line_number, code_line[instr_index]);

                *err = true;
            },
        }
    }
    
    return new_address;
}