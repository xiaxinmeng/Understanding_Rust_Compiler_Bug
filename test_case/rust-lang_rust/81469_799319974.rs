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
Diff in /checkout/library/panic_abort/src/lib.rs at line 34:
 // "Leak" the payload and shim to the relevant abort on the platform in question.
 #[rustc_std_internal_symbol]
 pub unsafe extern "C" fn __rust_start_panic(_payload: *mut &mut dyn BoxMeUp) -> u32 {
-
     // Android has the ability to attach a message as part of the abort.
     #[cfg(target_os = "android")]
     android::android_set_abort_message(_payload);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/rustc-std-workspace-alloc/lib.rs" "/checkout/library/panic_abort/src/lib.rs" "/checkout/library/rustc-std-workspace-core/lib.rs" "/checkout/library/rtstartup/rsbegin.rs" "/checkout/library/alloc/tests/heap.rs" "/checkout/library/alloc/benches/string.rs" "/checkout/library/alloc/tests/str.rs" "/checkout/library/panic_abort/src/android.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
