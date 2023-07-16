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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/interpret/terminator.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Diff in /checkout/compiler/rustc_mir/src/interpret/terminator.rs at line 264:
             | ty::InstanceDef::CloneShim(..)
             | ty::InstanceDef::Item(_) => {
                 // We need MIR for this fn
-                let body = match M::find_mir_or_eval_fn(self, instance, caller_abi, args, ret, unwind)? {
-                    Some(body) => body,
-                    None => return Ok(()),
+                let body =
+                let body =
+                    match M::find_mir_or_eval_fn(self, instance, caller_abi, args, ret, unwind)? {
+                        Some(body) => body,
+                        None => return Ok(()),
 
                 self.push_stack_frame(
                     instance,
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
