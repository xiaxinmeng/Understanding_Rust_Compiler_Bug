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
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
error[E0603]: struct `Formatted` is private
   --> library/core/tests/num/flt2dec/mod.rs:5:36
    |
5   | use core::num::flt2dec::{round_up, Formatted, Part, Sign, MAX_SIG_DIGITS};
    |                                    ^^^^^^^^^ private struct
note: the struct `Formatted` is defined here
   --> /checkout/library/core/src/num/flt2dec/mod.rs:127:18
    |
    |
127 | use super::fmt::{Formatted, Part};


error[E0603]: enum `Part` is private
   --> library/core/tests/num/flt2dec/mod.rs:5:47
    |
5   | use core::num::flt2dec::{round_up, Formatted, Part, Sign, MAX_SIG_DIGITS};
    |                                               ^^^^ private enum
    |
note: the enum `Part` is defined here
   --> /checkout/library/core/src/num/flt2dec/mod.rs:127:29
    |
127 | use super::fmt::{Formatted, Part};


error[E0658]: use of unstable library feature 'numfmt': internal routines only exposed for testing
 --> library/core/tests/num/flt2dec/mod.rs:9:22
  |
9 | use core::num::fmt::{Formatted, Part};
  |
  |
  = help: add `#![feature(numfmt)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'numfmt': internal routines only exposed for testing
 --> library/core/tests/num/flt2dec/mod.rs:9:33
  |
9 | use core::num::fmt::{Formatted, Part};
  |
  |
  = help: add `#![feature(numfmt)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'numfmt': internal routines only exposed for testing
   --> library/core/tests/num/flt2dec/mod.rs:489:70
    |
489 |     F: for<'a> FnMut(&'a mut [MaybeUninit<u8>], &'a mut [MaybeUninit<Part<'a>>]) -> Formatted<'a>,
    |
    |
    = help: add `#![feature(numfmt)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'numfmt': internal routines only exposed for testing
   --> library/core/tests/num/flt2dec/mod.rs:489:85
    |
489 |     F: for<'a> FnMut(&'a mut [MaybeUninit<u8>], &'a mut [MaybeUninit<Part<'a>>]) -> Formatted<'a>,
    |
    |
    = help: add `#![feature(numfmt)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'numfmt': internal routines only exposed for testing
   --> library/core/tests/num/flt2dec/mod.rs:492:39
    |
492 |     let mut parts = [MaybeUninit::new(Part::Zero(0)); 16];
    |
    |
    = help: add `#![feature(numfmt)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'numfmt': internal routines only exposed for testing
   --> library/core/tests/num/flt2dec/mod.rs:494:37
    |
494 |     let mut ret = vec![0; formatted.len()];
    |
    |
    = help: add `#![feature(numfmt)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'numfmt': internal routines only exposed for testing
   --> library/core/tests/num/flt2dec/mod.rs:495:26
    |
495 |     assert_eq!(formatted.write(&mut ret), Some(ret.len()));
    |
    |
    = help: add `#![feature(numfmt)]` to the crate attributes to enable
error: aborting due to 9 previous errors

Some errors have detailed explanations: E0603, E0658.
For more information about an error, try `rustc --explain E0603`.
For more information about an error, try `rustc --explain E0603`.
error: could not compile `core`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "proc_macro" "-p" "core" "-p" "std" "-p" "alloc" "-p" "unwind" "-p" "panic_unwind" "-p" "panic_abort" "-p" "term" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:29
