
#[deriving(Show)] // error: conflicting implementations for trait `core::fmt::Show`
struct u16;

fn main() {
    let a: u16 = u16; // error: mismatched types `u16` (expected u16 but found struct u16)
    let b = u16; // compiles and is of type 'struct u16'
}
