use std::{marker::PhantomData, alloc::{self, alloc}, ptr::null_mut};

pub struct Circular<T> {
    first: *mut Node<T>,
    size: usize,
    _own: PhantomData<T>,
}

struct Node<T> {
    value: T,
    next: *mut Node<T>,
    previous: *mut Node<T>,
}

impl<T> Circular<T> {
    pub fn new() -> Self {
        Circular {
            first: null_mut(),
            size: 0,
            _own: PhantomData,
        }
    }

    pub fn prepend(&mut self, val: T) {
        self.insert(0, val);
    }

    pub fn insert(&mut self, loc: usize, val: T) {
        assert!(loc <= self.size);
        let layout = alloc::Layout::new::<Node<T>>();
        let ptr = unsafe {
            alloc(layout) as *mut Node<T>
        };

        if self.size > 0 {
            let mut curr = self.first;
            let mut i = loc;
            unsafe {
                while i > 0 {
                    curr = (*curr).next;
                    i -= 1;
                }

                let next = curr;
                let prev = (*next).previous;
                std::ptr::write(ptr, Node {
                    value: val,
                    next: next,
                    previous: prev,
                });
                (*next).previous = ptr;
                (*prev).next = ptr;
            }
        } else {
            unsafe {
                std::ptr::write(ptr, Node {
                    value: val,
                    next: ptr,
                    previous: ptr,
                })
            }
        }

        if loc == 0 {
            self.first = ptr;
        }
        
        self.size += 1;
    }
}
