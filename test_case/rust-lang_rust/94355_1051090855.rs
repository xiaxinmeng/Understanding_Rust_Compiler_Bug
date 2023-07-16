plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
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
Diff in /checkout/compiler/rustc_typeck/src/check/fn_ctxt/_impl.rs at line 317:
                     // having them slip to bug! causes ICE
                     // see #94291 for more info
                     (&[Adjustment { kind: Adjust::Deref(None), .. }], _) => {
-                        self.tcx.sess.delay_span_bug(DUMMY_SP, &format!("Can't compose Deref(None) expressions"))
+                        self.tcx.sess.delay_span_bug(
+                            DUMMY_SP,
+                            &format!("Can't compose Deref(None) expressions"),
                     }
                     }
                     // FIXME: currently we never try to compose autoderefs
                     // and ReifyFnPointer/UnsafeFnPointer, but we could.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/coherence/mod.rs" "/checkout/compiler/rustc_typeck/src/check/writeback.rs" "/checkout/compiler/rustc_typeck/src/coherence/unsafety.rs" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/_impl.rs" "/checkout/compiler/rustc_typeck/src/check/gather_locals.rs" "/checkout/compiler/rustc_typeck/src/check/place_op.rs" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/mod.rs" "/checkout/compiler/rustc_typeck/src/check/coercion.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
