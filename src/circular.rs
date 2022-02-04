use std::{marker::PhantomData, alloc, ptr::null_mut};

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
        let layout = alloc::Layout::new::<Node<T>>();
        let ptr = unsafe {
            alloc::alloc(layout) as *mut Node<T>
        };

        if self.size > 0 {
            let old_root = self.first;
            unsafe {
                let bottom = (*old_root).previous;
                std::ptr::write(ptr, Node {
                    value: val,
                    next: old_root,
                    previous: bottom,
                });
                let previous_of_old_root = &mut (*old_root).previous as *mut *mut Node<T>;
                std::ptr::write(previous_of_old_root, ptr);
                let next_of_bottom = &mut (*bottom).next as *mut *mut Node<T>;
                std::ptr::write(next_of_bottom, ptr);
            }
        } else {
            unsafe {
                std::ptr::write(ptr, Node {
                    value: val,
                    next: ptr,
                    previous: ptr,
                });
            }
        }

        self.size += 1;
        self.first = ptr;
    }

}