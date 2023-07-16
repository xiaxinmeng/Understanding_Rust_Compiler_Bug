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
Diff in /checkout/library/core/src/num/mod.rs at line 893:
 
     let mut result = T::from_u32(0);
 
-    if radix <= 16 && digits.len() <= mem::size_of::<T>() * 2 - is_signed_ty as usize
-    {
+    if radix <= 16 && digits.len() <= mem::size_of::<T>() * 2 - is_signed_ty as usize {
         // SAFETY: Consider the highest radix of 16:
         // `u8::MAX` is `ff` (2 characters), `u16::MAX` is `ffff` (4 characters)
         // We can be sure that any src len of 2 would fit in a u8 so we don't need
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/num/dec2flt/rawfp.rs" "/checkout/library/core/src/num/nonzero.rs" "/checkout/library/core/src/num/error.rs" "/checkout/library/core/src/num/mod.rs" "/checkout/library/core/src/num/int_macros.rs" "/checkout/library/core/src/num/bignum.rs" "/checkout/library/core/src/num/f64.rs" "/checkout/library/core/src/num/flt2dec/strategy/grisu.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
