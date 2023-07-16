rust
#[no_mangle]
pub static TEST: u64 = 0xdeadbeef;

pub fn main() {
    println!("{}", TEST);
}
