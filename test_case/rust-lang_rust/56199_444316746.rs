
applesaucers@DESKTOP-I15PRGM:/mnt/c/Users/applesaucers/Documents/rust_test$ RUST_BACKTRACE=1 rustc +nightly i56199.rs
warning: the feature `self_struct_ctor` has been stable since 1.32.0 and no longer requires an attribute to enable
 --> i56199.rs:1:12
  |
1 | #![feature(self_struct_ctor)]
  |            ^^^^^^^^^^^^^^^^
  |
  = note: #[warn(stable_features)] on by default

warning: path statement with no effect
 --> i56199.rs:8:9
  |
8 |         Self;
  |         ^^^^^
  |
  = note: #[warn(path_statements)] on by default

error: internal compiler error: cat_expr Errd
 --> i56199.rs:7:14
  |
7 |       fn bar() {
  |  ______________^
8 | |         Self;
9 | |     }
  | |_____^

error: internal compiler error: cat_expr Errd
 --> i56199.rs:8:9
  |
8 |         Self;
  |         ^^^^

error: internal compiler error: broken MIR in DefId(0/0:5 ~ i56199[317d]::{{impl}}[0]::bar[0]) ("return type"): bad type [type error]
 --> i56199.rs:7:5
  |
7 | /     fn bar() {
8 | |         Self;
9 | |     }
  | |_____^

error: internal compiler error: broken MIR in DefId(0/0:5 ~ i56199[317d]::{{impl}}[0]::bar[0]) (LocalDecl { mutability: Mut, is_user_variable: None, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, name: None, source_info: SourceInfo { span: i56199.rs:7:5: 9:6, scope: scope[0] }, visibility_scope: scope[0] }): bad type [type error]
 --> i56199.rs:7:5
  |
7 | /     fn bar() {
8 | |         Self;
9 | |     }
  | |_____^

error: internal compiler error: QualifyAndPromoteConstants: Mir had errors
 --> i56199.rs:7:5
  |
7 | /     fn bar() {
8 | |         Self;
9 | |     }
  | |_____^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:334:17
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:59
             at src/libstd/panicking.rs:211
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:227
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:495
   6: std::panicking::begin_panic
   7: <rustc_errors::Handler as core::ops::drop::Drop>::drop
   8: core::ptr::real_drop_in_place
   9: core::ptr::real_drop_in_place
  10: core::ptr::real_drop_in_place
  11: rustc_driver::monitor::{{closure}}
  12: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:102
  13: <F as alloc::boxed::FnBox<A>>::call_box
  14: std::sys::unix::thread::Thread::new::thread_start
             at /rustc/0c999ed132d67bf2520643e9bd619972cf3888ba/src/liballoc/boxed.rs:683
             at src/libstd/sys_common/thread.rs:24
             at src/libstd/sys/unix/thread.rs:90
  15: start_thread
  16: __clone
query stack during panic:
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.32.0-nightly (0c999ed13 2018-12-03) running on x86_64-unknown-linux-gnu
