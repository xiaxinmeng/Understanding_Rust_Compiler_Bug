plain
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-05-20/cargo-beta-x86_64-unknown-linux-gnu.tar.xz
Building rustbuild
    Updating crates.io index
    Updating git repository `https://github.com/michaelwoerister/xxh3-port`
---
   Compiling rustc-main v0.0.0 (/checkout/compiler/rustc)
    Checking difference v2.0.0
   Compiling coverage_test_macros v0.0.0 (/checkout/compiler/rustc_mir_transform/src/coverage/test_macros)
    Checking instant v0.1.12
    Checking xxh3-port v0.1.0 (https://github.com/michaelwoerister/xxh3-port#f2071121)
    Checking libloading v0.7.1
    Checking tracing-core v0.1.21
    Checking sharded-slab v0.1.1
   Compiling unic-langid-impl v0.9.0
---
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
   Compiling rustc-main v0.0.0 (/checkout/compiler/rustc)
    Checking difference v2.0.0
   Compiling coverage_test_macros v0.0.0 (/checkout/compiler/rustc_mir_transform/src/coverage/test_macros)
    Checking instant v0.1.12
    Checking xxh3-port v0.1.0 (https://github.com/michaelwoerister/xxh3-port#f2071121)
    Checking tracing-core v0.1.21
    Checking sharded-slab v0.1.1
   Compiling unic-langid-impl v0.9.0
    Checking thread_local v1.1.4
---
   Compiling regex v1.5.5
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 7.67s
tidy check
tidy error: invalid source: "git+https://github.com/michaelwoerister/xxh3-port#f2071121875c9b0a474dcdbbd4c4411011b0c266"
Checking which error codes lack tests...
* 630 error codes
* highest error code: E0788
Found 505 error codes
Found 505 error codes
Found 0 error(s) in error codes
Done!
tidy error: dependency `xxh3-port 0.1.0 (git+https://github.com/michaelwoerister/xxh3-port#f2071121875c9b0a474dcdbbd4c4411011b0c266)` does not define a license expression
tidy error: Dependencies not explicitly permitted:
* xxh3-port 0.1.0 (git+https://github.com/michaelwoerister/xxh3-port#f2071121875c9b0a474dcdbbd4c4411011b0c266)
some tidy checks failed
Build completed unsuccessfully in 0:00:10
