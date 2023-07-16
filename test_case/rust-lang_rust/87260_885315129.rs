plain
    Checking cranelift-object v0.75.0 (https://github.com/bytecodealliance/wasmtime.git?branch=main#c71ad949)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
    Finished release [optimized] target(s) in 27.63s
Checking stage0 gcc artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Updating git repository `https://github.com/antoyo/gccjit.rs`
---
   Compiling target-lexicon v0.10.0
    Checking libc v0.1.12
    Checking ar v0.8.0
   Compiling indexmap v1.6.2
    Checking gccjit_sys v0.0.1 (https://github.com/antoyo/gccjit.rs#c736ae6c)
    Checking gccjit v1.0.0 (https://github.com/antoyo/gccjit.rs#c736ae6c)
    Checking object v0.25.3
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
Checking stage0 clippy artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
   Compiling target-lexicon v0.10.0
    Checking libc v0.1.12
    Checking ar v0.8.0
   Compiling indexmap v1.6.2
    Checking gccjit_sys v0.0.1 (https://github.com/antoyo/gccjit.rs#c736ae6c)
    Checking gccjit v1.0.0 (https://github.com/antoyo/gccjit.rs#c736ae6c)
error[E0308]: mismatched types
   --> /cargo/git/checkouts/gccjit.rs-13c2e290f2fb9e4d/c736ae6/src/context.rs:388:80
    |
388 |             let ptr = gccjit_sys::gcc_jit_type_get_vector(types::get_ptr(&ty), num_units);
    |                                                                                ^^^^^^^^^ expected `u32`, found `u64`
    |
help: you can convert a `u64` to a `u32` and panic if the converted value doesn't fit
    |
388 |             let ptr = gccjit_sys::gcc_jit_type_get_vector(types::get_ptr(&ty), num_units.try_into().unwrap());

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> /cargo/git/checkouts/gccjit.rs-13c2e290f2fb9e4d/c736ae6/src/context.rs:732:72
732 | ...                   value);
    |                       ^^^^^ expected `i32`, found `i64`
    |
    |
help: you can convert an `i64` to an `i32` and panic if the converted value doesn't fit
732 |                                                                        value.try_into().unwrap());
    |                                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 2 previous errors
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
error: could not compile `gccjit`
To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:02:02
