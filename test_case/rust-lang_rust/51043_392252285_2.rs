rust
fn main() {
    match &5 {
        3..=6 => {}   // no &, no (), no whatever
        _ => {}
    }
}
