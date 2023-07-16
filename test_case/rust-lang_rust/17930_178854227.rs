 rust
fn main() {
    unsafe { 0 as i32 };  // ok
    unsafe { 0 } as i32;  // expected identifier, found keyword `as`
    (unsafe { 0 }) as i32;  // ok
}
