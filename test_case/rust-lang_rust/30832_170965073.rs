 rust
use std::rc::Rc;
use std::cell::RefCell;

fn foo(a: &mut u32) -> Option<()> {
    if *a < 10 { *a += 1; Some(()) } else { None }
}

fn main() {
    let a = Rc::new(RefCell::new(4));
    loop {
        match foo(&mut *a.borrow_mut()) {
        // works:
        // match { let mut b = a.borrow_mut(); foo(&mut *b) } {
            Some(()) => { println!("{}", *a.borrow()); }
            None => break,
        }
    };
}
