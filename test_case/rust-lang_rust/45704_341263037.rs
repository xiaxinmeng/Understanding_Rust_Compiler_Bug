Rust
fn cplusplus_mode(x: isize) -> &'static isize { &x }

fn cplusplus_mode_exceptionally_unsafe(x: &mut Option<&'static mut isize>) {
    let z = 0;
    *x = Some(&mut z);
    panic!("catch me for a dangling pointer!")
}
