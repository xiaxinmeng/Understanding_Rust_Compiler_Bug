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
Diff in /checkout/compiler/rustc_mir/src/transform/lower_intrinsics.rs at line 55:
                         );
                         drop(args);
 
-                        let stmt = if let (Some(1), Some(src), Some(dst)) = (eval_operand_to_usize(tcx, param_env, &count), src.place(), dst.place()) {
+                        let stmt = if let (Some(1), Some(src), Some(dst)) = (
+                            eval_operand_to_usize(tcx, param_env, &count),
+                            src.place(),
+                            dst.place(),
+                        ) {
                             StatementKind::Assign(box (
                                 tcx.mk_place_deref(dst),
-                                Rvalue::Use(Operand::Copy(tcx.mk_place_deref(src)))
+                                Rvalue::Use(Operand::Copy(tcx.mk_place_deref(src))),
                         } else {
                             StatementKind::CopyNonOverlapping(
                             StatementKind::CopyNonOverlapping(
Diff in /checkout/compiler/rustc_mir/src/transform/lower_intrinsics.rs at line 65:
-                                box rustc_middle::mir::CopyNonOverlapping {
-                                    src, dst, count
-                                }
+                                box rustc_middle::mir::CopyNonOverlapping { src, dst, count },
                         };
 
 
Diff in /checkout/compiler/rustc_mir/src/transform/lower_intrinsics.rs at line 71:
-                        block.statements.push(Statement {
-                            source_info: terminator.source_info,
-                            kind: stmt,
-                        });
+                            .statements
+                            .statements
+                            .push(Statement { source_info: terminator.source_info, kind: stmt });
 
                         terminator.kind = TerminatorKind::Goto { target };
                     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/transform/coverage/counters.rs" "/checkout/compiler/rustc_mir/src/transform/const_prop.rs" "/checkout/compiler/rustc_mir/src/transform/lower_intrinsics.rs" "/checkout/compiler/rustc_mir/src/transform/simplify_branches.rs" "/checkout/compiler/rustc_mir/src/transform/nrvo.rs" "/checkout/compiler/rustc_mir/src/util/pretty.rs" "/checkout/compiler/rustc_mir/src/transform/uninhabited_enum_branching.rs" "/checkout/compiler/rustc_mir/src/transform/check_unsafety.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:13
