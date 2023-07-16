 Rust
const C: fn() = main;

fn foo(x: fn()) {
    match x {
        C => {}
        _ => {}
    }
}

fn main() {}
