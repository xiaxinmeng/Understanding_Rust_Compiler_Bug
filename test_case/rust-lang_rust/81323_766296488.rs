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
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
    Checking std v0.0.0 (/checkout/library/std)
error[E0658]: use of unstable library feature 'result_swap'
    |
    |
364 |     assert_eq!(Ok("err").swap(), Err("err"));
    |
    |
    = help: add `#![feature(result_swap)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'result_swap'
    |
    |
365 |     assert_eq!(Err("ok").swap(), Ok("ok"));
    |
    |
    = help: add `#![feature(result_swap)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'result_swap'
    |
    |
366 |     assert_eq!(Ok("ok").swap().swap(), Ok("ok"));
    |
    |
    = help: add `#![feature(result_swap)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'result_swap'
    |
    |
366 |     assert_eq!(Ok("ok").swap().swap(), Ok("ok"));
    |
    |
    = help: add `#![feature(result_swap)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
error: could not compile `core`
error: could not compile `core`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "std" "-p" "unwind" "-p" "alloc" "-p" "term" "-p" "proc_macro" "-p" "panic_abort" "-p" "panic_unwind" "-p" "core" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:49
