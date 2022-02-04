use std::{marker::PhantomData, alloc::{self, alloc, Layout}, ptr::{NonNull,self}};

pub struct Circular<T> {
    first: NonNull<Node<T>>,
    size: usize,
    _own: PhantomData<T>,
}

struct Node<T> {
    value: T,
    next: NonNull<Node<T>>,
    previous: NonNull<Node<T>>,
}

impl<T> Circular<T> {
    fn node_layout() -> Layout {
        Layout::new::<Node<T>>()
    }

    pub fn new() -> Self {
        Circular {
            first: NonNull::dangling(),
            size: 0,
            _own: PhantomData,
        }
    }

    pub fn prepend(&mut self, val: T) {
        self.insert(0, val);
    }

    pub fn insert(&mut self, loc: usize, val: T) {
        assert!(loc <= self.size);
        let layout = Self::node_layout();
        let ptr = unsafe {
            let loc = alloc(layout) as *mut Node<T>;
            match NonNull::new(loc) {
                Some(p) => p,
                None => alloc::handle_alloc_error(layout),
            }
        };

        if self.size > 0 {
            let mut curr = self.first;
            let mut i = loc;
            unsafe {
                while i > 0 {
                    curr = curr.as_ref().next;
                    i -= 1;
                }

                let mut next = curr;
                let mut prev = next.as_ref().previous;
                std::ptr::write(ptr.as_ptr(), Node {
                    value: val,
                    next: next,
                    previous: prev,
                });
                next.as_mut().previous = ptr;
                prev.as_mut().next = ptr;
            }
        } else {
            unsafe {
                std::ptr::write(ptr.as_ptr(), Node {
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

    pub fn remove(&mut self, loc: usize) -> T {
        assert!(loc < self.size);
        self.size -= 1;

        let mut curr = self.first;

        self.first = if self.size == 0 {
            // change to dangling pointer
            NonNull::dangling()
        } else if loc == 0 {
            // assign to next node
            unsafe {
                self.first.as_ref().next
            }
        } else {
            // else do nothing
            self.first
        };

        let mut i = loc;

        unsafe {
            while i > 0 {
                curr = curr.as_ref().next;
                i -= 1;
            }

            let mut next = curr.as_ref().next;
            let mut prev = curr.as_ref().previous;
            next.as_mut().previous = prev;
            prev.as_mut().next = next;

            let res = ptr::read(curr.as_ptr()).value;

            let layout = Self::node_layout();
            alloc::dealloc(curr.as_ptr() as *mut u8, layout);

            res
        }
    }
}
