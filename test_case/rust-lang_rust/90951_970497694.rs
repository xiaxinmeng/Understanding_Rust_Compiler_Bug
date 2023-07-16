plain
[RUSTC-TIMING] build_script_build test:false 0.338
[RUSTC-TIMING] build_script_build test:false 0.534
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
[RUSTC-TIMING] rustc_std_workspace_core test:false 0.062
error: symbol `_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$11assume_init17h375e5b71c3542e12E` is already defined
    |
    |
627 |     pub const unsafe fn assume_init(self) -> T {

[RUSTC-TIMING] core test:false 28.419
error: could not compile `core` due to previous error
warning: build failed, waiting for other jobs to finish...
