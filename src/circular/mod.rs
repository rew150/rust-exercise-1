mod test;

use std::{marker::PhantomData, alloc::{self, alloc, Layout}, ptr::{NonNull,self}, ops::{Index, IndexMut}};

pub struct Circular<T> {
    first: NonNull<Node<T>>,
    size: usize,
    _own: PhantomData<T>,
}

pub struct IterMut<'a, T> {
    next: *mut Node<T>,
    prev: *mut Node<T>,
    _own: PhantomData<&'a mut T>,
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

    pub fn value_at(&self, idx: usize) -> &T {
        assert!(idx < self.size);
        let mut curr = self.first;
        let mut i = idx;
        unsafe {
            while i > 0 {
                curr = curr.as_ref().next;
                i -= 1;
            }
            &curr.as_mut().value
        }
    }

    pub fn value_at_mut(&mut self, idx: usize) -> &mut T {
        assert!(idx < self.size);
        let mut curr = self.first;
        let mut i = idx;
        unsafe {
            while i > 0 {
                curr = curr.as_ref().next;
                i -= 1;
            }
            &mut curr.as_mut().value
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            next: if self.size == 0 {
                ptr::null_mut()
            } else {
                unsafe {
                    self.first.as_ptr()
                }
            },
            prev: if self.size == 0 {
                ptr::null_mut()
            } else {
                unsafe {
                    self.first.as_ref().previous.as_ptr()
                }
            },
            _own: PhantomData,
        }
    }
}

impl<T> Drop for Circular<T> {
    fn drop(&mut self) {
        while self.size > 0 {
            self.remove(0);
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_null() {
            None
        } else {
            let res = unsafe {
                &mut*self.next
            };
            self.next = res.next.as_ptr();
            Some(&mut res.value)
        }
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.prev.is_null() {
            None
        } else {
            let res = unsafe {
                &mut*self.prev
            };
            self.prev = res.previous.as_ptr();
            Some(&mut res.value)
        }
    }
}

impl<T> Index<usize> for Circular<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let real_idx = index % self.size;
        self.value_at(real_idx)
    }
}

impl<T> IndexMut<usize> for Circular<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let real_idx = index % self.size;
        self.value_at_mut(real_idx)
    }
}

