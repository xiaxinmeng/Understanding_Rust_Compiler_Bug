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
Diff in /checkout/compiler/rustc_lint/src/levels.rs at line 450:
                 // we don't warn about the name change.
                 if let CheckLintNameResult::Warning(_, Some(new_name)) = lint_result {
                     // Ignore any errors or warnings that happen because the new name is inaccurate
-                    if let CheckLintNameResult::Ok(ids) = store.check_lint_name(&new_name, tool_name) {
-                        let src = LintLevelSource::Node(Symbol::intern(&new_name), li.span(), reason);
+                    if let CheckLintNameResult::Ok(ids) =
+                        store.check_lint_name(&new_name, tool_name)
+                        let src =
+                        let src =
+                            LintLevelSource::Node(Symbol::intern(&new_name), li.span(), reason);
                         for &id in ids {
                             self.check_gated_lint(id, attr.span);
                             self.insert_spec(&mut specs, id, (level, src));
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/panic_unwind/src/emcc.rs" "/checkout/library/panic_unwind/src/dwarf/mod.rs" "/checkout/library/panic_unwind/src/hermit.rs" "/checkout/library/panic_unwind/src/dwarf/eh.rs" "/checkout/library/panic_unwind/src/gcc.rs" "/checkout/library/panic_unwind/src/dummy.rs" "/checkout/compiler/rustc_lint/src/levels.rs" "/checkout/compiler/rustc_lint/src/early.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
