rust
const FOO: ! = panic!();
fn main() {
    if false {
        let _ = FOO;
    }
}
