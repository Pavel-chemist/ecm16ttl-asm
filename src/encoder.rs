// this takes in the listing array, and performs decodiding of instructions and variables 
// into machine code, with look-ups in labels_table and instr_table
mod var;
mod instr;

use std::collections::HashMap;
use crate::{structs::{Code, Label}, enums::LineType};
use var::encode_variable;
use instr::encode_instruction;

pub fn encode(code_listing: &mut Vec<Code>, labels_table: &Vec<Label>, err: &mut bool) {
    let labels_map: HashMap<String, i32> = labels_table.iter().map(|l: &Label| (l.label.clone(), l.address)).collect();


    for i in 0..code_listing.len() {
        match code_listing[i].line_type {
            LineType::Instruction => {
                encode_instruction(&mut code_listing[i], &labels_map, err);
            },
            LineType::Variable => {
                encode_variable(&mut code_listing[i], err);
            },
            LineType::Untyped => {
                // nothing to do here
            },
        }
    }
}
