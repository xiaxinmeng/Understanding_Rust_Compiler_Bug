plain
####                                                                       5.6%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2021-03-25/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
    Updating crates.io index
    Updating git repository `https://github.com/ivmarkov/libc.git`
---
  Downloaded miniz_oxide v0.4.0
  Downloaded hashbrown v0.11.0
   Compiling cc v1.0.69
    Checking core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.98 (https://github.com/ivmarkov/libc.git#b942bc82)
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.47
   Compiling unwind v0.0.0 (/checkout/library/unwind)
    Checking rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
---
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
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
Checking stage0 std artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
   Compiling cc v1.0.69
    Checking core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.98 (https://github.com/ivmarkov/libc.git#b942bc82)
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.47
   Compiling unwind v0.0.0 (/checkout/library/unwind)
    Checking rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
---
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cc v1.0.69
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.98 (https://github.com/ivmarkov/libc.git#b942bc82)
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.47
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
---
   Compiling regex v1.4.6
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 11.81s
tidy check
tidy error: invalid source: "git+https://github.com/ivmarkov/libc.git#b942bc82ac3b4104e6ce7284ceffeba8fbdc84b3"
Checking which error codes lack tests...
* 625 error codes
* highest error code: E0783
Found 499 error codes
Found 499 error codes
Found 0 error codes with no tests
Done!
* 345 features
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:14
