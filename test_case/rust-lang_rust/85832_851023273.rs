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
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking miniz_oxide v0.4.0
    Checking object v0.22.0
    Checking addr2line v0.14.0
error[E0599]: no method named `unquoted_arg` found for mutable reference `&mut sys::windows::process::Command` in the current scope
   --> library/std/src/os/./windows/process.rs:151:29
    |
151 |         self.as_inner_mut().unquoted_arg(raw_text);
    |                             ^^^^^^^^^^^^ method not found in `&mut sys::windows::process::Command`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `CommandExt` defines an item `unquoted_arg`, perhaps you need to implement it
   --> library/std/src/os/./windows/process.rs:105:1
    |
105 | pub trait CommandExt: Sealed {

error[E0308]: mismatched types
   --> library/std/src/sys/windows/process.rs:155:23
    |
    |
155 |         CommandArgs { iter }
    |                       ^^^^ expected struct `OsString`, found enum `Arg`
    |
    = note: expected struct `core::slice::Iter<'_, OsString>`
               found struct `core::slice::Iter<'_, Arg>`

error[E0277]: `Arg` doesn't implement `core::fmt::Debug`
   --> library/std/src/sys/windows/process.rs:263:32
263 |               write!(f, " {:?}", arg)?;
    |               -------------------^^^-
    |               |                  |
    |               |                  |
    |               |                  `Arg` cannot be formatted using `{:?}`
    | 
   ::: /checkout/library/core/src/macros/mod.rs:472:1
    |
472 | / macro_rules! write {
472 | / macro_rules! write {
473 | |     ($dst:expr, $($arg:tt)*) => ($dst.write_fmt($crate::format_args!($($arg)*)))
474 | | }
474 | | }
    | |_- in this expansion of `write!` (#1)
829 | /     macro_rules! format_args {
829 | /     macro_rules! format_args {
830 | |         ($fmt:expr) => {{ /* compiler built-in */ }};
831 | |         ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    | |_____- in this expansion of `$crate::format_args!` (#2)
    |
    = help: the trait `core::fmt::Debug` is not implemented for `Arg`
    = help: the trait `core::fmt::Debug` is not implemented for `Arg`
    = note: add `#[derive(Debug)]` or manually implement `core::fmt::Debug`
    = note: required because of the requirements on the impl of `core::fmt::Debug` for `&Arg`
    = note: required by `core::fmt::Debug::fmt`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0308, E0599.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:17
