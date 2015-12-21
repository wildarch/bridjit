use std::mem;
use std::ops::{Index, IndexMut};

extern crate libc;

const PAGE_SIZE: usize = 4096;

pub struct JitMemory {
    contents: *mut u8,
    position: isize,
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
        }

    }

    pub fn as_fn(&self) -> (fn() -> i64) {
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

    pub fn contents_addr(&self) -> u64 {
        let ptr : *mut u8 = self.contents;
        unsafe {
            mem::transmute(ptr) //Address to u64
        }
    }

    pub fn put(&mut self, data : u8) -> &mut Self {
        unsafe {
            *self.contents.offset(self.position) = data;
        }
        self.position += 1;
        self
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

extern {
    fn memset(s: *mut libc::c_void, c: libc::uint32_t, n: libc::size_t) -> *mut libc::c_void;
}
