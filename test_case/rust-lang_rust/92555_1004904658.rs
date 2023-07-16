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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/thread/scoped.rs at line 116:
     // Throw any panic from `f`, or the return value of `f` if no thread panicked.
     match result {
         Err(e) => resume_unwind(e),
-        Ok(_) if scope.data.a_thread_panicked.load(Ordering::Relaxed) => panic!("a thread panicked"),
+        Ok(_) if scope.data.a_thread_panicked.load(Ordering::Relaxed) => {
+            panic!("a thread panicked")
         Ok(result) => result,
     }
 }
 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/io/tests.rs" "/checkout/library/std/src/io/readbuf.rs" "/checkout/library/std/src/io/impls.rs" "/checkout/library/std/src/error/tests.rs" "/checkout/library/std/src/thread/local.rs" "/checkout/library/std/src/thread/scoped.rs" "/checkout/library/std/src/thread/local/dynamic_tests.rs" "/checkout/library/std/src/io/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
