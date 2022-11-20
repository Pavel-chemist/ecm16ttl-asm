use crate::helpers;

pub fn add(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0x8000);

    if command_parts.len() > 1 {
        machine_instruction[0] = alu_op_three_operands(command_parts, machine_instruction[0]);
    } else {
        println!("ADD instruction error: no operands provided");
    }

    /* if command_parts.len() > 1 {
        if command_parts[1].chars().nth(0).unwrap_or(' ') == 'r' {
            machine_instruction[0] = machine_instruction[0] | 0x8000; 
    
            machine_instruction[0] = alu_op_three_operands(command_parts, machine_instruction[0])
        } else if command_parts[1] == "PC" || command_parts[1] == "SP" || command_parts[1] == "FP" || command_parts[1] == "BP" {
            println!("MemPointer arithmetic op -- TBD");
        } else {
            println!("ADD instruction error: wrong first operand");
        }
    } else {
        println!("ADD instruction error: no operands provided");
    } */

    return machine_instruction;
}

pub fn sub(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0x9000);
    
    if command_parts.len() > 1 {
        machine_instruction[0] = alu_op_three_operands(command_parts, machine_instruction[0]);
    } else {
        println!("SUB instruction error: no operands provided");
    }

    return machine_instruction;
}

pub fn addc(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0x8008);
    
    if command_parts.len() > 1 {
        machine_instruction[0] = alu_op_three_operands(command_parts, machine_instruction[0]);
    } else {
        println!("ADDC instruction error: no operands provided");
    }

    return machine_instruction;
}

pub fn subc(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0x9008);
    
    if command_parts.len() > 1 {
        machine_instruction[0] = alu_op_three_operands(command_parts, machine_instruction[0]);
    } else {
        println!("SUBC instruction error: no operands provided");
    }

    return machine_instruction;
}

pub fn xor(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0xA000);
    
    if command_parts.len() > 1 {
        machine_instruction[0] = alu_op_three_operands(command_parts, machine_instruction[0]);
    } else {
        println!("XOR instruction error: no operands provided");
    }

    return machine_instruction;
}

pub fn xnor(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0xB000);
    
    if command_parts.len() > 1 {
        machine_instruction[0] = alu_op_three_operands(command_parts, machine_instruction[0]);
    } else {
        println!("XNOR instruction error: no operands provided");
    }

    return machine_instruction;
}

pub fn or(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0xC000);
    
    if command_parts.len() > 1 {
        machine_instruction[0] = alu_op_three_operands(command_parts, machine_instruction[0]);
    } else {
        println!("OR instruction error: no operands provided");
    }

    return machine_instruction;
}

pub fn orn(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0xD000);
    
    if command_parts.len() > 1 {
        machine_instruction[0] = alu_op_three_operands(command_parts, machine_instruction[0]);
    } else {
        println!("ORN instruction error: no operands provided");
    }

    return machine_instruction;
}

pub fn and(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0xE000);
    
    if command_parts.len() > 1 {
        machine_instruction[0] = alu_op_three_operands(command_parts, machine_instruction[0]);
    } else {
        println!("AND instruction error: no operands provided");
    }

    return machine_instruction;
}

pub fn andn(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0xF000);
    
    if command_parts.len() > 1 {
        machine_instruction[0] = alu_op_three_operands(command_parts, machine_instruction[0]);
    } else {
        println!("ANDN instruction error: no operands provided");
    }

    return machine_instruction;
}

pub fn addi(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0x8800);
    
    if command_parts.len() > 1 {
        machine_instruction[0] = alu_op_with_const(command_parts, machine_instruction[0]);
    } else {
        println!("ADDi instruction error: no operands provided");
    }

    return machine_instruction;
}

pub fn subi(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0x9800);
    
    if command_parts.len() > 1 {
        machine_instruction[0] = alu_op_with_const(command_parts, machine_instruction[0]);
    } else {
        println!("SUBi instruction error: no operands provided");
    }

    return machine_instruction;
}

pub fn xori(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0xA800);
    
    if command_parts.len() > 1 {
        machine_instruction[0] = alu_op_with_const(command_parts, machine_instruction[0]);
    } else {
        println!("XORi instruction error: no operands provided");
    }

    return machine_instruction;
}

pub fn xnori(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0xB800);
    
    if command_parts.len() > 1 {
        machine_instruction[0] = alu_op_with_const(command_parts, machine_instruction[0]);
    } else {
        println!("XNORi instruction error: no operands provided");
    }

    return machine_instruction;
}

pub fn ori(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0xC800);
    
    if command_parts.len() > 1 {
        machine_instruction[0] = alu_op_with_const(command_parts, machine_instruction[0]);
    } else {
        println!("ORi instruction error: no operands provided");
    }

    return machine_instruction;
}

pub fn orni(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0xD800);
    
    if command_parts.len() > 1 {
        machine_instruction[0] = alu_op_with_const(command_parts, machine_instruction[0]);
    } else {
        println!("ORi instruction error: no operands provided");
    }

    return machine_instruction;
}

pub fn andi(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0xE800);
    
    if command_parts.len() > 1 {
        machine_instruction[0] = alu_op_with_const(command_parts, machine_instruction[0]);
    } else {
        println!("ANDi instruction error: no operands provided");
    }

    return machine_instruction;
}

pub fn andni(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0xF800);
    
    if command_parts.len() > 1 {
        machine_instruction[0] = alu_op_with_const(command_parts, machine_instruction[0]);
    } else {
        println!("ANDNi instruction error: no operands provided");
    }

    return machine_instruction;
}

pub fn cmp(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0xF008);
    
    let mut updated_parts: Vec<&str> = Vec::new();
    updated_parts.push(command_parts[0]);
    updated_parts.push("r0"); // dummy value, as nothing is written to the destination when comparing

    for i in 1..command_parts.len() {
        updated_parts.push(command_parts[i]);
    }

    if updated_parts.len() > 1 {
        machine_instruction[0] = alu_op_three_operands(updated_parts, machine_instruction[0]);
    } else {
        println!("CMP instruction error: no operands provided");
    }

    return machine_instruction;
}

pub fn shl(mut command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0xA010);

    if command_parts.len() == 3 {
        command_parts.push("r0"); //dummy value
        machine_instruction[0] = alu_op_three_operands(command_parts, machine_instruction[0]);
    } else {
        println!("SHL instruction error: wrong number of operands");
    }

    return machine_instruction;
}

pub fn shr(mut command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0xB010);

    if command_parts.len() == 3 {
        command_parts.push("r0"); //dummy value
        machine_instruction[0] = alu_op_three_operands(command_parts, machine_instruction[0]);
    } else {
        println!("SHR instruction error: wrong number of operands");
    }

    return machine_instruction;
}

pub fn ashl(mut command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0xA014);

    if command_parts.len() == 3 {
        command_parts.push("r0"); //dummy value
        machine_instruction[0] = alu_op_three_operands(command_parts, machine_instruction[0]);
    } else {
        println!("ASHL instruction error: wrong number of operands");
    }

    return machine_instruction;
}

pub fn ashr(mut command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0xB014);

    if command_parts.len() == 3 {
        command_parts.push("r0"); //dummy value
        machine_instruction[0] = alu_op_three_operands(command_parts, machine_instruction[0]);
    } else {
        println!("ASHR instruction error: wrong number of operands");
    }

    return machine_instruction;
}

pub fn rolc(mut command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0xA018);

    if command_parts.len() == 3 {
        command_parts.push("r0"); //dummy value
        machine_instruction[0] = alu_op_three_operands(command_parts, machine_instruction[0]);
    } else {
        println!("ROLC instruction error: wrong number of operands");
    }

    return machine_instruction;
}

pub fn rorc(mut command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0xB018);

    if command_parts.len() == 3 {
        command_parts.push("r0"); //dummy value
        machine_instruction[0] = alu_op_three_operands(command_parts, machine_instruction[0]);
    } else {
        println!("RORC instruction error: wrong number of operands");
    }

    return machine_instruction;
}

pub fn rot(mut command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0xC010);


    if command_parts.len() == 4 {
        if command_parts[3].chars().nth(0).unwrap_or(' ') == 'r' {
            machine_instruction[0] = alu_op_three_operands(command_parts, machine_instruction[0]);
        } else {
            let rotation_number: u16 = (helpers::number_parser(command_parts[3]) as u16) & 0xF;

            command_parts[3] = "r0";
            machine_instruction[0] = alu_op_three_operands(command_parts, machine_instruction[0]);
            machine_instruction[0] = machine_instruction[0] | rotation_number | 0x1000;
        } 
        
    } else {
        println!("ROT instruction error: wrong number of operands");
    }

    return machine_instruction;
}

pub fn inv (mut command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0x8010);

    if command_parts.len() == 3 {
        command_parts.push("r0");
        machine_instruction[0] = alu_op_three_operands(command_parts, machine_instruction[0]);
    } else if command_parts.len() > 3 {
        command_parts[3] = "r0";
        machine_instruction[0] = alu_op_three_operands(command_parts, machine_instruction[0]);
    } else {
        println!("INV instruction error: some operands missing");
    }

    return machine_instruction;
}

pub fn bse (mut command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0xE010);

    if command_parts.len() == 3 {
        command_parts.push("r0");
        machine_instruction[0] = alu_op_three_operands(command_parts, machine_instruction[0]);
    } else if command_parts.len() > 3 {
        command_parts[3] = "r0";
        machine_instruction[0] = alu_op_three_operands(command_parts, machine_instruction[0]);
    } else {
        println!("INV instruction error: some operands missing");
    }

    return machine_instruction;
}

fn alu_op_three_operands(command_parts: Vec<&str>, mut machine_instruction: u16) -> u16 {
    if command_parts.len() > 3 {
        if command_parts[1].chars().nth(0).unwrap_or(' ') == 'r' {
            if command_parts[1].chars().nth(1).unwrap_or('0').to_digit(10).unwrap() < 8 {
                machine_instruction = machine_instruction | ((command_parts[1].chars().nth(1).unwrap().to_digit(10).unwrap() << 8) as u16);
            }
        } else {
            println!("3-operand ALU instruction error: Incorrect name for operand 1");
        }

        if command_parts[2].chars().nth(0).unwrap_or(' ') == 'r' {
            if command_parts[2].chars().nth(1).unwrap_or('0').to_digit(10).unwrap() < 8 {
                machine_instruction = machine_instruction | ((command_parts[2].chars().nth(1).unwrap().to_digit(10).unwrap() << 5) as u16);
            }
        } else {
            println!("3-operand ALU instruction error: Incorrect name for operand 2");
        }

        if command_parts[3].chars().nth(0).unwrap_or(' ') == 'r' {
            if command_parts[3].chars().nth(1).unwrap_or('0').to_digit(10).unwrap() < 8 {
                machine_instruction = machine_instruction | ((command_parts[3].chars().nth(1).unwrap().to_digit(10).unwrap()) as u16);
            }
        } else {
            println!("3-operand ALU instruction error: Incorrect name for operand 3");
        }
    } else {
        println!("3-operand ALU instruction error: missing operand(s)");
    } 

  return machine_instruction;
}

fn alu_op_with_const (command_parts: Vec<&str>, mut machine_instruction: u16) -> u16 {
    if command_parts.len() > 2 {
        if command_parts[1].chars().nth(0).unwrap_or(' ') == 'r' {
            if command_parts[1].chars().nth(1).unwrap_or('0').to_digit(10).unwrap() < 8 {
                machine_instruction = machine_instruction | ((command_parts[1].chars().nth(1).unwrap().to_digit(10).unwrap() << 8) as u16);
            }
        } else {
            println!("1-operand immediate ALU instruction error: Incorrect name for operand 1");
        }

        // machine_instruction = machine_instruction | command_parts[2].parse::<u8>().unwrap_or_default() as u16;
        machine_instruction = machine_instruction | (helpers::number_parser(command_parts[2]) as u16 & 0x00FF);
    } else {
        println!("1-operand immediate ALU instruction error: missing operand(s)");
    }

    return machine_instruction;
}