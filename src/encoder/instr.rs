use std::collections::HashMap;
use crate::enums::InstrType;
use crate::structs::{Code, InstrDescr};
use crate::lut::get_instr_table;

pub fn encode_instruction(code_line: &mut Code, labels_map: &HashMap<String, i32>) {
    let instr_map: HashMap<&str, InstrDescr> = get_instr_table();
    let instruction: &str = &code_line.code_parts[0];

    match instr_map.get(instruction) {
        Some(instr) => {
            //instruction is found in table
            code_line.machine_code.push(instr.base_word);

            match instr.itype {
                InstrType::Alu => {
                    
                },
                InstrType::AluConst => {

                },
                InstrType::AluTest => {

                },
                InstrType::AluOneSrc => {

                },
                InstrType::AluRot => {

                },
                InstrType::MemIgpr => {

                },
                InstrType::MemImp => {

                },
                InstrType::MemDirect => {

                },
                InstrType::Mem => {

                },
                InstrType::MemRo => {

                },
                InstrType::MemIo => {

                },
                InstrType::Jmp => {

                },
                InstrType::AddrArithm => {

                },
                InstrType::AddrArImm => {

                },
                InstrType::Mov => {

                },
                InstrType::Movs => {

                },
                InstrType::Misc3bit => {

                },
                InstrType::Misc8bit => {

                },
                InstrType::Misc => {

                },
                _ => {},
            }
        },
        None => {
            // couldn't find the instruction
        }
    }


}