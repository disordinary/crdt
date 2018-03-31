
use crdt::{List, Slice};



pub struct Text {
    text: List<u8>,
    // formats
}


impl Text {
    pub fn new() -> Text {
        Text {
            text: List::new(),
        }
    }
}