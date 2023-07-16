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
###########################                                               38.5%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-06-29/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/library/std/src/lib.rs at line 583:
 #[stable(feature = "rust1", since = "1.0.0")]
 #[allow(deprecated, deprecated_in_future)]
 pub use core::{
-    assert_eq, assert_ne, debug_assert, debug_assert_eq, debug_assert_ne, matches, r#try, todo,
+    assert_eq, assert_ne, debug_assert, debug_assert_eq, debug_assert_ne, matches, todo, r#try,
     unimplemented, unreachable, write, writeln,
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/os/freebsd/raw.rs" "/checkout/library/std/src/os/freebsd/mod.rs" "/checkout/library/std/src/os/macos/raw.rs" "/checkout/library/std/src/os/freebsd/fs.rs" "/checkout/library/std/src/rt.rs" "/checkout/library/std/src/lib.rs" "/checkout/library/std/src/f32/tests.rs" "/checkout/library/std/src/os/macos/fs.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
