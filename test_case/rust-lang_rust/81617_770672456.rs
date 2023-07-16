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
Diff in /checkout/library/alloc/src/vec/drain.rs at line 100:
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/vec/drain.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
         // implementation is thus safe, and reading the value is safe because
         // `Self: TrustedRandomAccess` implies `T: Copy` so the `Drop` impl below
         // won't cause each item to be dropped twice.
-        unsafe {
-            ptr::read(self.iter.__iterator_get_unchecked(idx) as *const _)
-        }
+        unsafe { ptr::read(self.iter.__iterator_get_unchecked(idx) as *const _) }
 }
 
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:20
