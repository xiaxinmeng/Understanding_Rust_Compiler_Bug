plain
    Finished release [optimized] target(s) in 12.39s
[TIMING] check::CodegenBackend { target: i686-pc-windows-gnu, backend: "cranelift" } -- 12.403
Checking stage0 gcc library (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
    Updating git repository `https://github.com/antoyo/gccjit.rs`
warning: spurious network error (2 tries remaining): failed to connect to github.com: Connection refused; class=Os (2)
warning: spurious network error (1 tries remaining): failed to connect to github.com: Connection refused; class=Os (2)
error: failed to get `gccjit` as a dependency of package `rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)`
Caused by:
Caused by:
  failed to load source for dependency `gccjit`
Caused by:
  Unable to update https://github.com/antoyo/gccjit.rs#fe242b7e

Caused by:
Caused by:
  failed to clone into: /cargo/git/db/gccjit.rs-13c2e290f2fb9e4d
Caused by:
  failed to connect to github.com: Connection refused; class=Os (2)
Build completed unsuccessfully in 0:01:39
