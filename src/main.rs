// declaring modules used
// declaring them in main makes it possible to other modules cross-use structs and functions

mod helpers;
mod alu;

// declaring functionality of used modules
use helpers::read_input;

fn main() {
    let command: String;
    let machine_instruction: Vec<u16>;

    println!("Enter the asm command:");

    command = read_input("");

    machine_instruction = command_interpreter(&command);

    show_code(machine_instruction);
}

fn show_code(machine_instruction: Vec<u16>) {

    for i in 0..machine_instruction.len() {
        println!("word {}: {:#016b} | {:#04X}", i+1, machine_instruction[i], machine_instruction[i]);
    }
}

fn command_interpreter(command: &str) -> Vec<u16> {
    let command_parts: Vec<&str> = command.split_whitespace().collect();
    let mut machine_instruction: Vec<u16> = Vec::new();

    /* println!("The command parts:");

    for i in 0..command_parts.len() {
        println!("{}", command_parts[i]);
    } */

    match command_parts[0] {
        "ADD" => machine_instruction = alu::add(command_parts),
        "SUB" => machine_instruction = alu::sub(command_parts),
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



