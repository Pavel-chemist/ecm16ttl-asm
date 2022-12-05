use std::collections::HashMap;
use crate::enums::{InstrType, ArgType};
use crate::helpers::arg_matcher;
use crate::structs::{Code, InstrDescr, Arg};
use crate::lut::get_instr_table;

pub fn encode_instruction(code_line: &mut Code, labels_map: &HashMap<String, i32>, err: &mut bool) {
    let instr_map: HashMap<&str, InstrDescr> = get_instr_table();
    let instruction: &str = &code_line.code_parts[0];
    let mut args: Vec<Arg> = Vec::with_capacity(4);

    for i in 1..code_line.code_parts.len() {
        let mut argument_value: Arg = arg_matcher(&code_line.code_parts[i]);

        match argument_value.get_type() {
            ArgType::Label => {
                match labels_map.get(&code_line.code_parts[i]) {
                    Some(val) => {
                        argument_value.update_val(val.clone() as i64);
                    },
                    None => {
                        println!("Error parsing arguments!\n on line {}\n For {} instruction: \n argument #{} is {}, which is not a register name and not a number, nor in the list of known labels!", code_line.line_number, code_line.code_parts[0], i, code_line.code_parts[i]);

                        *err = true;
                        break;
                    }
                }
            },
            ArgType::Gpr => {
                argument_value.update_val(argument_value.get_val() & 0x7);
            },
            ArgType::Mpr => {
                argument_value.update_val(argument_value.get_val() & 0x7);
            },
            ArgType::MP => {
                argument_value.update_val(argument_value.get_val() & 0x3);
            },
            ArgType::Special => {
                argument_value.update_val(argument_value.get_val() & 0x3);
            },
            ArgType::Value => {
                // nothing
                // the value is trimmed to particular length on per-instruction basis
            }
        };

        args.push(argument_value);
    }

    /* for i in 0..code_line.code_parts.len() {
        if i == 0 {
            println!("code part: {}", code_line.code_parts[i]);
        } else {
            println!("code part: {}, translated to value: {}", code_line.code_parts[i], args[i-1].get_val());
        }
    } */

    match instr_map.get(instruction) {
        Some(instr) => {
            //instruction is found in table
            code_line.machine_code.push(instr.base_word);

            match instr.itype {
                InstrType::Alu => {
                    //println!("istr type is Alu");

                    match args[0].get_type() {
                        ArgType::Gpr => {
                            code_line.machine_code[0] = 
                                code_line.machine_code[0] |
                                ((args[0].get_val() as u16) << 8);
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 1st argument for {} instruction should be one of gpr.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }

                    match args[1].get_type() {
                        ArgType::Gpr => {
                            code_line.machine_code[0] = 
                                code_line.machine_code[0] |
                                ((args[1].get_val() as u16) << 5);
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 2nd argument for {} instruction should be one of gpr.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }

                    match args[2].get_type() {
                        ArgType::Gpr => {
                            code_line.machine_code[0] = 
                                code_line.machine_code[0] |
                                (args[2].get_val() as u16);
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 3rd argument for {} instruction should be one of gpr.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }
                },
                InstrType::AluConst => {
                    //println!("istr type is AluConst");

                    match args[0].get_type() {
                        ArgType::Gpr => {
                            code_line.machine_code[0] = 
                                code_line.machine_code[0] |
                                ((args[0].get_val() as u16) << 8);
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 1st argument for {} instruction should be one of gpr.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }

                    match args[1].get_type() {
                        ArgType::Value => {
                            let val: i64 = args[1].get_val();

                            if val >= 0 && val < 256 {
                                code_line.machine_code[0] = code_line.machine_code[0] | (val as u16);
                            } else {
                                println!("Arguments Error\n on line {}\n 2nd argument for {} instruction should be in range 0..255 (0x00..0xFF), got {}.", code_line.line_number, code_line.code_parts[0], val);

                                *err = true;
                            }
                        },
                        ArgType::Label => {
                            let val: i64 = args[1].get_val();

                            if val >= 0 && val < 256 {
                                code_line.machine_code[0] = code_line.machine_code[0] | (val as u16);
                            } else {
                                println!("Arguments Error\n on line {}\n 2nd argument for {} instruction should be in range 0..255 (0x00..0xFF), got {}.", code_line.line_number, code_line.code_parts[0], val);

                                *err = true;
                            }
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 2nd argument for {} instruction should be one of number or labelled value.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }
                },
                InstrType::AluTest => {
                    //println!("istr type is AluTest");

                    match args[0].get_type() {
                        ArgType::Gpr => {
                            code_line.machine_code[0] = 
                                code_line.machine_code[0] |
                                ((args[0].get_val() as u16) << 5);
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 1st argument for {} instruction should be one of gpr.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }

                    match args[1].get_type() {
                        ArgType::Gpr => {
                            code_line.machine_code[0] = 
                                code_line.machine_code[0] |
                                (args[1].get_val() as u16);
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 2nd argument for {} instruction should be one of gpr.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }
                },
                InstrType::AluOneSrc => {
                    //println!("istr type is AluOneSrc");

                    match args[0].get_type() {
                        ArgType::Gpr => {
                            code_line.machine_code[0] = 
                                code_line.machine_code[0] |
                                ((args[0].get_val() as u16) << 8);
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 1st argument for {} instruction should be one of gpr.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }

                    match args[1].get_type() {
                        ArgType::Gpr => {
                            code_line.machine_code[0] = 
                                code_line.machine_code[0] |
                                ((args[1].get_val() as u16) << 5);
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 2nd argument for {} instruction should be one of gpr.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }
                },
                InstrType::AluRot => {
                    //println!("istr type is AluRot");

                    match args[0].get_type() {
                        ArgType::Gpr => {
                            code_line.machine_code[0] = 
                                code_line.machine_code[0] |
                                ((args[0].get_val() as u16) << 8);
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 1st argument for {} instruction should be one of gpr.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }

                    match args[1].get_type() {
                        ArgType::Gpr => {
                            code_line.machine_code[0] = 
                                code_line.machine_code[0] |
                                ((args[1].get_val() as u16) << 5);
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 2nd argument for {} instruction should be one of gpr.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }

                    match args[2].get_type() {
                        ArgType::Value => {
                            let val: i64 = args[2].get_val();

                            if val >= 0 && val < 15 {
                                code_line.machine_code[0] = code_line.machine_code[0] | (val as u16);
                            } else {
                                println!("Arguments Error\n on line {}\n 2nd argument for {} instruction should be in range 0..15 (0x0..0xF), got {}.", code_line.line_number, code_line.code_parts[0], val);

                                *err = true;
                            }
                        },
                        ArgType::Label => {
                            let val: i64 = args[2].get_val();

                            if val >= 0 && val < 15 {
                                code_line.machine_code[0] = code_line.machine_code[0] | (val as u16);
                            } else {
                                println!("Arguments Error\n on line {}\n 2nd argument for {} instruction should be in range 0..15 (0x0..0xF), got {}.", code_line.line_number, code_line.code_parts[0], val);

                                *err = true;
                            }
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 3rd argument for {} instruction should be one of number or labelled value.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }
                },
                InstrType::MemIgpr => {
                    //println!("istr type is MemIgpr");

                    match args[0].get_type() {
                        ArgType::Gpr => {
                            code_line.machine_code[0] = 
                                code_line.machine_code[0] |
                                ((args[0].get_val() as u16) << 8);
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 1st argument for {} instruction should be one of gpr.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }
                    
                    match args[1].get_type() {
                        ArgType::Value => {
                            let val: i64 = args[1].get_val();

                            if val >= -32768 && val <= 32767 {
                                code_line.machine_code.push(val as u16);
                            } else {
                                println!("Arguments Error\n on line {}\n 2nd argument for {} instruction should be in range -32768..32767 (0x0000..0xFFFF), got {}.", code_line.line_number, code_line.code_parts[0], val);

                                *err = true;
                            }

                            
                        },
                        ArgType::Label => {
                            let val: i64 = args[1].get_val();

                            if val >= -32768 && val <= 32767 {
                                code_line.machine_code.push(val as u16);
                            } else {
                                println!("Arguments Error\n on line {}\n 2nd argument for {} instruction should be in range -32768..32767 (0x0000..0xFFFF), got {}.", code_line.line_number, code_line.code_parts[0], val);

                                *err = true;
                            }
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 2nd argument for {} instruction should be one of number or labelled value.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }
                },
                InstrType::MemImp => {
                    //println!("istr type is MemImp");

                    match args[0].get_type() {
                        ArgType::MP => {
                            code_line.machine_code[0] = 
                                code_line.machine_code[0] |
                                ((args[0].get_val() as u16) << 9);
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 1st argument for {} instruction should be one of MP.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }
                    
                    match args[1].get_type() {
                        ArgType::Value => {
                            /* let val: i64 = args[1].get_val();

                            if val < -32768 || val > 32767 {
                                println!("Arguments Error\n on line {}\n 2nd argument for {} instruction should be in range -32768..32767 (0x0000..0xFFFF).", code_line.line_number, code_line.code_parts[0])
                            } */
                            let val: i64 = args[1].get_val();

                            code_line.machine_code[0] = code_line.machine_code[0] | (((val >> 16) as u16) & 0x1FF);
                            code_line.machine_code.push(val as u16);
                        },
                        ArgType::Label => {
                            let val: i64 = args[1].get_val();

                            code_line.machine_code[0] = code_line.machine_code[0] | (((val >> 16) as u16) & 0x1FF);
                            code_line.machine_code.push(val as u16);
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 2nd argument for {} instruction should be one of number or labelled value.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }
                },
                InstrType::MemDirect => {
                    //println!("istr type is MemDirect");

                    match args[0].get_type() {
                        ArgType::Gpr => {
                            // nothing
                        },
                        ArgType::Mpr => {
                            code_line.machine_code[0] = code_line.machine_code[0] | 0x0800;
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 1st argument for {} instruction should be one of gpr or mpr.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }

                    match args[1].get_type() {
                        ArgType::Value => {
                            code_line.machine_code[0] = 
                                code_line.machine_code[0] |
                                (((args[1].get_val().clone() >> 17) as u16) & 0xFF);

                            code_line.machine_code.push((args[1].get_val() >> 1) as u16);
                        },
                        ArgType::Label => {
                            code_line.machine_code[0] = 
                                code_line.machine_code[0] |
                                (((args[1].get_val().clone() >> 17) as u16) & 0xFF);

                            code_line.machine_code.push((args[1].get_val() >> 1) as u16);
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 2nd argument for {} instruction should be one of number or labelled value.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }
                },
                InstrType::Mem => {
                    //println!("istr type is Mem");

                    match args[0].get_type() {
                        ArgType::Gpr => {
                            // nothing
                        },
                        ArgType::Mpr => {
                            code_line.machine_code[0] = code_line.machine_code[0] | 0x0800;
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 1st argument for {} instruction should be one of gpr or mpr.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }

                    match args[1].get_type() {
                        ArgType::MP => {
                            code_line.machine_code[0] = code_line.machine_code[0] | (args[1].get_val() as u16);
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 2nd argument for {} instruction should be one of MP.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }
                },
                InstrType::MemRo => {
                    //println!("istr type is MemRo");

                    match args[0].get_type() {
                        ArgType::Gpr => {
                            // nothing
                        },
                        ArgType::Mpr => {
                            code_line.machine_code[0] = code_line.machine_code[0] | 0x0800;
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 1st argument for {} instruction should be one of gpr or mpr.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }

                    match args[1].get_type() {
                        ArgType::MP => {
                            code_line.machine_code[0] = code_line.machine_code[0] | (args[1].get_val() as u16);
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 2nd argument for {} instruction should be one of MP.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }

                    match args[2].get_type() {
                        ArgType::Gpr => {
                            code_line.machine_code[0] = code_line.machine_code[0] | 
                            ((args[2].get_val() as u16) << 5);
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 3rd argument for {} instruction should be one of gpr.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }
                },
                InstrType::MemIo => {
                    //println!("istr type is MemIo");

                    match args[0].get_type() {
                        ArgType::Gpr => {
                            // nothing
                        },
                        ArgType::Mpr => {
                            code_line.machine_code[0] = code_line.machine_code[0] | 0x0800;
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 1st argument for {} instruction should be one of gpr or mpr.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }

                    match args[1].get_type() {
                        ArgType::MP => {
                            code_line.machine_code[0] = code_line.machine_code[0] | (args[1].get_val() as u16);
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 2nd argument for {} instruction should be one of MP.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }

                    match args[2].get_type() {
                        ArgType::Value => {
                            // when numeric value is provided, it is assumed
                            // MemPointer other than PC is used
                            // and value is treated as offset from that pointer

                            let val: i64 = args[2].get_val();

                            if val >= -32768 && val <= 32767 {
                                code_line.machine_code.push(val as u16);
                            } else {
                                //error
                                println!("Arguments ERROR\n on line {}\n 3rd argument for {} instruction should be number in range [-32768..32767] (0x0000..0xFFFF), got {}.", code_line.line_number, code_line.code_parts[0], val);

                                *err = true;
                            }
                        },
                        ArgType::Label => {
                            // when there is a label used, it is assumed that
                            // PC is used as base address.
                            // As PC is always updating, and label is most probably an absolute address,
                            // the offset is calculated here

                            let offset: i32 = (args[2].get_val() as i32) - (code_line.address + 4);
                            code_line.machine_code.push(offset as u16);
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 3rd argument for {} instruction should be number or labelled address.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }
                },
                InstrType::Jmp => {
                    //println!("istr type is Jump");

                    match args[0].get_type() {
                        ArgType::Label => {
                            let offset: i32 = (args[0].get_val() as i32) - (code_line.address + 4);

                            code_line.machine_code[0] = code_line.machine_code[0] | (((offset >> 16) as u16) & 0x00FF);
                            code_line.machine_code.push(offset as u16);
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n argument for {} instruction should be labelled address.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }
                },
                InstrType::AddrArithm => {
                    //println!("istr type is AddrArithm");

                    match args[0].get_type() {
                        ArgType::MP => {
                            code_line.machine_code[0] = code_line.machine_code[0] | ((args[0].get_val() as u16) << 9);
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 1st argument for {} instruction should be one of the MP.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }

                    match args[1].get_type() {
                        ArgType::Gpr => {
                            code_line.machine_code[0] = code_line.machine_code[0] | ((args[1].get_val() as u16) << 5);
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 1st argument for {} instruction should be one of the gpr.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }

                },
                InstrType::AddrArImm => {
                    //println!("istr type is AddrArImm");

                    match args[0].get_type() {
                        ArgType::MP => {
                            code_line.machine_code[0] = code_line.machine_code[0] | ((args[0].get_val() as u16) << 9);
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 1st argument for {} instruction should be one of the MP.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }

                    match args[1].get_type() {
                        ArgType::Value => {
                            let val: i64 = args[1].get_val();

                            if val >= -8388608 && val <= 83886078 {
                                code_line.machine_code[0] = 
                                    code_line.machine_code[0] | 
                                    (((val >> 16) as u16) & 0x00FF);

                                code_line.machine_code.push(val as u16);
                            } else {
                                //error
                                println!("Arguments ERROR\n on line {}\n 2nd argument for {} instruction should be number in range [-8388608..8388607] (0x000000..0xFFFFFF), got {}.", code_line.line_number, code_line.code_parts[0], val);

                                *err = true;
                            }
                        },
                        ArgType::Label => {
                            let val: i64 = args[1].get_val();

                            if val >= -8388608 && val <= 83886078 {
                                code_line.machine_code[0] = 
                                    code_line.machine_code[0] | 
                                    (((val >> 16) as u16) & 0x00FF);

                                code_line.machine_code.push(val as u16);
                            } else {
                                //error
                                println!("Arguments ERROR\n on line {}\n 2nd argument for {} instruction should be number in range [-8388608..8388607] (0x000000..0xFFFFFF), got {}.", code_line.line_number, code_line.code_parts[0], val);

                                *err = true;
                            }
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 2nd argument for {} instruction should be number or labeled value.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }
                },
                InstrType::Mov => {
                    //println!("istr type is Mov");

                    match args[0].get_type() {
                        ArgType::Gpr => {
                            code_line.machine_code[0] = 
                                code_line.machine_code[0] | 
                                ((args[0].get_val() as u16) << 8);
                        },
                        ArgType::Mpr => {
                            code_line.machine_code[0] = 
                                code_line.machine_code[0] | 
                                ((args[0].get_val() as u16) << 8) |
                                0x0800;
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 1st argument for {} instruction should be one of the gpr or mpr.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }

                    match args[1].get_type() {
                        ArgType::Gpr => {
                            code_line.machine_code[0] = 
                                code_line.machine_code[0] | 
                                ((args[1].get_val() as u16) << 5);
                        },
                        ArgType::Mpr => {
                            code_line.machine_code[0] = 
                                code_line.machine_code[0] | 
                                ((args[1].get_val() as u16) << 5) |
                                0x0010;
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 2nd argument for {} instruction should be one of the gpr or mpr.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }
                },
                InstrType::Movs => {
                    //println!("istr type is Movs");

                    match args[0].get_type() {
                        ArgType::Gpr => {
                            code_line.machine_code[0] = 
                                code_line.machine_code[0] | 
                                ((args[0].get_val() as u16) << 8) |
                                ((args[0].get_val() as u16) << 5);

                            match args[1].get_type() {
                                ArgType::Special => {
                                    code_line.machine_code[0] = 
                                        code_line.machine_code[0] | 
                                        (args[1].get_val() as u16);
                                },
                                _ => {
                                    //error
                                    println!("Arguments ERROR\n on line {}\n 2nd argument for {} instruction should be one of special registers, in case when first one is gpr or mpr.", code_line.line_number, code_line.code_parts[0]);

                                    *err = true;
                                }
                            }
                        },
                        ArgType::Mpr => {
                            code_line.machine_code[0] = 
                                code_line.machine_code[0] | 
                                ((args[0].get_val() as u16) << 8) |
                                ((args[0].get_val() as u16) << 5) |
                                0x0810;
                            
                            match args[1].get_type() {
                                ArgType::Special => {
                                    code_line.machine_code[0] = 
                                        code_line.machine_code[0] | 
                                        (args[1].get_val() as u16);
                                },
                                _ => {
                                    //error
                                    println!("Arguments ERROR\n on line {}\n 2nd argument for {} instruction should be one of special registers, in case when first one is gpr or mpr.", code_line.line_number, code_line.code_parts[0]);

                                    *err = true;
                                }
                            }
                        },
                        ArgType::Special => {
                            code_line.machine_code[0] = 
                                code_line.machine_code[0] |
                                (args[0].get_val() as u16) |
                                0x0004; // set "write special" bit

                            match args[1].get_type() {
                                ArgType::Gpr => {
                                    code_line.machine_code[0] = 
                                        code_line.machine_code[0] | 
                                        ((args[1].get_val() as u16) << 8) |
                                        ((args[1].get_val() as u16) << 5);
                                },
                                ArgType::Mpr => {
                                    code_line.machine_code[0] = 
                                        code_line.machine_code[0] | 
                                        ((args[1].get_val() as u16) << 8) |
                                        ((args[1].get_val() as u16) << 5) |
                                        0x0810;
                                },
                                _ => {
                                    //error
                                    println!("Arguments ERROR\n on line {}\n 2nd argument for {} instruction should be one of gpr or mpr, in case when first one is special register.", code_line.line_number, code_line.code_parts[0]);

                                    *err = true;
                                }
                            }
                        }
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n 1st argument for {} instruction should be one of the gpr, mpr or special registers.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }
                },
                InstrType::Misc3bit => {
                    //println!("istr type is Misc3bit");

                    match args[0].get_type() {
                        ArgType::Value => {
                            let val: i64 = args[0].get_val();

                            if val >= 0 && val < 8 {
                                code_line.machine_code[0] = code_line.machine_code[0] | ((val as u16) << 5);
                            } else {
                                //error
                                println!("Arguments ERROR\n on line {}\n argument for {} instruction should be number in range [0..7] (0x0..0x7), got {}.", code_line.line_number, code_line.code_parts[0], val);

                                *err = true;
                            }
                        },
                        ArgType::Label => {
                            let val: i64 = args[0].get_val();

                            if val >= 0 && val < 8 {
                                code_line.machine_code[0] = code_line.machine_code[0] | ((val as u16) << 5);
                            } else {
                                //error
                                println!("Arguments ERROR\n on line {}\n argument for {} instruction should be number in range [0..7] (0x0..0x7), got {}.", code_line.line_number, code_line.code_parts[0], val);

                                *err = true;
                            }
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n argument for {} instruction should be number or labeled value.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }
                },
                InstrType::Misc8bit => {
                    //println!("istr type is Misc8bit");

                    match args[0].get_type() {
                        ArgType::Value => {
                            let val: i64 = args[0].get_val();

                            if val >= 0 && val < 256 {
                                code_line.machine_code[0] = code_line.machine_code[0] | (val as u16);
                            } else {
                                //error
                                println!("Arguments ERROR\n on line {}\n argument for {} instruction should be number in range [0..255] (0x0..0xFF), got {}.", code_line.line_number, code_line.code_parts[0], val);

                                *err = true;
                            }
                        },
                        ArgType::Label => {
                            let val: i64 = args[0].get_val();

                            if val >= 0 && val < 256 {
                                code_line.machine_code[0] = code_line.machine_code[0] | (val as u16);
                            } else {
                                //error
                                println!("Arguments ERROR\n on line {}\n argument for {} instruction should be number in range [0..255] (0x0..0xFF), got {}.", code_line.line_number, code_line.code_parts[0], val);

                                *err = true;
                            }
                        },
                        _ => {
                            //error
                            println!("Arguments ERROR\n on line {}\n argument for {} instruction should be number or labeled value.", code_line.line_number, code_line.code_parts[0]);

                            *err = true;
                        }
                    }
                },
                InstrType::Misc => {
                    //println!("istr type is Misc");

                    // there are no arguments here
                },
            }
        },
        None => {
            // couldn't find the instruction

            println!("Error finding instructions\n on line {}\n {} is not an instruction!", code_line.line_number, code_line.code_parts[0]);

            *err = true;
        }
    }


}