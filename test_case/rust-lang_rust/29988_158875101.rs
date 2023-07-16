 rust
fn foo() -> (i64, i64) {
    let x = (0i32, 0i32, 0i32);
    let y = &x as *const _ as *const (i64, i64);
    unsafe {
        *y
    }
}

fn main() {
    println!("foo() = {:?}", foo());
}
