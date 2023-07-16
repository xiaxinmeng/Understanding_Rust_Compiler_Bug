rust
    let magic_bytes = header.magic.to_ne_bytes();
    if magic_bytes != *b"hpkr" {
