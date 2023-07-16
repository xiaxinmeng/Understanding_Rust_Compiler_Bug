sh
~/devspace/personal/rust-dist/library-master  $ ../../rust/x doc --stage 0 std --json && sha256sum build/x86_64-unknown-linux-gnu/json-doc/*
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.04s
Documenting stage0 std (x86_64-unknown-linux-gnu) in JSON format
 Documenting core v0.0.0 (/home/nimda/devspace/personal/rust/library/core)
    Finished release [optimized] target(s) in 7.20s
   Compiling compiler_builtins v0.1.82
 Documenting alloc v0.0.0 (/home/nimda/devspace/personal/rust/library/alloc)
    Finished release [optimized] target(s) in 3.17s
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
    Checking std_detect v0.1.5 (/home/nimda/devspace/personal/rust/library/stdarch/crates/std_detect)
    Checking object v0.26.2
    Checking miniz_oxide v0.4.0
    Checking hashbrown v0.12.3
    Checking addr2line v0.16.0
 Documenting std v0.0.0 (/home/nimda/devspace/personal/rust/library/std)
    Finished release [optimized] target(s) in 6.77s
Build completed successfully in 0:00:17
07d31750fe73615c800498041adf20e485b86ae6b15860f9fea6c3b0cedcf884  build/x86_64-unknown-linux-gnu/json-doc/alloc.json
c232267e96f186971622d46a55b0471ccef78dba85fe9ad47a43392629b9c5c1  build/x86_64-unknown-linux-gnu/json-doc/core.json
81b83dd44026f5a3705f48e8be5f0f04ceb7ea14e566bd46939882f11868efc7  build/x86_64-unknown-linux-gnu/json-doc/std.json
