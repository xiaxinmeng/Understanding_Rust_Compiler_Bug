rust
// a.rs
pub const FOO: *const i8 = 0 as *const i8;

pub static mut BAR: [*const i8; 4] = [FOO, FOO, FOO, FOO];

pub static mut BAA: (*const i8, i32, *const i8) = (
    unsafe { &BAR as *const _ as *const i8 },
    42,
    unsafe { &BOO as *const _ as *const i8 },
);
pub static mut BOO: (*const i8, *const i8, u32) = (
    unsafe { &BAA as *const _ as *const i8 },
    &FOO as *const _ as *const i8,
    99,
);

// b.rs
extern crate a;

static mut BOO: *const i32 = unsafe { &a::BOO as *const _ as *const i32 };

fn main() {}
