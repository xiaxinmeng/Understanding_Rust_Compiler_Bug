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
Diff in /checkout/compiler/rustc_metadata/src/dependency_format.rs at line 291:
 
     // All crates are available in an rlib format, so we're just going to link
     // everything in explicitly so long as it's actually required.
+    let mut ret = tcx
+        .crates(())
+        .iter()
+        .iter()
         .map(|&cnum| {
             if tcx.dep_kind(cnum) == CrateDepKind::Explicit {
                 Linkage::Static
Diff in /checkout/compiler/rustc_metadata/src/rmeta/encoder.rs at line 1709:
     fn encode_crate_deps(&mut self) -> Lazy<[CrateDep]> {
         empty_proc_macro!(self);
-        let deps = self.tcx.crates(())
+        let deps = self
+            .tcx
+            .crates(())
+            .crates(())
             .iter()
             .map(|&cnum| {
                 let dep = CrateDep {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_metadata/src/lib.rs" "/checkout/compiler/rustc_session/src/output.rs" "/checkout/compiler/rustc_metadata/src/dependency_format.rs" "/checkout/compiler/rustc_metadata/src/dynamic_lib.rs" "/checkout/compiler/rustc_session/src/filesearch.rs" "/checkout/compiler/rustc_metadata/src/foreign_modules.rs" "/checkout/compiler/rustc_session/src/parse.rs" "/checkout/compiler/rustc_session/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
