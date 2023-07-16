plain
   Compiling tokio-codec v0.1.2
   Compiling jsonrpc-core v14.2.0
   Compiling lsp-types v0.60.0
   Compiling addr2line v0.13.0
error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:317 ~ mio[c9d7]::poll::{impl#21}::update), const_param_did: None }) (end of phase Optimization) at bb2[20]:
use of local _12, which has no storage here
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/mio-0.6.22/src/poll.rs:1888:21
     |
1888 |         let other = *other;
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:167:36

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:961:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.49.0-nightly (2c2a5c65b 2020-10-20) running on x86_64-unknown-linux-gnu
note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C linker=clang -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib

note: some of the compiler flags provided by cargo are hidden

---
   Compiling chrono v0.4.15
   Compiling mio-extras v2.0.6
   Compiling notify v5.0.0-pre.3
   Compiling vfs-notify v0.0.0 (/checkout/src/tools/rust-analyzer/crates/vfs-notify)
error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:317 ~ mio[a47d]::poll::{impl#21}::update), const_param_did: None }) (end of phase Optimization) at bb2[20]:
use of local _12, which has no storage here
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/mio-0.6.22/src/poll.rs:1888:21
     |
1888 |         let other = *other;
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:167:36

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:961:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.49.0-nightly (2c2a5c65b 2020-10-20) running on x86_64-unknown-linux-gnu
note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C linker=clang -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib

note: some of the compiler flags provided by cargo are hidden

---
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rust-analyzer/crates/rust-analyzer/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
thread 'main' panicked at 'rust-analyzer always builds', src/bootstrap/dist.rs:1399:14
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest
Build completed unsuccessfully in 0:33:56
== clock drift check ==
  local time: Tue Oct 20 23:41:17 UTC 2020
  local time: Tue Oct 20 23:41:17 UTC 2020
  network time: Tue, 20 Oct 2020 01:04:22 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (4762) (python)
