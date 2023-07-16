rust
static FOO: MyAllocator = ...;

mod some_internal_name {
    #[std_special_symbol]
    #[no_mangle]
    fn __rg_alloc(...) -> ... {
        FOO.some_method(...)
    }

    ...
}
