rust
fn main() {
    macro_rules! a {
        ($e:expr) => { $e; }
    }
    let _val = a!(true);
}
