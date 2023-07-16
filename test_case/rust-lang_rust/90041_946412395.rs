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
Diff in /checkout/library/core/src/intrinsics.rs at line 2071:
     }
     #[cfg(all(not(bootstrap), debug_assertions))]
     // SAFETY: referential transparency is upheld by internal miri cheks
-    unsafe { const_eval_select((src, dst, count), compiletime_check, runtime_check) };
+    unsafe {
+        const_eval_select((src, dst, count), compiletime_check, runtime_check)
 
 
     // SAFETY: the safety contract for `copy_nonoverlapping` must be
     // upheld by the caller.
Diff in /checkout/library/core/src/intrinsics.rs at line 2161:
     }
     #[cfg(all(not(bootstrap), debug_assertions))]
     // SAFETY: referential transparency is upheld by internal miri cheks
-    unsafe { const_eval_select((src, dst, count), compiletime_check, runtime_check) };
+    unsafe {
+        const_eval_select((src, dst, count), compiletime_check, runtime_check)
 
 
     // SAFETY: the safety contract for `copy` must be upheld by the caller.
     unsafe { copy(src, dst, count) }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/char/convert.rs" "/checkout/library/core/src/char/mod.rs" "/checkout/library/core/tests/lib.rs" "/checkout/library/core/src/intrinsics.rs" "/checkout/library/core/tests/mem.rs" "/checkout/library/core/src/char/decode.rs" "/checkout/library/core/src/char/methods.rs" "/checkout/library/core/src/slice/iter.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
