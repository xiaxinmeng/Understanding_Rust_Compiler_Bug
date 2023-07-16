
thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1', /rustc/85976442558bf2d09cec3aa49c9c9ba86fb15c1f/src/libcore/slice/mod.rs:2791:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.42.0-nightly (859764425 2020-01-07) running on x86_64-unknown-linux-musl

note: compiler flags: -C debuginfo=2 -C incremental -C target-feature=-crt-static --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: internal compiler error: bad placeholder type
  --> src/lib.rs:40:25
   |
40 |                         _, diesel::query_source::joins::Inner, _
   |                         ^

error: internal compiler error: bad placeholder type
  --> src/lib.rs:40:64
   |
40 |                         _, diesel::query_source::joins::Inner, _
   |                                                                ^

error: internal compiler error: cat_expr Errd
  --> src/lib.rs:44:9
   |
44 |         table.inner_join(wines::table.inner_join(purchases::table))
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: internal compiler error: PromoteTemps: MIR had errors
  --> src/lib.rs:38:5
   |
38 | /     pub fn top<Table: diesel::Table +
39 | |                       diesel::query_dsl::InternalJoinDsl<
40 | |                         _, diesel::query_source::joins::Inner, _
41 | |                       >>(
...  |
44 | |         table.inner_join(wines::table.inner_join(purchases::table))
45 | |     }
   | |_____^

error: internal compiler error: broken MIR in DefId(0:10 ~ vinoteca[fe16]::model[0]::top[0]) ("return type"): bad type [type error]
  --> src/lib.rs:38:5
   |
38 | /     pub fn top<Table: diesel::Table +
39 | |                       diesel::query_dsl::InternalJoinDsl<
40 | |                         _, diesel::query_source::joins::Inner, _
41 | |                       >>(
...  |
44 | |         table.inner_join(wines::table.inner_join(purchases::table))
45 | |     }
   | |_____^

error: internal compiler error: broken MIR in DefId(0:10 ~ vinoteca[fe16]::model[0]::top[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: src/lib.rs:38:5: 45:6, scope: scope[0] } }): bad type [type error]
  --> src/lib.rs:38:5
   |
38 | /     pub fn top<Table: diesel::Table +
39 | |                       diesel::query_dsl::InternalJoinDsl<
40 | |                         _, diesel::query_source::joins::Inner, _
41 | |                       >>(
...  |
44 | |         table.inner_join(wines::table.inner_join(purchases::table))
45 | |     }
   | |_____^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:347:17
stack backtrace:
   0:     0x7f495cc6e4c4 - backtrace::backtrace::libunwind::trace::h208b77d6980cdba3
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1:     0x7f495cc6e4c4 - backtrace::backtrace::trace_unsynchronized::h279c5063b29b89e0
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2:     0x7f495cc6e4c4 - std::sys_common::backtrace::_print_fmt::h0dd5ce3b2c0c1196
                               at src/libstd/sys_common/backtrace.rs:77
   3:     0x7f495cc6e4c4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h64e0000db2a39f17
                               at src/libstd/sys_common/backtrace.rs:59
   4:     0x7f495cc9e9ec - core::fmt::write::h97bceedab1a3faef
                               at src/libcore/fmt/mod.rs:1057
   5:     0x7f495cc618c7 - std::io::Write::write_fmt::hdf931701cccb1c38
                               at src/libstd/io/mod.rs:1426
   6:     0x7f495cc7296e - std::sys_common::backtrace::_print::h206f5e67efbe6095
                               at src/libstd/sys_common/backtrace.rs:62
   7:     0x7f495cc7296e - std::sys_common::backtrace::print::h2e216a34252f7d84
                               at src/libstd/sys_common/backtrace.rs:49
   8:     0x7f495cc7296e - std::panicking::default_hook::{{closure}}::hdf8b9b1ee9f4dcfe
                               at src/libstd/panicking.rs:195
   9:     0x7f495cc72661 - std::panicking::default_hook::h98bc5701a731ade9
                               at src/libstd/panicking.rs:215
  10:     0x7f495d5866b3 - rustc_driver::report_ice::hdbf2866e344d6245
  11:     0x7f49563348f8 - <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call::hdd55ec429bea19f0
                               at /rustc/85976442558bf2d09cec3aa49c9c9ba86fb15c1f/src/liballoc/boxed.rs:1029
  12:     0x7f495631d354 - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}::hbc8b828c4363939a
                               at /rustc/85976442558bf2d09cec3aa49c9c9ba86fb15c1f/src/libproc_macro/bridge/client.rs:305
  13:     0x7f495cc73089 - std::panicking::rust_panic_with_hook::h7277f8b6e4c099fd
                               at src/libstd/panicking.rs:467
  14:     0x7f4961835dee - std::panicking::begin_panic::hd548f822c14e0cbe
  15:     0x7f4961868cfc - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::hcf14cbbd78c44984
  16:     0x7f495d571e76 - core::ptr::real_drop_in_place::hc517d173be821d25
  17:     0x7f495d56c0f3 - core::ptr::real_drop_in_place::ha038d4c0a443cf63
  18:     0x7f495d5596cc - core::ptr::real_drop_in_place::h3255152b619a5178
  19:     0x7f495d553cad - rustc_interface::interface::run_compiler_in_existing_thread_pool::h8e3bab68016629f6
  20:     0x7f495d52a6fd - scoped_tls::ScopedKey<T>::set::h32b9d2143e69dde9
  21:     0x7f495d5a3d54 - syntax::with_globals::hb27713d83228f67a
  22:     0x7f495d5240c0 - std::sys_common::backtrace::__rust_begin_short_backtrace::h1eb36621cab00684
  23:     0x7f495cc82e3a - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:79
  24:     0x7f495d534e19 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hf5249265f3209535
  25:     0x7f495cc5366f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h0bf48004752a5fb0
                               at /rustc/85976442558bf2d09cec3aa49c9c9ba86fb15c1f/src/liballoc/boxed.rs:1015
  26:     0x7f495cc82010 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h8142139182c9e7b6
                               at /rustc/85976442558bf2d09cec3aa49c9c9ba86fb15c1f/src/liballoc/boxed.rs:1015
  27:     0x7f495cc82010 - std::sys_common::thread::start_thread::h1e46798ecd04df83
                               at src/libstd/sys_common/thread.rs:13
  28:     0x7f495cc82010 - std::sys::unix::thread::Thread::new::thread_start::h8fe9d151ada3a9de
                               at src/libstd/sys/unix/thread.rs:80
  29:     0x7f496386f72b - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.42.0-nightly (859764425 2020-01-07) running on x86_64-unknown-linux-musl

note: compiler flags: -C debuginfo=2 -C incremental -C target-feature=-crt-static --crate-type lib

note: some of the compiler flags provided by cargo are hidden

thread panicked while panicking. aborting.
