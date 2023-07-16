
fn fn_with_ref_arg(_s: &str) {}

fn main() {
    let s = "hello";
    let _b = fn_with_ref_arg(&s);
}
