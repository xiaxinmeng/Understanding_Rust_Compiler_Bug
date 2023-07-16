plain
   Compiling libc v0.2.93
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.45
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0603]: enum import `Part` is private
   --> library/core/src/fmt/float.rs:18:42
    |
18  |     let mut parts: [MaybeUninit<flt2dec::Part<'_>>; 4] = MaybeUninit::uninit_array();
    |                                          ^^^^ private enum import
    |
note: the enum import `Part` is defined here...
   --> library/core/src/num/flt2dec/mod.rs:127:29
    |
127 | use super::fmt::{Formatted, Part};
    |                             ^^^^
note: ...and refers to the enum `Part` which is defined here
   --> library/core/src/num/fmt.rs:11:1
    |
11  | pub enum Part<'a> {
    | ^^^^^^^^^^^^^^^^^ consider importing it directly

error[E0603]: enum import `Part` is private
   --> library/core/src/fmt/float.rs:44:42
    |
44  |     let mut parts: [MaybeUninit<flt2dec::Part<'_>>; 4] = MaybeUninit::uninit_array();
    |                                          ^^^^ private enum import
    |
note: the enum import `Part` is defined here...
   --> library/core/src/num/flt2dec/mod.rs:127:29
    |
127 | use super::fmt::{Formatted, Part};
    |                             ^^^^
note: ...and refers to the enum `Part` which is defined here
   --> library/core/src/num/fmt.rs:11:1
    |
11  | pub enum Part<'a> {
    | ^^^^^^^^^^^^^^^^^ consider importing it directly

error[E0603]: enum import `Part` is private
   --> library/core/src/fmt/float.rs:88:42
    |
88  |     let mut parts: [MaybeUninit<flt2dec::Part<'_>>; 6] = MaybeUninit::uninit_array();
    |                                          ^^^^ private enum import
    |
note: the enum import `Part` is defined here...
   --> library/core/src/num/flt2dec/mod.rs:127:29
    |
127 | use super::fmt::{Formatted, Part};
    |                             ^^^^
note: ...and refers to the enum `Part` which is defined here
   --> library/core/src/num/fmt.rs:11:1
    |
11  | pub enum Part<'a> {
    | ^^^^^^^^^^^^^^^^^ consider importing it directly

error[E0603]: enum import `Part` is private
   --> library/core/src/fmt/float.rs:115:42
    |
115 |     let mut parts: [MaybeUninit<flt2dec::Part<'_>>; 6] = MaybeUninit::uninit_array();
    |                                          ^^^^ private enum import
    |
note: the enum import `Part` is defined here...
   --> library/core/src/num/flt2dec/mod.rs:127:29
    |
127 | use super::fmt::{Formatted, Part};
    |                             ^^^^
note: ...and refers to the enum `Part` which is defined here
   --> library/core/src/num/fmt.rs:11:1
    |
11  | pub enum Part<'a> {
    | ^^^^^^^^^^^^^^^^^ consider importing it directly
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0603`.
error: could not compile `core`
error: could not compile `core`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:08
