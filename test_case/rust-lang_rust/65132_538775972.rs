rust
pub fn main() -> Result<(), &'static str> {
    let v = std::collections::VecDeque::<u8>::with_capacity(127);
    match v.capacity() {
        127 => Ok(()),
        _ => Err("unexpected capacity")
    }
}
