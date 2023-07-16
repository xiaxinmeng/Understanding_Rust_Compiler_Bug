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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/core/src/num/mod.rs at line 896:
     };
 
     let mut result = T::from_u32(0);
-    if radix <= 16
-        && src.len()
-            <= mem::size_of::<T>() * 2 - (is_signed_ty && is_positive) as usize
+    if radix <= 16 && src.len() <= mem::size_of::<T>() * 2 - (is_signed_ty && is_positive) as usize
     {
         // SAFETY: Consider the highest radix of 16:
         // u8 max out by ff which is 2 characters, u16 max out by ffff which is 4 characters
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/num/flt2dec/strategy/dragon.rs" "/checkout/library/core/src/num/mod.rs" "/checkout/library/core/src/num/error.rs" "/checkout/library/core/src/num/bignum.rs" "/checkout/library/core/src/num/f64.rs" "/checkout/library/core/src/num/wrapping.rs" "/checkout/library/core/src/num/shells/i32.rs" "/checkout/library/core/src/num/flt2dec/strategy/grisu.rs"` failed.
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
