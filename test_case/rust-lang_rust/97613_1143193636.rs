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
Diff in /checkout/src/librustdoc/html/render/print_item.rs at line 822:
             }
         }
 
-        let (local, foreign) = implementors.iter().partition::<Vec<_>, _>(|i| {
-            i.is_on_local_type(cache)
+        let (local, foreign) =
+        let (local, foreign) =
+            implementors.iter().partition::<Vec<_>, _>(|i| i.is_on_local_type(cache));
 
         let (mut synthetic, mut concrete): (Vec<&&Impl>, Vec<&&Impl>) =
             local.iter().partition(|i| i.inner_impl().kind.is_auto());
Diff in /checkout/src/librustdoc/html/render/mod.rs at line 2285:
     if let Some(implementors) = cache.implementors.get(&it.item_id.expect_def_id()) {
             .iter()
             .iter()
-            .filter(|i| { !i.is_on_local_type(cache) })
+            .filter(|i| !i.is_on_local_type(cache))
             .filter_map(|i| extract_for_impl_name(&i.impl_item, cx))
             .collect::<Vec<_>>();
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/render/tests.rs" "/checkout/src/librustdoc/html/render/search_index.rs" "/checkout/src/librustdoc/html/render/print_item.rs" "/checkout/src/librustdoc/html/render/context.rs" "/checkout/src/librustdoc/html/render/mod.rs" "/checkout/src/librustdoc/html/render/write_shared.rs" "/checkout/src/librustdoc/html/markdown.rs" "/checkout/src/librustdoc/html/toc.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
