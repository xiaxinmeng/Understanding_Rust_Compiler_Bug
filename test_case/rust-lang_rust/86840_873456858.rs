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
error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
 --> library/core/tests/num/const_from.rs:7:23
  |
7 |     const FROM: i64 = i64::from(1i32);

error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
  --> library/core/tests/num/const_from.rs:11:54
   |
   |
11 |     const U8_FROM_U16: Result<u8, TryFromIntError> = u8::try_from(1u16);

error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
  --> library/core/tests/num/const_from.rs:15:54
   |
   |
15 |     const I8_FROM_I16: Result<i8, TryFromIntError> = i8::try_from(1i16);

error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
  --> library/core/tests/num/const_from.rs:19:56
   |
   |
19 |     const I16_FROM_U16: Result<i16, TryFromIntError> = i16::try_from(1u16);

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0015`.
For more information about this error, try `rustc --explain E0015`.
error: could not compile `core`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "panic_unwind" "-p" "alloc" "-p" "unwind" "-p" "std" "-p" "core" "-p" "panic_abort" "-p" "proc_macro" "-p" "term" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:27
