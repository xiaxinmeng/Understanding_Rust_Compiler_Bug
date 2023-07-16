plain
2020-02-25T01:00:40.1203538Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-02-25T01:00:47.9204519Z    Compiling compiler_builtins v0.1.25
2020-02-25T01:00:54.6509984Z     Checking rustc-std-workspace-core v1.99.0 (/checkout/src/tools/rustc-std-workspace-core)
2020-02-25T01:00:57.0421962Z  Documenting alloc v0.0.0 (/checkout/src/liballoc)
2020-02-25T01:00:59.1671528Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:355:17
2020-02-25T01:00:59.2500206Z error: Could not document `alloc`.
2020-02-25T01:00:59.2500490Z 
2020-02-25T01:00:59.2500639Z Caused by:
2020-02-25T01:00:59.2500639Z Caused by:
2020-02-25T01:00:59.2503157Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name alloc src/liballoc/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="compiler-builtins-c"' --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc --generate-redirect-pages --resource-suffix 1.43.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-36783e0ef99b38ac.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcore-af6810a058aa653a.rmeta` (exit code: 1)
2020-02-25T01:00:59.2533732Z 
2020-02-25T01:00:59.2533732Z 
2020-02-25T01:00:59.2536091Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-Z" "unstable-options" "-p" "alloc" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "--resource-suffix" "1.43.0" "--index-page" "/checkout/src/doc/index.md"
2020-02-25T01:00:59.2537395Z 
2020-02-25T01:00:59.2537492Z 
2020-02-25T01:00:59.2546928Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
2020-02-25T01:00:59.2547389Z Build completed unsuccessfully in 1:35:01
2020-02-25T01:00:59.2547389Z Build completed unsuccessfully in 1:35:01
2020-02-25T01:00:59.2602707Z == clock drift check ==
2020-02-25T01:00:59.2625026Z   local time: Tue Feb 25 01:00:59 UTC 2020
2020-02-25T01:00:59.7736971Z   network time: Tue, 25 Feb 2020 01:00:59 GMT
2020-02-25T01:00:59.7739835Z == end clock drift check ==
2020-02-25T01:01:01.0444599Z 
2020-02-25T01:01:01.0518785Z ##[error]Bash exited with code '1'.
2020-02-25T01:01:01.0604593Z ##[section]Starting: Checkout rust-lang/rust@try to s
2020-02-25T01:01:01.0609430Z ==============================================================================
2020-02-25T01:01:01.0609790Z Task         : Get sources
2020-02-25T01:01:01.0610350Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
