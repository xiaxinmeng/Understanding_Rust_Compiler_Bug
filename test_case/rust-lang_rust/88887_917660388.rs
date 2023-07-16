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
Diff in /checkout/library/alloc/src/borrow.rs at line 331:
 
 #[stable(feature = "rust1", since = "1.0.0")]
 #[rustc_const_unstable(feature = "const_deref", issue = "none")]
-impl<B: ?Sized + ToOwned> const Deref for Cow<'_, B> where B::Owned: ~const Borrow<B> {
+impl<B: ?Sized + ToOwned> const Deref for Cow<'_, B>
+where
+    B::Owned: ~const Borrow<B>,
     type Target = B;
 
 
     fn deref(&self) -> &B {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/lib.rs" "/checkout/library/alloc/src/prelude/mod.rs" "/checkout/library/alloc/src/sync.rs" "/checkout/library/alloc/src/slice.rs" "/checkout/library/alloc/src/borrow.rs" "/checkout/library/alloc/src/prelude/v1.rs" "/checkout/library/alloc/src/task.rs" "/checkout/library/alloc/src/rc/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
