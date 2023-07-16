rust
fn my_text_file_with_crlf() -> &'static str {
    unsafe { 
        std::str::from_utf8_unchecked(&include_bytes!("my_text_file_with_crlf.txt")[..])
    }
}

#[test] 
fn my_text_file_is_acutualy_a_text_file() {
    std::str::from_utf8(&include_bytes!("my_text_file_with_crlf.txt")[..])
        .unwrap()
}
