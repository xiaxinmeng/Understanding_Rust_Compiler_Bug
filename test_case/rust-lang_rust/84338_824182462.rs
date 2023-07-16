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
error[E0061]: this function takes 1 argument but 2 arguments were supplied
   --> library/core/tests/alloc/prefix.rs:26:13
    |
26  |             PrefixAllocator::<System, Prefix>::prefix(memory.as_non_null_ptr(), layout)
    |             |
    |             expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/library/core/src/alloc/helper.rs:101:19
    |
101 |     pub unsafe fn prefix<T: ?Sized>(ptr: NonNull<T>) -> NonNull<Prefix> {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0061`.
For more information about this error, try `rustc --explain E0061`.
error: could not compile `core`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "term" "-p" "proc_macro" "-p" "core" "-p" "panic_unwind" "-p" "unwind" "-p" "alloc" "-p" "std" "-p" "panic_abort" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:45
