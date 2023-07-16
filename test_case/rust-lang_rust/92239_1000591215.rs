plain
####                                                                       5.7%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2021-11-30/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
    Updating crates.io index
    Updating git repository `https://github.com/nnethercote/hashbrown`
---
    Checking gimli v0.25.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking miniz_oxide v0.4.0
    Checking object v0.26.2
    Checking hashbrown v0.11.2 (https://github.com/nnethercote/hashbrown?branch=add-is_empty-checks#dec8e8cc)
    Checking rustc-std-workspace-std v1.99.0 (/checkout/library/rustc-std-workspace-std)
    Checking proc_macro v0.0.0 (/checkout/library/proc_macro)
    Checking unicode-width v0.1.8
    Checking getopts v0.2.21
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
    Checking panic_abort v0.0.0 (/checkout/library/panic_abort)
    Checking gimli v0.25.0
    Checking object v0.26.2
    Checking miniz_oxide v0.4.0
    Checking hashbrown v0.11.2 (https://github.com/nnethercote/hashbrown?branch=add-is_empty-checks#dec8e8cc)
    Checking addr2line v0.16.0
    Checking rustc-std-workspace-std v1.99.0 (/checkout/library/rustc-std-workspace-std)
    Checking proc_macro v0.0.0 (/checkout/library/proc_macro)
    Checking unicode-width v0.1.8
---
   Compiling rustc-std-workspace-alloc v1.99.0 (/checkout/library/rustc-std-workspace-alloc)
   Compiling panic_unwind v0.0.0 (/checkout/library/panic_unwind)
   Compiling panic_abort v0.0.0 (/checkout/library/panic_abort)
   Compiling gimli v0.25.0
   Compiling hashbrown v0.11.2 (https://github.com/nnethercote/hashbrown?branch=add-is_empty-checks#dec8e8cc)
   Compiling miniz_oxide v0.4.0
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling addr2line v0.16.0
   Compiling rustc-std-workspace-std v1.99.0 (/checkout/library/rustc-std-workspace-std)
---
   Compiling regex v1.5.4
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 10.02s
tidy check
tidy error: invalid source: "git+https://github.com/nnethercote/hashbrown?branch=add-is_empty-checks#dec8e8cccbb6e2742b9f551bb9b95d4a8a9d0b63"
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


Build completed unsuccessfully in 0:00:12
