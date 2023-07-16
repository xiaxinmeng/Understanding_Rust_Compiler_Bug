
; x build --stage 0 compiler
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 8m 13s

; ls build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/{librustc_driver.so,rustc-main} -lh
-rwxrwxr-x 4 jnelson jnelson 115M Nov 28 13:20 build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/librustc_driver.so
-rwxrwxr-x 3 jnelson jnelson  17K Nov 28 13:20 build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/rustc-main
