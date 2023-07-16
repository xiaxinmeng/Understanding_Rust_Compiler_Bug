rust
fn main() {
    let mut x = vec![0u32];
    let px = &mut x as *mut _;

    let y = x;

    // unsafe { *px = vec![1]; } // Would drop old value in `x` that has been moved into `y`, see below.
    unsafe { std::ptr::write(px, vec![1]) }

   // `vec![1]` will leak, since `x` is not initialized according to `rustc`.
}

