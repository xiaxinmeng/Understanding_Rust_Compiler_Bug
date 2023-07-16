plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/sys_common/thread_local_key.rs at line 213:
 impl Drop for Key {
     fn drop(&mut self) {
         #[cfg(not(all(target_vendor = "fortanix", target_env = "sgx")))]
-        unsafe { imp::destroy(self.key) }
+        unsafe {
+            imp::destroy(self.key)
     }
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/os/wasi/io/fd.rs" "/checkout/library/std/src/sys_common/thread_info.rs" "/checkout/library/std/src/os/wasi/io/fd/tests.rs" "/checkout/library/std/src/sys_common/memchr.rs" "/checkout/library/std/src/sys_common/wtf8/tests.rs" "/checkout/library/std/src/sys_common/condvar.rs" "/checkout/library/std/src/sys_common/thread_parker/mod.rs" "/checkout/library/std/src/sys_common/thread_local_key.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
