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
##################                                                        25.7%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-06-29/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_codegen_llvm/src/abi.rs at line 577:
             let arg_ty = self.args[element_type_index as usize].layout.ty;
             let pointee_ty = arg_ty.builtin_deref(true).expect("Must be pointer argument").ty;
             let element_type_attr = unsafe {
-                llvm::LLVMRustCreateElementTypeAttr(
-                    bx.llcx,
-                    bx.layout_of(pointee_ty).llvm_type(bx),
-                )
+                llvm::LLVMRustCreateElementTypeAttr(bx.llcx, bx.layout_of(pointee_ty).llvm_type(bx))
             attributes::apply_to_callsite(
                 callsite,
                 callsite,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/metadata/enums/native.rs" "/checkout/compiler/rustc_codegen_llvm/src/allocator.rs" "/checkout/compiler/rustc_codegen_llvm/src/value.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/mod.rs" "/checkout/compiler/rustc_codegen_llvm/src/consts.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/gdb.rs" "/checkout/compiler/rustc_codegen_llvm/src/abi.rs" "/checkout/compiler/rustc_codegen_llvm/src/debuginfo/metadata/enums/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
