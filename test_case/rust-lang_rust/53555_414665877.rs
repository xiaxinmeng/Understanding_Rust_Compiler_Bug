rust
union Trans { x: &'static i32, y: usize }
const BAR : usize = unsafe { Trans { x: &0 }.y };
