rust
#[repr(C)]
struct Foo<T> {
    bar: T,
    baz: usize,
}

/// # Safety
/// This function is unsafe because it extends the calling convention.
/// The caller of this function MUST put the value of `baz` in `r12` before calling it.
unsafe extern "C" fn qux<T>(foo: &mut Foo<T>) {
    asm!("mov {}, r12", in(mem) &foo.bar);
}
