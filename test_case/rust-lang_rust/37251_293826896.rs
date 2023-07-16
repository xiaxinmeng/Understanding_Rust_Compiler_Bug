
extern {
    #[link_name = "llvm.prefetch"]
    fn prefetch(address: *const i8, rw: i32, locality: i32, cache_type: i32);
}
