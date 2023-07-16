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
Diff in /checkout/library/std/src/sys/unix/process/process_unsupported.rs at line 5:
 use crate::num::NonZeroI32;
 use crate::os::raw::NonZero_c_int;
 use crate::sys;
-use crate::sys::unix::unsupported::*;
 use crate::sys::cvt;
 use crate::sys::pipe::AnonPipe;
 use crate::sys::process::process_common::*;
Diff in /checkout/library/std/src/sys/unix/process/process_unsupported.rs at line 12:
+use crate::sys::unix::unsupported::*;
 
 use libc::{c_int, pid_t};
Diff in /checkout/library/std/src/sys/unix/args.rs at line 252:
     use super::Args;
 
 
     #[inline(always)]
-    pub unsafe fn init(_argc: isize, _argv: *const *const u8) {
-    }
+    pub unsafe fn init(_argc: isize, _argv: *const *const u8) {}
     pub fn args() -> Args {
     pub fn args() -> Args {
         Args { iter: Vec::new().into_iter() }
Diff in /checkout/library/std/src/sys/unix/mod.rs at line 46:
 pub use crate::sys_common::os_str_bytes as os_str;
 
 #[cfg(all(not(test), target_os = "espidf"))]
-pub fn init(argc: isize, argv: *const *const u8) {
-}
+pub fn init(argc: isize, argv: *const *const u8) {}
 
 #[cfg(all(not(test), not(target_os = "espidf")))]
 // SAFETY: must be called only once during runtime initialization.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/unix/process/process_vxworks.rs" "/checkout/library/std/src/sys/unix/args.rs" "/checkout/library/std/src/sys/unix/l4re.rs" "/checkout/library/std/src/sys/unix/process/process_fuchsia.rs" "/checkout/library/std/src/sys/unix/fd.rs" "/checkout/library/std/src/sys/unix/net.rs" "/checkout/library/std/src/sys/unix/weak.rs" "/checkout/library/std/src/sys/unix/mutex.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
