rust
    extern {
         #[lang = "panic_implementation"]
         fn panic_impl(pi: &PanicInfo) -> !;
    }
    
    /// Constructs PanicInfo and calls the `panic_implementation`.
    fn panic_payload<M: Any + Send>(msg: M, file_line_col: &(&'static str, u32, u32)) -> ! {
        panic_impl(&...)
    }
    