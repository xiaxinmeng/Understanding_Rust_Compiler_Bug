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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/core/src/iter/traits/iterator.rs at line 38:
         _Self = "[]",
         label = "`{Self}` is not an iterator; try calling `.into_iter()` or `.iter()`"
-    on(
-    on(
-        _Self = "&[]",
-        label = "`{Self}` is not an iterator; try calling `.iter()`"
-    ),
+    on(_Self = "&[]", label = "`{Self}` is not an iterator; try calling `.iter()`"),
     on(
         _Self = "&str",
         label = "`{Self}` is not an iterator; try calling `.chars()` or `.bytes()`"
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/iter/traits/accum.rs" "/checkout/library/core/src/iter/traits/mod.rs" "/checkout/library/core/src/iter/traits/exact_size.rs" "/checkout/library/core/src/iter/traits/double_ended.rs" "/checkout/library/core/src/iter/traits/marker.rs" "/checkout/library/core/src/iter/traits/iterator.rs" "/checkout/library/core/src/iter/traits/collect.rs" "/checkout/library/core/src/iter/sources.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
