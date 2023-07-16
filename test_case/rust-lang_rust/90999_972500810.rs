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
Diff in /checkout/compiler/rustc_const_eval/src/interpret/intrinsics.rs at line 425:
 
                 for i in 0..dest_len {
                     let place = self.mplace_index(&dest, i)?;
-                    let value = if i == index { *elem } else { self.mplace_index(&input, i)?.into() };
+                    let value =
+                        if i == index { *elem } else { self.mplace_index(&input, i)?.into() };
                     self.copy_op(&value, &place.into())?;
             }
             }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_const_eval/src/interpret/intrinsics/type_name.rs" "/checkout/compiler/rustc_const_eval/src/interpret/cast.rs" "/checkout/compiler/rustc_const_eval/src/interpret/traits.rs" "/checkout/compiler/rustc_const_eval/src/interpret/step.rs" "/checkout/compiler/rustc_const_eval/src/interpret/intrinsics.rs" "/checkout/compiler/rustc_const_eval/src/util/find_self_call.rs" "/checkout/compiler/rustc_const_eval/src/interpret/terminator.rs" "/checkout/compiler/rustc_const_eval/src/interpret/visitor.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
