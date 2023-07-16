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
Diff in /checkout/src/librustdoc/lib.rs at line 598:
                 "[unversioned-shared-resources,toolchain-shared-resources,invocation-specific]",
         }),
         }),
-        unstable("no-run", |o| o.optflagmulti("", "no-run", "Compile doctests without running them")),
+        unstable("no-run", |o| {
+            o.optflagmulti("", "no-run", "Compile doctests without running them")
+        }),
         unstable("show-type-layout", |o| {
             o.optflagmulti("", "show-type-layout", "Include the memory layout of types in the docs")
         }),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/lib.rs" "/checkout/src/librustdoc/passes/collect_trait_impls.rs" "/checkout/src/librustdoc/doctree.rs" "/checkout/src/librustdoc/passes/bare_urls.rs" "/checkout/src/librustdoc/html/render/cache.rs" "/checkout/src/librustdoc/docfs.rs" "/checkout/src/librustdoc/json/mod.rs" "/checkout/src/librustdoc/passes/unindent_comments/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
