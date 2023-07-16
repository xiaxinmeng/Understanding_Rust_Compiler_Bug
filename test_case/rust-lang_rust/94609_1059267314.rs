plain
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-02-23/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
Building rustbuild
    Updating crates.io index
    Updating git repository `https://github.com/MabezDev/libc`
---
  Downloaded object v0.26.2
  Downloaded gimli v0.25.0
   Compiling cc v1.0.69
    Checking core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.119 (https://github.com/MabezDev/libc?branch=esp-idf-stat-types#ab623802)
   Compiling libc v0.2.119
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.70
   Compiling unwind v0.0.0 (/checkout/library/unwind)
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
Warning: --all-targets is now on by default and does not need to be passed explicitly.
Checking stage0 std artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
   Compiling cc v1.0.69
    Checking core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.119 (https://github.com/MabezDev/libc?branch=esp-idf-stat-types#ab623802)
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.70
   Compiling unwind v0.0.0 (/checkout/library/unwind)
---
    Finished dev [unoptimized] target(s) in 0.17s
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cc v1.0.69
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.119 (https://github.com/MabezDev/libc?branch=esp-idf-stat-types#ab623802)
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.70
   Compiling unwind v0.0.0 (/checkout/library/unwind)
---
   Compiling regex v1.5.4
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 9.38s
tidy check
tidy error: invalid source: "git+https://github.com/MabezDev/libc?branch=esp-idf-stat-types#ab62380241b431beb929a2f986fe08375e87a091"
Checking which error codes lack tests...
* 629 error codes
* highest error code: E0787
Found 504 error codes
