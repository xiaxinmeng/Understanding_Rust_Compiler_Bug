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
Diff in /checkout/compiler/rustc_middle/src/ty/layout.rs at line 2619:
                                 tcx.normalize_erasing_regions(param_env, fn_sig.subst(tcx, substs))
                             }
                             Err(e) => {
-                                bug!(
-                                    "failed to normalize function signature {:?}: {:?}",
-                                    fn_sig,
-                                )
-                                )
+                                bug!("failed to normalize function signature {:?}: {:?}", fn_sig, e)
                         }
                     }
                     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/ty/fast_reject.rs" "/checkout/compiler/rustc_middle/src/ty/assoc.rs" "/checkout/compiler/rustc_middle/src/ty/list.rs" "/checkout/compiler/rustc_middle/src/ty/vtable.rs" "/checkout/compiler/rustc_middle/src/ty/codec.rs" "/checkout/compiler/rustc_middle/src/ty/layout.rs" "/checkout/compiler/rustc_middle/src/util/common.rs" "/checkout/compiler/rustc_middle/src/ty/normalize_erasing_regions.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
