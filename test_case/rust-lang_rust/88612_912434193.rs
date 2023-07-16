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
Diff in /checkout/library/std/src/sync/mpsc/shared.rs at line 248:
     // Returns true if blocking should proceed.
     fn decrement(&self, token: SignalToken) -> StartResult {
         unsafe {
-            assert_eq!(self.to_wake.load(Ordering::SeqCst), 0, "This is a known bug in rust. See https://github.com/rust-lang/rust/issues/39364");
+            assert_eq!(
+                self.to_wake.load(Ordering::SeqCst),
+                0,
+                "This is a known bug in rust. See https://github.com/rust-lang/rust/issues/39364"
             let ptr = token.cast_to_usize();
             let ptr = token.cast_to_usize();
             self.to_wake.store(ptr, Ordering::SeqCst);
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sync/mpsc/sync.rs" "/checkout/library/std/src/sync/mpsc/blocking.rs" "/checkout/library/std/src/sync/mpsc/stream.rs" "/checkout/library/std/src/sync/mpsc/shared.rs" "/checkout/library/std/src/sync/mpsc/spsc_queue/tests.rs" "/checkout/library/std/src/sync/mpsc/sync_tests.rs" "/checkout/library/std/src/sync/mpsc/mod.rs" "/checkout/library/std/src/sync/mpsc/cache_aligned.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
