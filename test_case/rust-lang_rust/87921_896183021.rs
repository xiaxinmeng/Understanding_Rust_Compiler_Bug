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
Diff in /checkout/library/core/src/num/saturating.rs at line 82:
 #[allow(unused_macros)]
 #[allow(unused_macros)]
 macro_rules! sh_impl_signed {
-    ($t:ident, $f:ident) => {
-    };
-    };
+    ($t:ident, $f:ident) => {};
 
 
 macro_rules! sh_impl_unsigned {
Diff in /checkout/library/core/src/num/saturating.rs at line 91:
-    ($t:ident, $f:ident) => {
-    };
-    };
+    ($t:ident, $f:ident) => {};
 
 
 // FIXME (#23545): uncomment the remaining impls
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/num/shells/i64.rs" "/checkout/library/core/src/num/shells/u16.rs" "/checkout/library/core/src/num/shells/isize.rs" "/checkout/library/core/src/num/shells/i8.rs" "/checkout/library/core/src/num/shells/u128.rs" "/checkout/library/core/src/num/saturating.rs" "/checkout/library/core/src/num/int_log10.rs" "/checkout/library/core/src/num/shells/usize.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
