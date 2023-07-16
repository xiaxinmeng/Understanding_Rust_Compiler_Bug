plain
   Compiling rustc_llvm v0.0.0 (/checkout/compiler/rustc_llvm)
   Compiling ena v0.14.0
   Compiling polonius-engine v0.13.0
   Compiling tracing-log v0.1.2
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: failed to process buffered lint here (dummy = false)
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.6/src/atomic/atomic_cell.rs:347:23
297 | / macro_rules! impl_arithmetic {
297 | / macro_rules! impl_arithmetic {
298 | |     ($t:ty, fallback, $example:tt) => {
299 | |         impl AtomicCell<$t> {
300 | |             /// Increments the current value by `val` and returns the previous value.
...   |
347 | |                 #[cfg(crossbeam_loom)]
...   |
546 | |     };
547 | | }
    | |_- in this expansion of `impl_arithmetic!`
    | |_- in this expansion of `impl_arithmetic!`
...
567 |   impl_arithmetic!(i128, fallback, "let a = AtomicCell::new(7i128);");
    |
    = note: delayed at /checkout/compiler/rustc_lint/src/early.rs:463:18


error: internal compiler error: failed to process buffered lint here (dummy = false)
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.6/src/atomic/atomic_cell.rs:316:23
297 | / macro_rules! impl_arithmetic {
297 | / macro_rules! impl_arithmetic {
298 | |     ($t:ty, fallback, $example:tt) => {
299 | |         impl AtomicCell<$t> {
300 | |             /// Increments the current value by `val` and returns the previous value.
...   |
316 | |                 #[cfg(crossbeam_loom)]
...   |
546 | |     };
547 | | }
    | |_- in this expansion of `impl_arithmetic!`
    | |_- in this expansion of `impl_arithmetic!`
...
567 |   impl_arithmetic!(i128, fallback, "let a = AtomicCell::new(7i128);");
    |
    = note: delayed at /checkout/compiler/rustc_lint/src/early.rs:463:18


error: internal compiler error: failed to process buffered lint here (dummy = false)
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.6/src/atomic/atomic_cell.rs:405:23
297 | / macro_rules! impl_arithmetic {
297 | / macro_rules! impl_arithmetic {
298 | |     ($t:ty, fallback, $example:tt) => {
299 | |         impl AtomicCell<$t> {
300 | |             /// Increments the current value by `val` and returns the previous value.
...   |
405 | |                 #[cfg(crossbeam_loom)]
...   |
546 | |     };
547 | | }
    | |_- in this expansion of `impl_arithmetic!`
    | |_- in this expansion of `impl_arithmetic!`
...
566 |   impl_arithmetic!(u128, fallback, "let a = AtomicCell::new(7u128);");
    |
    = note: delayed at /checkout/compiler/rustc_lint/src/early.rs:463:18


error: internal compiler error: failed to process buffered lint here (dummy = false)
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.6/src/atomic/atomic_cell.rs:434:23
297 | / macro_rules! impl_arithmetic {
297 | / macro_rules! impl_arithmetic {
298 | |     ($t:ty, fallback, $example:tt) => {
299 | |         impl AtomicCell<$t> {
300 | |             /// Increments the current value by `val` and returns the previous value.
...   |
434 | |                 #[cfg(crossbeam_loom)]
...   |
546 | |     };
547 | | }
    | |_- in this expansion of `impl_arithmetic!`
    | |_- in this expansion of `impl_arithmetic!`
...
567 |   impl_arithmetic!(i128, fallback, "let a = AtomicCell::new(7i128);");
    |
    = note: delayed at /checkout/compiler/rustc_lint/src/early.rs:463:18


error: internal compiler error: failed to process buffered lint here (dummy = false)
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.6/src/atomic/atomic_cell.rs:376:23
297 | / macro_rules! impl_arithmetic {
297 | / macro_rules! impl_arithmetic {
298 | |     ($t:ty, fallback, $example:tt) => {
299 | |         impl AtomicCell<$t> {
300 | |             /// Increments the current value by `val` and returns the previous value.
...   |
376 | |                 #[cfg(crossbeam_loom)]
...   |
546 | |     };
547 | | }
    | |_- in this expansion of `impl_arithmetic!`
    | |_- in this expansion of `impl_arithmetic!`
...
566 |   impl_arithmetic!(u128, fallback, "let a = AtomicCell::new(7u128);");
    |
    = note: delayed at /checkout/compiler/rustc_lint/src/early.rs:463:18


error: internal compiler error: failed to process buffered lint here (dummy = false)
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.6/src/atomic/atomic_cell.rs:347:23
297 | / macro_rules! impl_arithmetic {
297 | / macro_rules! impl_arithmetic {
298 | |     ($t:ty, fallback, $example:tt) => {
299 | |         impl AtomicCell<$t> {
300 | |             /// Increments the current value by `val` and returns the previous value.
...   |
347 | |                 #[cfg(crossbeam_loom)]
...   |
546 | |     };
547 | | }
    | |_- in this expansion of `impl_arithmetic!`
    | |_- in this expansion of `impl_arithmetic!`
...
566 |   impl_arithmetic!(u128, fallback, "let a = AtomicCell::new(7u128);");
    |
    = note: delayed at /checkout/compiler/rustc_lint/src/early.rs:463:18


error: internal compiler error: failed to process buffered lint here (dummy = false)
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.6/src/atomic/atomic_cell.rs:316:23
297 | / macro_rules! impl_arithmetic {
297 | / macro_rules! impl_arithmetic {
298 | |     ($t:ty, fallback, $example:tt) => {
299 | |         impl AtomicCell<$t> {
300 | |             /// Increments the current value by `val` and returns the previous value.
...   |
316 | |                 #[cfg(crossbeam_loom)]
...   |
546 | |     };
547 | | }
    | |_- in this expansion of `impl_arithmetic!`
    | |_- in this expansion of `impl_arithmetic!`
...
566 |   impl_arithmetic!(u128, fallback, "let a = AtomicCell::new(7u128);");
    |
    = note: delayed at /checkout/compiler/rustc_lint/src/early.rs:463:18


error: internal compiler error: failed to process buffered lint here (dummy = false)
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.6/src/atomic/atomic_cell.rs:405:23
297 | / macro_rules! impl_arithmetic {
297 | / macro_rules! impl_arithmetic {
298 | |     ($t:ty, fallback, $example:tt) => {
299 | |         impl AtomicCell<$t> {
300 | |             /// Increments the current value by `val` and returns the previous value.
...   |
405 | |                 #[cfg(crossbeam_loom)]
...   |
546 | |     };
547 | | }
    | |_- in this expansion of `impl_arithmetic!`
    | |_- in this expansion of `impl_arithmetic!`
...
567 |   impl_arithmetic!(i128, fallback, "let a = AtomicCell::new(7i128);");
    |
    = note: delayed at /checkout/compiler/rustc_lint/src/early.rs:463:18


error: internal compiler error: failed to process buffered lint here (dummy = false)
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.6/src/atomic/atomic_cell.rs:434:23
297 | / macro_rules! impl_arithmetic {
297 | / macro_rules! impl_arithmetic {
298 | |     ($t:ty, fallback, $example:tt) => {
299 | |         impl AtomicCell<$t> {
300 | |             /// Increments the current value by `val` and returns the previous value.
...   |
434 | |                 #[cfg(crossbeam_loom)]
...   |
546 | |     };
547 | | }
    | |_- in this expansion of `impl_arithmetic!`
    | |_- in this expansion of `impl_arithmetic!`
...
566 |   impl_arithmetic!(u128, fallback, "let a = AtomicCell::new(7u128);");
    |
    = note: delayed at /checkout/compiler/rustc_lint/src/early.rs:463:18


error: internal compiler error: failed to process buffered lint here (dummy = false)
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.6/src/atomic/atomic_cell.rs:376:23
297 | / macro_rules! impl_arithmetic {
297 | / macro_rules! impl_arithmetic {
298 | |     ($t:ty, fallback, $example:tt) => {
299 | |         impl AtomicCell<$t> {
300 | |             /// Increments the current value by `val` and returns the previous value.
...   |
376 | |                 #[cfg(crossbeam_loom)]
...   |
546 | |     };
547 | | }
    | |_- in this expansion of `impl_arithmetic!`
    | |_- in this expansion of `impl_arithmetic!`
...
567 |   impl_arithmetic!(i128, fallback, "let a = AtomicCell::new(7i128);");
    |
    = note: delayed at /checkout/compiler/rustc_lint/src/early.rs:463:18

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1188:13
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1188:13
stack backtrace:
   0:     0x7f0f6422e5ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h641e0a65e2615a08
   1:     0x7f0f6429b4ae - core::fmt::write::h39191aa5431a5380
   2:     0x7f0f6421d7a1 - std::io::Write::write_fmt::hb1861dc9906df921
   3:     0x7f0f6422e3db - std::sys_common::backtrace::print::h0ae42f20033c9262
   4:     0x7f0f64232bf4 - std::panicking::default_hook::{{closure}}::hd2976bf86056b49a
   5:     0x7f0f642327d6 - std::panicking::default_hook::hd02e8d479b982aaa
   6:     0x7f0f64c85e21 - rustc_driver[1687b6f54ec85a2d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f0f64233303 - std::panicking::rust_panic_with_hook::hbbba1d0bbca4f6bf
   8:     0x7f0f674a8bc3 - std[9f788053d5631f34]::panicking::begin_panic::<rustc_errors[9531b398d7fbf46a]::ExplicitBug>::{closure#0}
   9:     0x7f0f674a8a26 - std[9f788053d5631f34]::sys_common::backtrace::__rust_end_short_backtrace::<std[9f788053d5631f34]::panicking::begin_panic<rustc_errors[9531b398d7fbf46a]::ExplicitBug>::{closure#0}, !>
  10:     0x7f0f64c4c16f - std[9f788053d5631f34]::panicking::begin_panic::<rustc_errors[9531b398d7fbf46a]::ExplicitBug>
  11:     0x7f0f674df846 - std[9f788053d5631f34]::panic::panic_any::<rustc_errors[9531b398d7fbf46a]::ExplicitBug>
  12:     0x7f0f674d89e1 - <rustc_errors[9531b398d7fbf46a]::HandlerInner as core[8382fcd636289ab6]::ops::drop::Drop>::drop
  13:     0x7f0f64c99985 - core[8382fcd636289ab6]::ptr::drop_in_place::<rustc_session[ff90c9fddf4ef87c]::parse::ParseSess>
  14:     0x7f0f64c9ee47 - <alloc[75caa6313e26ebc1]::rc::Rc<rustc_session[ff90c9fddf4ef87c]::session::Session> as core[8382fcd636289ab6]::ops::drop::Drop>::drop
  15:     0x7f0f64cec18c - core[8382fcd636289ab6]::ptr::drop_in_place::<rustc_interface[e9a32af1e3f110c9]::interface::Compiler>
  16:     0x7f0f64cf1bc9 - rustc_interface[e9a32af1e3f110c9]::interface::create_compiler_and_run::<core[8382fcd636289ab6]::result::Result<(), rustc_errors[9531b398d7fbf46a]::ErrorReported>, rustc_driver[1687b6f54ec85a2d]::run_compiler::{closure#1}>
  17:     0x7f0f64cabc59 - std[9f788053d5631f34]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[e9a32af1e3f110c9]::util::run_in_thread_pool_with_globals<rustc_interface[e9a32af1e3f110c9]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[9531b398d7fbf46a]::ErrorReported>, rustc_driver[1687b6f54ec85a2d]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[9531b398d7fbf46a]::ErrorReported>>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[9531b398d7fbf46a]::ErrorReported>>
  18:     0x7f0f64cfeb5e - std[9f788053d5631f34]::panic::catch_unwind::<core[8382fcd636289ab6]::panic::unwind_safe::AssertUnwindSafe<<std[9f788053d5631f34]::thread::Builder>::spawn_unchecked_<rustc_interface[e9a32af1e3f110c9]::util::run_in_thread_pool_with_globals<rustc_interface[e9a32af1e3f110c9]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[9531b398d7fbf46a]::ErrorReported>, rustc_driver[1687b6f54ec85a2d]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[9531b398d7fbf46a]::ErrorReported>>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[9531b398d7fbf46a]::ErrorReported>>::{closure#1}::{closure#0}>, core[8382fcd636289ab6]::result::Result<(), rustc_errors[9531b398d7fbf46a]::ErrorReported>>
  19:     0x7f0f64ca1b10 - <<std[9f788053d5631f34]::thread::Builder>::spawn_unchecked_<rustc_interface[e9a32af1e3f110c9]::util::run_in_thread_pool_with_globals<rustc_interface[e9a32af1e3f110c9]::interface::run_compiler<core[8382fcd636289ab6]::result::Result<(), rustc_errors[9531b398d7fbf46a]::ErrorReported>, rustc_driver[1687b6f54ec85a2d]::run_compiler::{closure#1}>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[9531b398d7fbf46a]::ErrorReported>>::{closure#0}, core[8382fcd636289ab6]::result::Result<(), rustc_errors[9531b398d7fbf46a]::ErrorReported>>::{closure#1} as core[8382fcd636289ab6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7f0f64242293 - std::sys::unix::thread::Thread::new::thread_start::hd363d8910f104f91
  21:     0x7f0f5e5b4609 - start_thread
  22:     0x7f0f640ab293 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.61.0-nightly (72e817964 2022-03-02) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z tls-model=initial-exec -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
