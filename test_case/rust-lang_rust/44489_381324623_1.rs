rust
    #[panic_implementation]
    fn my_panic_implementation(pi: &PanicInfo) -> ! {
        // body
    }
    
    // expands to
    
    #[lang="panic_implementation"]
    fn my_panic_implementation(pi: &PanicInfo) -> ! {
        // body
    }
    