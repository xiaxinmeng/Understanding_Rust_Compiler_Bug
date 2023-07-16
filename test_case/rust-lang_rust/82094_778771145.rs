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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/core/src/char/methods.rs at line 332:
     #[inline]
     pub fn to_digit(self, radix: u32) -> Option<u32> {
         assert!(radix <= 36, "to_digit: radix is too high (maximum 36)");
-        const ASCII_DIGIT_MASK : u32 = 0b11_0000;
+        const ASCII_DIGIT_MASK: u32 = 0b11_0000;
         // the code is split up here to improve execution speed for cases where
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/char/methods.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
         // the `radix` is constant and 10 or smaller
         let val = if likely(radix <= 10) {
Build completed unsuccessfully in 0:00:16
