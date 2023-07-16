 Rust
mod m {
    use std::vec::Vec;
    mod n {
        type A = super::Vec<u8>;
    }
}
