mod test;

use std::alloc::{self, Layout};
use std::marker::PhantomData;
use std::mem;
use std::ptr::{NonNull, self};
use std::ops::{Deref, DerefMut};
use std::slice::{from_raw_parts, from_raw_parts_mut};

pub struct Vector<T> {
    loc: NonNull<T>,
    cap: usize,
    size: usize,
    _own: PhantomData<T>,
}

pub struct IntoIter<T> {
    buf: NonNull<T>,
    cap: usize,
    start: *const T,
    end: *const T,
    _own: PhantomData<T>,
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        Vector{
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

    pub fn push(&mut self, elem: T) {
        if self.size == self.cap {
            self.grow();
        }

        unsafe {
            ptr::write(self.loc.as_ptr().add(self.size), elem);
        }
        
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            self.size -= 1;
            unsafe {
                Some(ptr::read(self.loc.as_ptr().add(self.size)))
            }
        }
    }

    pub fn insert(&mut self, loc: usize, elem: T) {
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

    pub fn remove(&mut self, loc: usize) -> T {
        assert!(loc < self.size);
        unsafe {
            self.size -= 1;
            let res = ptr::read(self.as_ptr().add(loc));
            ptr::copy(self.loc.as_ptr().add(loc + 1), self.loc.as_ptr().add(loc), self.size - loc);
            res
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        let ptr = self.loc;
        let cap = self.cap;
        let size = self.size;
        mem::forget(self);
        unsafe {
            IntoIter {
                buf: ptr,
                cap: cap,
                start: ptr.as_ptr(),
                end: if cap == 0 {
                    ptr.as_ptr()
                } else {
                    ptr.as_ptr().add(size)
                },
                _own: PhantomData
            }
        }
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        if self.cap > 0 {
            // drop every element
            while let Some(_) = self.pop() {}
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.loc.as_ptr() as *mut u8, layout);
            }
        } 
    }
}

impl<T> Deref for Vector<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe {
            from_raw_parts(self.loc.as_ptr(), self.size)
        }
    }
}

impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            from_raw_parts_mut(self.loc.as_ptr(), self.size)
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let res = ptr::read(self.start);
                self.start = self.start.offset(1);
                Some(res)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end as usize - self.start as usize) / mem::size_of::<T>();
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                self.end = self.end.offset(-1);
                Some(ptr::read(self.end))
            }
        }
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            for _ in &mut *self {}
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.buf.as_ptr() as *mut u8, layout);
            }
        }
    }
}