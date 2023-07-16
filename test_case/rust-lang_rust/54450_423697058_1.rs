rust
pub static symbol: *const c_char = {
    extern "C" {
        #[link_name = "symbol"]
        static inner: (); // or some more suitable dummy type
    }
    &inner as *const () as *const c_char
};
