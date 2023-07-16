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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/panic_abort/src/lib.rs at line 40:
 
 /// Shim to the relevant abort on the platform in question.
 fn do_abort() -> ! {
-    unsafe { abort(); }
+        abort();
+    }
 
 
     cfg_if::cfg_if! {
         if #[cfg(unix)] {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/tests/ptr.rs" "/checkout/library/core/tests/task.rs" "/checkout/library/term/src/terminfo/searcher.rs" "/checkout/library/core/tests/const_ptr.rs" "/checkout/library/unwind/src/libunwind.rs" "/checkout/library/panic_abort/src/lib.rs" "/checkout/library/unwind/src/lib.rs" "/checkout/library/core/tests/ops/control_flow.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:22
