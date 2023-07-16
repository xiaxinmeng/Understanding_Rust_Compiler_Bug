
   Compiling bug v0.1.0 (/tmp/bug)
warning: the feature `const_generics` is incomplete and may cause the compiler to crash
 --> src/main.rs:1:12
  |
1 | #![feature(const_generics)]
  |            ^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default

warning: Error finalizing incremental compilation session directory `/tmp/bug/target/debug/incremental/bug-19qm6gngth5a1/s-flslstafo1-1loj99m-working`: No such file or directory (os error 2)

error: internal compiler error: mir_const_qualif: MIR had errors
  --> src/main.rs:13:13
   |
13 |     <S as T<0usize>>::f();
   |             ^^^^^^

error: internal compiler error: PromoteTemps: MIR had errors
  --> src/main.rs:13:13
   |
13 |     <S as T<0usize>>::f();
   |             ^^^^^^

error: internal compiler error: broken MIR in DefId(0:12 ~ bug[538c]::main[0]::{{constant}}[0]) ("return type"): bad type [type error]
  --> src/main.rs:13:13
   |
13 |     <S as T<0usize>>::f();
   |             ^^^^^^

error: internal compiler error: broken MIR in DefId(0:12 ~ bug[538c]::main[0]::{{constant}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: src/main.rs:13:13: 13:19, scope: scope[0] } }): bad type [type error]
  --> src/main.rs:13:13
   |
13 |     <S as T<0usize>>::f();
   |             ^^^^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:360:17
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/backtrace/libunwind.rs:86
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.44/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:78
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1069
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1427
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:198
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:218
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:515
  12: std::panicking::begin_panic
  13: <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop
  14: core::ptr::drop_in_place
  15: <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop
  16: core::ptr::drop_in_place
  17: rustc_interface::interface::run_compiler_in_existing_thread_pool
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.0-nightly (f509b26a7 2020-03-18) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `bug`.

To learn more, run the command again with --verbose.
