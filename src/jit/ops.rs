pub struct Operation {
    content: Box<[u8]>
}

impl Operation {
    pub fn inc_ptr() -> Operation {
        Operation {
            content: Box::new([0x48, 0xFF, 0xC0])           //inc rax
        }
    }
    pub fn dec_ptr() -> Operation {
        Operation {
            content: Box::new([0x48, 0xFF, 0xC8])           //dec rax
        }
    }
    pub fn inc_data() -> Operation {
        Operation {
            content: Box::new([0xFE, 0x00])                 //inc BYTE PTR [rax]
        }
    }
    pub fn dec_data() -> Operation {
        Operation {
            content: Box::new([0xFE, 0x08])                 //dec BYTE PTR [rax]
        }
    }
    pub fn read_byte() -> Operation {
        Operation {
            content: Box::new([
                0x50,                                       //push  rax
                0x48, 0x89, 0xC6,                           //mov   rsi, rax
                0x48, 0xC7, 0xC0, 0x00, 0x00, 0x00, 0x00,   //mov   rax, 0
                0x48, 0xC7, 0xC7, 0x00, 0x00, 0x00, 0x00,   //mov   rdi, 0
                0x48, 0xC7, 0xC2, 0x01, 0x00, 0x00, 0x00,   //mov   rdx, 1
                0x0F, 0x05,                                 //syscall
                0x58                                        //pop   rax
            ])
        }
    }
    pub fn write_byte() -> Operation {
        Operation {
            content: Box::new([
                0x50,                                       //push  rax
                0x48, 0x89, 0xC6,                           //mov   rsi, rax
                0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00,   //mov   rax, 1
                0x48, 0xC7, 0xC7, 0x01, 0x00, 0x00, 0x00,   //mov   rdi, 1
                0x48, 0xC7, 0xC2, 0x01, 0x00, 0x00,0x00,    //mov   rdx, 1
                0x0F, 0x05,                                 //syscall
                0x58                                        //pop   rax
            ])
        }
    }
}

impl Into<Box<[u8]>> for Operation {
    fn into(self) -> Box<[u8]> {
        self.content
    }
}

pub const BRANCH_OP_SIZE : usize = 6;
pub const BRANCH_OFFSET_SIZE : usize = 4;

pub struct BranchOperation {
    pub content: Box<[u8]>,
}

impl BranchOperation {
    pub fn loop_start() -> BranchOperation {
        BranchOperation {
            content: Box::new([
                0x48, 0x80, 0x38, 0x00,                     //cmp BYTE PTR [rax], 0
                0x0F, 0x84, 0x00, 0x00, 0x00, 0x00          //je <offset as i32>
            ]),
        }
    }
    pub fn loop_end() -> BranchOperation {
        BranchOperation {
            content: Box::new([
                0x48, 0x80, 0x38, 0x00,                     //cmp BYTE PTR [rax], 0
                0x0F, 0x85, 0x00, 0x00, 0x00, 0x00          //jne <offset as i32>
            ]),
        }
    }
}
