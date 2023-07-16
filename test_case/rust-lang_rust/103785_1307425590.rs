sh
~/devspace/personal/rust-dist/library-master  $ ../../rust/x doc --stage 0 std --json && sha256sum build/x86_64-unknown-linux-gnu/json-doc/*
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.06s
Documenting stage0 std (x86_64-unknown-linux-gnu) in JSON format
 Documenting core v0.0.0 (/home/nimda/devspace/personal/rust/library/core)
    Finished release [optimized] target(s) in 7.34s
   Compiling compiler_builtins v0.1.82
 Documenting alloc v0.0.0 (/home/nimda/devspace/personal/rust/library/alloc)
    Finished release [optimized] target(s) in 3.14s
   Compiling compiler_builtins v0.1.82
   Compiling memchr v2.5.0
   Compiling unwind v0.0.0 (/home/nimda/devspace/personal/rust/library/unwind)
   Compiling std v0.0.0 (/home/nimda/devspace/personal/rust/library/std)
    Checking alloc v0.0.0 (/home/nimda/devspace/personal/rust/library/alloc)
    Checking cfg-if v1.0.0
    Checking adler v0.2.3
    Checking rustc-demangle v0.1.21
    Checking rustc-std-workspace-alloc v1.99.0 (/home/nimda/devspace/personal/rust/library/rustc-std-workspace-alloc)
    Checking panic_abort v0.0.0 (/home/nimda/devspace/personal/rust/library/panic_abort)
    Checking panic_unwind v0.0.0 (/home/nimda/devspace/personal/rust/library/panic_unwind)
    Checking gimli v0.25.0
    Checking hashbrown v0.12.3
    Checking miniz_oxide v0.4.0
    Checking std_detect v0.1.5 (/home/nimda/devspace/personal/rust/library/stdarch/crates/std_detect)
    Checking object v0.26.2
    Checking addr2line v0.16.0
 Documenting std v0.0.0 (/home/nimda/devspace/personal/rust/library/std)
    Finished release [optimized] target(s) in 7.13s
Build completed successfully in 0:00:18
b4b090b23a1ca1d7b870fbf2c9d3827ffe9aa7b2fdb17faf34e7a64852c141c9  build/x86_64-unknown-linux-gnu/json-doc/alloc.json
83db7b1380a39c886640e0da0fd2590264cc27ca40ec9bdda8e0b4c02fcc3586  build/x86_64-unknown-linux-gnu/json-doc/core.json
e789adebf4ca15850e9c0b3cd3c9126d48cd7ea9e1ae272339e692423009f353  build/x86_64-unknown-linux-gnu/json-doc/std.json
