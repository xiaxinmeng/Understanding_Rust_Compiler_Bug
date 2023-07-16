Rust
fn main() {
    let product = "macos";
    let is_not = |s: &str| !product.eq_ignore_ascii_case(s);

    let _res = is_not(obfstr::obfstr!("windows")) && is_not(obfstr::obfstr!("macos"));
}
