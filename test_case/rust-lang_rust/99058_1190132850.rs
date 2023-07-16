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
Diff in /checkout/compiler/rustc_typeck/src/check/writeback.rs at line 732:
     }
     }
     fn fold_region(&mut self, r: ty::Region<'tcx>) -> ty::Region<'tcx> {
-        if r.is_late_bound() {
-        } else {
-        } else {
-            self.tcx.lifetimes.re_erased
-        }
+        if r.is_late_bound() { r } else { self.tcx.lifetimes.re_erased }
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/method/suggest.rs" "/checkout/compiler/rustc_typeck/src/check/place_op.rs" "/checkout/compiler/rustc_typeck/src/check/cast.rs" "/checkout/compiler/rustc_typeck/src/check/autoderef.rs" "/checkout/compiler/rustc_typeck/src/check/expr.rs" "/checkout/compiler/rustc_typeck/src/check/region.rs" "/checkout/compiler/rustc_typeck/src/check/check.rs" "/checkout/compiler/rustc_typeck/src/check/writeback.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
 }
 
 
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
