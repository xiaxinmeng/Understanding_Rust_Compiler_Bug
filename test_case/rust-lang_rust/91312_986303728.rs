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
Diff in /checkout/compiler/rustc_typeck/src/collect/type_of.rs at line 184:
                     });
                 let (arg_index, segment) = match filtered {
                     None => {
-                        tcx.sess
-                            .delay_span_bug(tcx.def_span(def_id), "no arg matching AnonConst in path");
+                        tcx.sess.delay_span_bug(
+                            tcx.def_span(def_id),
+                            "no arg matching AnonConst in path",
                         return None;
                     }
                     }
                     Some(inner) => inner,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/outlives/outlives_bounds.rs" "/checkout/compiler/rustc_typeck/src/outlives/implicit_infer.rs" "/checkout/compiler/rustc_typeck/src/outlives/explicit.rs" "/checkout/compiler/rustc_typeck/src/errors.rs" "/checkout/compiler/rustc_typeck/src/collect.rs" "/checkout/compiler/rustc_typeck/src/structured_errors.rs" "/checkout/compiler/rustc_typeck/src/hir_wf_check.rs" "/checkout/compiler/rustc_typeck/src/collect/type_of.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
