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
Diff in /checkout/library/std/src/sys/unix/args.rs at line 125:
             // Load ARGC and ARGV without a lock. If the store to either ARGV or
             // ARGC isn't visible yet, we'll return an empty argument list.
             let argv = ARGV.load(Ordering::Relaxed);
-            let argc = if argv.is_null() {
-            } else {
-            } else {
-                ARGC.load(Ordering::Relaxed)
-            };
+            let argc = if argv.is_null() { 0 } else { ARGC.load(Ordering::Relaxed) };
             (0..argc)
                 .map(|i| {
                     let cstr = CStr::from_ptr(*argv.offset(i) as *const libc::c_char);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/unix/condvar.rs" "/checkout/library/std/src/sys/unix/process/mod.rs" "/checkout/library/std/src/sys/unix/pipe.rs" "/checkout/library/std/src/sys/unix/alloc.rs" "/checkout/library/std/src/sys/unix/process/process_fuchsia.rs" "/checkout/library/std/src/sys/unix/args.rs" "/checkout/library/std/src/sys/unix/process/zircon.rs" "/checkout/library/std/src/sys/unix/kernel_copy/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
