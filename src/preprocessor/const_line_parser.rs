use crate::structs::Label;
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
        let value: i32 = number_parser(code_line[2]).get().unwrap_or(0) as i32;

        if code_line[1] == "=" {
            let const_name: String = String::from(code_line[0]);
            labels_table.push(Label::new(const_name, value));
        } else {
            println!("\nConstants segment error:\nOn line {}\nAssignment of value should be done with '=' character.\n expected to look like this:\n const_name = <numeric_value>", line_number);

            *err = true;
        }
    }
}