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
Diff in /checkout/compiler/rustc_target/src/spec/illumos_base.rs at line 12:
             // libc, but at least historically these have been provided in
             // libssp.so on illumos and Solaris systems.
             "-lssp".to_string(),
-
             // The illumos libc contains a stack unwinding implementation, as
             // does libgcc_s.  The latter implementation includes several
             // additional symbols that are not always in base libc.  To force
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_target/src/spec/riscv_base.rs" "/checkout/compiler/rustc_target/src/spec/thumbv7a_uwp_windows_msvc.rs" "/checkout/compiler/rustc_target/src/spec/thumb_base.rs" "/checkout/compiler/rustc_target/src/spec/aarch64_apple_ios.rs" "/checkout/compiler/rustc_target/src/spec/x86_64_pc_windows_gnu.rs" "/checkout/compiler/rustc_target/src/spec/aarch64_fuchsia.rs" "/checkout/compiler/rustc_target/src/spec/illumos_base.rs" "/checkout/compiler/rustc_target/src/spec/powerpc64_unknown_linux_gnu.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
