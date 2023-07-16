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
Diff in /checkout/library/panic_abort/src/android.rs at line 1:
 use alloc::string::String;
 use core::mem::transmute;
-use core::ptr::copy_nonoverlapping;
 use core::panic::BoxMeUp;
+use core::ptr::copy_nonoverlapping;
 
 const ANDROID_SET_ABORT_MESSAGE: &[u8] = b"android_set_abort_message\0";
 type SetAbortMessageType = unsafe extern "C" fn(*const libc::c_char) -> ();
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/panic_abort/src/lib.rs" "/checkout/library/panic_abort/src/android.rs" "/checkout/library/rustc-std-workspace-std/lib.rs" "/checkout/library/rustc-std-workspace-alloc/lib.rs" "/checkout/library/term/src/lib.rs" "/checkout/library/term/src/win.rs" "/checkout/library/term/src/terminfo/mod.rs" "/checkout/src/etc/test-float-parse/u64-pow2.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
