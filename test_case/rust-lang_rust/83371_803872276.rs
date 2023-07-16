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
Diff in /checkout/library/core/src/num/mod.rs at line 870:
     };
 
     let mut result = T::from_u32(0);
-    if radix <= 16 && src.len() <= mem::size_of::<T>() * 2 - if is_signed_ty { 1 } else {0 } {
+    if radix <= 16 && src.len() <= mem::size_of::<T>() * 2 - if is_signed_ty { 1 } else { 0 } {
         // SAFETY: A little conservative for lower radixes but we know that
         // we can store any incoming number of that length
         // in type T without overflow.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/num/uint_macros.rs" "/checkout/library/core/src/num/mod.rs" "/checkout/library/core/src/num/int_macros.rs" "/checkout/library/core/src/stream/mod.rs" "/checkout/library/core/src/str/converts.rs" "/checkout/library/core/src/str/iter.rs" "/checkout/library/core/src/option.rs" "/checkout/library/core/src/num/f32.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
