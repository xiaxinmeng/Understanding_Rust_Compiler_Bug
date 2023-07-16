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
Diff in /checkout/library/core/src/mem/mod.rs at line 664:
     // SAFETY: the caller must guarantee that an unitialized value is valid for `T`.
     // Because an uninitialized value does not guarantee any specific bit
     // representation, it is therefore no less safe to return a zeroed value.
-        zeroed::<T>()
-    }
-    }
+    unsafe { zeroed::<T>() }
 
 
 /// Swaps the values at two mutable locations, without deinitializing either one.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/mem/maybe_uninit.rs" "/checkout/library/core/src/fmt/nofloat.rs" "/checkout/library/std/src/io/stdio/tests.rs" "/checkout/library/std/src/io/tests.rs" "/checkout/library/std/src/io/buffered/bufreader.rs" "/checkout/library/std/src/io/util.rs" "/checkout/library/std/src/io/buffered/linewritershim.rs" "/checkout/library/core/src/mem/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
