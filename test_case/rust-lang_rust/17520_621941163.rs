
thread 'main' panicked at 'bad output: thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/backtrace.rs:17:9
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: rust_metadata_std_1b375e3cf1aedcda5e72be7c39da1f61
   3: rust_metadata_std_1b375e3cf1aedcda5e72be7c39da1f61
   4: rust_metadata_std_1b375e3cf1aedcda5e72be7c39da1f61
   5: std::panicking::rust_panic_with_hook
   6: <unknown>
   7: <unknown>
   8: <unknown>
   9: <unknown>
  10: std::rt::lang_start_internal
  11: <unknown>
  12: __libc_init
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
', /checkout/src/test/ui/backtrace.rs:67:5
