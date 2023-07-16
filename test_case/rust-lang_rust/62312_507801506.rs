rust
fn main() {
    let _ = loop {
        break Box::new(()) as Box<dyn Send>;
    };
}
