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
Diff in /checkout/compiler/rustc_mir/src/transform/promote_consts.rs at line 662:
                                 Operand::Constant(c) => {
                                     c.literal.try_eval_bits(self.tcx, self.param_env, lhs_ty)
-                                _ => None
+                                _ => None,
+                                _ => None,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/transform/promote_consts.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
                             match const_val {
                             match const_val {
-                                Some(x) if x != 0 => {} // okay
-                                _ => return Err(Unpromotable) // value not known or 0 -- not okay
+                                Some(x) if x != 0 => {}        // okay
+                                _ => return Err(Unpromotable), // value not known or 0 -- not okay
                         }
                     }
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:17
