
$ rustup run nightly cargo version
cargo 0.13.0-nightly (2b061c5 2016-07-08)
$ rustup run nightly rustc --version
rustc 1.12.0-nightly (f93aaf84c 2016-07-09)
$ RUST_BACKTRACE=1 rustup run nightly cargo run --verbose
warning: TOML file found which contains invalid syntax and will soon not parse
at `/home/whs/.cargo/registry/src/github.com-1ecc6299db9ec823/url-0.5.9/Cargo.toml`.

The TOML spec requires newlines after table definitions (e.g. `[a] b = 1` is
invalid), but this file has a table header which does not have a newline after
it. A newline needs to be added and this warning will soon become a hard error
in the future.
       Fresh lazy_static v0.1.16
       Fresh httparse v1.1.2
       Fresh typeable v0.1.2
       Fresh log v0.3.6
       Fresh bitflags v0.7.0
       Fresh unicode-normalization v0.1.2
       Fresh hpack v0.2.0
       Fresh language-tags v0.2.2
       Fresh modifier v0.1.0
       Fresh cfg-if v0.1.0
       Fresh solicit v0.4.4
       Fresh lazy_static v0.2.1
       Fresh matches v0.1.2
       Fresh semver v0.1.20
       Fresh route-recognizer v0.1.11
       Fresh unicode-bidi v0.2.3
       Fresh rustc_version v0.1.7
       Fresh gcc v0.3.28
       Fresh serde v0.7.14
       Fresh libc v0.2.13
       Fresh traitobject v0.0.1
       Fresh winapi v0.2.7
       Fresh rustc-serialize v0.3.19
       Fresh idna v0.1.0
       Fresh unix_socket v0.5.0
       Fresh rand v0.3.14
       Fresh url v1.1.1
       Fresh num_cpus v0.2.13
       Fresh conduit-mime-types v0.7.3
       Fresh uuid v0.2.2
       Fresh winapi-build v0.1.1
       Fresh pkg-config v0.3.8
       Fresh url v0.5.9
       Fresh itoa v0.1.1
       Fresh mime v0.2.1
       Fresh openssl-sys v0.7.14
       Fresh traitobject v0.0.3
       Fresh error v0.1.9
       Fresh openssl-sys-extras v0.7.14
       Fresh unsafe-any v0.4.1
       Fresh openssl v0.7.14
       Fresh typemap v0.3.3
       Fresh unicase v1.4.0
       Fresh openssl-verify v0.1.0
       Fresh kernel32-sys v0.2.2
       Fresh plugin v0.2.6
       Fresh num-traits v0.1.32
       Fresh time v0.1.35
       Fresh cookie v0.2.5
       Fresh hyper v0.8.1
       Fresh hyper v0.9.9
       Fresh docker v0.0.41 (file:///home/whs/apps/d2dpush/rust-docker)
       Fresh iron v0.3.0
       Fresh serde_json v0.7.4
       Fresh router v0.1.1
   Compiling d2dpush v0.1.0 (file:///home/whs/apps/d2dpush)
     Running `rustc src/main.rs --crate-name d2dpush --crate-type bin -g --out-dir /home/whs/apps/d2dpush/target/debug --emit=dep-info,link -L dependency=/home/whs/apps/d2dpush/target/debug -L dependency=/home/whs/apps/d2dpush/target/debug/deps --extern docker=/home/whs/apps/d2dpush/target/debug/deps/libdocker-88b211894c135e52.rlib --extern iron=/home/whs/apps/d2dpush/target/debug/deps/libiron-f08359998141b18e.rlib --extern serde=/home/whs/apps/d2dpush/target/debug/deps/libserde-e281201342afb4d4.rlib --extern router=/home/whs/apps/d2dpush/target/debug/deps/librouter-ad43dc1f9ec0990f.rlib --extern serde_json=/home/whs/apps/d2dpush/target/debug/deps/libserde_json-55921106e25a0359.rlib --extern d2dpush=/home/whs/apps/d2dpush/target/debug/libd2dpush.rlib -L native=/home/whs/apps/d2dpush/target/debug/build/openssl-5464f8f6e728c35a/out -L native=/home/whs/apps/d2dpush/target/debug/build/openssl-sys-extras-5c7e4d8925825f00/out -L native=/usr/lib`
error: internal compiler error: ../src/librustc/infer/mod.rs:680: Encountered errors `[FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<[closure@<d2dpush macros>:3:1: 6:65 arc:std::sync::Arc<std::sync::Mutex<d2dpush::DockerToDockerRegistry>>] as std::ops::Fn<(&mut iron::Request<'_, '_>,)>>)),depth=1),Unimplemented)]` resolving bounds after type-checking
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', ../src/librustc_errors/lib.rs:580
stack backtrace:
   1:     0x7f43ae1ab6ef - std::sys::backtrace::tracing::imp::write::h29f5fdb9fc0a7395
   2:     0x7f43ae1c2dcb - std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::h2cc84f0378700526
   3:     0x7f43ae1bb8d7 - std::panicking::default_hook::hbbe7fa36a995aca0
   4:     0x7f43ae1bbffc - std::panicking::rust_panic_with_hook::h105c3d42fcd2fb5e
   5:     0x7f43ad7092f8 - std::panicking::begin_panic::hccc513334ab977d2
   6:     0x7f43ad76e8a5 - rustc::session::opt_span_bug_fmt::_$u7b$$u7b$closure$u7d$$u7d$::h62b0957667555cfe
   7:     0x7f43ad76e67d - rustc::ty::context::tls::with_opt::_$u7b$$u7b$closure$u7d$$u7d$::h9c829922aa02dbc6
   8:     0x7f43ad76cefd - rustc::ty::context::tls::with::_$u7b$$u7b$closure$u7d$$u7d$::hbf1d3065f68fcd1e
   9:     0x7f43ad037e7c - rustc::session::span_bug_fmt::hcdfbe9cf944f6fc3
  10:     0x7f43ad194b04 - rustc_trans::common::fulfill_obligation::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::h242bc657568ce531
  11:     0x7f43ad1876c3 - rustc::infer::InferCtxtBuilder::enter::_$u7b$$u7b$closure$u7d$$u7d$::haf361390b3a399bf
  12:     0x7f43ad197f5c - rustc_trans::common::fulfill_obligation::_$u7b$$u7b$closure$u7d$$u7d$::h3c2350df65e54c57
  13:     0x7f43ad0cff29 - rustc_trans::common::fulfill_obligation::h142143d641a8784a
  14:     0x7f43ad0c595e - _<rustc_trans..collector..MirNeighborCollector<'a, 'tcx> as rustc..mir..visit..Visitor<'tcx>>::visit_rvalue::hce35b59709f05077
  15:     0x7f43ad0356cd - rustc::mir::visit::Visitor::visit_mir::h51f79e0325d1dd39
  16:     0x7f43ad0c4745 - rustc_trans::collector::collect_items_rec::h70a0d55736c81a75
  17:     0x7f43ad0c4bac - rustc_trans::collector::collect_items_rec::h70a0d55736c81a75
  18:     0x7f43ad0c4bac - rustc_trans::collector::collect_items_rec::h70a0d55736c81a75
  19:     0x7f43ad19855e - rustc_trans::collector::collect_crate_translation_items::_$u7b$$u7b$closure$u7d$$u7d$::h50d2ebc7de2a2fe3
  20:     0x7f43ad0a3fad - rustc_trans::base::collect_and_partition_translation_items::h93e866795c3facdc
  21:     0x7f43ad09ccf7 - rustc_trans::base::trans_crate::h9baf3a0389546061
  22:     0x7f43ae798d8f - rustc_driver::driver::phase_4_translate_to_llvm::h54e99578fb1bb696
  23:     0x7f43ae824d2d - rustc_driver::driver::compile_input::_$u7b$$u7b$closure$u7d$$u7d$::h7236bd0d96fe8ce9
  24:     0x7f43ae7d7fed - rustc_driver::driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::h94bde61ccfd9ab2d
  25:     0x7f43ae7d0bc2 - rustc::ty::context::tls::enter_global::_$u7b$$u7b$closure$u7d$$u7d$::hf534c52a5122a522
  26:     0x7f43ae730adc - rustc::ty::context::TyCtxt::create_and_enter::h7a94c61bd28cb0b9
  27:     0x7f43ae78d52b - rustc_driver::driver::compile_input::hb4cc34cf85dc1edf
  28:     0x7f43ae7b3844 - rustc_driver::run_compiler::h50f95674bd902ab5
  29:     0x7f43ae829417 - rustc_driver::run::_$u7b$$u7b$closure$u7d$$u7d$::h578f58fb80a79b9c
  30:     0x7f43ae82424b - rustc_driver::monitor::_$u7b$$u7b$closure$u7d$$u7d$::he296944eac047e81
  31:     0x7f43ae7093fd - std::panicking::try::call::h4577500a5284c6ff
  32:     0x7f43ae1d0e0b - __rust_try
  33:     0x7f43ae1d0cee - __rust_maybe_catch_panic
  34:     0x7f43ae7cf6dd - std::thread::Builder::spawn::_$u7b$$u7b$closure$u7d$$u7d$::haaf8628155e4305c
  35:     0x7f43ae72316e - _<F as alloc..boxed..FnBox<A>>::call_box::h24f3eb0b42327962
  36:     0x7f43ae1b9824 - std::sys::thread::Thread::new::thread_start::h8f3bd45211e9f5ea
  37:     0x7f43a5599483 - start_thread
  38:     0x7f43addf36dc - clone
  39:                0x0 - <unknown>

error: Could not compile `d2dpush`.

Caused by:
  Process didn't exit successfully: `rustc src/main.rs --crate-name d2dpush --crate-type bin -g --out-dir /home/whs/apps/d2dpush/target/debug --emit=dep-info,link -L dependency=/home/whs/apps/d2dpush/target/debug -L dependency=/home/whs/apps/d2dpush/target/debug/deps --extern docker=/home/whs/apps/d2dpush/target/debug/deps/libdocker-88b211894c135e52.rlib --extern iron=/home/whs/apps/d2dpush/target/debug/deps/libiron-f08359998141b18e.rlib --extern serde=/home/whs/apps/d2dpush/target/debug/deps/libserde-e281201342afb4d4.rlib --extern router=/home/whs/apps/d2dpush/target/debug/deps/librouter-ad43dc1f9ec0990f.rlib --extern serde_json=/home/whs/apps/d2dpush/target/debug/deps/libserde_json-55921106e25a0359.rlib --extern d2dpush=/home/whs/apps/d2dpush/target/debug/libd2dpush.rlib -L native=/home/whs/apps/d2dpush/target/debug/build/openssl-5464f8f6e728c35a/out -L native=/home/whs/apps/d2dpush/target/debug/build/openssl-sys-extras-5c7e4d8925825f00/out -L native=/usr/lib` (exit code: 101)
