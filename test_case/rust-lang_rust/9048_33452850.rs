 rust
fn main() {
    let mut rec_f = None;
    let f: &proc(uint) -> bool = |n: uint| { n == 0 || (*rec_f.get_ref())(n - 1) };
    rec_f = Some(f);
}
