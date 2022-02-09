use std::{rc::{Rc, Weak}, cell::{RefCell, Ref}};

type MRc<T> = Rc<RefCell<T>>;
type MWeak<T> = Weak<RefCell<T>>;

pub struct Circular<T> {
    size: usize,
    root: MRc<Root<T>>
}

struct Root<T> {
    first: Option<MRc<Node<T>>>,
    last: Option<MWeak<Node<T>>>,
}

enum Next<T> {
    Next(MRc<Node<T>>),
    Root(MWeak<Root<T>>),
}

enum Prev<T> {
    Prev(MWeak<Node<T>>),
    Root(MWeak<Root<T>>),
}

struct Node<T> {
    value: T,
    next: Next<T>,
    prev: Prev<T>,
}

impl<T> Circular<T> {
    pub fn new() -> Self {
        Circular {
            size:0,
            root: Rc::new(RefCell::new(Root {
                first: None,
                last: None,
            }))
        }
    }

    pub fn insert(&mut self, idx: usize, val: T) {
        assert!(idx <= self.size);
        let root = &self.root;
        let borrowed_root = root.borrow();

        if self.size == 0 {
            let node = Rc::new(RefCell::new(Node {
                value: val,
                next: Next::Root(Rc::downgrade(root)),
                prev: Prev::Root(Rc::downgrade(root)),
            }));
            let weak_ref = Rc::downgrade(&node);
            root.borrow_mut().first = Some(node);
            root.borrow_mut().last = Some(weak_ref);
        } else {
            todo!();
            let mut next: Ref<Node<T>>;
            let mut curr = borrowed_root.first.as_ref().unwrap().borrow();
            let mut i = idx;
            while i > 0 {
                next = match curr.next {
                    Next::Next(ref n) => n,
                    _ => panic!("size/pointer error"),
                }.borrow();
                curr = next;
            }
        }

        self.size += 1;
    }
}