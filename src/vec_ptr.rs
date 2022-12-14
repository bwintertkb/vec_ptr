use std::{
    fmt::Debug,
    ops::Index,
    ptr,
    slice::{Iter, IterMut},
};

/// A vector that operates on raw pointers. It's a normal vector, just a little bit more unsafe...
#[derive(Debug, Clone, Default)]
pub struct VecPtr<T> {
    ptrs: Vec<*mut T>,
    raw_vals: Vec<T>,
}

impl<T> VecPtr<T> {
    /// Constructor for the VecPtr struct
    pub fn new() -> Self {
        VecPtr {
            ptrs: Vec::new(),
            raw_vals: Vec::new(),
        }
    }

    /// Define a VecPtr with a predefined size
    pub fn with_capacity(size: usize) -> Self {
        VecPtr {
            ptrs: Vec::with_capacity(size),
            raw_vals: Vec::with_capacity(size),
        }
    }

    /// Push a value to the end of the VecPtr
    pub fn push(&mut self, v: T) {
        self.raw_vals.push(v);
        let l = self.raw_vals.len();
        let ptr: *mut T = &mut self.raw_vals[l - 1] as *mut T;
        self.ptrs.push(ptr);
    }

    /// Insert a value at a user defined index
    pub fn insert(&mut self, idx: usize, mut v: T) {
        self.ptrs.insert(idx, &mut v as *mut T);
        self.raw_vals.push(v);
    }

    /// Remove a value at a user defined index
    pub fn remove(&mut self, idx: usize) {
        let r_ptr = self.ptrs.remove(idx);
        self.raw_vals.retain(|v| v as *const T != r_ptr);
    }

    /// Remove a value from the end of the VecPtr
    pub fn pop(&mut self) -> Option<*mut T> {
        let ptr = self.ptrs.pop();
        self.raw_vals.pop();
        ptr
    }

    /// Swap values in the VecPtr
    pub fn swap(&mut self, idx_1: usize, idx_2: usize) {
        let p1 = ptr::addr_of_mut!(self.ptrs[idx_1]);
        let p2 = ptr::addr_of_mut!(self.ptrs[idx_2]);

        unsafe {
            ptr::swap(p1, p2);
        }
    }

    /// Update a value in the VecPtr
    pub fn update(&mut self, idx: usize, new_v: T) {
        unsafe { *self.ptrs[idx] = new_v }
    }

    /// Get an immutable value from the VecPtr
    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx >= self.ptrs.len() {
            return None;
        }
        unsafe { Some(&*self.ptrs[idx]) }
    }

    /// Get an mutable value from the VecPtr
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        if idx >= self.ptrs.len() {
            return None;
        }
        unsafe { Some(&mut *self.ptrs[idx]) }
    }

    /// Convert VecPtr to an borrowed iterator
    pub fn iter(&self) -> Iter<'_, *mut T> {
        self.ptrs.iter()
    }

    /// Convert VecPtr to a owned iterator
    pub fn iter_mut(&mut self) -> IterMut<'_, *mut T> {
        self.ptrs.iter_mut()
    }
}

pub struct VecPtrIntoIter<T> {
    ptrs: Vec<*mut T>,
    idx: usize,
}

impl<T> IntoIterator for VecPtr<T> {
    type Item = *mut T;

    type IntoIter = VecPtrIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        VecPtrIntoIter {
            ptrs: self.ptrs,
            idx: 0,
        }
    }
}

impl<T> Iterator for VecPtrIntoIter<T> {
    type Item = *mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.ptrs[self.idx];
        self.idx += 1;
        Some(val)
    }
}

impl<T> Index<usize> for VecPtr<T> {
    type Output = *mut T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.ptrs[index]
    }
}

/// A declarative macro that allows for the create of a VecPtr object.
#[macro_export]
macro_rules! vec_ptr {
    ($($element:expr),*) => {{
        #[allow(unused_mut)]
        let mut v = VecPtr::with_capacity($crate::count![@COUNT; $($element),*]);
        $(
            v.push($element);
        )*
        v
    }};
    ($($element:expr,)*) => {{
        $crate::vec_ptr![$($element),*]
    }};
    ($element:expr; $count:expr) => {{
        let count = $count;
        let mut v = VecPtr::with_capacity(count);
        let x = $element;
        for _ in 0..count {
            v.push(x.clone());
        }
        v
    }};

}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($element:expr),*) => {
        <[()]>::len(&[$($crate::count![@SUBST; $element]),*])
    };

    (@SUBST; $_element:expr) => {
        ()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vec_ptr_macro() {
        let v_ptr: VecPtr<u32> = vec_ptr![];
        assert!(v_ptr.ptrs.is_empty());
        assert!(v_ptr.raw_vals.is_empty());
    }

    #[test]
    fn test_single_entry_vec_ptr_macro() {
        let v_ptr: VecPtr<u32> = vec_ptr!(1);

        assert!(!v_ptr.ptrs.is_empty());
        assert!(!v_ptr.raw_vals.is_empty());
        assert_eq!(v_ptr.ptrs.len(), 1);
        assert_eq!(v_ptr.raw_vals.len(), 1);
    }

    #[test]
    fn test_double_entry_vec_ptr_macro() {
        let v_ptr: VecPtr<u32> = vec_ptr!(1, 2);

        assert!(!v_ptr.ptrs.is_empty());
        assert!(!v_ptr.raw_vals.is_empty());
        assert_eq!(v_ptr.ptrs.len(), 2);
        assert_eq!(v_ptr.raw_vals.len(), 2);
    }

    #[test]
    fn test_trailing_vec_ptr_macro() {
        let v_ptr: VecPtr<u32> = vec_ptr!(1, 2,);

        assert!(!v_ptr.ptrs.is_empty());
        assert!(!v_ptr.raw_vals.is_empty());
        assert_eq!(v_ptr.ptrs.len(), 2);
        assert_eq!(v_ptr.raw_vals.len(), 2);
    }

    #[test]
    fn test_count_vec_ptr_macro() {
        let v_ptr: VecPtr<u32> = vec_ptr!(1; 3);

        assert!(!v_ptr.ptrs.is_empty());
        assert!(!v_ptr.raw_vals.is_empty());
        assert_eq!(v_ptr.ptrs.len(), 3);
        assert_eq!(v_ptr.raw_vals.len(), 3);
    }
}
