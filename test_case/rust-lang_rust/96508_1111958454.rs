plain
   Compiling cc v1.0.69
    Checking core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.121
   Compiling memchr v2.4.1
   Compiling libc v0.2.124 (https://github.com/rust-lang/libc#f6df53fd)
   Compiling compiler_builtins v0.1.71
   Compiling unwind v0.0.0 (/checkout/library/unwind)
    Checking rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
    Checking alloc v0.0.0 (/checkout/library/alloc)
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
   Compiling cc v1.0.69
    Checking core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.121
   Compiling memchr v2.4.1
   Compiling libc v0.2.124 (https://github.com/rust-lang/libc#f6df53fd)
   Compiling compiler_builtins v0.1.71
   Compiling unwind v0.0.0 (/checkout/library/unwind)
    Checking rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
    Checking alloc v0.0.0 (/checkout/library/alloc)
---
   Compiling cc v1.0.69
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.121
   Compiling memchr v2.4.1
   Compiling libc v0.2.124 (https://github.com/rust-lang/libc#f6df53fd)
   Compiling compiler_builtins v0.1.71
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
   Compiling alloc v0.0.0 (/checkout/library/alloc)
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 8.62s
tidy check
tidy: Skipping binary file check, read-only filesystem
tidy error: invalid source: "git+https://github.com/rust-lang/libc#f6df53fd694f6fc903058c765efc10d77725b31b"
* 629 error codes
* highest error code: E0787
Found 504 error codes
Found 0 error(s) in error codes
