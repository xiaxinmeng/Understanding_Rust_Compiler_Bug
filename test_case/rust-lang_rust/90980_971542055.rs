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
Diff in /checkout/library/core/src/char/decode.rs at line 74:
 #[inline]
 #[unstable(feature = "decode_utf8", issue = "none")]
 pub fn decode_utf8<I: IntoIterator<Item = u8>>(iter: I) -> DecodeUtf8<I::IntoIter> {
-    DecodeUtf8 {
-        iter: iter.into_iter(),
-        buf: DecodeUtf8Buffer::Empty,
-    }
+    DecodeUtf8 { iter: iter.into_iter(), buf: DecodeUtf8Buffer::Empty }
 
 #[derive(Clone, Debug)]
 #[derive(Clone, Debug)]
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/char/decode.rs" "/checkout/library/core/src/prelude/v1.rs" "/checkout/library/core/src/char/mod.rs" "/checkout/library/core/src/lib.rs" "/checkout/library/core/src/stream/mod.rs" "/checkout/library/core/src/iter/range.rs" "/checkout/library/core/src/iter/sources/from_fn.rs" "/checkout/library/core/src/prelude/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
