rust
fn main() {
    let s = "";
    let f: &dyn Fn(&str) -> &str = &|_| s;
    f(s);
}
