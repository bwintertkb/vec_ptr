use std::{fmt::Debug, ptr};

#[derive(Debug, Clone, Default)]
pub struct VecPtr<T> {
    ptrs: Vec<*mut T>,
    raw_vals: Vec<T>,
    len: usize, // Length of vectors
}

impl<T: Debug> VecPtr<T> {
    pub fn new() -> Self {
        VecPtr {
            ptrs: Vec::new(),
            raw_vals: Vec::new(),
            len: 0,
        }
    }

    pub fn push(&mut self, v: T) {
        self.raw_vals.push(v);
        self.len += 1;
        let ptr: *mut T = &mut self.raw_vals[self.len - 1] as *mut T;
        self.ptrs.push(ptr);
    }

    pub fn insert(&mut self, idx: usize, mut v: T) {
        self.ptrs.insert(idx, &mut v as *mut T);
        self.raw_vals.push(v);
        self.len += 1;
    }

    pub fn remove(&mut self, idx: usize) {
        let r_ptr = self.ptrs.remove(idx);
        self.raw_vals.retain(|v| v as *const T != r_ptr);
        self.len -= 1;
    }

    pub fn pop(&mut self) -> Option<*mut T> {
        let ptr = self.ptrs.pop();
        self.raw_vals.pop();
        self.len -= 1;
        ptr
    }

    pub fn swap(&mut self, idx_1: usize, idx_2: usize) {
        let p1 = ptr::addr_of_mut!(self.ptrs[idx_1]);
        let p2 = ptr::addr_of_mut!(self.ptrs[idx_2]);

        unsafe {
            println!("{:?}", *p1);
            //println!("{:?}", *self.ptrs[idx_1]);
            ptr::swap(p1, p2);
            for v in self.raw_vals.iter() {
                println!("{:?}", v as *const T);
            }
        }
    }

    pub fn update(&mut self, idx: usize, new_v: T) {
        unsafe { *self.ptrs[idx] = new_v }
    }

    pub fn get(&self, idx: usize) -> &T {
        unsafe { &*self.ptrs[idx] }
    }
}
