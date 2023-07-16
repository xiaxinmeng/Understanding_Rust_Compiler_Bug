 rust
for byte in &mut (bytes: Vec<u8>) {
    *byte = byte.to_ascii_lowercase()
}

for byte in unsafe { (s: String).as_mut_vec() } {
    *byte = byte.to_ascii_lowercase()
}
