rust
use std::cell::RefCell;
use std::io::Write;

thread_local! {
    static FOO: RefCell<Option<Box<Write>>> = RefCell::new(None);
}

pub fn foo() -> bool {
    FOO.with(|c| c.borrow().is_some())
}
