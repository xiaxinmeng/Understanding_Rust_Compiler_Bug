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
Diff in /checkout/library/std/src/sync/mpmc/utils.rs at line 111:
     #[inline]
     pub fn spin(&self) {
         let step = self.step.get().min(SPIN_LIMIT);
-        for _ in 0..step.pow(2)  {
+        for _ in 0..step.pow(2) {
             crate::hint::spin_loop();
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sync/mpmc/counter.rs" "/checkout/library/std/src/sync/condvar/tests.rs" "/checkout/library/std/src/sync/mpmc/zero.rs" "/checkout/library/std/src/sync/mpmc/context.rs" "/checkout/library/std/src/sync/mpmc/utils.rs" "/checkout/library/std/src/sys_common/thread_local_key/tests.rs" "/checkout/library/std/src/sync/mpmc/mod.rs" "/checkout/library/std/src/sync/mpmc/select.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
