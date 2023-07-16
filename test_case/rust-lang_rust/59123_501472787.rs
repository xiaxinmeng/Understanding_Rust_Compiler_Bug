rust
macro_rules! drop_with_guaranteed_move_elision {
    ($val:ident) => {
        unsafe {
            std::ptr::drop_in_place(&mut $val);
            std::mem::forget($val);
        }
    }
}

fn main() {
    let mut x = "foo".to_string(); // errors if `x` isn't `mut`
    drop_with_guaranteed_move_elision!(x);
}
