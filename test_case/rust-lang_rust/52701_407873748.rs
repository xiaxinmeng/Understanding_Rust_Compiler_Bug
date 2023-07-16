Rust
fn rec() -> *const impl Copy {
    // This should *NEVER* run, but just to trigger the recursion
    if false {
        rec()
        
    } else {

        0u8 as *const _ // cast is needed to make compiler happy about the types
    }
}

fn main() {
    rec();
}
