rust
#[inline(never)]
#[no_mangle]
fn ◌() {
    println!("hi there!");
}

fn main() {
    ◌();
}
