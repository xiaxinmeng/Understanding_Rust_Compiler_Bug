rust
/// I cannot comment it here!
pub use libc::termios as Termios

/// But this works fine!
pub type Termios = libc::termios;
