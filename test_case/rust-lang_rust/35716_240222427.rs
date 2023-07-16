 rust
struct A{
    x: u8
}

fn main() {
    let x: *const u8 = 0 as *const u8;
    unsafe {
        let a = A{
            x: *x
        };
    }
}
