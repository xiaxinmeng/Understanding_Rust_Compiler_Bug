Rust
extern "C" {
    #[rustc_args_required_const(0, 1)]
    pub fn print_raw(data: *const u8, len: usize);
}

// later...
print_raw(b"hello".as_ptr(), 5);
