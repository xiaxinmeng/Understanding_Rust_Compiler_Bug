rust
fn better_black_box<T>(mut x: T) -> T {
    unsafe {
        if mem::size_of::<T>() <= mem::size_of::<usize>() {
            let man_x = mem::ManuallyDrop::new(x);
            let mut int_x: usize = mem::transmute_copy(&man_x);
            asm!("/* {x} */", x = inout(reg) int_x, options(nostack));
            x = mem::transmute_copy(&int_x);
        } else {
            asm!("/* {x} */", x = in(reg) &mut x);
        }
    }
    x
}
