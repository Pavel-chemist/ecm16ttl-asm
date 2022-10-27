enum Incr {
    Pre,
    Post,
    None,
    Err,
}

pub fn ldi(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    let imm_val: u32;

    machine_instruction.push(0x2000);

    if command_parts.len() > 2 {
        imm_val = command_parts[2].parse::<u32>().unwrap_or_default();

        machine_instruction.push((imm_val & 0xFFFF) as u16);

        if command_parts[1].chars().nth(0).unwrap_or(' ') == 'r' {
            println!("immediate load to GPR");

            if command_parts[1].chars().nth(1).unwrap_or('0').to_digit(10).unwrap() < 8 {
                machine_instruction[0] = machine_instruction[0] | ((command_parts[1].chars().nth(1).unwrap().to_digit(10).unwrap() << 8) as u16);
            }

            if imm_val > 0xFFFF {
                println!("Immediate value too big! It will be trimmed to 16bit.");
            }
        } else if command_parts[1] == "PC" || command_parts[1] == "SP" || command_parts[1] == "FP" || command_parts[1] == "BP" {
            println!("immediate load to MP pair");

            match command_parts[1] {
                "PC" => machine_instruction[0] = machine_instruction[0] | (0b00 << 9),
                "SP" => machine_instruction[0] = machine_instruction[0] | (0b01 << 9),
                "FP" => machine_instruction[0] = machine_instruction[0] | (0b10 << 9),
                "BP" => machine_instruction[0] = machine_instruction[0] | (0b11 << 9),
                &_ => {},
            }

            machine_instruction[0] = machine_instruction[0] | 0x0800 | (((imm_val >> 16) & 0x01FF) as u16);

            if imm_val > 0x01FFFFFF {
                println!("Immediate value too big! It will be trimmed to 16bit.");
            }
        } else {
            println!("LDi instruction error: wrong first operand");        
        }

    } else {
        println!("LDi instruction error: not enough operands provided");
    }

    return machine_instruction;
}

///////////////////////////////////////////////////////////////////////////////////////

pub fn ldd(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    
    machine_instruction.push(0x4000);
   
    return direct_address(command_parts, machine_instruction);
}

pub fn std(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    
    machine_instruction.push(0x5000);
   
    return direct_address(command_parts, machine_instruction);
}

///////////////////////////////////////////////////////////////////////////////////////

pub fn ld(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    
    machine_instruction.push(0x6000);

    return indirect_access_decode(command_parts, machine_instruction);
}

pub fn st(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    
    machine_instruction.push(0x7000);

    return indirect_access_decode(command_parts, machine_instruction);
}

///////////////////////////////////////////////////////////////////////////////////////

pub fn jump(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    
    machine_instruction.push(0x1000);

    return jump_mem_access_decode(command_parts, machine_instruction);
}

pub fn jz(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    
    machine_instruction.push(0x1100);

    return jump_mem_access_decode(command_parts, machine_instruction);
}

pub fn jn(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    
    machine_instruction.push(0x1200);

    return jump_mem_access_decode(command_parts, machine_instruction);
}

pub fn jo(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    
    machine_instruction.push(0x1400);

    return jump_mem_access_decode(command_parts, machine_instruction);
}

pub fn jc(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    
    machine_instruction.push(0x1800);

    return jump_mem_access_decode(command_parts, machine_instruction);
}

pub fn jnz(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    
    machine_instruction.push(0x1E00);

    return jump_mem_access_decode(command_parts, machine_instruction);
}

pub fn jnn(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    
    machine_instruction.push(0x1D00);

    return jump_mem_access_decode(command_parts, machine_instruction);
}

pub fn jno(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    
    machine_instruction.push(0x1B00);

    return jump_mem_access_decode(command_parts, machine_instruction);
}

pub fn jnc(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    
    machine_instruction.push(0x1700);

    return jump_mem_access_decode(command_parts, machine_instruction);
}

pub fn jsr(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    
    machine_instruction.push(0x1F00);

    return jump_mem_access_decode(command_parts, machine_instruction);
}

///////////////////////////////////////////////////////////////////////////////////////

fn direct_address(command_parts: Vec<&str>, mut machine_instruction: Vec<u16>) -> Vec<u16> {
    let address: u32;
    let register: u16;

    if command_parts.len() > 2 {
        if check_address_in_parentheses(command_parts[2]) {
            address = command_parts[2][1..(command_parts[2].len()-1)].parse::<u32>().unwrap_or_default() >> 1;
        } else {
            println!("Direct Address Memory Access instruction error:\nDirect address should be in parentheses!");
            address = 0;
        }

        register = reg_address_decode(command_parts[1]);

        if register != 0xFFFF {
            machine_instruction[0] = machine_instruction[0] | 
            (((address >> 16) & 0x00FF) as u16) |
            register;

            machine_instruction.push((address & 0xFFFF) as u16);
        } else {
            println!("Direct Address Memory Access instruction error: Wrong register address!");    
        }
        
    } else {
        println!("Direct Address Memory Access instruction error: not enough operands provided");
    }
   
    return machine_instruction;
}

fn indirect_access_decode(command_parts: Vec<&str>, mut machine_instruction: Vec<u16>) -> Vec<u16> {
    let reg_address: u16;
    let mem_pointer: u16;
    let offset: Vec<u16>;

    match command_parts.len() {
        1 => {
            println!("Indirect mem access error: No operands provided");
        },
        2 => {
            println!("Indirect mem access error: No address pointer provided");
        },
        3 => {
            reg_address = reg_address_decode(command_parts[1]);
            mem_pointer = mem_pointer_matcher(command_parts[2]);

            if reg_address != 0xFFFF && mem_pointer != 0xFFFF {
                machine_instruction[0] = machine_instruction[0] | reg_address | mem_pointer;
            } else {
                println!("Indirect mem access error: wrong reg_address and/or mem_pointer");
            }
        },
        _ => {
            reg_address = reg_address_decode(command_parts[1]);
            mem_pointer = mem_pointer_matcher(command_parts[2]);
            offset = interpret_offset(command_parts[3]);

            if reg_address != 0xFFFF && mem_pointer != 0xFFFF {
                machine_instruction[0] = machine_instruction[0] | reg_address | mem_pointer | offset[0];

                if offset.len() > 1 {
                    machine_instruction.push(offset[1]);
                }
            } else {
                println!("Indirect mem access error: wrong reg_address and/or mem_pointer");
            }
        },
    }

    return machine_instruction;
}

fn jump_mem_access_decode(command_parts: Vec<&str>, mut machine_instruction: Vec<u16>) -> Vec<u16> {
    let mem_pointer: u16;
    let offset: Vec<u16>;

    match command_parts.len() {
        1 => {
            println!("Indirect mem access error: No operands provided");
        },
        2 => {
            mem_pointer = mem_pointer_matcher(command_parts[1]);

            if mem_pointer != 0xFFFF {
                machine_instruction[0] = machine_instruction[0] | mem_pointer;
            } else {
                println!("Indirect mem access error: wrong mem_pointer");
            }
        },
        _ => {
            mem_pointer = mem_pointer_matcher(command_parts[1]);
            offset = interpret_offset(command_parts[2]);

            if mem_pointer != 0xFFFF {
                machine_instruction[0] = machine_instruction[0] | mem_pointer | offset[0];

                if offset.len() > 1 {
                    machine_instruction.push(offset[1]);
                }
            } else {
                println!("Indirect mem access error: wrong mem_pointer");
            }
        },
    }

    return machine_instruction;
}

fn check_address_in_parentheses(address_part: &str) -> bool {
    return address_part.chars().nth(0).unwrap() == '(' && address_part.chars().nth(address_part.len() - 1).unwrap() == ')';
}

fn check_offset_in_brackets(offset_part: &str) -> bool {
    return offset_part.chars().nth(0).unwrap() == '[' && offset_part.chars().nth(offset_part.len() - 1).unwrap() == ']';
}

fn check_pre_post_increment(offset_part: &str) -> Incr {
    let length: usize = offset_part.len();

    if length > 2 {
        let post: bool = offset_part.chars().nth(length - 2).unwrap() == '+';
        let pre: bool = offset_part.chars().nth(1).unwrap() == '+';


        match length {
            3 => {
                if pre || post {
                    return Incr::Err;
                } else {
                    return Incr::None;
                }
            },
            _ => {
                if pre && post {
                    return Incr::Err;
                } else if pre {
                    return Incr::Pre;
                } else if post {
                    return Incr::Post;
                } else {
                    return Incr::None;
                }
            }
        }
    } else {
        return Incr::Err;
    }
}

fn interpret_offset(offset_part: &str) -> Vec<u16> {
    let length: usize = offset_part.len();
    let mut offset_values: Vec<u16> = Vec::new();

    match length {
        0 => offset_values.push(0x0000),
        1 => {
            println!("Error: Offset should be at lest 2 characters, or none; defaulting to none");
            offset_values.push(0x0000);
        },
        2 => {
            if check_offset_in_brackets(offset_part) {
                offset_values.push(0x0004);
                offset_values.push(0x0000);
            } else {
                println!("Error: Offset should be in square brackets; defaulting to no offset");
                offset_values.push(0x0000);
            }
        },
        _ => {
            if check_offset_in_brackets(offset_part) {
                offset_values = offset_value_decode(offset_part, offset_values, check_pre_post_increment(offset_part));
            } else {
                println!("Error: Offset should be in square brackets; defaulting to no offset");
                offset_values.push(0x0000);
            }
            

        },
    };

    return offset_values;
}

fn offset_value_decode(offset_part: &str, mut offset_values: Vec<u16>, pre_post: Incr) -> Vec<u16> {
    let offset_string: &str;
    let offset_bits: u16;

    if offset_values.len() == 0 {
        offset_values.push(0x0000);
    }

    match pre_post {
        Incr::None => {
            offset_string = &offset_part[1..(offset_part.len() - 1)];
            offset_values[0] = offset_values[0] | 0x0008;    
        },
        Incr::Pre => {
            offset_string = &offset_part[2..(offset_part.len() - 1)];
            offset_values[0] = offset_values[0] | 0x0018;
        },
        Incr::Post => {
            offset_string = &offset_part[1..(offset_part.len() - 2)];
            offset_values[0] = offset_values[0] | 0x0010;
        },
        Incr::Err => {
            offset_string = "0";
        },
    }

    offset_bits = reg_offset_address_decode(offset_string);

    if offset_bits != 0xFFFF {
        offset_values[0] = offset_values[0] | offset_bits;
    } else {
        offset_values[0] = offset_values[0] | 0x0004;

        offset_values.push(offset_string.parse::<u16>().unwrap_or_default());

        if offset_values[1] == 0x0000 {
            println!("Offset value decode warning: Value might be out of i16 range or malformed; defaulting to 0");
        }
    }

    return offset_values;
}

fn reg_address_decode(register_name: &str) -> u16 {
    match register_name {
        "r0" => 0x0000,
        "r1" => 0x0100,
        "r2" => 0x0200,
        "r3" => 0x0300,
        "r4" => 0x0400,
        "r5" => 0x0500,
        "r6" => 0x0600,
        "r7" => 0x0700,
        "mp0" => 0x0800,
        "mp1" => 0x0900,
        "mp2" => 0x0A00,
        "mp3" => 0x0B00,
        "mp4" => 0x0C00,
        "mp5" => 0x0D00,
        "mp6" => 0x0E00,
        "mp7" => 0x0F00,
        "PCH" => 0x0800,
        "PCL" => 0x0900,
        "SPH" => 0x0A00,
        "SPL" => 0x0B00,
        "FPH" => 0x0C00,
        "FPL" => 0x0D00,
        "BPH" => 0x0E00,
        "BPL" => 0x0F00,
        _ => {
            println!("Wrong register name!");
            return 0xFFFF;
        }
    }
}

fn reg_offset_address_decode(register_name: &str) -> u16 {
    match register_name {
        "r0" => 0x0 << 5,
        "r1" => 0x1 << 5,
        "r2" => 0x2 << 5,
        "r3" => 0x3 << 5,
        "r4" => 0x4 << 5,
        "r5" => 0x5 << 5,
        "r6" => 0x6 << 5,
        "r7" => 0x7 << 5,
        _ =>  0xFFFF,
    }
}

fn mem_pointer_matcher(mem_pointer_name: &str) -> u16 {
    match &mem_pointer_name[1..(mem_pointer_name.len() - 1)] {
        "PC" => 0x0000,
        "SP" => 0x0001,
        "FP" => 0x0002,
        "BP" => 0x0003,
        _ => 0xFFFF,
    }
}