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
Diff in /checkout/compiler/rustc_expand/src/expand.rs at line 534:
                             // This will cause us to synthesize fake tokens
                             // when `nt_to_tokenstream` is called on this item.
                             match &mut item {
-                                Annotatable::Item(item) => {
-                                    item.tokens = None
-                                }
+                                Annotatable::Item(item) => item.tokens = None,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_expand/src/expand.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
                                 Annotatable::Stmt(stmt) => {
                                     if let StmtKind::Item(item) = &mut stmt.kind {
                                         item.tokens = None
Diff in /checkout/compiler/rustc_expand/src/expand.rs at line 544:
                                         panic!("Unexpected stmt {:?}", stmt);
                                 }
                                 }
-                                _ => panic!("Unexpected annotatable {:?}", item)
+                                _ => panic!("Unexpected annotatable {:?}", item),
                         }
 
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:20
