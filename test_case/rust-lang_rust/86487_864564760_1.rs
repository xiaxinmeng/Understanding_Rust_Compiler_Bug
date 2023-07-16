rust
macro_rules! dummy_of_typeof {( $e:expr $(,)? ) => ({
    let mut it = ::core::option::Option::None.expect("dummy_of_typeof!");
    if false { // help type inference
        let _ = async { it = $e; };
        loop {}
    }
    it
})}
