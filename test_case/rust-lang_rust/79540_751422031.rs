plain

####################                                                      28.5%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2020-11-18/cargo-beta-x86_64-unknown-linux-gnu.tar.xz
Wrote /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/.rustc-stamp.tmp

########                                                                  12.2%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2020-11-19/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
extracting /checkout/obj/build/cache/2020-11-19/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
Wrote /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/.rustfmt-stamp.tmp
---
    Checking unicode-width v0.1.8
    Checking getopts v0.2.21
    Checking test v0.0.0 (/checkout/library/test)
    Finished release [optimized] target(s) in 29.78s
[src/bootstrap/compile.rs:1240] stamp = "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/.libstd-check.stamp"
---
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
    Checking rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
    Checking rustc-main v0.0.0 (/checkout/compiler/rustc)
    Finished release [optimized] target(s) in 1m 03s
[src/bootstrap/compile.rs:1240] stamp = "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/.librustc-check.stamp"
---
    Checking regex v1.3.9
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
    Checking rustdoc-tool v0.0.0 (/checkout/src/tools/rustdoc)
    Finished release [optimized] target(s) in 17.99s
[src/bootstrap/compile.rs:1240] stamp = "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/.rustdoc-check.stamp"
    Updating git repository `https://github.com/bjorn3/rust-ar.git`
    Updating git repository `https://github.com/bytecodealliance/wasmtime/`
    Updating git submodule `https://github.com/WebAssembly/WASI`
    Updating git submodule `https://github.com/WebAssembly/wasm-c-api`
---
    Checking cranelift-simplejit v0.68.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#19640367)
    Checking cranelift-object v0.68.0 (https://github.com/bytecodealliance/wasmtime/?branch=main#19640367)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
    Finished release [optimized] target(s) in 25.98s
[src/bootstrap/compile.rs:1240] stamp = "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/.librustc_codegen_cranelift-check.stamp"
---
    Checking toml v0.5.7
    Checking cargo_metadata v0.12.0
    Checking clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
    Finished release [optimized] target(s) in 24.68s
[src/bootstrap/compile.rs:1240] stamp = "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/.clippy-check.stamp"
   Compiling syn v1.0.38
   Compiling log v0.4.11
    Checking fnv v1.0.7
    Checking same-file v1.0.6
---
    Checking serde v1.0.118
    Checking toml v0.5.7
    Checking serde_json v1.0.59
    Finished release [optimized] target(s) in 16.04s
[src/bootstrap/compile.rs:1240] stamp = "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/.bootstrap-check.stamp"
configure: processing command line
configure: 
configure: llvm.ccache          := sccache
configure: build.submodules     := False
---
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
Wrote config.toml.tmp
Wrote Makefile.tmp
configure: 
---

##############                                                            19.7%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2020-11-18/cargo-beta-x86_64-unknown-linux-gnu.tar.xz
Wrote /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/.rustc-stamp.tmp

###################                                                       27.5%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2020-11-19/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
extracting /checkout/obj/build/cache/2020-11-19/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
Wrote /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/.rustfmt-stamp.tmp
   Compiling unicode-xid v0.2.1
   Compiling syn v1.0.38
   Compiling autocfg v1.0.0
   Compiling version_check v0.9.1
---
    Finished release [optimized] target(s) in 6.07s
Build completed successfully in 0:00:06
+ python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
warning: creating symbolic link `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/src/rust` to `/checkout` failed with File exists (os error 17)
   Compiling cc v1.0.60
    Checking core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.79
   Compiling std v0.0.0 (/checkout/library/std)
---
    Checking unicode-width v0.1.8
    Checking getopts v0.2.21
    Checking test v0.0.0 (/checkout/library/test)
    Finished release [optimized] target(s) in 29.77s
[src/bootstrap/compile.rs:1240] stamp = "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/.libstd-check.stamp"
---
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
    Finished release [optimized] target(s) in 14.96s
[src/bootstrap/compile.rs:1240] stamp = "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/.libstd-check-test.stamp"
---
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
    Checking rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
    Checking rustc-main v0.0.0 (/checkout/compiler/rustc)
    Finished release [optimized] target(s) in 1m 02s
[src/bootstrap/compile.rs:1240] stamp = "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/i686-pc-windows-gnu/release/.librustc-check.stamp"
    Checking cfg-if v0.1.10
   Compiling autocfg v1.0.0
    Checking lazy_static v1.4.0
   Compiling proc-macro2 v1.0.19
---
    Checking tempfile v3.1.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
    Checking rustdoc-tool v0.0.0 (/checkout/src/tools/rustdoc)
    Finished release [optimized] target(s) in 18.12s
[src/bootstrap/compile.rs:1240] stamp = "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/.rustdoc-check.stamp"
   Compiling proc-macro2 v1.0.24
   Compiling autocfg v1.0.1
   Compiling unicode-xid v0.2.1
   Compiling syn v1.0.48
---

warning: 1 warning emitted

    Finished release [optimized] target(s) in 18.45s
[src/bootstrap/compile.rs:1240] stamp = "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-codegen/i686-pc-windows-gnu/release/.librustc_codegen_cranelift-check.stamp"
---
    Checking rustfix v0.5.1
    Checking clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Finished release [optimized] target(s) in 25.59s
[src/bootstrap/compile.rs:1240] stamp = "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/.clippy-check.stamp"
---
    Checking serde v1.0.118
    Checking serde_json v1.0.59
    Checking toml v0.5.7
    Finished release [optimized] target(s) in 16.26s
[src/bootstrap/compile.rs:1240] stamp = "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/.bootstrap-check.stamp"
+ python3 ../x.py build --stage 0 src/tools/build-manifest
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
warning: creating symbolic link `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/src/rust` to `/checkout` failed with File exists (os error 17)
---
    Finished release [optimized] target(s) in 16.08s
Build completed successfully in 0:00:16
+ python3 ../x.py test --stage 0 src/tools/compiletest
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
warning: creating symbolic link `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/src/rust` to `/checkout` failed with File exists (os error 17)
   Compiling cc v1.0.60
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.79
   Compiling std v0.0.0 (/checkout/library/std)
---
   Compiling unicode-width v0.1.8
   Compiling getopts v0.2.21
   Compiling test v0.0.0 (/checkout/library/test)
    Finished release [optimized] target(s) in 1m 08s
[src/bootstrap/compile.rs:1240] stamp = "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/.libstd.stamp"
---

Build completed successfully in 0:01:29
+ python3 ../x.py test --stage 2 src/tools/tidy
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
warning: creating symbolic link `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/src/rust` to `/checkout` failed with File exists (os error 17)
---
Checking which error codes lack tests...
Found 434 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/src/bootstrap/bootstrap.py:1139: line longer than 100 chars
some tidy checks failed

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build"
expected success, got: exit code: 1

