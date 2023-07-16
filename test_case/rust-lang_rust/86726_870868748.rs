plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
    Checking alloc v0.0.0 (/checkout/library/alloc)
error: duplicate diagnostic item found: `unwind_safe_trait`.
    |
141 | pub auto trait UnwindSafe {}
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: the diagnostic item is first defined in crate `std`.

error: duplicate diagnostic item found: `ref_unwind_safe_trait`.
    |
159 | pub auto trait RefUnwindSafe {}
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: the diagnostic item is first defined in crate `std`.
error: aborting due to 2 previous errors

error: could not compile `std`


To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "term" "-p" "proc_macro" "-p" "std" "-p" "unwind" "-p" "alloc" "-p" "core" "-p" "panic_abort" "-p" "panic_unwind" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:30
