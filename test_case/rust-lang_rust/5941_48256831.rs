 rust
macro_rules! guard {
    ($thing: expr) => { let _guard = Guarder::new($thing); }
}
