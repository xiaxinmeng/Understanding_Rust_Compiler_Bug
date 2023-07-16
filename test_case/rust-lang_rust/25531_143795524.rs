 rust
use std::rc::Rc;

fn main() {
    let x: Rc<[i32]> = Rc::new([1, 2, 3]);
}
