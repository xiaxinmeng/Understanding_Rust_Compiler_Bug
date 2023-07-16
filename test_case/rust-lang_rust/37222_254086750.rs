 rust
#![feature(core_intrinsics)]

use std::{mem, intrinsics};

pub struct Binding {
    action: Action,
    mode: u8,
}

pub enum Action {
    Esc(i64),
    Char(u32),
    Paste,
}

static V_BINDINGS: &'static [Binding] = &[
    Binding { action: Action::Paste, mode: 0xff },
    Binding { action: Action::Paste, mode: 0xff },
];

fn main() {
    let first = unsafe { intrinsics::discriminant_value(&V_BINDINGS[0].action) };
    let second = unsafe { intrinsics::discriminant_value(&V_BINDINGS[1].action) };

    // Edit: added the next three asserts.
    assert_eq!(mem::size_of::<Action>(), 16);
    assert_eq!(mem::size_of::<Binding>(), 24);
    assert_eq!(&V_BINDINGS[1] as *const _ as usize - &V_BINDINGS[0] as *const _ as usize, 24); 

    // Edit: added hexdump
    let start = &V_BINDINGS[0] as *const _ as *const u8;
    for i in 0 .. 48 {
        print!("{:02X} ", unsafe { *start.offset(i) });
        if i % 24 == 23 { println!(""); }
    }

    assert_eq!(first, 2);
    assert_eq!(second, 2); // <-- This fails
}
