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
Diff in /checkout/src/librustdoc/clean/mod.rs at line 2315:
             if matchers.len() <= 1 {
                 format!(
                     "{}macro {}{} {{\n    ...\n}}",
-                    vis.print_with_space(cx.tcx, cx.tcx.hir().local_def_id(item.hir_id).to_def_id()),
+                    vis.print_with_space(
+                        cx.tcx,
+                        cx.tcx.hir().local_def_id(item.hir_id).to_def_id()
                     name,
                     name,
                     matchers.iter().map(|span| span.to_src(cx)).collect::<String>(),
                 )
Diff in /checkout/src/librustdoc/clean/mod.rs at line 2322:
                 format!(
                 format!(
                     "{}macro {} {{\n{}}}",
-                    vis.print_with_space(cx.tcx, cx.tcx.hir().local_def_id(item.hir_id).to_def_id()),
+                    vis.print_with_space(
+                        cx.tcx,
+                        cx.tcx.hir().local_def_id(item.hir_id).to_def_id()
                     name,
                     matchers
                         .iter()
                         .iter()
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/clean/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:18
