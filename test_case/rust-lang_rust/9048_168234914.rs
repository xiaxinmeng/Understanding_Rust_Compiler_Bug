
fn main() {
    let mut rec_f = None;
    let f = |n: usize| { n == 0 || (*rec_f.as_ref())(n - 1) };
    rec_f = Some(f);
}
