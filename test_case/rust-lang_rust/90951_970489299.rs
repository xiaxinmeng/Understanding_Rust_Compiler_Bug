plain
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
error: symbol `_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$11assume_init17h89fc0bff3de614f3E` is already defined
    |
    |
627 |     pub const unsafe fn assume_init(self) -> T {

error: could not compile `core` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
