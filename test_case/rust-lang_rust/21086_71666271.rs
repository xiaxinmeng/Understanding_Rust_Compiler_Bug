 rust
#[allow(unstable)]
extern crate libc;
use libc::{c_void, c_int};

struct State;

trait Push {
    fn push_to(state: &mut State, value: Self);
}

type CFun = extern fn(s: *mut c_void) -> c_int;
impl Push for CFun {
    fn push_to(_state: &mut State, _value: CFun) {
        // do something
    }
}

fn main() {
    extern fn foo(_s: *mut c_void) -> c_int { 1 }
    let mut st = State;
    Push::push_to(&mut st, foo);  // error: trait `Push` not implemented for ...
    Push::push_to(&mut st, foo as CFun);  // works!
}
