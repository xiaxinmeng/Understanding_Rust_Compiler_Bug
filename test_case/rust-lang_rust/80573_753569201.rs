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
    Checking fnv v1.0.7
   Compiling serde_json v1.0.59
error[E0463]: can't find crate for `core`
  |
  = note: the `i686-pc-windows-gnu` target may not be installed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
error: could not compile `cfg-if`
error: could not compile `cfg-if`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error[E0463]: can't find crate for `core`
  |
  = note: the `i686-pc-windows-gnu` target may not be installed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
error[E0463]: can't find crate for `std`
error[E0463]: can't find crate for `std`
  |
  = note: the `i686-pc-windows-gnu` target may not be installed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
error[E0463]: can't find crate for `std`
error[E0463]: can't find crate for `std`
  |
  = note: the `i686-pc-windows-gnu` target may not be installed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
error: build failed
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/bootstrap/Cargo.toml" "--all-targets" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:03:09
