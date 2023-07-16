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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/core/src/hash/sip.rs at line 244:
 #[unstable(feature = "hashmap_internals", issue = "none")]
 #[unstable(feature = "hashmap_internals", issue = "none")]
 #[rustc_const_unstable(feature = "const_hash", issue = "104061")]
-impl const super::Hasher for SipHasher13
+impl const super::Hasher for SipHasher13 {
     #[inline]
     #[inline]
     fn write(&mut self, msg: &[u8]) {
         self.hasher.write(msg)
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/panic_abort/src/lib.rs" "/checkout/library/panic_abort/src/android.rs" "/checkout/library/proc_macro/src/bridge/scoped_cell.rs" "/checkout/library/core/src/default.rs" "/checkout/library/core/tests/hash/mod.rs" "/checkout/library/core/tests/hash/sip.rs" "/checkout/library/core/src/hash/sip.rs" "/checkout/library/unwind/src/libunwind.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
