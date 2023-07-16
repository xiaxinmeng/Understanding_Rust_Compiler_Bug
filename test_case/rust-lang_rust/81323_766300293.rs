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
    Checking alloc v0.0.0 (/checkout/library/alloc)
error[E0282]: type annotations needed
   --> library/core/tests/result.rs:364:16
    |
364 |     assert_eq!(Ok("err").swap(), Err("err"));
    |                ^^--------------
    |                |
    |                this method call resolves to `std::result::Result<E, T>`
    |                cannot infer type for type parameter `E` declared on the enum `Result`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
error: could not compile `core`
error: could not compile `core`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "std" "-p" "alloc" "-p" "unwind" "-p" "panic_unwind" "-p" "proc_macro" "-p" "core" "-p" "panic_abort" "-p" "term" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:40
