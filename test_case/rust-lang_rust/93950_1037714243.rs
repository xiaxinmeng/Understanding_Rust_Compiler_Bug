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
Diff in /checkout/library/std/src/sys/itron/error.rs at line 146:
 #[cold]
 #[cold]
 pub fn fail(e: impl fmt::Display, msg: &&str) -> ! {
-    if crate::thread::panicking() {
-        fail_aborting(e, msg)
-    } else {
-        panic!("{} failed: {e}", *msg)
-    }
+    if crate::thread::panicking() { fail_aborting(e, msg) } else { panic!("{} failed: {e}", *msg) }
 
 #[cold]
 #[cold]
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/alloc.rs" "/checkout/library/std/src/sys/sgx/condvar.rs" "/checkout/library/std/src/sys/itron/spin.rs" "/checkout/library/std/src/sys/unsupported/process.rs" "/checkout/library/std/src/sys/itron/error.rs" "/checkout/library/std/src/sys/unsupported/thread_local_key.rs" "/checkout/library/std/src/sys/itron/task.rs" "/checkout/library/std/src/sys/sgx/rwlock/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
