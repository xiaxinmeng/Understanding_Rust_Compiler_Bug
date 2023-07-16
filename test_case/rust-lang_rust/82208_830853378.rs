plain
    Checking semver-parser v0.7.0
    Checking vec_map v0.8.2
    Checking smallvec v1.6.1
   Compiling rustfmt-nightly v1.4.37 (/checkout/src/tools/rustfmt)
    Checking yansi-term v0.1.2
    Checking diff v0.1.12
    Checking unicode_categories v0.1.1
   Compiling proc-macro2 v1.0.24
   Compiling syn v1.0.65
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
    Checking winapi-util v0.1.5
    Checking atty v0.2.14
    Checking dirs-sys v0.3.6
    Checking curl-sys v0.4.39+curl-7.74.0
    Checking yansi-term v0.1.2
    Checking clap v2.33.3
    Checking term v0.6.1
    Checking annotate-snippets v0.8.0
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
---
* highest error code: E0781
Found 515 error codes
Found 0 error codes with no tests
Done!
tidy error: Dependencies not explicitly permitted:
* yansi-term 0.1.2 (registry+https://github.com/rust-lang/crates.io-index)
some tidy checks failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:12
