rust
fn main() {
    macro_rules! a {
        ($e:expr) => { $e; }
    }
    a!(true);
}
