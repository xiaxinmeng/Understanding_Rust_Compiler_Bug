 rust
use std::ptr;
use std::thread::Thread;

mod obscure {
    struct Obscure {
        ptr: *const ()
    }
}

struct A {
    inner: obscure::Obscure
}

impl A {
    fn new() -> A {
        A { inner: obscure::Obscure { ptr: ptr::null() } }
    }
}

fn main() {
    let me_bad = A::new();
    let me_ok = 0i32;
    Thread::spawn(|| {
        let _guess_where_it_is = (me_bad, me_ok);
    });
}
