 rust
fn main() {
    struct S { f: i32 };
    let z = S { f: 0 };
    match 0 { _ => &z }.f   // <-- note the lack of semicolon
    match 0 { _ => &z }.f   // <-- note the lack of semicolon
    ()
}
