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
Diff in /checkout/compiler/rustc_codegen_ssa/src/mir/rvalue.rs at line 191:
                     mir::CastKind::PointerFromExposedAddress => {
                         assert!(bx.cx().is_backend_immediate(cast));
                         let lladdr = operand.immediate();
-                        let usize_lladdr = bx.intcast(lladdr, bx.cx().type_isize(), /*signed*/ false);
+                        let usize_lladdr =
+                            bx.intcast(lladdr, bx.cx().type_isize(), /*signed*/ false);
                         let llcast_ty = bx.cx().immediate_backend_type(cast);
                         let llptr = bx.inttoptr(usize_lladdr, llcast_ty);
                         OperandValue::Immediate(llptr)
Diff in /checkout/compiler/rustc_codegen_cranelift/src/base.rs at line 608:
                     lval.write_cvalue(fx, operand.cast_pointer_to(to_layout));
                 Rvalue::Cast(
                 Rvalue::Cast(
-                    CastKind::Misc | CastKind::PointerExposeAddress | CastKind::PointerFromExposedAddress,
+                    CastKind::Misc
+                    | CastKind::PointerExposeAddress
+                    | CastKind::PointerFromExposedAddress,
                     ref operand,
                     to_ty,
                 ) => {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/mir/mod.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/analyze.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/rvalue.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/debuginfo.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/coverageinfo.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/block.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/statement.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/operand.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
