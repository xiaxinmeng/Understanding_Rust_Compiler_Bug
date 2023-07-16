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
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 1028:
         };
         };
         let tcx = cx.tcx();
-        let vis = meth.visibility.print_with_space(tcx, meth.def_id, cx.cache(), relative_to).to_string();
+        let vis =
+            meth.visibility.print_with_space(tcx, meth.def_id, cx.cache(), relative_to).to_string();
         let constness = header.constness.print_with_space();
         let asyncness = header.asyncness.print_with_space();
         let unsafety = header.unsafety.print_with_space();
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/static_files.rs" "/checkout/src/librustdoc/passes/html_tags.rs" "/checkout/src/librustdoc/html/render/context.rs" "/checkout/src/librustdoc/html/render/cache.rs" "/checkout/src/librustdoc/html/render/tests.rs" "/checkout/src/librustdoc/html/render/print_item.rs" "/checkout/src/librustdoc/html/render/mod.rs" "/checkout/src/librustdoc/passes/stripper.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
