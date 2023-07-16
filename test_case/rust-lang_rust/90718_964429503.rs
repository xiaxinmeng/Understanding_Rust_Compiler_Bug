rust
fn foo<'a>(_: &'a i32) -> &'a i32 {
    let y = 42;
    let x: &'a i32;

    let px = std::ptr::addr_of_mut!(x);
    unsafe {
        std::ptr::write(px, &y);
        std::ptr::read(px)
    }
}
