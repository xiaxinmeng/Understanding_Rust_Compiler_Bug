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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/error.rs at line 489:
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/error.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
 
 #[stable(feature = "arc_error", since = "1.52.0")]
 impl<T: Error + ?Sized> Error for Arc<T> {
-
     #[allow(deprecated, deprecated_in_future)]
     fn description(&self) -> &str {
         Error::description(&**self)
Diff in /checkout/library/std/src/error.rs at line 511:
 
 #[stable(feature = "error_by_ref", since = "1.51.0")]
 impl<'a, T: Error + ?Sized> Error for &'a T {
-
     #[allow(deprecated, deprecated_in_future)]
     fn description(&self) -> &str {
         Error::description(&**self)
Build completed unsuccessfully in 0:00:16
