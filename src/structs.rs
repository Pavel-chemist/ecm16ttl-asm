pub struct Code {
    pub line_number: i32,
    pub is_instruction: bool,  //true if mnemonic is in code_parts[0]
    pub address: i32,
    // pub num_words: i32,  //number of machine code words: 1, 2 or 3 
    pub machine_code: Vec<u16>,  //output of assembler
    // pub label: String,
    pub code_parts: Vec<&'static str>,  //extracted mnemonics for instructions+operands
    pub original_line: String,
}

impl Code {
    pub fn new(number: i32, orig_line: &str) -> Code {
        return Code {
            line_number: number,
            is_instruction: false,
            address: 0,
            machine_code: Vec::with_capacity(2),
            code_parts: Vec::with_capacity(4),
            original_line: String::from(orig_line), 
        };
    }
}

pub struct Label {
    pub label: &'static str,
    pub address: i32,
}

impl Label {
    pub fn new(name: &'static str, addr: i32) -> Label {
        return Label {
            label: name,
            address: addr,
        };
    }
}

pub struct InstrDescr {
    pub itype: &'static str,
    pub args: u32,
    pub bytes: u32,
    pub base_word: u16,
}