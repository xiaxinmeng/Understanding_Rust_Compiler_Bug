plain
    Checking addr2line v0.14.0
error: cannot find macro `impl_command_sized` in this scope
   --> library/std/src/os/./unix/process.rs:360:5
    |
360 |     impl_command_sized! { xargs args_ext }
    |
    |
    = help: have you added the `#[macro_use]` on the module/import?
error: cannot find macro `impl_command_sized` in this scope
   --> library/std/src/os/./unix/process.rs:359:5
    |
    |
359 |     impl_command_sized! { margs maybe_arg_ext }
    |
    |
    = help: have you added the `#[macro_use]` on the module/import?
error: cannot find macro `impl_command_sized` in this scope
   --> library/std/src/os/./unix/process.rs:358:5
    |
    |
358 |     impl_command_sized! { marg  maybe_arg_ext }
    |
    |
    = help: have you added the `#[macro_use]` on the module/import?
error: cannot find macro `impl_command_sized` in this scope
  --> library/std/src/os/./unix/process.rs:13:1
   |
   |
13 | impl_command_sized! { prelude }
   |
   |
   = help: have you added the `#[macro_use]` on the module/import?

error[E0405]: cannot find trait `Arg` in this scope
   --> library/std/src/os/./unix/process.rs:341:62
    |
341 | fn maybe_arg_ext(celf: &mut sys::process::Command, arg: impl Arg) -> io::Result<()> {
    |
help: consider importing this trait
    |
4   | use crate::sys_common::process_ext::Arg;
4   | use crate::sys_common::process_ext::Arg;
    |

error[E0405]: cannot find trait `Arg` in this scope
   --> library/std/src/os/./unix/process.rs:347:41
347 |     args: impl IntoIterator<Item = impl Arg>,
    |                                         ^^^ not found in this scope
    |
help: consider importing this trait
help: consider importing this trait
    |
4   | use crate::sys_common::process_ext::Arg;
    |

error: unused macro definition
   --> library/std/src/sys_common/process_ext.rs:90:1
    |
90  | / macro_rules! impl_command_sized {
91  | |     (prelude) => {
92  | |         use crate::sys::process::{Arg, Problem};
93  | |         use core::convert::TryFrom;
168 | |     };
169 | | }
    | |_^
    |
    |
    = note: `-D unused-macros` implied by `-D warnings`
error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0405`.
error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:17
