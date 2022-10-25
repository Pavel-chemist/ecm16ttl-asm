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