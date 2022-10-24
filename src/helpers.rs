// common functions to be used in multiple modules
use std::io;
/* pub struct PixCoord {
    pub x: f64,
    pub y: f64,
}

pub fn exp (n: f64, e: i32) -> f64 {
    let mut res: f64 = 1.0;
    
    if e >= 0 {
        for _i in 0..e {
            res = res * n;
        }
    } else {
        for _i in 0..(-e) {
            res = res / n;
        }
    }
    
    return res;
} */

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