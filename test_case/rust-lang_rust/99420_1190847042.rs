plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-06-29/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_const_eval/src/interpret/terminator.rs at line 564:
                 let vptr = self.scalar_to_ptr(receiver_place.meta.unwrap_meta())?;
                 let (_dyn_ty, dyn_trait) = self.get_ptr_vtable(vptr)?;
                 if dyn_trait != data.principal() {
-                    throw_ub_format!("`dyn` call on a pointer whose vtable does not match its type");
+                    throw_ub_format!(
+                        "`dyn` call on a pointer whose vtable does not match its type"
                 }
                 }
                 // FIXME can we do this without the `idx`? It must match the index of this method in the vtable for `dyn_trait`.
                 // But I guess there is no way that can be wrong?
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_const_eval/src/interpret/memory.rs" "/checkout/compiler/rustc_const_eval/src/interpret/util.rs" "/checkout/compiler/rustc_const_eval/src/transform/promote_consts.rs" "/checkout/compiler/rustc_const_eval/src/interpret/intrinsics.rs" "/checkout/compiler/rustc_const_eval/src/transform/validate.rs" "/checkout/compiler/rustc_const_eval/src/interpret/step.rs" "/checkout/compiler/rustc_const_eval/src/interpret/terminator.rs" "/checkout/compiler/rustc_const_eval/src/interpret/operator.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
