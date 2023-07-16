
unsafe fn foo(x: *T) {
    let y = unsafe::reinterpret_cast::<int>(x);
    32 >> y
}
