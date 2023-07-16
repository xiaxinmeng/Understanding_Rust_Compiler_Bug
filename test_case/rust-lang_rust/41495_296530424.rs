rust
#![allow(dead_code)]
use std::cell::RefCell;

fn passes_compile() {
    let container = RefCell::new(0);
    if container.try_borrow().is_ok() { }
}

fn fails_compile_if_let_true() {
    let container = RefCell::new(0);
    if let true = container.try_borrow().is_ok() { }
}

fn fails_compile_match() {
    let container = RefCell::new(0);
    match container.try_borrow().is_ok() {
        true => (),
        _ => ()
    }
}
