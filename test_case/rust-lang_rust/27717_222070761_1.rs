 rust
lazy_static! {
    pub static ref LOCK: Mutex<()> = Mutex::new(());
}
