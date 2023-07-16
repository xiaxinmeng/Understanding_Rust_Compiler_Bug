 rust
pub static SELF_PTR: usize = &SELF_PTR as *const _ as usize;

let real_to_virt = SELF_PTR - (&SELF_PTR as *const _ as usize);
