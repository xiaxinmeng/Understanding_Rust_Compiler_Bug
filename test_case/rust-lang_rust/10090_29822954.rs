 rust
pub use Thing::Thing;

pub mod Thing {
    pub enum Thing {}
}

pub fn do_a_thing(thing: Thing) {}
