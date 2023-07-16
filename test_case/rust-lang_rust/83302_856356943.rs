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
Diff in /checkout/compiler/rustc_mir_build/src/check_unsafety.rs at line 144:
     fn visit_block(&mut self, block: &Block) {
         match block.safety_mode {
             BlockSafety::BuiltinUnsafe => {
-                self.in_safety_context(
-                    SafetyContext::BuiltinUnsafeBlock,
-                    |this| visit::walk_block(this, block),
-                );
+                self.in_safety_context(SafetyContext::BuiltinUnsafeBlock, |this| {
+                    visit::walk_block(this, block)
+                });
             }
             BlockSafety::ExplicitUnsafe(hir_id) => {
                 self.in_safety_context(
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/remote-test-server/src/main.rs" "/checkout/compiler/rustc_mir_build/src/thir/mod.rs" "/checkout/compiler/rustc_mir_build/src/thir/constant.rs" "/checkout/compiler/rustc_mir_build/src/thir/visit.rs" "/checkout/src/tools/rust-demangler/src/main.rs" "/checkout/src/tools/rust-demangler/src/lib.rs" "/checkout/compiler/rustc_mir_build/src/thir/util.rs" "/checkout/compiler/rustc_mir_build/src/check_unsafety.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:16
