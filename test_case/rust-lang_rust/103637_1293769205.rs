plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/sys/windows/mod.rs at line 29:
 pub mod pipe;
 pub mod process;
 pub mod rand;
+pub mod stdio;
 pub mod thread;
 pub mod thread_local_dtor;
 pub mod thread_local_key;
Diff in /checkout/library/std/src/sys/windows/mod.rs at line 35:
 pub mod thread_parker;
 pub mod time;
-pub mod stdio;
 cfg_if::cfg_if! {
     if #[cfg(not(target_vendor = "uwp"))] {
         pub mod stack_overflow;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/wasi/net.rs" "/checkout/library/std/src/sys/windows/io.rs" "/checkout/library/std/src/sys/wasi/os.rs" "/checkout/library/std/src/sys/windows/pipe.rs" "/checkout/library/std/src/sys/wasi/io.rs" "/checkout/library/std/src/sys/windows/mod.rs" "/checkout/library/std/src/sys/wasi/mod.rs" "/checkout/library/std/src/sys/windows/stack_overflow.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
