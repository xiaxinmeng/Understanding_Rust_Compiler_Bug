 shell
$ DYLD_LIBRARY_PATH=../rust-build/x86_64-apple-darwin/stage1/lib \
  ../rust-build/x86_64-apple-darwin/stage1/bin/rustc -C codegen-units=2 \
  src/test/compile-fail/asm-src-loc-codegen-units.rs
thread 'codegen-1' panicked at 'Box<Any>', /Users/tamird/src/rust/src/libsyntax/diagnostic.rs:192
<rustc is hung here>
