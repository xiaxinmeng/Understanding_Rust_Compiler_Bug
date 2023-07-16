plain
Successfully built fda035b1b83c
Successfully tagged rust-ci:latest
Built container sha256:fda035b1b83cd5f6805d0bcad8fc6a869bee24a5e432abf6d70a056c37131880
Uploading finished image to https://ci-caches.rust-lang.org/docker/c165c0dbe07f979c65839814f65d46fc3b20640fec5c6162fa0d12eccd662a5501f008e9d9c1953dc5ee940f259fa67db4f7e378e34b63c10ab0f0e1d88ab56a
upload failed: - to s3://rust-lang-ci-sccache2/docker/c165c0dbe07f979c65839814f65d46fc3b20640fec5c6162fa0d12eccd662a5501f008e9d9c1953dc5ee940f259fa67db4f7e378e34b63c10ab0f0e1d88ab56a Unable to locate credentials
[CI_JOB_NAME=mingw-check]
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
   Compiling regex v1.5.5
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 9.18s
tidy check
tidy error: invalid source: "git+https://github.com/rust-lang/libc#f6df53fd694f6fc903058c765efc10d77725b31b"
Checking which error codes lack tests...
* 629 error codes
* highest error code: E0787
Found 504 error codes
