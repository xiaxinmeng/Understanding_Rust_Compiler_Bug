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
Diff in /checkout/compiler/rustc_plugin_impl/src/load.rs at line 55:
     metadata_loader: &dyn MetadataLoader,
     ident: Ident,
-    let lib =
-    let lib =
-        locator::find_plugin_registrar(sess, metadata_loader, ident.span, ident.name);
+    let lib = locator::find_plugin_registrar(sess, metadata_loader, ident.span, ident.name);
     let fun = dylink_registrar(sess, ident.span, lib);
     plugins.push(fun);
Diff in /checkout/compiler/rustc_plugin_impl/src/load.rs at line 63:
 
 
 // Dynamically link a registrar function into the compiler process.
-fn dylink_registrar(
-    sess: &Session,
-    span: Span,
-    path: PathBuf,
-) -> PluginRegistrarFn {
+fn dylink_registrar(sess: &Session, span: Span, path: PathBuf) -> PluginRegistrarFn {
     use rustc_metadata::dynamic_lib::DynamicLibrary;
 
     // Make sure the path contains a / or the linker will search for it.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_plugin_impl/src/lib.rs" "/checkout/src/rustdoc-json-types/tests.rs" "/checkout/src/rustdoc-json-types/lib.rs" "/checkout/compiler/rustc_mir/src/transform/function_item_references.rs" "/checkout/compiler/rustc_mir/src/shim.rs" "/checkout/compiler/rustc_mir/src/util/aggregate.rs" "/checkout/compiler/rustc_mir/src/transform/multiple_return_terminators.rs" "/checkout/compiler/rustc_plugin_impl/src/load.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
