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
Diff in /checkout/library/std/src/sys/unix/stack_overflow.rs at line 149:
         // is a no-op in most systems
         #[cfg(not(target_os = "openbsd"))]
         let flags = MAP_PRIVATE | MAP_ANON;
-        let stackp = mmap(
-            ptr::null_mut(),
-            SIGSTKSZ + page_size(),
-            flags,
-            -1,
-            0,
-        );
-        );
+        let stackp =
+            mmap(ptr::null_mut(), SIGSTKSZ + page_size(), PROT_READ | PROT_WRITE, flags, -1, 0);
         if stackp == MAP_FAILED {
             panic!("failed to allocate an alternative stack: {}", io::Error::last_os_error());
         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/unsupported/thread_local_dtor.rs" "/checkout/library/std/src/sys/unsupported/mutex.rs" "/checkout/library/std/src/sys/unsupported/env.rs" "/checkout/library/std/src/sys/mod.rs" "/checkout/library/std/src/sys/unix/stack_overflow.rs" "/checkout/library/std/src/sys/common/mod.rs" "/checkout/library/std/src/sys/common/alloc.rs" "/checkout/library/std/src/sys/unsupported/time.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
