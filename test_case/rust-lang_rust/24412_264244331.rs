rust
fn bit_str(bit: usize) -> String {
    let byte = bit >> 3;
    let lobits = 1 << (bit & 0b111); // <-- NB: b111
    format!("[{}:{}-{:02x}]", bit, byte, lobits)
}
