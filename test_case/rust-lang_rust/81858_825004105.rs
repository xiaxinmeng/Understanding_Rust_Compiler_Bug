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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/panicking.rs at line 258:
     static GLOBAL_PANIC_COUNT: AtomicUsize = AtomicUsize::new(0);
 
     pub fn increase() -> (bool, usize) {
-        (GLOBAL_PANIC_COUNT.fetch_add(1, Ordering::Acquire) & ALWAYS_ABORT_FLAG != 0,
-         LOCAL_PANIC_COUNT.with(|c| {
-             let next = c.get() + 1;
-             c.set(next);
-         }))
+        (
+        (
+            GLOBAL_PANIC_COUNT.fetch_add(1, Ordering::Acquire) & ALWAYS_ABORT_FLAG != 0,
+            LOCAL_PANIC_COUNT.with(|c| {
+                let next = c.get() + 1;
+                c.set(next);
+            }),
+        )
     }
 
 
     pub fn decrease() {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/os/freebsd/raw.rs" "/checkout/library/std/src/os/freebsd/mod.rs" "/checkout/library/std/src/panicking.rs" "/checkout/library/std/src/os/fuchsia/fs.rs" "/checkout/library/std/src/os/fuchsia/raw.rs" "/checkout/library/std/src/os/fuchsia/mod.rs" "/checkout/library/std/src/path.rs" "/checkout/library/std/src/os/freebsd/fs.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
