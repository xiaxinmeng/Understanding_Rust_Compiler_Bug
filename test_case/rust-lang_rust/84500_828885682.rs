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
    Checking pretty_assertions v0.6.1
    Checking serde v1.0.125
    Checking serde_json v1.0.59
    Checking toml v0.5.7
error[E0063]: missing field `run` in initializer of `flags::Subcommand`
   --> src/bootstrap/builder/tests.rs:482:22
    |
482 |         config.cmd = Subcommand::Test {
    |                      ^^^^^^^^^^^^^^^^ missing `run`

error[E0063]: missing field `run` in initializer of `flags::Subcommand`
   --> src/bootstrap/builder/tests.rs:522:22
    |
522 |         config.cmd = Subcommand::Test {
    |                      ^^^^^^^^^^^^^^^^ missing `run`

error[E0063]: missing field `run` in initializer of `flags::Subcommand`
   --> src/bootstrap/builder/tests.rs:577:22
    |
577 |         config.cmd = Subcommand::Test {
    |                      ^^^^^^^^^^^^^^^^ missing `run`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0063`.
error: could not compile `bootstrap`
error: could not compile `bootstrap`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/bootstrap/Cargo.toml" "--all-targets" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:03:03
