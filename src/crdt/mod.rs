extern crate libc;
use libc::{c_void, size_t, malloc, free};
use std::{mem, ptr};

mod clock;




pub struct List<T> {
    pub head: *mut Node<T>,
}

pub struct Node<T> {
    pub id: (u16, u32),
    pub data: T,
    pub next: *mut Node<T>,
}

pub struct Slice<T> {
    pub head:   *mut Node<T>,
    pub tail:   *mut Node<T>,
    pub cursor: *mut Node<T>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        unsafe {
            let mut head = malloc(mem::size_of::<Node<T>>()) as *mut Node<T>;
            List {
                head
            }
        }
    }

    pub fn insert(&mut self, data: T) {
        unsafe {
            let node = malloc(mem::size_of::<Node<T>>()) as *mut Node<T>;
            ptr::write(node, Node {
                data,
                next: ptr::null_mut(),
                id: (0, 0),
            });

            if self.head.is_null() {
                self.head = node;
            } else {
                (*node).next = (*self.head).next;
                (*self.head).next = node;
            }
        }
    }
}

impl <T>Slice<T> {
    pub fn new(head: *mut Node<T>, tail: *mut Node<T>) -> Slice<T> {
        Slice {
            head,
            tail,
            cursor: head,
        }
    }

    pub fn move_forward(&mut self) {
        if self.cursor != self.tail {
            unsafe {
                self.cursor = (*self.cursor).next;
            }
        }
    }

    pub fn move_from_start(&mut self, offset: usize) {
        self.cursor = self.head;
        for i in 0..offset {
            self.move_forward();
        }
    }
}
