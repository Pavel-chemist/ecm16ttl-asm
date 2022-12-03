/* 
directives:

.const
const_label: .equ value   // the values are 32-bit unsigned integers -- these are addresses, usually for MMIO
const_label = value

.text
(label:) INSTR operand1 operand2 ... (#comment)  //comments are ignored

(label:)    //if label is detected, it is added to labels table, associated with current counter value
    INSTR ...       // the counter is updated only when instruction is found

    // btw, the option where labels are on separate lines seems simpler for assembler -- no additional logic is needed

.data
data_label1:  .byte  (value) // reserve 1 byte;        //if no value provided, fill with zeros
data_label2:  .word (value)  // reserve 2 bytes;
data_label3:  .dword (value) // reserve 4 bytes;
data_label4:  .long (value) // reserve 8 bytes;

data_label5:  .string "some string"  //array of bytes,     

formats for value:
no value     -- assume 0
plain value  -- any number, in binary, hex or decimal -- will be truncated to data size, with warnings if value is bigger
[ ... ]      -- array, 3 ways:
    [val1, val2, val4, ... ]  -- array with values 


basic principle: 
the text of program is split into lines, forming an array
going line by line, each line is split into words
rules

set byte counter to 0

after encountering .const directive
    push label-value pair into "constants" table ({str, u32} tuple)

after encountering .text directive
if label -> push label:current counter tuple into "labels" table
else if instruction -> push all parts (except comment) into "code" array as subarray
else ignore

after encountering .data directive

-------------------------------------------------------

[&str1, &str2, ... ] -- line from input
 
converted to object
{
    line_number: u32;
    instruction: bool;  //true if mnemonic is in original_line[0]
    address?: u32;
    num_words?: u32;  //number of machine code words: 1, 2 or 3 
    machine_code?: Vec<u16>;  //output of assembler
    label?: &str;
    code_parts?: Vec<&str>;  //extracted mnemonics for instructions+operands
    original_line: &str;
}

object for table of constants -- same as for labels
these constants are to be placed before .text segment

object for table of labels:
{
    label: &str;
    address: u32;
}

object for table of static variables:

static variables goes together with code table
all static variables are only in .data segment




directives:

.const
.text
.data
#comment
.byte
.str
.word
.dword
.long

labels are any string in original_line[0] ending with ':'
label should be on preceding line before labeled instruction or variable

.const
    const_name = 0xff000010     #comment

.text
label_name1:
    ADD r0 r1 r2    #comment
    
.data

variable_name1:
    .word   525     #comment
    
var_name2:
    .string   "string content"    #comment



....


on second pass:
encode code lines into machine instructions, with look-up in "labels" and "constants"

*/

// use std::collections::HashMap;
mod data_line_parser;
mod const_line_parser;
mod text_line_parser;

use crate::structs::{Code, Label};
use crate::enums::Segment;
use data_line_parser::parse_data_line;
use const_line_parser::parse_constants_line;
use text_line_parser::parse_text_line;



pub fn first_read(raw_lines: Vec<&str>, labels_table: &mut Vec<Label>, err: &mut bool) -> Vec<Code> {

    let mut code_table: Vec<Code> = Vec::new();
    let mut address_counter: i32 = 0;
    let mut segment: Segment = Segment::None;

    for i in 0..raw_lines.len() {
        let raw_line: String = String::from(raw_lines[i]); //copy line for scoping reasons

        let words: Vec<&str> = raw_line.split_whitespace().collect();
        let comment_index: usize = find_comment_start_index(&words);
        let mut code_line: Code = Code::new((i + 1) as i32, &raw_line);

        if comment_index > 0 {
            match words[0] {
                ".const" => { segment = Segment::Const },
                ".text" => { segment = Segment::Text },
                ".data" => { segment = Segment::Data },
                _ => {
                    match segment {
                        Segment::Const => {
                            parse_constants_line(
                                i,
                                words,
                                comment_index,
                                labels_table,
                                err);
                        },
                        Segment::Text => {
                            address_counter = parse_text_line(
                                i,
                                words,
                                comment_index,
                                address_counter,
                                labels_table,
                                &mut code_line,
                                err)
                        },
                        Segment::Data => {
                            address_counter = parse_data_line(
                                i,
                                words,
                                comment_index,
                                address_counter,
                                labels_table,
                                &mut code_line,
                                err)
                        },
                        _ => {},
                    }
                },
            }
        }

        code_table.push(code_line);
    }

    return code_table;
}

fn find_comment_start_index(code_line: &Vec<&str>) -> usize{
    let mut index: usize = 0;

    for i in 0..code_line.len() {
        if code_line[i].chars().nth(0).unwrap() == '#' {
            break;
        }

        index = i + 1;
    }

    return index;
}