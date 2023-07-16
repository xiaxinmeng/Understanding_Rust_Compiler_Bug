rust
static mut FOO: i32 = 4;
const FOO_MUT: &mut i32 = unsafe { &mut FOO };

fn main() { unsafe {
    FOO += 5;
    *FOO_MUT = 2;
} }
