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
Diff in /checkout/compiler/rustc_mir_transform/src/const_prop_lint.rs at line 13:
 use rustc_hir::HirId;
 use rustc_index::bit_set::BitSet;
 use rustc_index::vec::IndexVec;
-use rustc_middle::mir::{self, ConstantKind};
 use rustc_middle::mir::visit::Visitor;
+use rustc_middle::mir::{self, ConstantKind};
 use rustc_middle::mir::{
     AssertKind, BinOp, Body, Constant, Local, LocalDecl, Location, Operand, Place, Rvalue,
     SourceInfo, SourceScope, SourceScopeData, Statement, StatementKind, Terminator, TerminatorKind,
Diff in /checkout/compiler/rustc_mir_transform/src/const_prop_lint.rs at line 303:
                 ConstantKind::Unevaluated(mir::UnevaluatedConst { promoted: Some(_), .. }, _)
             if need_note {
-                self.ecx
-                    .tcx
-                    .sess
-                    .sess
-                    .span_note_without_error(c.span, "erroneous constant used");
+                self.ecx.tcx.sess.span_note_without_error(c.span, "erroneous constant used");
         }
         res
         res
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_transform/src/const_prop_lint.rs" "/checkout/compiler/rustc_mir_transform/src/check_packed_ref.rs" "/checkout/compiler/rustc_mir_transform/src/ffi_unwind_calls.rs" "/checkout/compiler/rustc_mir_transform/src/deref_separator.rs" "/checkout/compiler/rustc_mir_transform/src/simplify_comparison_integral.rs" "/checkout/compiler/rustc_mir_transform/src/deduplicate_blocks.rs" "/checkout/compiler/rustc_mir_transform/src/add_retag.rs" "/checkout/compiler/rustc_infer/src/traits/error_reporting/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
