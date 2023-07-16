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
    Checking core v0.0.0 (/checkout/library/core)
error[E0308]: mismatched types
  --> library/std/src/backtrace/tests.rs:46:16
   |
46 |           inner: Inner::Captured(LazilyResolvedCapture::new(Capture {
47 | |             actual_start: 1,
48 | |             resolved: true,
48 | |             resolved: true,
49 | |             frames: generate_fake_frames(),
50 | |         })),
   | |___________^ expected *-ptr, found enum `backtrace::Inner`
   |
   = note: expected raw pointer `*mut (dyn RawBacktrace + 'static)`
                     found enum `backtrace::Inner`
error[E0308]: mismatched types
  --> library/std/src/backtrace/tests.rs:69:16
   |
   |
69 |           inner: Inner::Captured(LazilyResolvedCapture::new(Capture {
70 | |             actual_start: 1,
71 | |             resolved: true,
71 | |             resolved: true,
72 | |             frames: generate_fake_frames(),
73 | |         })),
   | |___________^ expected *-ptr, found enum `backtrace::Inner`
   |
   = note: expected raw pointer `*mut (dyn RawBacktrace + 'static)`
                     found enum `backtrace::Inner`

error[E0599]: no method named `frames` found for struct `core::backtrace::Backtrace` in the current scope
  --> library/std/src/backtrace/tests.rs:76:28
   |
76 |     let frames = backtrace.frames();
   |                            ^^^^^^ method not found in `core::backtrace::Backtrace`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `std`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "proc_macro" "-p" "term" "-p" "panic_abort" "-p" "std" "-p" "alloc" "-p" "unwind" "-p" "panic_unwind" "-p" "core" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:41
