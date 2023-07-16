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
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/panic_unwind/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Diff in /checkout/library/panic_unwind/src/lib.rs at line 17:
     html_root_url = "https://doc.rust-lang.org/nightly/",
     issue_tracker_base_url = "https://github.com/rust-lang/rust/issues/"
-
 #![feature(core_intrinsics)]
 #![feature(int_bits_const)]
 #![feature(lang_items)]
 #![feature(lang_items)]
Diff in /checkout/library/panic_unwind/src/lib.rs at line 31:
 #![feature(raw)]
 #![panic_runtime]
 #![feature(panic_runtime)]
-
 // `real_imp` is unused with Miri, so silence warnings.
-
 #![cfg_attr(miri, allow(dead_code))]
 use alloc::boxed::Box;
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:20
