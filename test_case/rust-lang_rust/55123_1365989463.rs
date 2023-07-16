rust
enum Empty {}

fn foo(x: &Empty) {
    match x {} // ERROR: missing match arm.
}
