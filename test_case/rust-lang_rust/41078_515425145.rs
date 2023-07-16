rust
fn main() {
    let s = "";
    ((&|_| s) as &dyn Fn(&str) -> &str)(s);
}
