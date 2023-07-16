rust
macro_rules! foo {
    () => { 0 }
}

fn main() {
    let x = foo!() as *const [u8];
}
