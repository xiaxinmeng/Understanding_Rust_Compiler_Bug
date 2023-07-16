plain

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2021-03-25/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
    Updating crates.io index
    Updating git repository `https://github.com/rylev/measureme`
---
   Compiling cstr v0.2.8
    Checking rand_core v0.5.1
    Checking rand_chacha v0.3.0
    Checking crossbeam-deque v0.7.3
    Checking measureme v9.1.2 (https://github.com/rylev/measureme?branch=integer-support#e6b27c0d)
    Checking chrono v0.4.19
    Checking rand v0.8.3
    Checking block-buffer v0.9.0
    Checking digest v0.9.0
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
    Checking ansi_term v0.12.1
    Checking tempfile v3.2.0
    Checking termcolor v1.1.2
    Checking parking_lot v0.11.1
    Checking measureme v9.1.2 (https://github.com/rylev/measureme?branch=integer-support#e6b27c0d)
    Checking rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
   Compiling tracing-attributes v0.1.13
   Compiling chalk-derive v0.55.0
    Checking chalk-ir v0.55.0
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 10.97s
tidy check
tidy: Skipping binary file check, read-only filesystem
tidy error: invalid source: "git+https://github.com/rylev/measureme?branch=integer-support#e6b27c0dc01e43eb6da36060a0e4b74d65d4e514"
* 625 error codes
* highest error code: E0783
Found 499 error codes
Found 0 error codes with no tests
Found 0 error codes with no tests
Done!
* 340 features
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:13
