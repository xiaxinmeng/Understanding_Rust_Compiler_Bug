plain
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/sys/windows/rand.rs at line 41:
     #[cfg(miri)]
     fn open() -> Result<Self, c::NTSTATUS> {
         const BCRYPT_RNG_ALG_HANDLE: c::BCRYPT_ALG_HANDLE = ptr::invalid_mut(0x81);
-        let _ = (c::BCryptOpenAlgorithmProvider, c::BCryptCloseAlgorithmProvider, c::BCRYPT_RNG_ALGORITHM, c::STATUS_NOT_SUPPORTED);
+        let _ = (
+            c::BCryptOpenAlgorithmProvider,
+            c::BCryptCloseAlgorithmProvider,
+            c::BCRYPT_RNG_ALGORITHM,
+            c::STATUS_NOT_SUPPORTED,
+        );
         Ok(Self(BCRYPT_RNG_ALG_HANDLE))
     }
     #[cfg(not(miri))]
Running `"/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/windows/handle/tests.rs" "/checkout/library/std/src/sys/unsupported/io.rs" "/checkout/library/std/src/sys/windows/stdio.rs" "/checkout/library/std/src/sys/unsupported/stdio.rs" "/checkout/library/std/src/sys/windows/rand.rs" "/checkout/library/std/src/sys/unsupported/thread.rs" "/checkout/library/std/src/sys/windows/locks/mutex.rs" "/checkout/library/std/src/sys/windows/io.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
