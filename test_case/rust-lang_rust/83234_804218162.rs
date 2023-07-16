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
Diff in /checkout/compiler/rustc_codegen_ssa/src/mir/operand.rs at line 102:
                     ty::Str => {
                         let s = valtree.unwrap_str().as_str();
                         encode_slice(s.as_bytes())
+                    }
+                    }
                     ty::Slice(elem_ty) if elem_ty == bx.tcx().types.u8 => {
                         let s: Vec<u8> = valtree
                             .unwrap_branch()
Diff in /checkout/compiler/rustc_codegen_ssa/src/mir/operand.rs at line 110:
                             .map(|b| u8::try_from(b.unwrap_leaf()).unwrap())
                             .collect();
                         encode_slice(&s)
+                    }
+                    }
                     ty::Array(elem_ty, _) if elem_ty == bx.tcx().types.u8 => {
                         let s: Vec<u8> = valtree
                             .unwrap_branch()
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/traits/asm.rs" "/checkout/compiler/rustc_codegen_ssa/src/traits/builder.rs" "/checkout/compiler/rustc_codegen_ssa/src/common.rs" "/checkout/compiler/rustc_feature/src/accepted.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/coverageinfo.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/symbol_export.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/lto.rs" "/checkout/compiler/rustc_codegen_ssa/src/mir/operand.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
