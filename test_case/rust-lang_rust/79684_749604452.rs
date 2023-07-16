rust
#[test]
fn const_read() {
    let foo = 42;
    assert_eq!(read_ptr(&foo), foo);

    const fn read_ptr(x: &i32) -> i32 {
        // SAFETY: x is a reference which is garantueed to be safe to read from
        // thus the same goes for the pointer
        unsafe{ read(x as *const i32) }
    }
}
