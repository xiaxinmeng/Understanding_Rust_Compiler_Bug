`
   Compiling failure v0.1.3
error: internal compiler error: src/librustc/middle/stability.rs:709: encountered unmarked API: DefId(8/1:9 ~ failure_derive[affe]::Fail[0])
 --> /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/failure-0.1.3/src/as_fail.rs:1:5
  |
1 | use Fail;
  |     ^^^^

thread 'main' panicked at 'Box<Any>', src/librustc_errors/lib.rs:538:9
stack backtrace:
   0:     0x7f85320fbdb3 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h30dc5cfcbecdb8ee
                               at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x7f85320f33d8 - std::sys_common::backtrace::_print::h738136e39b2ff032
                               at src/libstd/sys_common/backtrace.rs:71
   2:     0x7f85320f8752 - std::panicking::default_hook::{{closure}}::h0e38423a53e40139
                               at src/libstd/sys_common/backtrace.rs:59
                               at src/libstd/panicking.rs:211
   3:     0x7f85320f84bd - std::panicking::default_hook::hb15d6962c4d977af
                               at src/libstd/panicking.rs:227
   4:     0x7f852e8fed93 - rustc::util::common::panic_hook::ha05cea7920277881
   5:     0x7f85320f8f39 - std::panicking::rust_panic_with_hook::h0482cfb580b29bda
                               at src/libstd/panicking.rs:495
   6:     0x7f852e872f8c - std::panicking::begin_panic::hbbdd326685af653c
   7:     0x7f852e36280c - rustc_errors::Handler::span_bug::h054aed9ab434b8fa
   8:     0x7f852e5828d8 - rustc::util::bug::opt_span_bug_fmt::{{closure}}::h72631351e48224a1
   9:     0x7f852e580189 - rustc::ty::context::tls::with_opt::{{closure}}::h73406d6c9899a826
  10:     0x7f852e5006cf - rustc::ty::context::tls::with_context_opt::h961ef4edeef47b84
  11:     0x7f852e57ff66 - rustc::ty::context::tls::with_opt::hdd5501e7366471eb
  12:     0x7f852e7203c4 - rustc::util::bug::opt_span_bug_fmt::h33de9e9b44c739c4
  13:     0x7f852e72036f - rustc::util::bug::span_bug_fmt::hac69dfaa8c525f9e
  14:     0x7f852e0a9e76 - rustc::middle::stability::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::check_stability::h41e79ae064d47f48
  15:     0x7f852e701a32 - <rustc::middle::stability::Checker<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_path::hb1925b42fa17c0a8
  16:     0x7f852e7018dd - <rustc::middle::stability::Checker<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item::h12960347cd8a68a0
  17:     0x7f852e700fa8 - rustc::middle::stability::check_unstable_api_usage::h80848c7856a559f0
  18:     0x7f8532449405 - rustc::util::common::time::h1dc0266e2705cba1
  19:     0x7f85324cb90b - rustc::ty::context::tls::enter_context::h2864bcd5a7a2ccfb
  20:     0x7f853249b2c1 - <std::thread::local::LocalKey<T>>::with::he703e066f6a652f7
  21:     0x7f85324d59b5 - rustc::ty::context::TyCtxt::create_and_enter::hd10225802af5341c
  22:     0x7f853241a495 - rustc_driver::driver::compile_input::h4eddff7ffac748f2
  23:     0x7f85324f2352 - rustc_driver::run_compiler_with_pool::he0721bde5c2fe83d
  24:     0x7f8532422fd5 - <scoped_tls::ScopedKey<T>>::set::hfff22433bf243346
  25:     0x7f85324f137a - rustc_driver::run_compiler::h4188f970d38ab689
  26:     0x7f8532440bee - rustc_driver::monitor::{{closure}}::hfb76be33c568b420
  27:     0x7f8532116129 - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:102
  28:     0x7f85324ef47b - rustc_driver::run::h898afc54317b4182
  29:     0x7f85324fd05b - rustc_driver::main::heea2cfb5459d561f
  30:     0x558c24f3e962 - std::rt::lang_start::{{closure}}::h99e9c089f60b75f5
  31:     0x7f85320f8862 - std::panicking::try::do_call::hc68fc651a32ba0f3
                               at src/libstd/rt.rs:59
                               at src/libstd/panicking.rs:310
  32:     0x7f8532116129 - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:102
  33:     0x7f85320f92f3 - std::rt::lang_start_internal::hc67394d2f6e3934f
                               at src/libstd/panicking.rs:289
                               at src/libstd/panic.rs:398
                               at src/libstd/rt.rs:58
  34:     0x558c24f3e951 - main
  35:     0x7f8531ea7222 - __libc_start_main
  36:     0x558c24f3e808 - <unknown>
query stack during panic:
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.32.0-nightly (d09466ceb 2018-11-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z force-unstable-if-unmarked -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `failure`.
