#![feature(unsafe_destructor)]
#![feature(libc)]
extern crate libc;
mod crdt;
use crdt::{List, Slice};

mod text;

fn main() {
    println!("Hello, world!");
    let mut list: List<u8> = List::new();
    list.insert(3u8);
    list.insert(4u8);
    list.insert(5u8);
    unsafe {
        println!("OUTPUT1: {:?}", (*list.head).data.to_string());
        println!("OUTPUT2: {:?}", (*(*list.head).next).data.to_string());
        println!("OUTPUT3: {:?}", (*(*(*list.head).next).next).data.to_string());
        (*(*(*list.head).next).next).data = 13u8;
        println!("OUTPUT4: {:?}", (*(*(*(*list.head).next).next).next).data.to_string());
        println!("OUTPUT3: {:?}", (*(*(*list.head).next).next).data.to_string());

        let mut slice = Slice::new(list.head, (*(*(*list.head).next).next).next);
        slice.move_from_start(3);
        (*slice.cursor).data = 33u8;
        println!("OUTPUT4: {:?}", (*(*(*(*list.head).next).next).next).data.to_string());
    }
}
