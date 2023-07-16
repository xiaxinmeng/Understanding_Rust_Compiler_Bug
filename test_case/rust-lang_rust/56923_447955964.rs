rust
fn type_code(switch: bool) -> u8 {
    (if switch { 128 } else { 0 }) + base_type_code()
}
