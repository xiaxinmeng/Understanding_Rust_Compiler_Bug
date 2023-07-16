ignore ` tests, divided into 6 categories:

    * code demo which looks better with syntax highlighting only (10 instances)
    * private modules (6 instances)
    * doc test with `include!(...)` (3 instances)
    * doc test with macro exporting across multiple crates (6 instances)
    * `compile_fail` cannot check SIMD failure (1 instance)
    * doc test cannot "fail on Linux/Windows/etc but pass on macOS" (1 instance)

2. There are 8 error codes which are no longer emitted. I just added
    `#### Note: this error code is no longer emitted by the compiler.`, but I'm 
    not sure if they should be removed instead. 

    * E0073 (indirectly recursive struct)
    * E0074 (`#[simd]` on generic tuple struct)
    * E0082 (C-like enum overflows — apparantly merged into E0080)
    * E0139 (transmute to wrapper type)
    * E0193 (`where` clause without generic type parameters)
    * E0398 (`&Box<Trait>` means `&Box<Trait+'static>` after Rust 1.3)
    * E0447 (`pub` inside function)
    
    E0329 (associated const of generic parameter) is just commented out since the description explicitly said "This is not supported yet".

