pub fn jump(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    
    machine_instruction.push(0x1000);
    return jump_decode(command_parts, machine_instruction);
}

pub fn jz(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    
    machine_instruction.push(0x1100);
    return jump_decode(command_parts, machine_instruction);
}

pub fn jn(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    
    machine_instruction.push(0x1200);
    return jump_decode(command_parts, machine_instruction);
}

pub fn jo(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    
    machine_instruction.push(0x1400);
    return jump_decode(command_parts, machine_instruction);
}

pub fn jc(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    
    machine_instruction.push(0x1800);
    return jump_decode(command_parts, machine_instruction);
}

pub fn jnz(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    
    machine_instruction.push(0x1E00);
    return jump_decode(command_parts, machine_instruction);
}

pub fn jnn(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    
    machine_instruction.push(0x1D00);
    return jump_decode(command_parts, machine_instruction);
}

pub fn jno(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    
    machine_instruction.push(0x1B00);
    return jump_decode(command_parts, machine_instruction);
}

pub fn jnc(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    
    machine_instruction.push(0x1700);
    return jump_decode(command_parts, machine_instruction);
}

pub fn jsr(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    
    machine_instruction.push(0x1F00);
    return jump_decode(command_parts, machine_instruction);
}

fn jump_decode(command_parts: Vec<&str>, mut machine_instruction: Vec<u16>) -> Vec<u16> {
    let mem_pointer: u16;
    let offset: Vec<u16>;
    match command_parts.len() {
        1 => {
            println!("Jumps error: No operands provided");
        },
        2 => {
            println!("Jumps error: Needs 2 operands");
        },
        _ => {
            if check_address_in_parentheses(command_parts[1]) {
                mem_pointer = mem_pointer_matcher(&command_parts[1][1..(command_parts[1].len() - 1)]);
                offset = interpret_offset(command_parts[2]);
                if mem_pointer != 0xFFFF {
                    machine_instruction[0] = machine_instruction[0] | mem_pointer | offset[0];
                    if offset.len() > 1 {
                        machine_instruction.push(offset[1]);
                    }
                } else {
                    println!("Jumps error: wrong mem_pointer: \"{}\"", mem_pointer);
                }
            } else {
                println!("Jumps error: mem_pointer should be in parentheses");
            }
            
        },
    }
    return machine_instruction;
}

fn mem_pointer_matcher(mem_pointer_name: &str) -> u16 {
    match mem_pointer_name {
        "PC" => 0x0,
        "SP" => 0x1,
        "FP" => 0x2,
        "BP" => 0x3,
        _ => 0xFFFF,
    }
}

fn gpr_matcher(gpr_name: &str) -> u16 {
    match gpr_name {
        "r0" => 0x0 << 5,
        "r1" => 0x1 << 5,
        "r2" => 0x2 << 5,
        "r3" => 0x3 << 5,
        "r4" => 0x4 << 5,
        "r5" => 0x5 << 5,
        "r6" => 0x6 << 5,
        "r7" => 0x7 << 5,
        _ => 0xFFFF,
    }
}

fn interpret_offset(offset_part: &str) -> Vec<u16> {
    let mut offset_values: Vec<u16> = Vec::new();

    if offset_part.len() > 1 {
        if check_offset_in_brackets(offset_part) {
            offset_values = offset_value_decode(offset_part, offset_values);
        } else {
            println!("Jumps Error: Offset should be in square brackets; defaulting to zero offset");
            offset_values.push(0x0004);
            offset_values.push(0x0000);
        }
    } else {

    }
    return offset_values;
}

fn check_address_in_parentheses(address_part: &str) -> bool {
    return address_part.chars().nth(0).unwrap() == '(' && address_part.chars().nth(address_part.len() - 1).unwrap() == ')';
}

fn check_offset_in_brackets(offset_part: &str) -> bool {
    return offset_part.chars().nth(0).unwrap() == '[' && offset_part.chars().nth(offset_part.len() - 1).unwrap() == ']';
}

fn offset_value_decode(offset_part: &str, mut offset_values: Vec<u16>) -> Vec<u16> {
    let offset_string: &str = &offset_part[1..(offset_part.len() - 1)];
    let offset_bits: u16;

    if offset_values.len() == 0 {
        offset_values.push(0x0000);
    }

    offset_bits = gpr_matcher(offset_string);

    if offset_bits != 0xFFFF {
        offset_values[0] = offset_values[0] | offset_bits;
    } else {
        offset_values[0] = offset_values[0] | 0x0004;

        offset_values.push(offset_string.parse::<u16>().unwrap_or_default());
    }

    return offset_values;
}