rust
pub static mut BAA: *const i8 = unsafe { &BOO as *const _ as *const i8 };
pub static mut BOO: *const i8 = unsafe { &BAA as *const _ as *const i8 };
fn main() {}
