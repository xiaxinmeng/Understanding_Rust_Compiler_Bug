plain
   Compiling rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
error: unused variable: `new_hash`
    --> compiler/rustc_span/src/hygiene.rs:1320:13
     |
1320 |         let new_hash: Fingerprint = hasher.finish();
     |             ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_new_hash`
     |
     = note: `-D unused-variables` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustc_span`


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build --target=x86_64-unknown-linux-gnu --host=x86_64-unknown-linux-gnu --stage 2 library/std --rust-profile-generate=/tmp/rustc-pgo
Build completed unsuccessfully in 0:08:23
