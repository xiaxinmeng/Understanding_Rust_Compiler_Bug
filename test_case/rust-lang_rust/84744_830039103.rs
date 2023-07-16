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
Diff in /checkout/library/std/src/sys/windows/mod.rs at line 71:
         c::ERROR_PATH_NOT_FOUND => return ErrorKind::NotFound,
         c::ERROR_NO_DATA => return ErrorKind::BrokenPipe,
         c::ERROR_INVALID_PARAMETER => return ErrorKind::InvalidInput,
-        c::ERROR_NOT_ENOUGH_MEMORY
-        | c::ERROR_OUTOFMEMORY => return ErrorKind::OutOfMemory,
+        c::ERROR_NOT_ENOUGH_MEMORY | c::ERROR_OUTOFMEMORY => return ErrorKind::OutOfMemory,
         c::ERROR_SEM_TIMEOUT
         | c::WAIT_TIMEOUT
         | c::ERROR_DRIVER_CANCEL_TIMEOUT
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/windows/mod.rs" "/checkout/library/std/src/sys/hermit/mod.rs" "/checkout/library/std/src/sys/windows/os/tests.rs" "/checkout/library/std/src/sys/windows/memchr.rs" "/checkout/library/std/src/sys/windows/alloc/tests.rs" "/checkout/library/std/src/sys/windows/path/tests.rs" "/checkout/library/std/src/sys/windows/thread_parker.rs" "/checkout/library/std/src/sys/hermit/mutex.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:13
