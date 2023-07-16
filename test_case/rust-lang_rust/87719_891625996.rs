plain
[RUSTC-TIMING] dissimilar test:false 3.094
   Compiling env_logger v0.9.0
[RUSTC-TIMING] tracing_core test:false 2.511
[RUSTC-TIMING] memmap2 test:false 0.667
   Compiling kqueue-sys v1.0.2
error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/kqueue-sys-1.0.2/src/lib.rs:68:20
   |
68 |             udata: ptr::null_mut(),
   |                    ^^^^^^^^^^^^^^^ expected `isize`, found *-ptr
   = note:     expected type `isize`
   = note:     expected type `isize`
           found raw pointer `*mut _`
For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
[RUSTC-TIMING] kqueue_sys test:false 0.246
error: could not compile `kqueue-sys` due to previous error
[RUSTC-TIMING] proc_macro2 test:false 1.432
[RUSTC-TIMING] proc_macro2 test:false 1.450
[RUSTC-TIMING] itertools test:false 2.916
[RUSTC-TIMING] crossbeam_utils test:false 1.967
---
[RUSTC-TIMING] memchr test:false 1.969
[RUSTC-TIMING] env_logger test:false 2.258
[RUSTC-TIMING] regex test:false 5.182
error: build failed
thread 'main' panicked at 'rust-analyzer always builds', src/bootstrap/dist.rs:1102:14
[TIMING] ToolBuild { compiler: Compiler { stage: 1, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-netbsd", file: None }, tool: "rust-analyzer", path: "src/tools/rust-analyzer/crates/rust-analyzer", mode: ToolRustc, is_optional_tool: true, source_type: Submodule, extra_features: [] } -- 7.259
[TIMING] RustAnalyzer { compiler: Compiler { stage: 1, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-netbsd", file: None }, extra_features: [] } -- 0.000
Build completed unsuccessfully in 0:25:07
