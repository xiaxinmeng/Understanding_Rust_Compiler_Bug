rust
fn main() {
    let f: &'static () = &loop {break};
}
