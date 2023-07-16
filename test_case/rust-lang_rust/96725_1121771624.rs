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
Diff in /checkout/library/std/src/sys/windows/process/tests.rs at line 26:
 #[test]
 fn test_thread_handle() {
 fn test_thread_handle() {
-    use crate::os::windows::process::{ChildExt, CommandExt};
     use crate::os::windows::io::AsRawHandle;
+    use crate::os::windows::process::{ChildExt, CommandExt};
     const CREATE_SUSPENDED: u32 = 0x00000004;
 
     let p = Command::new("cmd").args(&["/C", "exit 0"]).creation_flags(CREATE_SUSPENDED).spawn();
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/windows/process/tests.rs" "/checkout/library/std/src/sys/hermit/memchr.rs" "/checkout/library/std/src/sys/windows/c/errors.rs" "/checkout/library/std/src/sys/hermit/mutex.rs" "/checkout/library/std/src/sys/windows/args.rs" "/checkout/library/std/src/sys/hermit/fs.rs" "/checkout/library/std/src/sys/windows/os_str.rs" "/checkout/library/std/src/sys/hermit/os.rs"` failed.
Build completed unsuccessfully in 0:00:12
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
