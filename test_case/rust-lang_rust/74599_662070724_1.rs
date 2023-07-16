rust
extern "C" {
    #[rustc_args_required_const(0, 1)]
    pub fn print_raw(data: *const u8, len: usize);
}
