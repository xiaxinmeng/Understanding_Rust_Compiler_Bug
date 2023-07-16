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
error[E0277]: the trait bound `[T; N]: Default` is not satisfied
    |
361 |     Default::default()
361 |     Default::default()
    |     ^^^^^^^^^^^^^^^^ the trait `Default` is not implemented for `[T; N]`
    = help: the following implementations were found:
    = help: the following implementations were found:
              <&[T] as Default>
              <&mut [T] as Default>
              <[T; 0] as Default>
              <[T; 10] as Default>
    = note: required by `std::default::Default::default`

error: aborting due to previous error


For more information about this error, try `rustc --explain E0277`.
error: could not compile `core`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "panic_unwind" "-p" "unwind" "-p" "alloc" "-p" "std" "-p" "term" "-p" "panic_abort" "-p" "core" "-p" "proc_macro" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:43
