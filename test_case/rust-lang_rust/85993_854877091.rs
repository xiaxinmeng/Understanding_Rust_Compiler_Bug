plain
   Compiling rls-data v0.19.1
error: unused variable: `d`
    --> compiler/rustc_ast/src/ast.rs:2367:15
     |
2367 |     fn decode(d: &mut D) -> Result<AttrId, D::Error> {
     |               ^ help: if this is intentional, prefix it with an underscore: `_d`
     |
     = note: `-D unused-variables` implied by `-D warnings`
   Compiling rustc_target v0.0.0 (/checkout/compiler/rustc_target)
   Compiling tracing-subscriber v0.2.16
error: aborting due to previous error


error: could not compile `rustc_ast`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build --target=x86_64-unknown-linux-gnu --host=x86_64-unknown-linux-gnu --stage 2 library/std --rust-profile-generate=/tmp/rustc-pgo
Build completed unsuccessfully in 0:08:59
