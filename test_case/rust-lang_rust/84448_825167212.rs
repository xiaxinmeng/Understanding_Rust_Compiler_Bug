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
Diff in /checkout/library/std/src/sys/common/time.rs at line 1:
 use crate::env;
-use crate::sync::atomic::{Ordering, AtomicU8};
+use crate::sync::atomic::{AtomicU8, Ordering};
 #[repr(u8)]
 #[derive(Copy, Clone)]
Diff in /checkout/library/std/src/sys/common/time.rs at line 7:
Diff in /checkout/library/std/src/sys/common/time.rs at line 7:
     // 0 is the placeholder for initialization pending
     Default = 1,
     AssumeMonotonic = 2,
-    AssumeBroken = 3
+    AssumeBroken = 3,
 
 
 static INSTANT_RELIABILITY: AtomicU8 = AtomicU8::new(0);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/windows/alloc.rs" "/checkout/library/std/src/sys/unsupported/mod.rs" "/checkout/library/std/src/sys/common/time.rs" "/checkout/library/std/src/sys/sgx/thread.rs" "/checkout/library/std/src/sys/sgx/net.rs" "/checkout/library/std/src/sys/common/alloc.rs" "/checkout/library/std/src/sys/sgx/mutex.rs" "/checkout/library/std/src/sys/windows/env.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
