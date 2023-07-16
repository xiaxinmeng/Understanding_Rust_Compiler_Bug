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
Diff in /checkout/library/core/src/bool.rs at line 15:
     #[unstable(feature = "bool_to_option", issue = "80967")]
     #[inline]
     pub fn then_some<T>(self, t: T) -> Option<T> {
-        if self {
-            Some(t)
-            None
-        }
-        }
+        if self { Some(t) } else { None }
 
 
     /// Returns `Some(f())` if the `bool` is `true`, or `None` otherwise.
Diff in /checkout/library/core/src/bool.rs at line 33:
     #[stable(feature = "lazy_bool_to_option", since = "1.50.0")]
     #[inline]
     pub fn then<T, F: FnOnce() -> T>(self, f: F) -> Option<T> {
-        if self {
-            Some(f())
-            None
-        }
-        }
+        if self { Some(f()) } else { None }
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/collections/mod.rs" "/checkout/library/core/src/bool.rs" "/checkout/library/std/src/collections/hash/mod.rs" "/checkout/library/core/src/default.rs" "/checkout/library/core/src/array/mod.rs" "/checkout/library/core/src/ptr/mod.rs" "/checkout/library/core/src/cell.rs" "/checkout/library/std/src/error.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
