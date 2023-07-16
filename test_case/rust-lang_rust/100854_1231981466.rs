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
Diff in /checkout/compiler/rustc_mir_build/src/thir/pattern/check_match.rs at line 994:
         let mut occurences = vec![];
         for (span, name) in conflicts_mut_mut {
         for (span, name) in conflicts_mut_mut {
-            occurences.push(MultipleMutBorrowOccurence::Mutable {
-                name,
-            });
-            });
+            occurences.push(MultipleMutBorrowOccurence::Mutable { span, name });
         for (span, name) in conflicts_mut_ref {
         for (span, name) in conflicts_mut_ref {
-            occurences.push(MultipleMutBorrowOccurence::Immutable {
-                name,
-            });
-            });
+            occurences.push(MultipleMutBorrowOccurence::Immutable { span, name });
         for (span, name) in conflicts_move {
         for (span, name) in conflicts_move {
-            occurences.push(MultipleMutBorrowOccurence::Moved {
-                name,
-            });
-            });
+            occurences.push(MultipleMutBorrowOccurence::Moved { span, name });
 
 
-        sess.emit_err(MultipleMutBorrows {
-            span: pat.span,
-            occurences,
-            name,
-        });
-        });
+        sess.emit_err(MultipleMutBorrows { span: pat.span, binding_span, occurences, name });
     } else if !conflicts_mut_ref.is_empty() {
         // Report mutability conflicts for e.g. `ref x @ Some(ref mut y)` or the converse.
         let (primary, also) = match mut_outer {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_interface/src/passes.rs" "/checkout/compiler/rustc_mir_build/src/thir/pattern/check_match.rs" "/checkout/compiler/rustc_mir_build/src/thir/constant.rs" "/checkout/library/proc_macro/src/lib.rs" "/checkout/compiler/rustc_lint/src/nonstandard_style.rs" "/checkout/compiler/rustc_lint/src/levels.rs" "/checkout/compiler/rustc_lint/src/types.rs" "/checkout/compiler/rustc_mir_build/src/thir/pattern/deconstruct_pat.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
