plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_mir_dataflow/src/value_analysis.rs at line 767:
 pub enum ValueOrPlaceOrRef<V> {
     Value(V),
     Place(PlaceIndex),
-    Ref(PlaceIndex)
+    Ref(PlaceIndex),
 
 
 impl<V: HasTop> ValueOrPlaceOrRef<V> {
Diff in /checkout/compiler/rustc_mir_dataflow/src/value_analysis.rs at line 775:
         ValueOrPlaceOrRef::Value(V::top())
 }
-
 
 
 impl<V> From<ValueOrPlace<V>> for ValueOrPlaceOrRef<V> {
     fn from(x: ValueOrPlace<V>) -> Self {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_dataflow/src/move_paths/abs_domain.rs" "/checkout/compiler/rustc_mir_dataflow/src/move_paths/mod.rs" "/checkout/compiler/rustc_mir_dataflow/src/un_derefer.rs" "/checkout/compiler/rustc_mir_dataflow/src/value_analysis.rs" "/checkout/compiler/rustc_mir_dataflow/src/drop_flag_effects.rs" "/checkout/compiler/rustc_mir_dataflow/src/elaborate_drops.rs" "/checkout/compiler/rustc_symbol_mangling/src/lib.rs" "/checkout/compiler/rustc_mir_dataflow/src/move_paths/builder.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
