
; x b rustc_span --stage 0
Building stage0 library artifacts (x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.07s
Building compiler artifacts {rustc_span} (stage0 -> stage1, x86_64-unknown-linux-gnu)
   Compiling rustc_span v0.0.0 (/home/jyn/src/rust/compiler/rustc_span)
    Finished release [optimized + debuginfo] target(s) in 0.87s
; touch library/test/src/lib.rs 
; x b rustc_span --stage 0
Building stage0 library artifacts (x86_64-unknown-linux-gnu)
   Compiling test v0.0.0 (/home/jyn/src/rust/library/test)
   Compiling sysroot v0.0.0 (/home/jyn/src/rust/library/sysroot)
    Finished release [optimized] target(s) in 0.30s
Building compiler artifacts {rustc_span} (stage0 -> stage1, x86_64-unknown-linux-gnu)
    Finished release [optimized + debuginfo] target(s) in 0.09s
