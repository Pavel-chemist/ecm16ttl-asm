// declaring modules used
// declaring them in main makes it possible to other modules cross-use structs and functions

mod helpers;
mod alu;
mod mem_access;

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
        println!(
            "word {}: {:04b} {:04b} {:04b} {:04b} | {:04X}",
            i+1,
            machine_instruction[i] >> 12,
            (machine_instruction[i] >> 8) & 0xF,
            (machine_instruction[i] >> 4) & 0xF,
            machine_instruction[i] & 0xF,
            machine_instruction[i],
        );
    }
}

fn command_interpreter(command: &str) -> Vec<u16> {
    let command_parts: Vec<&str> = command.split_whitespace().collect();
    let mut machine_instruction: Vec<u16> = Vec::new();

    match command_parts[0] {
        "ADD" => machine_instruction = alu::add(command_parts),
        "SUB" => machine_instruction = alu::sub(command_parts),
        "AND" => machine_instruction = alu::and(command_parts),
        "OR" => machine_instruction = alu::or(command_parts),
        "XOR" => machine_instruction = alu::xor(command_parts),
        "ANDN" => machine_instruction = alu::andn(command_parts),
        "ORN" => machine_instruction = alu::orn(command_parts),
        "XNOR" => machine_instruction = alu::xnor(command_parts),
        "ADDi" => machine_instruction = alu::addi(command_parts),
        "SUBi" => machine_instruction = alu::subi(command_parts),
        "ANDi" => machine_instruction = alu::andi(command_parts),
        "ORi" => machine_instruction = alu::ori(command_parts),
        "XORi" => machine_instruction = alu::xori(command_parts),
        "ANDNi" => machine_instruction = alu::andni(command_parts),
        "ORNi" => machine_instruction = alu::orni(command_parts),
        "XNORi" => machine_instruction = alu::xnori(command_parts),
        "ADDC" => machine_instruction = alu::addc(command_parts),
        "SUBC" => machine_instruction = alu::subc(command_parts),
        "CMP" => machine_instruction = alu::cmp(command_parts),
        "SHL" => machine_instruction = alu::shl(command_parts),
        "SHR" => machine_instruction = alu::shr(command_parts),
        "ASHL" => machine_instruction = alu::ashl(command_parts),
        "ASHR" => machine_instruction = alu::ashr(command_parts),
        "ROLC" => machine_instruction = alu::rolc(command_parts),
        "RORC" => machine_instruction = alu::rorc(command_parts),
        "ROT" => machine_instruction = alu::rot(command_parts),
        "INV" => machine_instruction = alu::inv(command_parts),
        "BSE" => machine_instruction = alu::bse(command_parts),
        "LDi" => machine_instruction = mem_access::ldi(command_parts),
        "LDd" => machine_instruction = mem_access::ldd(command_parts),
        "STd" => machine_instruction = mem_access::std(command_parts),
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



