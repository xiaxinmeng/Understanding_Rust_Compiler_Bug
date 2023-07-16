 rust
fn main() {
    const C: u8 = 1;
    match 1 {
        C @ 2 => { println!("{}", C); }
        _ => {}
    };
}
