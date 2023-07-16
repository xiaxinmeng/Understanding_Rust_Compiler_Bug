rust
fn main() {
    let mut x = 10;
    unsafe {
        let y = &mut x;
    }
    let z = &x;
}

