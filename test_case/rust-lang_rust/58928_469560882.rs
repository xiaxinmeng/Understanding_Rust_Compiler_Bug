rust
fn main() {
    // The tool module also resolves uniformly in this context
    let x = rustfmt; // ERROR expected value, found tool module `rustfmt`
    let y = rustfmt::skip; // ERROR expected value, found tool attribute `rustfmt::skip`
}
