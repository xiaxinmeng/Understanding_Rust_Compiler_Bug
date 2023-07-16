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
Diff in /checkout/library/core/src/sync/atomic.rs at line 2640:
             _ => panic!("a failure ordering can't be stronger than a success ordering"),
     };
-    if ok {
-        Ok(val)
-    } else {
-    } else {
-        Err(val)
-    }
+    if ok { Ok(val) } else { Err(val) }
 
 #[inline]
Diff in /checkout/library/core/src/sync/atomic.rs at line 2673:
Diff in /checkout/library/core/src/sync/atomic.rs at line 2673:
             _ => panic!("a failure ordering can't be stronger than a success ordering"),
     };
-    if ok {
-        Ok(val)
-    } else {
-    } else {
-        Err(val)
-    }
+    if ok { Ok(val) } else { Err(val) }
 
 #[inline]
 #[inline]
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/sync/mod.rs" "/checkout/library/core/src/iter/adapters/copied.rs" "/checkout/library/core/src/sync/atomic.rs" "/checkout/library/core/src/iter/adapters/skip_while.rs" "/checkout/library/core/src/iter/adapters/take_while.rs" "/checkout/library/core/src/array/iter.rs" "/checkout/library/core/src/iter/adapters/fuse.rs" "/checkout/library/core/src/time.rs"` failed.
Build completed unsuccessfully in 0:00:12
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
