
Documenting stage2 std (x86_64-apple-darwin)
    Finished release [optimized] target(s) in 0.0 secs
   Compiling cssparser v0.13.7
 Documenting rustc v0.0.0 (file:///./src/librustc)
error: dlopen(./build/x86_64-apple-darwin/stage1-rustc/release/deps/libcssparser_macros-7faf70d30b03767d.dylib, 1): Library not loaded: @rpath/libproc_macro-4a190002959d1e04.dylib
  Referenced from: ./build/x86_64-apple-darwin/stage1-rustc/release/deps/libcssparser_macros-7faf70d30b03767d.dylib
  Reason: image not found
  --> ~/.cargo/registry/src/github.com-1ecc6299db9ec823/cssparser-0.13.7/src/lib.rs:71:14
   |
71 | #[macro_use] extern crate cssparser_macros;
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: Could not compile `cssparser`.
warning: build failed, waiting for other jobs to finish...
