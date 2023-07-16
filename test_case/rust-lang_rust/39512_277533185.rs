rust
pub enum TryWaitError {
    Io(std::io::Error),
    WouldBlock,
}
