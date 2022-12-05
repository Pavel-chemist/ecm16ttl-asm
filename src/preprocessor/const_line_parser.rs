use crate::structs::{Label, NumParseRes};
use crate::helpers::number_parser;

pub fn parse_constants_line(
    line_number: usize,
    code_line: Vec<&str>,
    code_length: usize,
    labels_table: &mut Vec<Label>,
    err: &mut bool,
) {
    //push found constants to labels table
    if code_length != (3 as usize) {
        println!("\nConstants segment error:\nOn line {} part of constant definition is missing or there are extra parts.\nexpected to look like this:\n const_name = <numeric_value>", line_number);

        *err = true;
    } else {
        let value: NumParseRes = number_parser(code_line[2]);

        if value.is_num() {
            if code_line[1] == "=" {
                let const_name: String = String::from(code_line[0]);

                for i in 0..labels_table.len() {
                    if labels_table[i].label == const_name {
                        // error
                        println!("\nConstants segment ERROR\nOn line {}\nLabel \"{}\" is not unique!.", line_number, const_name);
        
                        *err = true;
                        break;
                    }
                }
        
                if !*err {
                    labels_table.push(Label::new(const_name, value.get().unwrap() as i32));
                }
            } else {
                println!("\nConstants segment ERROR:\nOn line {}\nAssignment of value should be done with '=' character.\n expected to look like this:\n const_name = <numeric_value>", line_number);

                *err = true;
            }
        } else {
            println!("\nConstants segment ERROR:\nOn line {}\nValue to the right of '=' should be a number.\ngot \"{}\"", line_number, code_line[2]);

            *err = true;
        }
    }
}