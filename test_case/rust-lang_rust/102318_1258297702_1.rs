rust
pub mod __alloc_error_handler {
    use crate::alloc::Layout;

    // called via generated `__rust_alloc_error_handler`

    // if there is no `#[alloc_error_handler]`
    #[rustc_std_internal_symbol]
    pub unsafe fn __rdl_oom(size: usize, _align: usize) -> ! {
        panic!("memory allocation of {size} bytes failed")
    }

    // if there is an `#[alloc_error_handler]`
    #[rustc_std_internal_symbol]
    pub unsafe fn __rg_oom(size: usize, align: usize) -> ! {
        let layout = unsafe { Layout::from_size_align_unchecked(size, align) };
        extern "Rust" {
            #[lang = "oom"]
            fn oom_impl(layout: Layout) -> !;
        }
        unsafe { oom_impl(layout) }
    }
}
