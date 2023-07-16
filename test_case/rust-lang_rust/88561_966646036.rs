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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/sys/unix/process/process_common.rs at line 407:
                     fd.duplicate()?
                 };
                 Ok((ChildStdio::Owned(fd), None))
+            }
 
             Stdio::MakePipe => {
             Stdio::MakePipe => {
                 let (reader, writer) = pipe::anon_pipe()?;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/unix/process/process_common/tests.rs" "/checkout/library/std/src/sys/unix/process/process_fuchsia.rs" "/checkout/library/std/src/sys/sgx/rwlock.rs" "/checkout/library/std/src/sys/unix/fd.rs" "/checkout/library/std/src/sys/sgx/net.rs" "/checkout/library/std/src/sys/unix/android.rs" "/checkout/library/std/src/sys/sgx/path.rs" "/checkout/library/std/src/sys/unix/process/process_common.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
