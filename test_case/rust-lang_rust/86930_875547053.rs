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
Diff in /checkout/library/core/src/num/int_log10.rs at line 117:
 macro_rules! impl_checked {
     ($T:ident) => {
         pub const fn $T(val: $T) -> Option<$T> {
-            if val > 0 {
-                Some(unchecked::$T(val) as $T)
-                None
-            }
-            }
+            if val > 0 { Some(unchecked::$T(val) as $T) } else { None }
     };
 }
 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/f32.rs" "/checkout/library/std/src/lazy.rs" "/checkout/library/core/src/unit.rs" "/checkout/library/std/src/ascii.rs" "/checkout/library/core/src/unicode/mod.rs" "/checkout/library/core/src/num/error.rs" "/checkout/library/core/src/num/int_log10.rs" "/checkout/library/std/src/macros.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
