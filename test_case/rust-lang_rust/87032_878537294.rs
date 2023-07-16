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
Diff in /checkout/library/core/src/mem/mod.rs at line 686:
     #[cfg(not(any(miri, sanitize = "memory")))]
     // SAFETY: Because an uninitialized value does not guarantee any specific bit
     // representation, it is therefore no less safe to return a zeroed value.
-    unsafe { zeroed::<T>() }
+        zeroed::<T>()
+    }
 }
 
 
 /// Swaps the values at two mutable locations, without deinitializing either one.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/collections/btree/borrow.rs" "/checkout/library/core/src/alloc/global.rs" "/checkout/library/core/src/alloc/layout.rs" "/checkout/library/alloc/src/collections/btree/set/tests.rs" "/checkout/library/alloc/src/collections/btree/set.rs" "/checkout/library/core/src/mem/mod.rs" "/checkout/library/core/src/ptr/non_null.rs" "/checkout/library/core/src/alloc/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
