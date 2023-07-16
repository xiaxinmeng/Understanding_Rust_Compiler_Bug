 rust
#[no_mangle]
extern fn foo<T: Int>(a: T, b: T) -> T { a + b }

fn main() {
    assert_eq!(99u8, foo(255u8, 100u8));
    assert_eq!(99u16, foo(65535u16, 100u16));
}
