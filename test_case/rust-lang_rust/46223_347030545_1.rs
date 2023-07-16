rust
macro_rules! make_ref {
    ($ptr:expr) => { &*$ptr };
}
