use crate::helpers::number_parser;
use crate::structs::Code;
use crate::enums::VarType;


pub fn encode_variable(code_line: &mut Code) {
    match code_line.var_type {
        VarType::Word => {
            code_line.machine_code.push(number_parser(&code_line.code_parts[0]) as u16);
        },
        VarType::Dword => {
            let value: u32 = number_parser(&code_line.code_parts[0]) as u32;
            code_line.machine_code.push((value.clone() >> 16) as u16);
            code_line.machine_code.push(value as u16);
        },
        VarType::Long => {
            let value: u64 = number_parser(&code_line.code_parts[0]) as u64;
            code_line.machine_code.push((value.clone() >> 48) as u16);
            code_line.machine_code.push((value.clone() >> 32) as u16);
            code_line.machine_code.push((value.clone() >> 16) as u16);
            code_line.machine_code.push(value as u16);
        },
        VarType::String => {
            if code_line.code_parts[0].is_ascii() {
                let mut bytes: Vec<u8> = code_line.code_parts[0].as_bytes().to_vec();
                if bytes.len() & 0x1 == 1 {
                    bytes.push(0);
                } else {
                    bytes.push(0);
                    bytes.push(0);
                }
                

                for i in 0..((bytes.len() + 1) / 2) {
                    let couple_bytes: u16 = ((bytes[i * 2] as u16) << 8) | (bytes[i * 2 + 1] as u16);
                    code_line.machine_code.push(couple_bytes);
                }

            } else {
                //error about non-ascii string
                println!("Variable encoder ERROR\nOn line {}\nThe string contains non-ascii characters!", code_line.line_number);
            }
        },
        VarType::None => {
            //error about undefined type
            println!("Variable encoder ERROR\nOn line {}\nVariable {} has undefined type!", code_line.line_number, code_line.code_parts[0]);
        }
    }
}