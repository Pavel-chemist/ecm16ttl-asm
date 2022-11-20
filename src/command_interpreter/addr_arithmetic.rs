use crate::helpers;

pub fn addp(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    let mem_pointer: u16;
    let gpr: u16;
    let const_value: u16;
    
    machine_instruction.push(0x0800);

    if command_parts.len() > 2 {
        mem_pointer = mem_pointer_matcher(command_parts[1]);

        if mem_pointer != 0xFFFF {
            machine_instruction[0] = machine_instruction[0] | mem_pointer;

            gpr = gpr_matcher(command_parts[2]);

            if gpr == 0xFFFF {
                const_value = helpers::number_parser(command_parts[2]) as u16;

                if const_value < 256 {
                    machine_instruction[0] = machine_instruction[0] | const_value | 0x0100;
                } else {
                    machine_instruction[0] = machine_instruction[0] | 0x0004;

                    machine_instruction.push(const_value);
                }
            } else {
                machine_instruction[0] = machine_instruction[0] | gpr;
            }
        } else {
            println!("Address arithmetic error: Wrong mem pointer name.");
        }
    } else {
        println!("Address arithmetic error: Not enough arguments.");
    }

   
    return machine_instruction;
}

fn mem_pointer_matcher(mem_pointer_name: &str) -> u16 {
    match mem_pointer_name {
        "PC" => 0x0000,
        "SP" => 0x0200,
        "FP" => 0x0400,
        "BP" => 0x0600,
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