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
    Checking core v0.0.0 (/checkout/library/core)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking std v0.0.0 (/checkout/library/std)
error: duplicate diagnostic item found: `format_macro`.
    |
111 | / macro_rules! format {
112 | |     ($($arg:tt)*) => {{
113 | |         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
113 | |         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
114 | |         res
115 | |     }}
116 | | }
    | |_^
    |
    = note: the diagnostic item is first defined in crate `alloc`.
error: aborting due to previous error

error: could not compile `alloc`


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "proc_macro" "-p" "term" "-p" "panic_unwind" "-p" "alloc" "-p" "unwind" "-p" "std" "-p" "panic_abort" "-p" "core" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
expected success, got: exit code: 101
