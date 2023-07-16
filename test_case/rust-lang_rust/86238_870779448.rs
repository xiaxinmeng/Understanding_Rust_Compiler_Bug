rust
#![no_core]
#[lang = "sized"]
trait Sized {}
fn main() {
    match 0 {
        e if (|| e == 0)() => 1,
    }
}
