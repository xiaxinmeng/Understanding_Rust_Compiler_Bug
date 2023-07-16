plain
   Compiling addr2line v0.14.0
error: cannot find macro `impl_command_sized` in this scope
   --> library/std/src/os/./unix/process.rs:405:5
    |
405 |     impl_command_sized! { xargs args_ext }
    |
    |
    = help: have you added the `#[macro_use]` on the module/import?
error: cannot find macro `impl_command_sized` in this scope
   --> library/std/src/os/./unix/process.rs:404:5
    |
    |
404 |     impl_command_sized! { margs maybe_arg_ext }
    |
    |
    = help: have you added the `#[macro_use]` on the module/import?
error: cannot find macro `impl_command_sized` in this scope
   --> library/std/src/os/./unix/process.rs:403:5
    |
    |
403 |     impl_command_sized! { marg  maybe_arg_ext }
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
    |
    |
386 | fn maybe_arg_ext(celf: &mut sys::process::Command, arg: impl Arg) -> io::Result<()> {
    |
help: consider importing this trait
    |
4   | use crate::sys_common::process_ext::Arg;
4   | use crate::sys_common::process_ext::Arg;
    |

error[E0405]: cannot find trait `Arg` in this scope
    |
392 |     args: impl IntoIterator<Item = impl Arg>,
    |                                         ^^^ not found in this scope
    |
---

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
