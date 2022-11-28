use crate::enums::{LineType, VarType, InstrType, ArgType};

pub struct Code {
    pub line_number: i32,
    pub line_type: LineType,
    pub address: i32,
    pub machine_code: Vec<u16>,  //output of assembler
    pub code_parts: Vec<String>,  //extracted mnemonics for instructions+operands
    pub var_type: VarType,
    pub original_line: String,
}

impl Code {
    pub fn new(number: i32, orig_line: &str) -> Code {
        return Code {
            line_number: number,
            line_type: LineType::Untyped,
            address: 0,
            machine_code: Vec::with_capacity(4),
            code_parts: Vec::with_capacity(4),
            var_type: VarType::None,
            original_line: String::from(orig_line), 
        };
    }

    /* pub fn substitute_labels(self) {
        //in code_parts, find non-regName and non-number values, and substitute them with labeled values from labels_table
        //if label is not found, throw error
    }

    pub fn decode_instructions(self) {
        //translate code_parts into u16 values and push them into machine_code
    }

    pub fn put_variable_value(self) {
        //translate code_parts into u16 values and push them into machine_code
    } */
}

pub struct Label {
    pub label: String,
    pub address: i32,
}

impl Label {
    pub fn new(name: String, addr: i32) -> Label {
        return Label {
            label: name,
            address: addr,
        };
    }
}

pub struct InstrDescr {
    pub itype: InstrType,
    pub args: usize,
    pub bytes: i32,
    pub base_word: u16,
}

pub struct Arg {
    pub arg_type: ArgType,
    pub value: i32,
}

impl Arg {
    pub fn new(arg_type: ArgType, value: i32) -> Arg {
        return Arg {
            arg_type,
            value,
        };
    }
}

pub struct NumParseRes {
    num_val: i64,
    is_num: bool,
}

impl NumParseRes {
    pub fn new() -> NumParseRes {
        return NumParseRes { num_val: 0, is_num: false };
    }

    pub fn set(&mut self, num: i64) {
        self.is_num = true;
        self.num_val = num;
    }

    pub fn clear(&mut self) {
        self.is_num = false;
        self.num_val = 0;
    }

    pub fn get(&self) -> Option<i64> {
        if self.is_num {
            return Some(self.num_val);
        } else {
            return None;
        }
    }

    pub fn update(&mut self, num: i64) {
        if self.is_num {
            self.num_val = num;
        } else {
            println!("Error updating a number: trying to update NaN value");
        }
    }

    pub fn push_binary_char(&mut self, digit: char) {
        if self.is_num {
            match digit {
                '0' => self.num_val = self.num_val << 1,
                '1' => self.num_val = (self.num_val << 1) + 1,
                _ => {
                    println!("\nError adding binary digit:\n {} is not 0 or 1", digit);
                    self.is_num = false;
                }
            }
        } else {
            println!("Error adding binary digit:\n trying to update NaN value");
        }
    }

    pub fn push_dec_char(&mut self, digit: char) {
        if self.is_num {
            match digit {
                '0' => self.num_val = self.num_val * 10,
                '1' => self.num_val = (self.num_val * 10) + 1,
                '2' => self.num_val = (self.num_val * 10) + 2,
                '3' => self.num_val = (self.num_val * 10) + 3,
                '4' => self.num_val = (self.num_val * 10) + 4,
                '5' => self.num_val = (self.num_val * 10) + 5,
                '6' => self.num_val = (self.num_val * 10) + 6,
                '7' => self.num_val = (self.num_val * 10) + 7,
                '8' => self.num_val = (self.num_val * 10) + 8,
                '9' => self.num_val = (self.num_val * 10) + 9,
                _ => {
                    println!("\nError adding decimal digit:\n {} is not in range [0-9]", digit);
                    self.is_num = false;
                }
            }
        } else {
            println!("Error adding decimal digit:\n trying to update NaN value");
        }
    }

    pub fn push_hex_char(&mut self, digit: char) {
        if self.is_num {
            match digit {
                '0' => self.num_val = self.num_val << 4,
                '1' => self.num_val = (self.num_val << 4) + 1,
                '2' => self.num_val = (self.num_val << 4) + 2,
                '3' => self.num_val = (self.num_val << 4) + 3,
                '4' => self.num_val = (self.num_val << 4) + 4,
                '5' => self.num_val = (self.num_val << 4) + 5,
                '6' => self.num_val = (self.num_val << 4) + 6,
                '7' => self.num_val = (self.num_val << 4) + 7,
                '8' => self.num_val = (self.num_val << 4) + 8,
                '9' => self.num_val = (self.num_val << 4) + 9,
                'a' => self.num_val = (self.num_val << 4) + 10,
                'A' => self.num_val = (self.num_val << 4) + 10,
                'b' => self.num_val = (self.num_val << 4) + 11,
                'B' => self.num_val = (self.num_val << 4) + 11,
                'c' => self.num_val = (self.num_val << 4) + 12,
                'C' => self.num_val = (self.num_val << 4) + 12,
                'd' => self.num_val = (self.num_val << 4) + 13,
                'D' => self.num_val = (self.num_val << 4) + 13,
                'e' => self.num_val = (self.num_val << 4) + 14,
                'E' => self.num_val = (self.num_val << 4) + 14,
                'f' => self.num_val = (self.num_val << 4) + 15,
                'F' => self.num_val = (self.num_val << 4) + 15,
                _ => {
                    println!("\nError adding hex digit:\n {} is not in range [0-9a-fA-F]", digit);
                    self.is_num = false;
                }
            }
        } else {
            println!("Error adding hex digit:\n trying to update NaN value");
        }
    }

    pub fn make_negative(&mut self) {
        if self.is_num {
            self.num_val = 0 - self.num_val;
        } else {
            println!("Error making negative: trying to update NaN value");
        }
    }


}