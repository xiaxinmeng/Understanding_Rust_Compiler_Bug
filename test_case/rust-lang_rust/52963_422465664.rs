rust
    let magic_bytes = header.magic.to_ne_bytes();
    if magic_bytes != ['h', 'p', 'k', 'r'] {
