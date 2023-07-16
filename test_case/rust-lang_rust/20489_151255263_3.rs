 rust
fn bar(f: x::Foo) {
    // rustc knows this is exhaustive because if expanded `V1` into equivalent
    // patterns; patterns you could not write by hand!
    match f {
        x::V1 => { }
        x::V2 => { }
    }
}
