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
Diff in /checkout/library/core/tests/iter/traits/iterator.rs at line 469:
     assert_eq!(sum, Some(None));
 
     let v = vec!["1", "2", "3", "4", "5"];
-    let max = v.into_iter().try_reduce(|x, y| {
-        if x.len() > y.len() {
-            Some(x)
-            Some(y)
-        }
-    });
-    });
+    let max = v.into_iter().try_reduce(|x, y| if x.len() > y.len() { Some(x) } else { Some(y) });
     assert_eq!(max, Some(Some("5")));
 
     let v = vec!["1", "2", "3", "4", "5"];
Diff in /checkout/library/core/tests/iter/traits/iterator.rs at line 482:
-    let max: Result<Option<_>, <usize as std::str::FromStr>::Err> = v.into_iter().try_reduce(|x, y| {
-        if x.parse::<usize>()? > y.parse::<usize>()? {
-            Ok(x)
-            Ok(y)
-        }
-    });
-    });
+    let max: Result<Option<_>, <usize as std::str::FromStr>::Err> =
+        v.into_iter().try_reduce(|x, y| {
+            if x.parse::<usize>()? > y.parse::<usize>()? { Ok(x) } else { Ok(y) }
+        });
     assert_eq!(max, Ok(Some("5")));
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/tests/iter/adapters/flat_map.rs" "/checkout/library/core/tests/iter/traits/iterator.rs" "/checkout/library/core/tests/iter/adapters/mod.rs" "/checkout/library/core/tests/iter/traits/accum.rs" "/checkout/library/core/tests/iter/adapters/cloned.rs" "/checkout/library/core/tests/iter/traits/step.rs" "/checkout/library/core/tests/iter/traits/mod.rs" "/checkout/library/core/tests/iter/traits/double_ended.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
