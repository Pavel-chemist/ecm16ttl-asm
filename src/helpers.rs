// common functions to be used in multiple modules
use std::io;

use crate::structs::{Arg, NumParseRes};
use crate::enums::ArgType;

pub fn read_input<T: std::str::FromStr>(error_message: &str) -> T {
    // generic function that reads input and checks for type (number or text)
    // the error handling should be here
    let mut input: String = String::new();
    let result: T;
    let error: &str;

    if error_message.trim() == "" {
        error = "use correct value type, e.g. number";
    } else {
        error = error_message;
    }

    loop {
        io::stdin()
        .read_line(&mut input)
        .expect("failed to read the line");

        input = input.trim().to_string();

        match input.parse::<T>() {
            Ok(parsed_value) => {
                result = parsed_value;
                break;
            },
            Err(_) => {
                println!("{}", error);
                input = String::new();
                continue;
            },
        };
    }

    return result;
}

pub fn number_parser(input_string: &str) -> NumParseRes {
    //this should interpret decimal, binary and hex strings, in the most primitive way

    let char_array: Vec<char> = input_string.chars().collect();
    let mut num: NumParseRes = NumParseRes::new();
    num.set(0);

    match char_array.len() {
        0 => {
            num.clear();
        },
        1 => { //must be decimal
            num.push_dec_char(char_array[0]);
        },
        2 => { //must be decimal, may be with sign
            // decimal
            if char_array[0] == '-' {
                num.push_dec_char(char_array[1]);
                num.make_negative();
            } else {
                for i in 0..char_array.len() {
                    num.push_dec_char(char_array[i]);
                }
            }
        },
        _ => { //3 and more chars, may be anything
            if char_array[0] == '0' {
                if char_array[1] == 'b' {
                    // binary number
                    for i in 2..char_array.len() {
                        num.push_binary_char(char_array[i]);
                    }
                } else if char_array[1] == 'x' {
                    // hexadecimal
                    for i in 2..char_array.len() {
                        num.push_hex_char(char_array[i]);
                    }
                } else {
                    num.clear();
                }
            } else {
                // decimal
                if char_array[0] == '-' {
                    for i in 1..char_array.len() {
                        num.push_dec_char(char_array[i]);
                    }
                } else {
                    for i in 0..char_array.len() {
                        num.push_dec_char(char_array[i]);
                    }
                }
            }

        },
    }

    return num;

}

pub fn register_matcher(arg_name: &str) -> Arg {
    match arg_name {
        "r0" => Arg::new(ArgType::Gpr, 0),
        "r1" => Arg::new(ArgType::Gpr, 1),
        "r2" => Arg::new(ArgType::Gpr, 2),
        "r3" => Arg::new(ArgType::Gpr, 3),
        "r4" => Arg::new(ArgType::Gpr, 4),
        "r5" => Arg::new(ArgType::Gpr, 5),
        "r6" => Arg::new(ArgType::Gpr, 6),
        "r7" => Arg::new(ArgType::Gpr, 7),
        "mp0" => Arg::new(ArgType::Mpr, 0),
        "mp1" => Arg::new(ArgType::Mpr, 1),
        "mp2" => Arg::new(ArgType::Mpr, 2),
        "mp3" => Arg::new(ArgType::Mpr, 3),
        "mp4" => Arg::new(ArgType::Mpr, 4),
        "mp5" => Arg::new(ArgType::Mpr, 5),
        "mp6" => Arg::new(ArgType::Mpr, 6),
        "mp7" => Arg::new(ArgType::Mpr, 7),
        "PCH" => Arg::new(ArgType::Mpr, 0),
        "PCL" => Arg::new(ArgType::Mpr, 1),
        "SPH" => Arg::new(ArgType::Mpr, 2),
        "SPL" => Arg::new(ArgType::Mpr, 3),
        "FPH" => Arg::new(ArgType::Mpr, 4),
        "FPL" => Arg::new(ArgType::Mpr, 5),
        "BPH" => Arg::new(ArgType::Mpr, 6),
        "BPL" => Arg::new(ArgType::Mpr, 7),
        "PC" => Arg::new(ArgType::MP, 0),
        "SP" => Arg::new(ArgType::MP, 1),
        "FP" => Arg::new(ArgType::MP, 2),
        "BP" => Arg::new(ArgType::MP, 3),
        "SR" => Arg::new(ArgType::Special, 1),
        "MDB" => Arg::new(ArgType::Special, 2),
        "IVB" => Arg::new(ArgType::Special, 3),
        _ => {

            Arg::new(ArgType::Value, 0)
        },
    }   


}