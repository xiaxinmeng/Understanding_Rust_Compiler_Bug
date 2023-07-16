 Rust
#![feature(unboxed_closures,core)]

use std::cell::Cell;

struct B<'a, F: FnOnce()+'a>(Cell<Option<&'a F>>);

struct Closure<'a> {
    arg: B<'a, Closure<'a>>
}

impl<'a> FnOnce<()> for Closure<'a> {
    type Output = ();
    extern "rust-call" fn call_once(self, args: ()) {}
}

fn main() {
 let (mut p, mut cls);
 p = B(Cell::new(None));
 cls = Closure { arg: p };
 cls.arg.0.set(Some(&cls));
}
