use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::mem;

mod jit;
use jit::mem::JitMemory;
use jit::ops::{self, Operation, BranchOperation};

const CELLS : usize = 30000;

fn main(){
    let file_name = env::args().nth(1).unwrap();
    let mut jit = JitMemory::new(100);  //Should be enough for most brainfuck programs (100Kb)

    //The array of cells
    let mut vm_data: [u8; CELLS] = [0; CELLS];
    let ptr = vm_data.as_mut_ptr();

    // Get the address of the array
    let addr = unsafe {
        mem::transmute(ptr)
    };

    // The jit will first load the address of the array into the data pointer
    let get_addr : Box<[u8]> = Box::new([0x48, 0x89, 0xF8]);        // mov rax, rsi
    jit.put(get_addr);

    let mut offset_indices = Vec::new();

    for byte in File::open(file_name).unwrap().bytes(){
        let character = byte.unwrap() as char;
        match character {
            '+' => jit.put(Operation::inc_data()),
            '-' => jit.put(Operation::dec_data()),
            '<' => jit.put(Operation::dec_ptr()),
            '>' => jit.put(Operation::inc_ptr()),
            '.' => jit.put(Operation::write_byte()),
            ',' => jit.put(Operation::read_byte()),
            '[' => {
                let op = BranchOperation::loop_start();
                let offset_index = jit.put_branch(op);
                offset_indices.push(offset_index);

            },
            ']' => {
                let op = BranchOperation::loop_end();
                jit.put_branch(op);

                let op_size = ops::BRANCH_OP_SIZE;
                let offset_size = ops::BRANCH_OFFSET_SIZE;
                let end_pos = jit.position as i32;
                // The position of the offset in the jit content
                let offset_index = offset_indices.pop().unwrap();
                // We want to jump to the instruction after the loop start
                let offset = end_pos - (offset_index + offset_size) as i32;
                jit.put_offset(offset_index, offset);

                let loop_start = (offset_index - op_size) as i32;
                let offset_index = jit.position - offset_size;
                let offset = loop_start - end_pos;
                jit.put_offset(offset_index, offset);
            },
            _ => ()
        }
    }
    if offset_indices.len() != 0 {
        panic!("Unclosed brackets!");
    }
    let function = jit.as_fn();
    function(addr);
    println!("");
}
