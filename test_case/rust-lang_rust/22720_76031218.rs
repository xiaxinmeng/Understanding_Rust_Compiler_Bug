 rust
use std::rc::Rc;

fn main() {
    match Rc::new(1) {
        Rc<_> => {}
    }
}
