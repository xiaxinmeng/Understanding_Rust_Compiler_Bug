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
Diff in /checkout/compiler/rustc_borrowck/src/region_infer/mod.rs at line 1363:
         }
 
         let result = self.scc_values.contains_points(sup_region_scc, sub_region_scc);
-        debug!(
-            "returning {} because of comparison between points in sup/sub",
-        );
-        );
+        debug!("returning {} because of comparison between points in sup/sub", result);
     }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_borrowck/src/region_infer/opaque_types.rs" "/checkout/compiler/rustc_borrowck/src/region_infer/values.rs" "/checkout/compiler/rustc_borrowck/src/region_infer/graphviz.rs" "/checkout/compiler/rustc_borrowck/src/region_infer/reverse_sccs.rs" "/checkout/compiler/rustc_borrowck/src/type_check/liveness/local_use_map.rs" "/checkout/compiler/rustc_borrowck/src/region_infer/mod.rs" "/checkout/compiler/rustc_borrowck/src/type_check/liveness/mod.rs" "/checkout/compiler/rustc_borrowck/src/region_infer/dump_mir.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
