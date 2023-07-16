
; x build --stage 0 compiler
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized + debuginfo] target(s) in 10m 55s

; ls build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/{librustc_driver.so,rustc-main} -lh
-rwxrwxr-x 4 jnelson jnelson 529M Nov 28 13:36 build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/librustc_driver.so
-rwxrwxr-x 3 jnelson jnelson  22K Nov 28 13:36 build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/rustc-main
