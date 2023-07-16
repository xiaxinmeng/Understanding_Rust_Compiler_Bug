rust
use std::collections::HashMap;

struct Def;

struct DefThings {
    things: HashMap<u32, Vec<Def>>
}

impl DefThings {
    fn get_thing(&self, id: u32) -> Option<Vec<Def>> {
        self.things.get(&id)
    }
}
