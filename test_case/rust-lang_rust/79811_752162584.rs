plain
   Compiling rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
error[E0282]: type annotations needed
    --> compiler/rustc_span/src/hygiene.rs:1326:13
     |
1326 |         let new_hash = hasher.finish();
     |             ^^^^^^^^ consider giving `new_hash` a type
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
error: could not compile `rustc_span`
error: could not compile `rustc_span`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build --target=x86_64-unknown-linux-gnu --host=x86_64-unknown-linux-gnu --stage 2 library/std --rust-profile-generate=/tmp/rustc-pgo
Build completed unsuccessfully in 0:08:41
