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
Diff in /checkout/compiler/rustc_ast_lowering/src/expr.rs at line 1354:
             .operands
             .iter()
             .map(|(op, op_sp)| {
-                let lower_reg = |reg| {
-                    match reg {
-                        InlineAsmRegOrRegClass::Reg(s) => {
-                            asm::InlineAsmRegOrRegClass::Reg(if let Some(asm_arch) = asm_arch {
-                                asm::InlineAsmReg::parse(
-                                    asm_arch,
-                                    |feature| {
-                                        sess.target_features.contains(&Symbol::intern(feature))
-                                    },
-                                    &sess.target,
-                                )
-                                )
-                                .unwrap_or_else(|e| {
-                                    let msg = format!("invalid register `{}`: {}", s.as_str(), e);
-                                    sess.struct_span_err(*op_sp, &msg).emit();
-                                    asm::InlineAsmReg::Err
-                            } else {
-                            } else {
+                let lower_reg = |reg| match reg {
+                    InlineAsmRegOrRegClass::Reg(s) => {
+                        asm::InlineAsmRegOrRegClass::Reg(if let Some(asm_arch) = asm_arch {
+                            asm::InlineAsmReg::parse(
+                                asm_arch,
+                                |feature| sess.target_features.contains(&Symbol::intern(feature)),
+                                &sess.target,
+                            )
+                            )
+                            .unwrap_or_else(|e| {
+                                let msg = format!("invalid register `{}`: {}", s.as_str(), e);
+                                sess.struct_span_err(*op_sp, &msg).emit();
                                 asm::InlineAsmReg::Err
-                        }
-                        }
-                        InlineAsmRegOrRegClass::RegClass(s) => {
-                            asm::InlineAsmRegOrRegClass::RegClass(if let Some(asm_arch) = asm_arch {
-                                asm::InlineAsmRegClass::parse(asm_arch, s)
-                                    .unwrap_or_else(|e| {
-                                        let msg = format!(
-                                            "invalid register class `{}`: {}",
-                                            s.as_str(),
-                                        );
-                                        );
-                                        sess.struct_span_err(*op_sp, &msg).emit();
-                                        asm::InlineAsmRegClass::Err
-                                } else {
-                                    asm::InlineAsmRegClass::Err
-                                }
-                            )
-                            )
-                        }
+                        } else {
+                            asm::InlineAsmReg::Err
+                    }
+                    }
+                    InlineAsmRegOrRegClass::RegClass(s) => {
+                        asm::InlineAsmRegOrRegClass::RegClass(if let Some(asm_arch) = asm_arch {
+                            asm::InlineAsmRegClass::parse(asm_arch, s).unwrap_or_else(|e| {
+                                let msg = format!("invalid register class `{}`: {}", s.as_str(), e);
+                                sess.struct_span_err(*op_sp, &msg).emit();
+                                asm::InlineAsmRegClass::Err
+                        } else {
+                            asm::InlineAsmRegClass::Err
+                        })
                     }
                     }
                 };
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc/src/main.rs" "/checkout/library/core/src/tuple.rs" "/checkout/library/core/src/result.rs" "/checkout/compiler/rustc_ast_lowering/src/lib.rs" "/checkout/compiler/rustc_ast_lowering/src/pat.rs" "/checkout/compiler/rustc_arena/src/tests.rs" "/checkout/compiler/rustc_ast_lowering/src/expr.rs" "/checkout/compiler/rustc_privacy/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
