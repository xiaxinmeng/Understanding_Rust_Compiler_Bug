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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_ast/src/mut_visit.rs at line 772:
         token::NtBlock(block) => vis.visit_block(block),
         token::NtStmt(stmt) => visit_clobber(stmt, |stmt| {
             // See reasoning above.
-            stmt.map(|stmt| vis.flat_map_stmt(stmt).expect_one("expected visitor to produce exactly one item"))
+            stmt.map(|stmt| {
+                vis.flat_map_stmt(stmt).expect_one("expected visitor to produce exactly one item")
         }),
         }),
         token::NtPat(pat) => vis.visit_pat(pat),
         token::NtExpr(expr) => vis.visit_expr(expr),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_ast/src/attr/mod.rs" "/checkout/compiler/rustc_ast/src/ptr.rs" "/checkout/compiler/rustc_ast/src/tokenstream.rs" "/checkout/compiler/rustc_ast/src/node_id.rs" "/checkout/compiler/rustc_ast/src/util/comments/tests.rs" "/checkout/compiler/rustc_ast/src/util/literal.rs" "/checkout/compiler/rustc_ast/src/util/classify.rs" "/checkout/compiler/rustc_ast/src/mut_visit.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
