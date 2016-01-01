use std::mem;
use std::ops::{Index, IndexMut, Drop};
use jit::ops::BranchOperation;

extern crate libc;

const PAGE_SIZE: usize = 4096;

pub struct JitMemory {
    contents: *mut u8,
    pub position: usize,
    pub size: usize
}

#[allow(unused_mut)]
impl JitMemory {
    pub fn new(pages: usize) -> JitMemory {
        let size = pages * PAGE_SIZE;
        let mut contents;
        unsafe {
            let page = Self::get_page(size);
            contents = mem::transmute(page);
        }
        JitMemory {
            contents: contents,
            position: 0,
            size: size
        }

    }

    pub fn as_fn(&self) -> (fn(a: u64) -> i64) {
        unsafe {
            mem::transmute(self.contents)
        }
    }

    unsafe fn get_page(size : usize) -> *mut libc::c_void {
        let mut page: *mut libc::c_void = mem::uninitialized();
        libc::posix_memalign(&mut page, PAGE_SIZE, size);
        let permissions = libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE;
        libc::mprotect(page, size, permissions);
        memset(page, 0xc3, size);
        return page;
    }

    pub fn put_one(&mut self, data : u8) -> &mut Self {
        if self.position == self.size { panic!("VM Memory limit reached: {}", self.size) }
        unsafe {
            *self.contents.offset(self.position as isize) = data;
        }
        self.position += 1;

        self
    }

    pub fn put<T: Into<Box<[u8]>>>(&mut self, data: T) {
        for op in data.into().iter() {
            self.put_one(*op);
        }
    }

    pub fn put_branch(&mut self, op: BranchOperation) -> usize {
        self.put(op.content);
        return self.position - 4
    }

    pub fn put_offset(&mut self, index: usize, data: i32) {
        unsafe {
            let bytes : [u8; 4] = mem::transmute(data);
            for i in 0..4 as usize {
                assert_eq!(self[index+i], 0x00);
                self[(index+i)] = bytes[i as usize];
            }
        }
    }

    #[allow(dead_code)]
    pub fn disp(&self) {
        for i in 0..self.position {
            print!("{:X} ", self[i as usize]);
        }
    }
}

impl Index<usize> for JitMemory {
    type Output = u8;

    fn index(&self, _index: usize) -> &u8 {
        unsafe {&*self.contents.offset(_index as isize) }
    }
}

impl IndexMut<usize> for JitMemory {
    fn index_mut(&mut self, _index: usize) -> &mut u8 {
        unsafe {&mut *self.contents.offset(_index as isize) }
    }
}

impl Drop for JitMemory {
    fn drop(&mut self){
        unsafe {
            libc::free(self.contents as *mut libc::c_void);
        }
    }
}

extern {
    fn memset(s: *mut libc::c_void, c: libc::uint32_t, n: libc::size_t) -> *mut libc::c_void;
}
