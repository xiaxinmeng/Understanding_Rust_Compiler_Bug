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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 2232:
         let mut res = implementors
             .iter()
             .filter(|i| {
-                i.inner_impl()
-                    .for_
-                    .def_id(cache)
-                    .map_or(false, |d| !cache.paths.contains_key(&d))
+                i.inner_impl().for_.def_id(cache).map_or(false, |d| !cache.paths.contains_key(&d))
             })
             .filter_map(|i| extract_for_impl_name(&i.impl_item, cx))
             .collect::<Vec<_>>();
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/render/mod.rs" "/checkout/src/librustdoc/html/render/tests.rs" "/checkout/src/librustdoc/html/render/context.rs" "/checkout/src/librustdoc/html/render/templates.rs" "/checkout/src/librustdoc/html/render/write_shared.rs" "/checkout/src/librustdoc/html/render/span_map.rs" "/checkout/src/librustdoc/html/render/print_item.rs" "/checkout/src/librustdoc/html/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
