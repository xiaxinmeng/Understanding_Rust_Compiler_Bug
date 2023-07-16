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
extracting /checkout/obj/build/cache/2022-06-29/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_const_eval/src/interpret/terminator.rs at line 520:
             }
             // cannot use the shim here, because that will only result in infinite recursion
             ty::InstanceDef::Virtual(def_id, _idx) => {
-                let fn_inst = self.resolve(ty::WithOptConstParam::unknown(def_id), instance.substs)?;
+                let fn_inst =
+                    self.resolve(ty::WithOptConstParam::unknown(def_id), instance.substs)?;
                 let mut args = args.to_vec();
                 // We have to implement all "object safe receivers". So we have to go search for a
                 // pointer or `dyn Trait` type, but it could be wrapped in newtypes. So recursively
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_const_eval/src/interpret/terminator.rs" "/checkout/compiler/rustc_const_eval/src/util/mod.rs" "/checkout/compiler/rustc_const_eval/src/interpret/operand.rs" "/checkout/compiler/rustc_const_eval/src/util/call_kind.rs" "/checkout/compiler/rustc_const_eval/src/interpret/cast.rs" "/checkout/compiler/rustc_const_eval/src/util/collect_writes.rs" "/checkout/compiler/rustc_const_eval/src/interpret/eval_context.rs" "/checkout/compiler/rustc_const_eval/src/util/aggregate.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
