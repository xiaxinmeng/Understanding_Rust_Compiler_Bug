plain
    Checking itoa v0.4.6
    Checking remove_dir_all v0.5.3
    Checking termcolor v1.1.2
    Checking macro-utils v0.1.3
    Checking font-awesome-as-a-crate v0.2.0
    Checking instant v0.1.12
    Checking lock_api v0.4.5
    Checking thread_local v1.0.1
    Checking tracing-core v0.1.21
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
    Checking either v1.6.0
    Checking itoa v0.4.6
    Checking macro-utils v0.1.3
    Checking arrayvec v0.7.0
    Checking font-awesome-as-a-crate v0.2.0
    Checking lock_api v0.4.5
    Checking thread_local v1.0.1
    Checking tracing-core v0.1.21
    Checking sharded-slab v0.1.1
---
* highest error code: E0786
Found 502 error codes
Found 0 error codes with no tests
Done!
tidy error: invalid license `CC-BY-4.0 AND MIT` in `font-awesome-as-a-crate 0.2.0 (registry+https://github.com/rust-lang/crates.io-index)`
some tidy checks failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:10
