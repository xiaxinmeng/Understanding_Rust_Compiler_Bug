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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/sys_common/condvar/check.rs at line 24:
     }
     pub fn verify(&self, mutex: &MovableMutex) {
         let addr = mutex.raw() as *const imp::Mutex as *const () as *mut _;
-        match self.addr.compare_exchange(ptr::null_mut(), addr, Ordering::Relaxed, Ordering::Relaxed)
+        match self.addr.compare_exchange(
+            ptr::null_mut(),
+            addr,
+            Ordering::Relaxed,
+            Ordering::Relaxed,
+            Ordering::Relaxed,
+        ) {
             Ok(_) => {}               // Stored the address
             Err(n) if n == addr => {} // Lost a race to store the same address
             _ => panic!("attempted to use a condition variable with two mutexes"),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys_common/memchr/tests.rs" "/checkout/library/std/src/sys_common/remutex.rs" "/checkout/library/std/src/sys_common/thread_local_key.rs" "/checkout/library/std/src/os/windows/raw.rs" "/checkout/library/std/src/sys_common/condvar/check.rs" "/checkout/library/std/src/sys_common/mutex.rs" "/checkout/library/std/src/sys_common/remutex/tests.rs" "/checkout/library/std/src/sys_common/thread_info.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
