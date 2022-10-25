pub fn add(command_parts: Vec<&str>) -> Vec<u16> {
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

pub fn sub(command_parts: Vec<&str>) -> Vec<u16> {
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