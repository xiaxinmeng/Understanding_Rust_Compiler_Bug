rust
struct S {}

fn main() {
    let s = S {};
    let z = S { #[nonexistent] ..s }; // error: expected identifier, found `..`
}
