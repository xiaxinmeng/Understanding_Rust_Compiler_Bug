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
extracting /checkout/obj/build/cache/2022-07-21/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/library/std/src/sys/unix/thread.rs at line 265:
 
 impl Drop for Thread {
     fn drop(&mut self) {
-        let _= unsafe { libc::pthread_detach(self.id) };
+        let _ = unsafe { libc::pthread_detach(self.id) };
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/unix/thread_local_key.rs" "/checkout/library/std/src/sys/unix/thread.rs" "/checkout/library/std/src/sys/unix/futex.rs" "/checkout/library/std/src/sys/unix/alloc.rs" "/checkout/library/std/src/sys/unix/io.rs" "/checkout/library/std/src/sys/hermit/args.rs" "/checkout/library/std/src/sys/sgx/mod.rs" "/checkout/library/std/src/sys/unix/os_str.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', thread 'format.rs:166:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
