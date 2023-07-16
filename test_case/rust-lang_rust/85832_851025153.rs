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
    Checking addr2line v0.14.0
error[E0408]: variable `s` is not bound in all patterns
   --> library/std/src/sys/windows/process.rs:593:30
    |
593 |             Arg::Quoted(s) | Arg::Unquoted => s.as_ref(),
    |                         -    ^^^^^^^^^^^^^ pattern doesn't bind `s`
    |                         variable not in all patterns


error[E0425]: cannot find value `arg` in this scope
   --> library/std/src/sys/windows/process.rs:150:38
    |
150 |         self.args.push(Arg::Unquoted(arg.to_os_string()))


error[E0532]: expected unit struct, unit variant or constant, found tuple variant `Arg::Unquoted`
   --> library/std/src/sys/windows/process.rs:593:30
    |
104 |     Unquoted(OsString),
    |     ------------------ `Arg::Unquoted` defined here
...
593 |             Arg::Quoted(s) | Arg::Unquoted => s.as_ref(),
    |                              ^^^^^^^^^^^^^ help: use the tuple variant pattern syntax instead: `Arg::Unquoted(_)`

error[E0277]: `Arg` doesn't implement `core::fmt::Debug`
   --> library/std/src/sys/windows/process.rs:267:32
267 |               write!(f, " {:?}", arg)?;
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

error[E0277]: `Arg` doesn't implement `core::fmt::Debug`
   --> library/std/src/sys/windows/process.rs:612:32
    |
612 |         f.debug_list().entries(self.iter.clone()).finish()
    |                                ^^^^^^^^^^^^^^^^^ `Arg` cannot be formatted using `{:?}`
    = help: the trait `core::fmt::Debug` is not implemented for `Arg`
    = help: the trait `core::fmt::Debug` is not implemented for `Arg`
    = note: add `#[derive(Debug)]` or manually implement `core::fmt::Debug`
    = note: required because of the requirements on the impl of `core::fmt::Debug` for `&Arg`
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0277, E0408, E0425, E0532.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:17
