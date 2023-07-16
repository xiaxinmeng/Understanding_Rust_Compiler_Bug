rust
>pub fn make_weird_raw_ptr() -> *const dyn Send {
>    unsafe { std::mem::transmute((1usize, 1usize)) }
>}
>