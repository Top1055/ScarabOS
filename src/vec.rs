use crate::scalloc::free;
use crate::scalloc::alloc;

#[derive(Debug)]
pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
            ptr: core::ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }

    pub fn push(&mut self, val: T) {
        if self.len == self.cap {
            let new_cap = if self.cap == 0 {
                1
            } else {
                self.cap * 2
            };
            let new_ptr = alloc(new_cap * core::mem::size_of::<T>()) as *mut T;
            unsafe {
                // Copy old elements to new memory
                core::ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len);

                // Free old memory
                free(self.ptr as *mut u8, self.cap * core::mem::size_of::<T>());

                // Update fields
                self.ptr = new_ptr;
                self.cap = new_cap;
            }
        }

        unsafe {
            // Write new element to end of vector
            core::ptr::write(self.ptr.offset(self.len as isize), val);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;

        unsafe {
            // Read and return last element of vector
            Some(core::ptr::read(self.ptr.offset(self.len as isize)))
        }
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx < self.len {
            unsafe {
                Some(&*self.ptr.offset(idx as isize))
            }
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        if idx < self.len {
            unsafe {
                Some(&mut *self.ptr.offset(idx as isize))
            }
        } else {
            None
        }
    }

    pub fn set(&mut self, idx: usize, val: T) -> Option<T> {
        if idx < self.len {
            unsafe {
                let old_val = core::ptr::replace(self.ptr.offset(idx as isize), val);
                Some(old_val)
            }
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        // Drop all elements of vector
        while let Some(_) = self.pop() {}

        // Free memory
        if !self.ptr.is_null() {
            free(self.ptr as *mut u8, self.cap * core::mem::size_of::<T>());
        }
    }
}

impl<T: PartialEq> PartialEq for Vec<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for i in 0..self.len() {
            if self.get(i) != other.get(i) {
                return false;
            }
        }

        true
    }
}

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {{
        let mut v = Vec::new();
        $(
            v.push($x);
        )*
        v
    }};
}
