 Rust
fn f(x: Option<u32>) {
    const C: Option<u32> = None;
    match x {
        C @ Some(x) => { println!("{}", x); }
        _ => {}
    }
}
fn main() { f(None) }
