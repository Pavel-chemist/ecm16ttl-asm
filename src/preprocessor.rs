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

use std::collections::HashMap;

use crate::structs::{Code, Label, InstrDescr};
use crate::lut;
use crate::enums::Segment;

pub fn first_read(raw_lines: Vec<&str>, labels_table: &mut Vec<Label>) -> Vec<Code> {
    let instructions_lut: HashMap<&str, InstrDescr> = lut::get_instr_table(); 

    let mut code_table: Vec<Code> = Vec::new();
    let mut address_counter: i32 = 0;

    let mut is_label: bool = false;
    let mut segment: Segment = Segment::None;

    for i in 0..raw_lines.len() {
        let words: Vec<&str> = raw_lines[i].split_whitespace().collect();
        let mut code_line: Code = Code::new(i as i32, raw_lines[i]);

        if words.len() > 0 {
            match words[0] {
                ".const" => { segment = Segment::Const },
                ".text" => { segment = Segment::Text },
                ".data" => { segment = Segment::Data },
                _ => {},
            }
        }

        match segment {
            Segment::Const => parse_constants_line(&words, labels_table),
            Segment::Text => parse_text_line(&words, labels_table, &mut code_line),
            Segment::Data => parse_data_line(&words, labels_table, &mut code_line),
            _ => {},
        }


/*         is_label = words[0].chars().last().unwrap_or('_') == ':';

        if is_label {
            instr_index = 1;
            // let 
            // labels_table.push()
        } else {
            instr_index = 0;
        }
 */

        match instructions_lut.get(words[0]) {
            Some(idescr) => println!("Instruction {} is of {} type", words[0], idescr.itype),
            None => println!("instruction {} is unknown.", words[0]),    
        };
        
    }

    println!("lines: ");
    
    for i in 0..raw_lines.len() {
        println!("{}", raw_lines[i]);
    }

    return code_table;
}

fn parse_constants_line(code_line: &Vec<&str>, labels_table: &mut Vec<Label>) {
    //push found constants to labels table
}

fn parse_text_line(code_line: &Vec<&str>, labels_table: &mut Vec<Label>, listing_line: &mut Code) {
    //push found labels to labels table
    //if found code, update listing_line
}

fn parse_data_line(code_line: &Vec<&str>, labels_table: &mut Vec<Label>, listing_line: &mut Code) {
    //push found labels to labels table
    //if found variable directive and also variable value, update listing_line
}