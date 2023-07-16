rust
macro_rules! repro {
    () => {
        #[allow(unused_must_use)] { 0 }
    };
}


fn f() {
    let _ = repro!();
}
