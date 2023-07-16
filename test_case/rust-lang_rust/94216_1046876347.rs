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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_target/src/abi/call/sparc64.rs at line 194:
 
                 arg.cast_to(CastTarget {
                     prefix: data.prefix,
-                    rest: Uniform {
-                        unit: Reg::i64(),
-                    },
-                    },
+                    rest: Uniform { unit: Reg::i64(), total: rest_size },
                     attrs: ArgAttributes {
                         regular: data.arg_attribute,
                         arg_ext: ArgExtension::None,
Diff in /checkout/compiler/rustc_target/src/abi/call/sparc64.rs at line 210:
     }
 
 
-    arg.cast_to(Uniform {
-        unit: Reg::i64(),
-        total,
-    });
+    arg.cast_to(Uniform { unit: Reg::i64(), total });
 
 
 pub fn compute_abi_info<'a, Ty, C>(cx: &C, fn_abi: &mut FnAbi<'a, Ty>)
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/abi/call/x86_64.rs" "/checkout/compiler/rustc_target/src/spec/powerpc_unknown_linux_musl.rs" "/checkout/compiler/rustc_target/src/abi/call/x86.rs" "/checkout/compiler/rustc_target/src/abi/call/m68k.rs" "/checkout/compiler/rustc_target/src/spec/aarch64_pc_windows_msvc.rs" "/checkout/compiler/rustc_target/src/abi/call/sparc64.rs" "/checkout/compiler/rustc_target/src/spec/armv7a_none_eabihf.rs" "/checkout/compiler/rustc_target/src/abi/call/s390x.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
