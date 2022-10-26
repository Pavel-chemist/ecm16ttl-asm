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

fn direct_address(command_parts: Vec<&str>, mut machine_instruction: Vec<u16>) -> Vec<u16> {
    let address: u32;

    if command_parts.len() > 2 {
        if command_parts[2].chars().nth(0).unwrap() == '(' && command_parts[2].chars().nth(command_parts[2].len() - 1).unwrap() == ')' {
            address = command_parts[2][1..(command_parts[2].len()-1)].parse::<u32>().unwrap_or_default() >> 1;
        } else {
            println!("Direct Address Memory Access instruction error:\nDirect address should be in parentheses!");
            address = 0;
        }

        machine_instruction.push((address & 0xFFFF) as u16);
        machine_instruction[0] = machine_instruction[0] | (((address >> 16) & 0x00FF) as u16);

        if command_parts[1].len() == 2 && command_parts[1].chars().nth(0).unwrap_or(' ') == 'r' {
            println!("direct load to GPR");

            if command_parts[1].chars().nth(1).unwrap_or('0').to_digit(10).unwrap() < 8 {
                machine_instruction[0] = machine_instruction[0] | ((command_parts[1].chars().nth(1).unwrap().to_digit(10).unwrap() << 8) as u16);
            }
        } else if command_parts[1].len() == 3 && command_parts[1].chars().nth(0).unwrap_or(' ') == 'm' && command_parts[1].chars().nth(1).unwrap_or(' ') == 'p' {
            println!("direct load to MPR");

            if command_parts[1].chars().nth(2).unwrap_or('0').to_digit(10).unwrap() < 8 {
                machine_instruction[0] = machine_instruction[0] | ((command_parts[1].chars().nth(2).unwrap().to_digit(10).unwrap() << 8) as u16) | 0x0800;
            }
        } else if command_parts[1] == "PCH" || command_parts[1] == "SPH" || command_parts[1] == "FPH" || command_parts[1] == "BPH" || command_parts[1] == "PCL" || command_parts[1] == "SPL" || command_parts[1] == "FPL" || command_parts[1] == "BPL" {
            println!("immediate load to MP register");

            match command_parts[1] {
                "PCH" => machine_instruction[0] = machine_instruction[0] | 0x0800 | 0x0000,
                "PCL" => machine_instruction[0] = machine_instruction[0] | 0x0800 | 0x0100,
                "SPH" => machine_instruction[0] = machine_instruction[0] | 0x0800 | 0x0200,
                "SPL" => machine_instruction[0] = machine_instruction[0] | 0x0800 | 0x0300,
                "FPH" => machine_instruction[0] = machine_instruction[0] | 0x0800 | 0x0400,
                "FPL" => machine_instruction[0] = machine_instruction[0] | 0x0800 | 0x0500,
                "BPH" => machine_instruction[0] = machine_instruction[0] | 0x0800 | 0x0600,
                "BPL" => machine_instruction[0] = machine_instruction[0] | 0x0800 | 0x0700,
                &_ => {},
            }
        } else {
            println!("Direct Address Memory Access instruction error: wrong first operand");        
        }

    } else {
        println!("Direct Address Memory Access instruction error: not enough operands provided");
    }
   
    return machine_instruction;
}