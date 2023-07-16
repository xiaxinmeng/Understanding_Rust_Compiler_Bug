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
    Checking toml v0.5.7
error: mismatched closing delimiter: `}`
  --> src/bootstrap/builder/tests.rs:91:5
   |
69 |     fn build_stage_0() {
...
87 |         assert_eq!(
   |                   - unclosed delimiter
...
...
91 |     }
   |     ^ mismatched closing delimiter

error: no rules expected the token `;`
  --> src/bootstrap/builder/tests.rs:90:10
90 |         );
90 |         );
   |          ^ no rules expected this token in macro call
error: aborting due to 2 previous errors

error: could not compile `bootstrap`


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/bootstrap/Cargo.toml" "--all-targets" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:03:22
