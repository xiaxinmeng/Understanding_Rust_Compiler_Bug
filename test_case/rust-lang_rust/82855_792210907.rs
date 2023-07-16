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
         let abi = print_abi_with_space(header.abi).to_string();
         // NOTE: `{:#}` does not print HTML formatting, `{}` does. So `g.print` can't be reused between the length calculation and `write!`.
         let generics_len = format!("{:#}", g.print(cx.cache())).len();
-        let mut header_len = "fn ".len() + vis.len() + constness.len() + asyncness.len() + unsafety.len() + defaultness.len() + abi.len() + name.as_str().len() + generics_len;
+        let mut header_len = "fn ".len()
+            + vis.len()
+            + constness.len()
+            + asyncness.len()
+            + unsafety.len()
+            + defaultness.len()
+            + abi.len()
+            + name.as_str().len()
+            + generics_len;
 
         let (indent, end_newline) = if parent == ItemType::Trait {
             header_len += 4;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/toc/tests.rs" "/checkout/src/librustdoc/html/render/mod.rs" "/checkout/src/librustdoc/html/render/cache.rs" "/checkout/src/librustdoc/passes/collect_intra_doc_links.rs" "/checkout/src/librustdoc/html/static_files.rs" "/checkout/src/librustdoc/passes/mod.rs" "/checkout/src/librustdoc/html/sources.rs" "/checkout/src/librustdoc/html/render/write_shared.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
