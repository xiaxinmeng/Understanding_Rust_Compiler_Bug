Rust
fn main() {
    let product = "macos";
    let is_not = |s| !product.eq_ignore_ascii_case(s);

    let _res = is_not(&"windows".to_string()) && is_not(&"macos".to_string());
}
