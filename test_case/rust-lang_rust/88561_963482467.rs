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
Diff in /checkout/library/std/src/sys/unix/process/process_common.rs at line 398:
             Stdio::StaticFd(fd) => unsafe {
                 let duped = OwnedFd::from_raw_fd(cvt_r(|| libc::dup(fd))?);
                 Ok((ChildStdio::Owned(FileDesc::from_inner(duped)), None))
+            },
 
             Stdio::MakePipe => {
             Stdio::MakePipe => {
                 let (reader, writer) = pipe::anon_pipe()?;
Diff in /checkout/library/std/src/sys/windows/process.rs at line 364:
 impl Stdio {
 impl Stdio {
     fn to_handle(&self, stdio_id: c::DWORD, pipe: &mut Option<AnonPipe>) -> io::Result<Handle> {
-        let use_stdio_id = |stdio_id| {
-            match stdio::get_handle(stdio_id) {
-                Ok(io) => unsafe {
-                    let io = Handle::from_raw_handle(io);
-                    let ret = io.duplicate(0, true, c::DUPLICATE_SAME_ACCESS);
-                    io.into_raw_handle();
-                },
-                },
-                Err(..) => unsafe { Ok(Handle::from_raw_handle(c::INVALID_HANDLE_VALUE)) },
-            }
+        let use_stdio_id = |stdio_id| match stdio::get_handle(stdio_id) {
+            Ok(io) => unsafe {
+                let io = Handle::from_raw_handle(io);
+                let ret = io.duplicate(0, true, c::DUPLICATE_SAME_ACCESS);
+                io.into_raw_handle();
+            },
+            },
+            Err(..) => unsafe { Ok(Handle::from_raw_handle(c::INVALID_HANDLE_VALUE)) },
 
         match *self {
         match *self {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/unix/process/process_common/tests.rs" "/checkout/library/std/src/sys/unix/process/mod.rs" "/checkout/library/std/src/sys/unix/process/process_common.rs" "/checkout/library/std/src/sys/sgx/condvar.rs" "/checkout/library/std/src/sys/unix/os_str/tests.rs" "/checkout/library/std/src/sys/unix/process/process_vxworks.rs" "/checkout/library/std/src/sys/unix/process/process_unsupported.rs" "/checkout/library/std/src/sys/unix/android.rs"` failed.
Build completed unsuccessfully in 0:00:13
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
