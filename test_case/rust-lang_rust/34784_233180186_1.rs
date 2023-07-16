 Rust
const C: *const u8 = &0; //~ ERROR error: constant evaluation error: unimplemented constant expression: address operator [E0471]

fn foo(x: *const u8) {
    match x {
        C => {}
        _ => {}
    }
}

fn main() {}
