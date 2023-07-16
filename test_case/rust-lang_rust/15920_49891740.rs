 rust
fn bad_code<'a>() -> &'a str {
    let x = "allocated string".to_string();
    unsafe { mem::transmute(x.as_slice()) }
}
