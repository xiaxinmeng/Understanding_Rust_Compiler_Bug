 rust
use std::rc::Rc;

struct Test {
    children: [Option<Rc<Test>>; 8],
}

impl Test {
    fn get_child() -> Option<Rc<Test>> {
        None
    }
}
