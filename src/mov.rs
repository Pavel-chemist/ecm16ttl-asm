pub fn mov(command_parts: Vec<&str>) -> Vec<u16> {
    let mut machine_instruction: Vec<u16> = Vec::new();
    let src: u16;
    let dest: u16;

    machine_instruction.push(0x3000);

    if command_parts.len() > 2 {
        dest = dest_reg_matcher(command_parts[1]);
        src = src_reg_matcher(command_parts[2]);

        if src != 0xFFFF && dest != 0xFFFF {
            machine_instruction[0] = machine_instruction[0] | src | dest;
        } else {
            println!("MOV error: wrong names for source and desstination registers");
        }
    } else {
        println!("MOV error: Not enough arguments.");
    }


    return machine_instruction;
}

fn src_reg_matcher(reg_name: &str) -> u16{
    match reg_name {
        "r0" => 0x0000,
        "r1" => 0x0020,
        "r2" => 0x0040,
        "r3" => 0x0060,
        "r4" => 0x0080,
        "r5" => 0x00A0,
        "r6" => 0x00C0,
        "r7" => 0x00E0,
        "mp0" => 0x0010,
        "mp1" => 0x0030,
        "mp2" => 0x0050,
        "mp3" => 0x0070,
        "mp4" => 0x0090,
        "mp5" => 0x00B0,
        "mp6" => 0x00D0,
        "mp7" => 0x00F0,
        "PCH" => 0x0010,
        "PCL" => 0x0030,
        "SPH" => 0x0050,
        "SPL" => 0x0070,
        "FPH" => 0x0090,
        "FPL" => 0x00B0,
        "BPH" => 0x00D0,
        "BPL" => 0x00F0,
        _ => 0xFFFF,
    }
}

fn dest_reg_matcher(reg_name: &str) -> u16{
    match reg_name {
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
        _ => 0xFFFF,
    }
}
