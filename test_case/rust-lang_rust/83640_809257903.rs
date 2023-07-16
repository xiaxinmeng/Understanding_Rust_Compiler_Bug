plain
  |
3 | use tracing::debug;
  |     ^^^^^^^^^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `rustc_fs_util::path_to_c_string`
 --> compiler/rustc_codegen_llvm/src/metadata.rs:5:5
  |
5 | use rustc_fs_util::path_to_c_string;
---

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build --target=x86_64-unknown-linux-gnu --host=x86_64-unknown-linux-gnu --stage 2 library/std --rust-profile-generate=/tmp/rustc-pgo
Build completed unsuccessfully in 0:11:08
