plain
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling profiler_builtins v0.0.0 (/checkout/library/profiler_builtins)
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
error[E0369]: binary operation `==` cannot be applied to type `[u64; 16]`
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.85/src/unix/linux_like/linux/mod.rs:2740:19
     |
2740 |         set1.bits == set2.bits
     |         --------- ^^ --------- [u64; 16]
     |         [u64; 16]

error: aborting due to previous error


For more information about this error, try `rustc --explain E0369`.
error: could not compile `libc`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: internal compiler error: compiler/rustc_metadata/src/rmeta/decoder.rs:838:18: get_adt_def called on a non-ADT DefId(2:0 ~ core[27a9])

thread 'rustc' panicked at 'Box<Any>', /rustc/21fceed3ad09e8e92e69111b015a4e53057aa405/library/std/src/panic.rs:59:5

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.52.0-nightly (21fceed3a 2021-02-13) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z macro-backtrace -Z save-analysis -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=10000 -C debuginfo=1 -C linker=clang -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C embed-bitcode=yes -C panic=abort --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [adt_def] computing ADT definition for `core`
#1 [mir_for_ctfe] caching mir of `rustc_std_workspace_core::num::<impl usize>::MAX` for CTFE
end of query stack

error: build failed
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build --target=x86_64-unknown-linux-gnu --host=x86_64-unknown-linux-gnu --stage 2 library/std --rust-profile-generate=/tmp/rustc-pgo
Build completed unsuccessfully in 0:13:59
