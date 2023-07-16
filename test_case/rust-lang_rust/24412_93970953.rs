
fn bit_str(bit: usize) -> String {
    let byte = bit >> 8;
    let lobits = 1 << (bit & 0xFF);
    format!("[{}:{}-{:02x}]", bit, byte, lobits)
}
