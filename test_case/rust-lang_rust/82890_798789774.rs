rust
fn main() {
    let data_ref: &[u8] = &[0u8];  // or `&[b'a']` or something

    let username_str: &str = std::str::from_utf8(data_ref).unwrap();
    println!("{:?}", username_str);
}
