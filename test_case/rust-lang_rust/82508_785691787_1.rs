rust
fn main() {
    let mut s = String::new();
    let v = {
        let res = ::alloc::fmt::format(...);
        res
    };
    s.push_str(&v);
}
