rust
let exit_code = if let Some(code) = status.code() {
    code as u8
} else {
    status.signal().unwrap() as u8 + 128u8
};
