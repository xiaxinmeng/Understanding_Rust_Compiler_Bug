plain
   Compiling synstructure v0.12.4
   Compiling tracing-attributes v0.1.13
   Compiling rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
   Compiling chalk-derive v0.55.0
error: identifier followed by `#` is a reserved prefix and cannot be used
    |
    |
947 |                 #name = #kind#value

error: aborting due to previous error


error: could not compile `tracing-attributes`
To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build --target=x86_64-unknown-linux-gnu --host=x86_64-unknown-linux-gnu --stage 2 library/std --rust-profile-generate=/tmp/rustc-pgo
Build completed unsuccessfully in 0:14:02
