plain
    Checking libc v0.2.126
    Checking xshell v0.2.2
    Checking parser v0.0.0 (/checkout/src/tools/rust-analyzer/crates/parser)
    Checking lock_api v0.4.7
    Checking sourcegen v0.0.0 (/checkout/src/tools/rust-analyzer/crates/sourcegen)
    Checking ena v0.14.0
    Checking tracing-log v0.1.3
    Checking crossbeam-channel v0.5.5
    Checking xflags v0.2.4
---
    Checking perf-event v0.4.7
    Checking parking_lot v0.11.2
    Checking text-edit v0.0.0 (/checkout/src/tools/rust-analyzer/crates/text-edit)
    Checking pulldown-cmark v0.9.1
    Checking xtask v0.1.0 (/checkout/src/tools/rust-analyzer/xtask)
    Checking regex v1.5.6
    Checking notify v5.0.0-pre.15
    Checking countme v3.0.1
    Checking profile v0.0.0 (/checkout/src/tools/rust-analyzer/crates/profile)
---

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/release/build/proc-macro-test-2f04565cab8a716e/build-script-build` (exit status: 101)
  --- stdout
  proc-macro-test failed to build, detailed output follows:
  === nested cargo stdout ===


  === nested cargo stderr ===
  error: the lock file /checkout/src/tools/rust-analyzer/crates/proc-macro-test/imp/Cargo.lock needs to be updated but --locked was passed to prevent this
  If you want to try to generate the lock file without accessing the network, remove the --locked flag and use --offline instead.


  --- stderr
  thread 'main' panicked at 'proc-macro-test failed to build', crates/proc-macro-test/build.rs:36:9
