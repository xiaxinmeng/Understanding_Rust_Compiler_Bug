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
Diff in /checkout/library/std/src/sys/unix/process/process_unix.rs at line 335:
             cvt(libc::pthread_sigmask(libc::SIG_SETMASK, set.as_ptr(), ptr::null_mut()))?;
             let mut action: libc::sigaction = mem::zeroed();
             action.sa_sigaction = libc::SIG_DFL;
-            cvt(libc::sigaction(libc::SIGPIPE, &action as * const _, ptr::null_mut()))?;
+            cvt(libc::sigaction(libc::SIGPIPE, &action as *const _, ptr::null_mut()))?;
 
         for callback in self.get_closures().iter_mut() {
         for callback in self.get_closures().iter_mut() {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/unix/process/process_vxworks.rs" "/checkout/library/std/src/sys/unix/process/process_unsupported.rs" "/checkout/library/std/src/sys/unix/thread_local_key.rs" "/checkout/library/std/src/sys/unsupported/condvar.rs" "/checkout/library/std/src/sys/unix/cmath.rs" "/checkout/library/std/src/sys/unsupported/pipe.rs" "/checkout/library/std/src/sys/unix/thread_local_dtor.rs" "/checkout/library/std/src/sys/unix/process/process_unix.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
