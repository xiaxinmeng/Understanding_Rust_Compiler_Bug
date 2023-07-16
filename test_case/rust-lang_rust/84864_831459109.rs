plain
configure: rust.channel         := stable
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
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   |
   |
60 |             ptr::write(&mut res as *mut _, 42);


error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   |
   |
84 |             (&mut res as *mut i32).write(42);

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0015`.
For more information about this error, try `rustc --explain E0015`.
error: could not compile `core`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "term" "-p" "proc_macro" "-p" "std" "-p" "alloc" "-p" "unwind" "-p" "core" "-p" "panic_unwind" "-p" "panic_abort" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:38
