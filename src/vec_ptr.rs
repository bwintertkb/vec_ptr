#[derive(Debug, Clone, Default)]
pub struct VecPtr<T> {
    ptrs: Vec<*mut T>,
    raw_vals: Vec<T>,
    len: usize, // Length of vectors
}

impl<T> VecPtr<T> {
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

    pub fn remove(&mut self, idx: usize) {
        self.ptrs.remove(idx);
        self.raw_vals.remove(idx);
        self.len -= 1;
    }

    pub fn pop(&mut self) -> Option<*mut T> {
        let ptr = self.ptrs.pop();
        self.raw_vals.pop();
        self.len -= 1;
        ptr
    }

    pub fn swap(&mut self, idx_1: usize, idx_2: usize) {
        unsafe {
            std::ptr::swap(&mut self.ptrs[idx_1], &mut self.ptrs[idx_2]);
        }
    }

    pub fn update(&mut self, idx: usize, new_v: T) {
        unsafe { *self.ptrs[idx] = new_v }
    }

    pub fn get_val(&self, idx: usize) -> &T {
        unsafe { &*self.ptrs[idx] }
    }
}
