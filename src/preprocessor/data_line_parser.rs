use crate::{structs::{Code, Label}, enums::{LineType, VarType}};


pub fn parse_data_line(
    line_number: usize,
    code_line: Vec<&str>,
    code_length: usize,
    prev_address: i32,
    labels_table: &mut Vec<Label>,
    listing_line: &mut Code,
    err: &mut bool,
)-> i32 {
    //push found labels to labels table
    //if found variable directive and also variable value, update listing_line
    let mut new_address: i32 = prev_address;
    let mut size_directive_index: usize = 0;

    if code_line[0].chars().last().unwrap() == ':' {
        //found label
        size_directive_index = 1;

        let mut new_found_label: String = String::new();

        for i in 0..(code_line[0].len()-1) {
            new_found_label.push(code_line[0].chars().nth(i).unwrap());
        }

        for i in 0..labels_table.len() {
            if labels_table[i].label == new_found_label {
                // error
                println!("\nData segment ERROR\nOn line {}\nLabel \"{}\" is not unique!.", line_number, new_found_label);

                *err = true;
                break;
            }
        }

        if !*err {
            labels_table.push(Label::new(new_found_label, prev_address + (prev_address & 0x1)));
        }
    }

    if size_directive_index < code_length {
        match code_line[size_directive_index] {
            ".word" => {
                new_address = prev_address + 2 + (prev_address & 0x1);
    
                listing_line.line_type = LineType::Variable;
                listing_line.address = prev_address + (prev_address & 0x1);
                listing_line.var_type = VarType::Word;
    
                if code_length > size_directive_index + 1 {
                    listing_line.code_parts.push(String::from(code_line[size_directive_index + 1]));
                } else {
                    listing_line.code_parts.push(String::from("0x0000"));
                }
            },
            ".dword" => {
                new_address = prev_address + 4 + (prev_address & 0x1);
    
                listing_line.line_type = LineType::Variable;
                listing_line.address = prev_address + (prev_address & 0x1);
                listing_line.var_type = VarType::Dword;
    
                if code_length > size_directive_index + 1 {
                    listing_line.code_parts.push(String::from(code_line[size_directive_index + 1]));
                } else {
                    listing_line.code_parts.push(String::from("0x00000000"));
                }
            },
            ".long" => {
                new_address = prev_address + 8 + (prev_address & 0x1);
    
                listing_line.line_type = LineType::Variable;
                listing_line.address = prev_address + (prev_address & 0x1);
                listing_line.var_type = VarType::Long;
    
                if code_length > size_directive_index + 1 {
                    listing_line.code_parts.push(String::from(code_line[size_directive_index + 1]));
                } else {
                    listing_line.code_parts.push(String::from("0"));
                }
            },
            ".string" => {
                // let concatenated_string: String = String::new();
                let mut string_value: String = String::new();
                let mut string_proper: bool = false;
                let mut string_length: i32 = 0;

                if code_length > size_directive_index + 1 {

                    let string_part: String = String::from(listing_line.original_line.clone());

                    for j in 0..string_part.len() {
                        if string_proper {
                            string_length = string_length + 1;

                            if string_part.chars().nth(j).unwrap() == '"' {
                                if string_part.chars().nth(j-1).unwrap() != '\\' {
                                    string_proper = false;
                                } else {
                                    string_value.pop();
                                    string_length = string_length - 1;
                                    string_value.push('"');
                                }
                                    
                            } else {
                                string_value.push(string_part.chars().nth(j).unwrap());
                            }                            
                        } else if string_part.chars().nth(j).unwrap() == '"' {
                            string_proper = true;
                        }
                    }
                    

                    new_address = prev_address + string_length + (prev_address & 0x1);
    
                    listing_line.line_type = LineType::Variable;
                    listing_line.address = prev_address + (prev_address & 0x1);

                    listing_line.code_parts.push(string_value);
                    listing_line.var_type = VarType::String;
                } else {
                    // error
                    println!("\nData segment ERROR:\nOn line {}\n String variables should always be initialized:\n [label:] .strig \"some string value\"", line_number);

                    *err = true;
                }
            },
            _ => {
                println!("\nData segment ERROR:\nOn line {}\n Variable should always have size directive,\n one of '.word', '.dword', '.long' or '.string'.\n got '{}'", line_number, code_line[size_directive_index]);

                *err = true;
            },
        }
    }

    return new_address;
}