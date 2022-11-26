// common functions to be used in multiple modules
use std::io;

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

pub fn number_parser(input_string: &str) -> i32 {
    //this should interpret decimal, binary and hex strings, in the most primitive way

    let char_array: Vec<char> = input_string.chars().collect();
    let mut parsed_number: i32 = 0;

    match char_array.len() {
        0 => {
            println!("Empty string provided! default to 0");
        },
        1 => { //must be decimal
            match char_array[0] {
                '0' => parsed_number = 0,
                '1' => parsed_number = 1,
                '2' => parsed_number = 2,
                '3' => parsed_number = 3,
                '4' => parsed_number = 4,
                '5' => parsed_number = 5,
                '6' => parsed_number = 6,
                '7' => parsed_number = 7,
                '8' => parsed_number = 8,
                '9' => parsed_number = 9,
                _ => {
                    parsed_number = 0;
                    println!("not a decimal number!");
                }
            }

            if is_decimal(char_array[0]) {
                parsed_number = decimal_parser(char_array[0])
            }
        },
        2 => { //must be decimal, may be with sign
            // decimal
            if char_array[0] == '-' {
                for i in 1..char_array.len() {
                    parsed_number = parsed_number * 10;

                    parsed_number = parsed_number - decimal_parser(char_array[i]);
                }
            } else {
                for i in 0..char_array.len() {
                    parsed_number = parsed_number * 10;

                    parsed_number = parsed_number + decimal_parser(char_array[i]);
                }
            }
        },
        _ => { //3 and more chars, may be anything
            if char_array[0] == '0' {
                if char_array[1] == 'b' {
                    // binary number
                    for i in 2..char_array.len() {
                        parsed_number = parsed_number << 1;

                        if char_array[i] == '1' {
                            parsed_number = parsed_number + 1;
                        }
                    }
                } else if char_array[1] == 'x' {
                    // hexadecimal
                    for i in 2..char_array.len() {
                        parsed_number = parsed_number << 4;

                        parsed_number = parsed_number + hex_parser(char_array[i]);
                    }
                } else {
                    // malformed
                    println!("Not a number!");
                }
            } else if is_decimal(char_array[0]) {
                // decimal
                if char_array[0] == '-' {
                    for i in 1..char_array.len() {
                        parsed_number = parsed_number * 10;

                        parsed_number = parsed_number - decimal_parser(char_array[i]);
                    }
                } else {
                    for i in 0..char_array.len() {
                        parsed_number = parsed_number * 10;

                        parsed_number = parsed_number + decimal_parser(char_array[i]);
                    }
                }
            } else {
                println!("Not a number! Defaulting to 0");
            }

        },
    }

    return parsed_number;

}

fn is_decimal(d: char) -> bool {
    return d == '1' || d == '2' || d == '3'  || d == '4' || d == '5' || d == '6' || d == '7' || d == '8' || d == '9' || d == '-';
}

fn decimal_parser(digit: char) -> i32 {
    let mut parsed_number: i32 = 0;

    match digit {
        '0' => parsed_number = 0,
        '1' => parsed_number = 1,
        '2' => parsed_number = 2,
        '3' => parsed_number = 3,
        '4' => parsed_number = 4,
        '5' => parsed_number = 5,
        '6' => parsed_number = 6,
        '7' => parsed_number = 7,
        '8' => parsed_number = 8,
        '9' => parsed_number = 9,
        _ => println!("not a decimal number!"),
    }

    return parsed_number;
}

fn hex_parser(digit: char) -> i32 {
    let mut parsed_number: i32 = 0;

    match digit {
        '0' => parsed_number = 0,
        '1' => parsed_number = 1,
        '2' => parsed_number = 2,
        '3' => parsed_number = 3,
        '4' => parsed_number = 4,
        '5' => parsed_number = 5,
        '6' => parsed_number = 6,
        '7' => parsed_number = 7,
        '8' => parsed_number = 8,
        '9' => parsed_number = 9,
        'a' => parsed_number = 10,
        'b' => parsed_number = 11,
        'c' => parsed_number = 12,
        'd' => parsed_number = 13,
        'e' => parsed_number = 14,
        'f' => parsed_number = 15,
        'A' => parsed_number = 10,
        'B' => parsed_number = 11,
        'C' => parsed_number = 12,
        'D' => parsed_number = 13,
        'E' => parsed_number = 14,
        'F' => parsed_number = 15,
        _ => println!("not a hexadecimal number!"),
    }

    return parsed_number;
}