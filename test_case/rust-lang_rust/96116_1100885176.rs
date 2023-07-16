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
Diff in /checkout/compiler/rustc_middle/src/mir/patch.rs at line 142:
         let mut delta = 0;
         let mut last_bb = START_BLOCK;
         let mut terminator_targets = Vec::new();
-        let mut statements:Vec<Statement<'_>> = Vec::new();
+        let mut statements: Vec<Statement<'_>> = Vec::new();
         for (mut loc, stmt) in new_statements {
             if loc.block != last_bb {
                 delta = 0;
Diff in /checkout/compiler/rustc_middle/src/mir/patch.rs at line 159:
                 let successors = term.successors().clone();
                 for i in successors {
                 for i in successors {
-                    statements.push(Statement{source_info,kind:stmt.clone()});
+                    statements.push(Statement { source_info, kind: stmt.clone() });
                     terminator_targets.push(i.clone());
                 delta += 1;
Diff in /checkout/compiler/rustc_middle/src/mir/patch.rs at line 174:
 
 
         for target in terminator_targets.iter().rev() {
             let stmt = statements.pop().unwrap();
-            body[*target]
-                .statements
-                .insert(0, stmt);
+            body[*target].statements.insert(0, stmt);
     }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_hir/src/def.rs" "/checkout/compiler/rustc_hir/src/def_path_hash_map.rs" "/checkout/compiler/rustc_hir/src/pat_util.rs" "/checkout/compiler/rustc_hir/src/lang_items.rs" "/checkout/compiler/rustc_hir/src/target.rs" "/checkout/compiler/rustc_middle/src/mir/patch.rs" "/checkout/compiler/rustc_middle/src/mir/tcx.rs" "/checkout/compiler/rustc_middle/src/mir/terminator.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
