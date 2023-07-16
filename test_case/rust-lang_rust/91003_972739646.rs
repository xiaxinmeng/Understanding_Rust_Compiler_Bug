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
Diff in /checkout/compiler/rustc_codegen_llvm/src/abi.rs at line 181:
         let mut args: Vec<_> = self
             .prefix
             .iter()
-            .flat_map(|option_kind| {
-                option_kind.map(|reg| reg.llvm_type(cx))
-            })
+            .flat_map(|option_kind| option_kind.map(|reg| reg.llvm_type(cx)))
             .chain((0..rest_count).map(|_| rest_ll_unit))
             .collect();
Diff in /checkout/compiler/rustc_codegen_llvm/src/abi.rs at line 537:
                 }
             }
             PassMode::Cast(cast) => {
             PassMode::Cast(cast) => {
-                cast.attrs.apply_attrs_to_callsite(llvm::AttributePlace::ReturnValue, &bx.cx, callsite);
+                cast.attrs.apply_attrs_to_callsite(
+                    llvm::AttributePlace::ReturnValue,
+                    &bx.cx,
+                );
             }
             _ => {}
         }
         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_llvm/src/coverageinfo/mapgen.rs" "/checkout/compiler/rustc_codegen_llvm/src/abi.rs" "/checkout/compiler/rustc_codegen_llvm/src/declare.rs" "/checkout/compiler/rustc_lexer/src/lib.rs" "/checkout/compiler/rustc_codegen_llvm/src/mono_item.rs" "/checkout/compiler/rustc_codegen_llvm/src/back/archive.rs" "/checkout/compiler/rustc_lexer/src/unescape/tests.rs" "/checkout/compiler/rustc_codegen_llvm/src/value.rs"` failed.
Build completed unsuccessfully in 0:00:13
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
