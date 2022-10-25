// declaring modules used
// declaring them in main makes it possible to other modules cross-use structs and functions

mod helpers;

// declaring functionality of used modules
use helpers::{read_input};

// let Instructions: Vec<&str> = vec!["ADD", "SUB"];

/* enum Instructions {
    ADD,
    SUB,
} */

fn main() {
    let command: String;
    let machine_instruction: Vec<u16>;

    println!("Enter the asm command:");

    command = read_input("");

    println!("The command entered: {}", command);

    machine_instruction = command_interpreter(&command);

    show_code(machine_instruction);
}

fn show_code(machine_instruction: Vec<u16>) {

    for i in 0..machine_instruction.len() {
        println!("bin: {:#016b}", machine_instruction[i]);
        println!("hex: {:#04X}", machine_instruction[i]);
    }
}

fn command_interpreter(command: &str) -> Vec<u16> {
    let command_parts: Vec<&str> = command.split_whitespace().collect();
    let mut machine_instruction: Vec<u16> = Vec::new();

    println!("The command parts:");

    for i in 0..command_parts.len() {
        println!("{}", command_parts[i]);
    }

    match command_parts[0] {
        "ADD" => {
            println!("addition");
            machine_instruction = add(command_parts);
        },
        "SUB" => {
            println!("subtraction");
            machine_instruction = sub(command_parts);
        },
        "AND" => {
            println!("and");
        },
        "OR" => {
            println!("or");
        },
        "XOR" => {
            println!("xor");
        },
        "ANDN" => {
            println!("andn");
        },
        "ORN" => {
            println!("orn");
        },
        "XNOR" => {
            println!("xnor");
        },
        "ADDi" => {
            println!("add immediate constant");
        },
        "SUBi" => {
            println!("subtract immediate constant");
        },
        "ANDi" => {
            println!("and immediate constant");
        },
        "ORi" => {
            println!("or immediate constant");
        },
        "XORi" => {
            println!("xor immediate constant");
        },
        "ADDC" => {
            println!("add with carry");
        },
        "SUBC" => {
            println!("subtract with borrow");
        },
        "SHL" => {
            println!("shift 1 bit left");
        },
        "SHR" => {
            println!("shift 1 bit right");
        },
        "ASHL" => {
            println!("arithmetic shift 1 bit left");
        },
        "ASHR" => {
            println!("arithmetic shift 1 bit right");
        },
        "ROLC" => {
            println!("rotate through carry 1 bit left");
        },
        "RORC" => {
            println!("rotate through carry 1 bit right");
        },
        "ROT" => {
            println!("rotate arbitrary number of bits left");
        },
        "INV" => {
            println!("invert bits");
        },
        "BSE" => {
            println!("byte sign extend");
        },
        "LDi" => {
            println!("load immediate");
        },
        "LDd" => {
            println!("load at direct address");
        },
        "STd" => {
            println!("store at direct address");
        },
        "LD" => {
            println!("load");
        },
        "ST" => {
            println!("store");
        },
        "J" => {
            println!("jump (unconditional)");
        },
        "JC" => {
            println!("jump if carry");
        },
        "JN" => {
            println!("jump if negative");
        },
        "JO" => {
            println!("jump if ovrflow");
        },
        "JZ" => {
            println!("jump if zero");
        },
        "JNC" => {
            println!("jump if not carry");
        },
        "JNN" => {
            println!("jump if not negative");
        },
        "JNO" => {
            println!("jump if not overflow");
        },
        "JNZ" => {
            println!("jump if not zero");
        },
        "JSR" => {
            println!("jump to subroutine");
        },
        _ => {
            println!("undefined");
        }
    }

    return machine_instruction;
}

fn add(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0x0000);

    if command_parts.len() > 1 {
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
    }

    return machine_instruction;
}

fn sub(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    machine_instruction.push(0x9000);
    
    if command_parts.len() > 1 {
        machine_instruction[0] = alu_op_three_operands(command_parts, machine_instruction[0])
    } else {
        println!("SUB instruction error: no operands provided");
    }

    return machine_instruction;
}

fn alu_op_three_operands(command_parts: Vec<&str>, mut machine_instruction: u16) -> u16 {
    if command_parts.len() > 1 {
        if command_parts[1].chars().nth(0).unwrap_or(' ') == 'r' {
            if command_parts[1].chars().nth(1).unwrap_or('0').to_digit(10).unwrap() < 8 {
                machine_instruction = machine_instruction | ((command_parts[1].chars().nth(1).unwrap().to_digit(10).unwrap() << 8) as u16);
            }
        } else {
            println!("3-operand ALU instruction error: Incorrect name for operand 1");
        }
    } else {
        println!("3-operand ALU instruction error: missing operand 1");
    }

    if command_parts.len() > 2 {
        if command_parts[2].chars().nth(0).unwrap_or(' ') == 'r' {
            if command_parts[2].chars().nth(1).unwrap_or('0').to_digit(10).unwrap() < 8 {
                machine_instruction = machine_instruction | ((command_parts[2].chars().nth(1).unwrap().to_digit(10).unwrap() << 5) as u16);
            }
        } else {
            println!("3-operand ALU instruction error: Incorrect name for operand 2");
        }
    } else {
        println!("3-operand ALU instruction error: missing operand 2");
    } 

    if command_parts.len() > 3 {
        if command_parts[3].chars().nth(0).unwrap_or(' ') == 'r' {
            if command_parts[3].chars().nth(1).unwrap_or('0').to_digit(10).unwrap() < 8 {
                machine_instruction = machine_instruction | ((command_parts[3].chars().nth(1).unwrap().to_digit(10).unwrap()) as u16);
            }
        } else {
            println!("3-operand ALU instruction error: Incorrect name for operand 3");
        }
    } else {
        println!("3-operand ALU instruction error: missing operand 3");
    } 

    return machine_instruction;
}

