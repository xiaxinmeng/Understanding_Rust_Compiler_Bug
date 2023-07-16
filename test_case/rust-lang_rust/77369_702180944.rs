plain
   Compiling gimli v0.22.0
   Compiling object v0.20.0
   Compiling miniz_oxide v0.4.0
   Compiling hashbrown v0.9.0
error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:4693 ~ core[5bc6]::iter::adapters::{impl#95}::size_hint), const_param_did: None }) (end of phase Optimization) at bb0[0]:
use of local _11, which has no storage here
     |
     |
2477 |         if self.n == 0 {
     |
     |
     = note: delayed at compiler/rustc_mir/src/transform/validate.rs:163:36

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:961:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.48.0-nightly (1bdd5353b 2020-10-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z save-analysis -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -C linker=clang -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

error: could not compile `core`.


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:2370 ~ object[fa71]::read::traits::Object::symbol_data), const_param_did: None }) (end of phase Optimization) at bb0[0]:
use of local _51, which has no storage here
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/object-0.20.0/src/read/traits.rs:97:9
   |
97 | /         if symbol.is_undefined() {
98 | |             return Ok(None);
   | |_________^
   |
   |
   = note: delayed at compiler/rustc_mir/src/transform/validate.rs:163:36

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:961:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.48.0-nightly (1bdd5353b 2020-10-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z save-analysis -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C linker=clang -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

error: build failed
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
Build completed unsuccessfully in 0:14:52
== clock drift check ==
  local time: Thu Oct  1 14:38:18 UTC 2020
  local time: Thu Oct  1 14:38:18 UTC 2020
  network time: Thu, 01 Oct 2020 14:38:18 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (5347) (python)
