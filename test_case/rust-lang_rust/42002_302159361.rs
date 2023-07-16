rust
pub trait Read {
    unsafe fn is_trusted(&self) -> bool { false }
}
