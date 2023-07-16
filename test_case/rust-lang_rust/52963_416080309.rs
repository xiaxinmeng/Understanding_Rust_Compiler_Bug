rust
let bytes = if cfg!(target_endian = "little") {
    x.to_le_bytes()
} else {
    x.to_be_bytes()
};
