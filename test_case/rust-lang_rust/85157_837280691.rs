plain
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling profiler_builtins v0.0.0 (/checkout/library/profiler_builtins)
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
thread 'rustc' panicked at 'range end index 18446744073709551615 out of range for slice of length 3', /rustc/f43378a4f09d5d51bfbc4bca4d5abbd28238f176/library/alloc/src/vec/drain.rs:142:32

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (f43378a4f 2021-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z save-analysis -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -C linker=clang -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `str::lossy::<impl at library/core/src/str/lossy.rs:47:1: 144:2>::next`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
error: could not compile `core`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build --target=x86_64-unknown-linux-gnu --host=x86_64-unknown-linux-gnu --stage 2 library/std --rust-profile-generate=/tmp/rustc-pgo
Build completed unsuccessfully in 0:13:41
