plain
####                                                                       6.5%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2021-11-30/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
    Updating crates.io index
    Updating git repository `https://github.com/davidtwco/thorin.git`
---
   Compiling tracing-attributes v0.1.18
   Compiling chalk-derive v0.55.0
    Checking chalk-ir v0.55.0
    Checking tracing v0.1.29
    Checking thorin v0.1.0 (https://github.com/davidtwco/thorin.git#a47d752f)
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
    Checking rustc_arena v0.0.0 (/checkout/compiler/rustc_arena)
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
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
   Compiling tracing-attributes v0.1.18
   Compiling chalk-derive v0.55.0
    Checking chalk-ir v0.55.0
    Checking tracing v0.1.29
    Checking thorin v0.1.0 (https://github.com/davidtwco/thorin.git#a47d752f)
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
    Checking rustc_arena v0.0.0 (/checkout/compiler/rustc_arena)
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
---
   Compiling regex v1.5.4
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 9.44s
tidy check
tidy error: invalid source: "git+https://github.com/davidtwco/thorin.git#a47d752f9e5e5556a9c1ad63443903c16e382633"
Checking which error codes lack tests...
* 628 error codes
* highest error code: E0786
Found 502 error codes
Found 502 error codes
Found 0 error codes with no tests
Done!
* 361 features
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:11
