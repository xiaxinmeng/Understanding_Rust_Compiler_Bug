plain
    Checking lock_api v0.4.7
    Checking always-assert v0.1.2
    Checking ena v0.14.0
    Checking tracing-log v0.1.3
    Checking sourcegen v0.0.0 (/checkout/src/tools/rust-analyzer/crates/sourcegen)
    Checking flate2 v1.0.24
    Checking xflags v0.2.4
    Checking parking_lot_core v0.9.3
    Checking stdx v0.0.0 (/checkout/src/tools/rust-analyzer/crates/stdx)
---
    Checking crossbeam-deque v0.8.1
    Checking dashmap v5.3.4
    Checking parking_lot v0.12.1
    Checking parking_lot v0.11.2
    Checking xtask v0.1.0 (/checkout/src/tools/rust-analyzer/xtask)
    Checking pulldown-cmark v0.9.1
    Checking regex-automata v0.1.10
    Checking regex v1.5.6
    Checking text-edit v0.0.0 (/checkout/src/tools/rust-analyzer/crates/text-edit)
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
