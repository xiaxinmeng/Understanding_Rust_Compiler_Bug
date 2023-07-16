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
Diff in /checkout/compiler/rustc_mir/src/monomorphize/partitioning/mod.rs at line 245:
         let s = &mut String::new();
         let _ = writeln!(s, "{}", label);
         for cgu in cgus {
-            let _ = writeln!(s, "CodegenUnit {} estimated size {} :", cgu.name(), cgu.size_estimate());
+            let _ =
+                writeln!(s, "CodegenUnit {} estimated size {} :", cgu.name(), cgu.size_estimate());
 
             for (mono_item, linkage) in cgu.items() {
                 let symbol_name = mono_item.symbol_name(tcx).name;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/monomorphize/partitioning/mod.rs" "/checkout/compiler/rustc_mir/src/transform/add_call_guards.rs" "/checkout/compiler/rustc_mir/src/transform/no_landing_pads.rs" "/checkout/compiler/rustc_mir/src/monomorphize/mod.rs" "/checkout/compiler/rustc_mir/src/transform/dump_mir.rs" "/checkout/compiler/rustc_mir/src/monomorphize/collector.rs" "/checkout/compiler/rustc_mir/src/transform/rustc_peek.rs" "/checkout/compiler/rustc_mir/src/transform/check_packed_ref.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:17
