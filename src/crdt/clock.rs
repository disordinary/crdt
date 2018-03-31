struct Clock {
    actor: u16,
    id: u32,
}

impl Clock {
    pub fn new(actor: u16, id: u32) -> Clock {
        Clock {
            actor,
            id,
        }
    }
    pub fn get(&mut self) -> (u16, u32) {
        let item = (self.actor, self.id);
        self.id = self.id + 1;
        item
    }
}