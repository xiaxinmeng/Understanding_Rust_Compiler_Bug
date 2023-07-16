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
Diff in /checkout/compiler/rustc_typeck/src/collect.rs at line 2926:
                             Some([meta]) => {
                                 let alignment = rustc_attr::parse_alignment(
                                     struct_span_err!(
-                                tcx.sess.diagnostic(),
-                                meta.span(),
-                                E0589,
-                                "invalid `repr(align)` attribute",
-                            ),
+                                        tcx.sess.diagnostic(),
+                                        meta.span(),
+                                        E0589,
+                                        "invalid `repr(align)` attribute",
                                     &meta,
                                 );
 
Diff in /checkout/compiler/rustc_typeck/src/collect.rs at line 2939:
Diff in /checkout/compiler/rustc_typeck/src/collect.rs at line 2939:
                                     Err(()) => {
                                         // Error already emitted by parse_alignment
                                         None
-                                    },
+                                    }
                                 }
                             }
                             _ => None,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/variance/test.rs" "/checkout/compiler/rustc_typeck/src/variance/mod.rs" "/checkout/compiler/rustc_typeck/src/collect.rs" "/checkout/compiler/rustc_typeck/src/check/gather_locals.rs" "/checkout/compiler/rustc_typeck/src/lib.rs" "/checkout/compiler/rustc_typeck/src/check/_match.rs" "/checkout/compiler/rustc_typeck/src/bounds.rs" "/checkout/compiler/rustc_typeck/src/variance/solve.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:17
