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
Diff in /checkout/compiler/rustc_typeck/src/check/method/suggest.rs at line 501:
                                                 // So we avoid suggestion method with Box<Self>
                                                 // for instance
                                                 self.tcx.at(span).type_of(*def_id) != actual
-                                                    && self.tcx.at(span).type_of(*def_id)
-                                                        != rcvr_ty
+                                                    && self.tcx.at(span).type_of(*def_id) != rcvr_ty
                                             }
                                             (Mode::Path, false, _) => true,
                                             _ => false,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/writeback.rs" "/checkout/compiler/rustc_typeck/src/check/check.rs" "/checkout/compiler/rustc_typeck/src/check/method/suggest.rs" "/checkout/compiler/rustc_typeck/src/check/method/probe.rs" "/checkout/compiler/rustc_typeck/src/check/method/mod.rs" "/checkout/compiler/rustc_typeck/src/check/method/confirm.rs" "/checkout/compiler/rustc_typeck/src/check/gather_locals.rs" "/checkout/compiler/rustc_typeck/src/check/generator_interior.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
