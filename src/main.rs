use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::mem;

mod jit;
use jit::mem::JitMemory;

const INC_RAX : &'static [u8] = &[0x48, 0xFF, 0xC0];
const DEC_RAX : &'static [u8] = &[0x48, 0xFF, 0xC8];
const INC_RAX_ADDR : &'static [u8] = &[0xFE, 0x00];
const DEC_RAX_ADDR : &'static [u8] = &[0xFE, 0x08];
const OUTPUT : &'static [u8] = &[0x50, 0x48, 0x89, 0xC6, 0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, 0x48, 0xC7, 0xC7, 0x01, 0x00, 0x00, 0x00, 0x48, 0xC7, 0xC2, 0x01, 0x00, 0x00, 0x00, 0x0F, 0x05, 0x58];
const INPUT : &'static [u8] = &[0x50, 0x48, 0x89, 0xC6, 0x48, 0xC7, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x48, 0xC7, 0xC2, 0x01, 0x00, 0x00, 0x00, 0x48, 0xC7, 0xC7, 0x00, 0x00, 0x00, 0x00, 0x0F, 0x05, 0x58];
const LOOP_START : &'static [u8] = &[0x48, 0x80, 0x38, 0x00, 0x0F, 0x84, 0x00, 0x00, 0x00, 0x00];             //Last byte becomes offset
const LOOP_END : &'static [u8] = &[0x48, 0x80, 0x38, 0x00, 0x0F, 0x85, 0x00, 0x00, 0x00, 0x00];               //Last bytes becomes offset
const LOOP_START_OFF : i32 = 6;
#[allow(dead_code)]
const OK : &'static [u8] = &[0x50, 0x48, 0xC7, 0xC7, 0x4F, 0x4B, 0x00, 0x00, 0x57, 0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, 0x48, 0xC7, 0xC2, 0x02, 0x00, 0x00, 0x00, 0x48, 0x89, 0xE6, 0x48, 0xC7, 0xC7, 0x01, 0x00, 0x00, 0x00, 0x0F, 0x05, 0x5F, 0x58];

const CELLS : usize = 30000;

fn main(){
    let file_name = env::args().nth(1).unwrap();
    let mut jit = JitMemory::new(100);

    let mut vm_data: [u8; CELLS] = [0; CELLS];
    let ptr = vm_data.as_mut_ptr();

    let addr = unsafe {
        mem::transmute(ptr)
    };

    jit.put(&[0x48, 0x89, 0xF8]);

    let mut loop_starts = Vec::new();

    for byte in File::open(file_name).unwrap().bytes(){
        let character = byte.unwrap() as char;
        match character {
            '+' => jit.put(INC_RAX_ADDR),
            '-' => jit.put(DEC_RAX_ADDR),
            '<' => jit.put(DEC_RAX),
            '>' => jit.put(INC_RAX),
            '.' => jit.put(OUTPUT),
            ',' => jit.put(INPUT),
            '[' => {
                loop_starts.push(jit.position as i32);
                jit.put(LOOP_START);

            },
            ']' => {
                jit.put(LOOP_END);
                let end_pos = jit.position as i32;
                let loop_start = loop_starts.pop().unwrap();
                let loop_start_offset_byte = loop_start + LOOP_START_OFF;
                if jit[loop_start_offset_byte as usize] != 0x00 { panic!("This ain't right at {}: {:X}", loop_start_offset_byte, jit[loop_start_offset_byte as usize]) }
                jit.put_offset(loop_start_offset_byte, end_pos - loop_start_offset_byte-4);

                let loop_end_offset_byte = jit.position as i32 - 4;
                let offset = loop_start - end_pos;
                assert_eq!(jit[loop_end_offset_byte as usize], 0x00);
                jit.put_offset(loop_end_offset_byte, offset);
            },
            //'#' => jit.put(OK),
            _ => {

            }
        }
    }
    let function = jit.as_fn();
    println!("Jit ready (size: {} bytes), executing...", jit.position);
    println!("---[JIT STARTS HERE]---");
    let ret = function(addr);
    println!("\n---[JIT ENDS HERE]---");
    println!("Jit exited, code: {}", ret);

}
