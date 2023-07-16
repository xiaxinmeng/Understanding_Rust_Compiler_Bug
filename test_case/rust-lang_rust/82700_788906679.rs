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
Diff in /checkout/compiler/rustc_mir/src/const_eval/eval_queries.rs at line 311:
             // <https://github.com/rust-lang/rust/issues/71800>.
             let emit_as_lint = if let Some(def) = def.as_local() {
                 // (Associated) consts only emit a lint, since they might be unused.
-                matches!(tcx.def_kind(def.did.to_def_id()), DefKind::Const | DefKind::AssocConst) || {
-                    let mut did = def.did.to_def_id();
-                    loop {
-                        use rustc_middle::ty::DefIdTree;
-                        match tcx.parent(did) {
-                            Some(parent) => match tcx.def_kind(did) {
-                                DefKind::TyAlias => break true,
-                                _ => did = parent,
+                matches!(tcx.def_kind(def.did.to_def_id()), DefKind::Const | DefKind::AssocConst)
+                    || {
+                        let mut did = def.did.to_def_id();
+                        loop {
+                            use rustc_middle::ty::DefIdTree;
+                            match tcx.parent(did) {
+                                Some(parent) => match tcx.def_kind(did) {
+                                    DefKind::TyAlias => break true,
+                                    _ => did = parent,
+                                },
+                                None => break false,
                             }
-                            None => break false,
                     }
-                }
             } else {
             } else {
                 // use of broken constant from other crate: always an error
                 false
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/const_eval/error.rs" "/checkout/compiler/rustc_mir/src/const_eval/mod.rs" "/checkout/compiler/rustc_mir/src/const_eval/machine.rs" "/checkout/compiler/rustc_mir/src/const_eval/fn_queries.rs" "/checkout/compiler/rustc_mir/src/const_eval/eval_queries.rs" "/checkout/compiler/rustc_mir/src/lib.rs" "/checkout/compiler/rustc_mir/src/borrow_check/def_use.rs" "/checkout/compiler/rustc_mir/src/dataflow/move_paths/abs_domain.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
