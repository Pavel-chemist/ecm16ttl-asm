pub struct Code {
    pub line_number: i32,
    pub is_instruction: bool,   //true if mnemonic is in code_parts[0]
    pub is_variable: bool,      //
    pub address: i32,
    pub machine_code: Vec<u16>,  //output of assembler
    pub code_parts: Vec<String>,  //extracted mnemonics for instructions+operands
    pub original_line: String,
}

impl Code {
    pub fn new(number: i32, orig_line: &str) -> Code {
        return Code {
            line_number: number,
            is_instruction: false,
            is_variable: false,
            address: 0,
            machine_code: Vec::with_capacity(4),
            code_parts: Vec::with_capacity(4),
            original_line: String::from(orig_line), 
        };
    }

    pub fn substitute_labels(self) {
        //in code_parts, find non-regName and non-number values, and substitute them with labeled values from labels_table
        //if label is not found, throw error
    }

    pub fn decode_instructions(self) {
        //translate code_parts into u16 values and push them into machine_code
    }

    pub fn put_variable_value(self) {
        //translate code_parts into u16 values and push them into machine_code
    }
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
    pub itype: &'static str,
    pub args: usize,
    pub bytes: i32,
    pub base_word: u16,
}