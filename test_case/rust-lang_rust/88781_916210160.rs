plain
    Checking opaque-debug v0.3.0
    Checking cpuid-bool v0.1.2
    Checking unicode-width v0.1.8
    Checking scoped-tls v1.0.0
    Checking unic-common v0.9.0
    Checking unic-char-range v0.9.0
    Checking annotate-snippets v0.8.0
    Checking rustc_fs_util v0.0.0 (/checkout/compiler/rustc_fs_util)
   Compiling serde_derive v1.0.125
   Compiling serde v1.0.125
---
    Checking sharded-slab v0.1.1
    Checking thread_local v1.0.1
    Checking itertools v0.9.0
    Checking getopts v0.2.21
    Checking unic-char-property v0.9.0
    Checking unic-ucd-version v0.9.0
   Compiling crossbeam-utils v0.7.2
   Compiling memoffset v0.5.5
   Compiling crossbeam-epoch v0.8.2
   Compiling num-traits v0.2.12
   Compiling num-traits v0.2.12
   Compiling num-integer v0.1.43
   Compiling generic-array v0.14.4
    Checking unicode-normalization v0.1.13
    Checking expect-test v1.0.1
    Checking unic-emoji-char v0.9.0
    Checking ena v0.14.0
    Checking polonius-engine v0.13.0
    Checking tracing-log v0.1.2
    Checking rustc_lexer v0.1.0 (/checkout/compiler/rustc_lexer)
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
    Checking opaque-debug v0.3.0
    Checking cpuid-bool v0.1.2
    Checking unicode-width v0.1.8
    Checking scoped-tls v1.0.0
    Checking unic-common v0.9.0
    Checking unic-char-range v0.9.0
    Checking rustc_fs_util v0.0.0 (/checkout/compiler/rustc_fs_util)
   Compiling serde_derive v1.0.125
   Compiling serde v1.0.125
    Checking datafrog v2.0.1
---
    Checking thread_local v1.0.1
    Checking sharded-slab v0.1.1
    Checking itertools v0.9.0
    Checking getopts v0.2.21
    Checking unic-ucd-version v0.9.0
    Checking unic-char-property v0.9.0
   Compiling crossbeam-utils v0.7.2
   Compiling memoffset v0.5.5
   Compiling crossbeam-epoch v0.8.2
   Compiling num-traits v0.2.12
   Compiling num-traits v0.2.12
   Compiling num-integer v0.1.43
   Compiling generic-array v0.14.4
    Checking unicode-normalization v0.1.13
    Checking expect-test v1.0.1
    Checking unic-emoji-char v0.9.0
    Checking rand_core v0.6.2
    Checking num_cpus v1.13.0
    Checking ena v0.14.0
    Checking polonius-engine v0.13.0
---
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
* 627 error codes
* highest error code: E0785
tidy error: /checkout/src/test/ui/parser/emoji-identifiers.rs: too many trailing newlines (2)
Found 0 error codes with no tests
Done!
Done!
tidy error: Dependencies not explicitly permitted:
* unic-char-property 0.9.0 (registry+https://github.com/rust-lang/crates.io-index)
* unic-char-range 0.9.0 (registry+https://github.com/rust-lang/crates.io-index)
* unic-common 0.9.0 (registry+https://github.com/rust-lang/crates.io-index)
* unic-emoji-char 0.9.0 (registry+https://github.com/rust-lang/crates.io-index)
* unic-ucd-version 0.9.0 (registry+https://github.com/rust-lang/crates.io-index)
some tidy checks failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:12
