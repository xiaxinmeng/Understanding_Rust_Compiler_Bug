plain

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-09-20/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
Checking stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Updating git repository `https://github.com/notriddle/thin-vec.git`
---
   Compiling self_cell v0.10.2
    Checking pin-project-lite v0.2.8
    Checking bitflags v1.3.2
   Compiling unicode-width v0.1.10
    Checking thin-vec v0.2.8 (https://github.com/notriddle/thin-vec.git?branch=notriddle/retain-mut#3c4d32a3)
   Compiling typenum v1.12.0
   Compiling memoffset v0.6.5
   Compiling rustc-rayon v0.4.0
   Compiling indexmap v1.9.1
---
    Checking itoa v1.0.2
    Checking either v1.6.0
    Checking remove_dir_all v0.5.3
    Checking bitflags v1.3.2
    Checking thin-vec v0.2.8 (https://github.com/notriddle/thin-vec.git?branch=notriddle/retain-mut#3c4d32a3)
    Checking itertools v0.10.1
    Checking minifier v0.2.2
    Checking aho-corasick v0.7.18
    Checking quote v1.0.18
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
   Compiling self_cell v0.10.2
    Checking pin-project-lite v0.2.8
   Compiling ahash v0.7.4
   Compiling unicode-width v0.1.10
    Checking thin-vec v0.2.8 (https://github.com/notriddle/thin-vec.git?branch=notriddle/retain-mut#3c4d32a3)
    Checking rand_core v0.6.2
   Compiling generic-array v0.14.4
   Compiling indexmap v1.9.1
   Compiling lock_api v0.4.7
---
    Checking itoa v1.0.2
    Checking bitflags v1.3.2
    Checking itertools v0.10.1
    Checking arrayvec v0.7.0
    Checking thin-vec v0.2.8 (https://github.com/notriddle/thin-vec.git?branch=notriddle/retain-mut#3c4d32a3)
    Checking regex-automata v0.1.10
    Checking regex v1.5.6
    Checking matchers v0.1.0
    Checking ansi_term v0.12.1
---
   Compiling regex v1.5.6
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 8.20s
tidy check
tidy error: invalid source: "git+https://github.com/notriddle/thin-vec.git?branch=notriddle/retain-mut#3c4d32a3d1da7700a770a5649f900c21ecc5f12f"
Checking which error codes lack tests...
* 632 error codes
* highest error code: E0790
Found 506 error codes
