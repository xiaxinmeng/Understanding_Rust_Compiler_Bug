rust
use std::cell::RefCell;
use std::ops::DerefMut;

struct S<'a> {
    f: &'a mut dyn FnMut(),
}

fn take_f(_: &mut dyn FnMut()) {}

fn test<'a>(s: &RefCell<S<'a>>) {
    let mut guard = s.borrow_mut();

    // In function call context
    take_f(DerefMut::deref_mut(&mut guard).f); // Works
    take_f(guard.f); // Doesn't work
    
    // In plain expression context
    let s2 = S { f: DerefMut::deref_mut(&mut guard).f }; // Works
    let s2 = S { f: guard.f }; // Doesn't work
}

fn main() {
    let f = &mut || ();
    let s = RefCell::new(S { f });
    test(&s);
}
