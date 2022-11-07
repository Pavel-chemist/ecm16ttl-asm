pub fn setim(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    let mask_number: u16;

    machine_instruction.push(0x0700);

    if command_parts.len() > 1 {
        mask_number = command_parts[1].parse::<u16>().unwrap_or_default();
        
        if mask_number < 8 {
            machine_instruction[0] = machine_instruction[0] | (mask_number << 5);
        } else {
            println!("SETIM error: Mask number should be in range 0..7, defaulting to 0.");    
        }
    } else {
        println!("SETIM error: Not enough arguments.");
    }

    return machine_instruction;
}

pub fn clrim(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    let mask_number: u16;

    machine_instruction.push(0x0600);

    if command_parts.len() > 1 {
        mask_number = command_parts[1].parse::<u16>().unwrap_or_default();
        
        if mask_number < 8 {
            machine_instruction[0] = machine_instruction[0] | (mask_number << 5);
        } else {
            println!("CLRIM error: Mask number should be in range 0..7, defaulting to 0.");    
        }
    } else {
        println!("CLRIM error: Not enough arguments.");
    }

    return machine_instruction;
}

pub fn setpr(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    let prefix_number: u16;

    machine_instruction.push(0x0500);

    if command_parts.len() > 1 {
        prefix_number = command_parts[1].parse::<u16>().unwrap_or_default();
        
        if prefix_number < 8 {
            machine_instruction[0] = machine_instruction[0] | (prefix_number << 5);
        } else {
            println!("SETPR error: Prefix number should be in range 0..7, defaulting to 0.");    
        }
    } else {
        println!("SETPR error: Not enough arguments.");
    }

    return machine_instruction;
}

pub fn eint(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    let interrupt_number: u16;

    machine_instruction.push(0x0400);

    if command_parts.len() > 1 {
        interrupt_number = command_parts[1].parse::<u16>().unwrap_or_default();
        
        if interrupt_number < 256 {
            machine_instruction[0] = machine_instruction[0] | interrupt_number;
        } else {
            println!("EINT error: Interrupt number should be in range 0..255, defaulting to 0.");    
        }
    } else {
        println!("EINT error: Not enough arguments.");
    }

    return machine_instruction;
}

pub fn dma() -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();

    machine_instruction.push(0x0300);

    return machine_instruction;
}

pub fn reset() -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();

    machine_instruction.push(0x0200);

    return machine_instruction;
}

pub fn hlt() -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();

    machine_instruction.push(0x0100);

    return machine_instruction;
}

pub fn nop() -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();

    machine_instruction.push(0x0000);

    return machine_instruction;
}

pub fn undefined() -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();

    machine_instruction.push(0x00FF);

    return machine_instruction;
}
