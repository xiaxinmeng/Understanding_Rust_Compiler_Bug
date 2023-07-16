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
Diff in /checkout/src/librustdoc/clean/inline.rs at line 416:
 
     debug!("build_impl: impl {:?} for {:?}", trait_.def_id(), for_.def_id());
 
-    let attrs = box merge_attrs(cx, parent_module.into(), load_attrs(cx, did), attrs, did.is_local());
+    let attrs =
+        box merge_attrs(cx, parent_module.into(), load_attrs(cx, did), attrs, did.is_local());
     debug!("merged_attrs={:?}", attrs);
 
     ret.push(clean::Item::from_def_id_and_attrs_and_parts(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/clean/utils.rs" "/checkout/src/librustdoc/html/format.rs" "/checkout/src/librustdoc/docfs.rs" "/checkout/src/librustdoc/json/conversions.rs" "/checkout/src/librustdoc/json/mod.rs" "/checkout/src/librustdoc/visit_ast.rs" "/checkout/src/librustdoc/html/highlight/tests.rs" "/checkout/src/librustdoc/clean/inline.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
