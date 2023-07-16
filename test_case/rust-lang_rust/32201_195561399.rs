 rust
extern {
    fn printf(_: *const u8, ...);
}

pub fn main() {
    unsafe {
        printf("'%s' has length %d\0".as_ptr(), "foobar\0");
    }
}
