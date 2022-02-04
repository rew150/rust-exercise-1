use std::alloc::{self, Layout};
use std::marker::PhantomData;
use std::ops::Add;
use std::ptr::{NonNull, self};
use std::ops::{Deref, DerefMut};
use std::slice::{from_raw_parts, from_raw_parts_mut};

pub struct vector<T> {
    loc: NonNull<T>,
    cap: usize,
    size: usize,
    _own: PhantomData<T>,
}

impl<T> vector<T> {
    fn new() -> Self {
        vector{
           loc: NonNull::dangling(),
           size: 0,
           cap: 0,
           _own: PhantomData,
        }
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = 2 * self.cap;

            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        let new_ptr = if self.cap == 0 {
            unsafe {
                alloc::alloc(new_layout)
            }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.loc.as_ptr() as *mut u8;
            unsafe {
                alloc::realloc(old_ptr, old_layout, new_layout.size())
            }
        };

        self.loc = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }

    fn push(&mut self, elem: T) {
        if self.size == self.cap {
            self.grow();
        }

        unsafe {
            ptr::write(self.loc.as_ptr().add(self.size), elem);
        }
        
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            self.size -= 1;
            unsafe {
                Some(ptr::read(self.loc.as_ptr().add(self.size)))
            }
        }
    }

    fn insert(&mut self, loc: usize, elem: T) {
        assert!(loc <= self.size);
        if self.cap == self.size {
            self.grow();
        }

        unsafe {
            ptr::copy(self.loc.as_ptr().add(loc), self.loc.as_ptr().add(loc + 1), self.size-loc);
            ptr::write(self.loc.as_ptr().add(loc), elem);
            self.size += 1;
        }
    }

    fn remove(&mut self, loc: usize) -> T {
        assert!(loc < self.size);
        unsafe {
            self.size -= 1;
            let res = ptr::read(self.as_ptr().add(loc));
            ptr::copy(self.loc.as_ptr().add(loc + 1), self.loc.as_ptr().add(loc), self.size - loc);
            res
        }
    }
}

impl<T> Drop for vector<T> {
    fn drop(&mut self) {
        if self.cap > 0 {
            // drop every element
            while let Some(_) = self.pop() {}
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.loc as *mut u8, layout);
            }
        } 
    }
}

impl<T> Deref for vector<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe {
            from_raw_parts(self.loc.as_ptr(), self.size)
        }
    }
}

impl<T> DerefMut for vector<T> {
    type Target = [T];
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            from_raw_parts_mut(self.loc.as_ptr(), self.size)
        }
    }
}
