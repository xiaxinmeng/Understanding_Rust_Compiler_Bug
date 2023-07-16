 rust
extern mod extra;
use std::cell::Cell;
use extra::rc::Rc;

fn is_freeze<T: Freeze>(_: T) {}

fn main() {
    // is_freeze(Cell::new(1)); // error: [...] Cell [...] does not fulfill Freeze

   is_freeze(Rc::from_send(Cell::new(1))); // compiles fine
}
