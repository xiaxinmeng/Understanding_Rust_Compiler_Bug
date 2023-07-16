plain
  Downloaded object v0.26.2
  Downloaded miniz_oxide v0.4.0
   Compiling cc v1.0.73
    Checking core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.137 (https://github.com/randomPoison/libc.git?rev=7ccd2753aa1e26301c73bdb6f9e2df8085ac39e6#7ccd2753)
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.82
   Compiling unwind v0.0.0 (/checkout/library/unwind)
    Checking rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
---
  Downloaded stacker v0.1.14
   Compiling proc-macro2 v1.0.46
   Compiling syn v1.0.102
   Compiling unicode-ident v1.0.5
   Compiling libc v0.2.137 (https://github.com/randomPoison/libc.git?rev=7ccd2753aa1e26301c73bdb6f9e2df8085ac39e6#7ccd2753)
   Compiling autocfg v1.1.0
   Compiling proc-macro-hack v0.5.19
   Compiling version_check v0.9.3
    Checking lazy_static v1.4.0
---
   Compiling memchr v2.5.0
   Compiling version_check v0.9.3
   Compiling serde_derive v1.0.147
   Compiling serde v1.0.147
   Compiling libc v0.2.137 (https://github.com/randomPoison/libc.git?rev=7ccd2753aa1e26301c73bdb6f9e2df8085ac39e6#7ccd2753)
    Checking once_cell v1.12.0
   Compiling autocfg v1.1.0
   Compiling getrandom v0.2.0
    Checking cfg-if v0.1.10
---
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
extracting /checkout/obj/build/cache/llvm-160b19429523ea44c4c3b7cad4233b2a35f58b8f-true/rust-dev-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm
Checking stage0 std artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
   Compiling cc v1.0.73
    Checking core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.137 (https://github.com/randomPoison/libc.git?rev=7ccd2753aa1e26301c73bdb6f9e2df8085ac39e6#7ccd2753)
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.82
   Compiling unwind v0.0.0 (/checkout/library/unwind)
    Checking rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
---
    Checking instant v0.1.12
    Checking tracing-core v0.1.28
   Compiling type-map v0.4.0
    Checking scopeguard v1.1.0
   Compiling libc v0.2.137 (https://github.com/randomPoison/libc.git?rev=7ccd2753aa1e26301c73bdb6f9e2df8085ac39e6#7ccd2753)
   Compiling self_cell v0.10.2
   Compiling unicode-width v0.1.10
    Checking pin-project-lite v0.2.8
    Checking thin-vec v0.2.8
---
    Checking percent-encoding v2.1.0
   Compiling futures-core v0.3.19
   Compiling camino v1.0.9
   Compiling semver v1.0.12
   Compiling libc v0.2.137 (https://github.com/randomPoison/libc.git?rev=7ccd2753aa1e26301c73bdb6f9e2df8085ac39e6#7ccd2753)
   Compiling futures-channel v0.3.19
   Compiling rustversion v1.0.5
    Checking tinyvec v1.6.0
    Checking unicode-bidi v0.3.4
---
  Downloaded xattr v0.2.2
  Downloaded hex v0.4.2
   Compiling cfg-if v1.0.0
   Compiling autocfg v1.1.0
   Compiling libc v0.2.137 (https://github.com/randomPoison/libc.git?rev=7ccd2753aa1e26301c73bdb6f9e2df8085ac39e6#7ccd2753)
   Compiling crossbeam-utils v0.8.8
   Compiling typenum v1.12.0
   Compiling serde_derive v1.0.147
   Compiling crossbeam-epoch v0.9.6
---
    Finished dev [unoptimized] target(s) in 0.05s
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cc v1.0.73
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.137 (https://github.com/randomPoison/libc.git?rev=7ccd2753aa1e26301c73bdb6f9e2df8085ac39e6#7ccd2753)
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.82
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
---
  Downloaded unified-diff v0.2.1
  Downloaded glob v0.3.0
   Compiling proc-macro2 v1.0.46
   Compiling cfg-if v1.0.0
   Compiling libc v0.2.137 (https://github.com/randomPoison/libc.git?rev=7ccd2753aa1e26301c73bdb6f9e2df8085ac39e6#7ccd2753)
   Compiling regex-syntax v0.6.26
   Compiling once_cell v1.12.0
   Compiling ryu v1.0.5
   Compiling serde v1.0.147
---
   Compiling miropt-test-tools v0.1.0 (/checkout/src/tools/miropt-test-tools)
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 6.94s
tidy check
tidy error: invalid source: "git+https://github.com/randomPoison/libc.git?rev=7ccd2753aa1e26301c73bdb6f9e2df8085ac39e6#7ccd2753aa1e26301c73bdb6f9e2df8085ac39e6"
Checking which error codes lack tests...
* 632 error codes
* highest error code: E0790
Found 506 error codes
