
$ RUST_BACKTRACE=1 cargo build --verbose
       Fresh unicase v0.0.5
       Fresh rustc-serialize v0.3.7
       Fresh bitflags v0.1.1
       Fresh pkg-config v0.3.2
       Fresh libc v0.1.3
       Fresh gcc v0.3.3
       Fresh httparse v0.0.5
       Fresh matches v0.1.2
       Fresh log v0.3.0
       Fresh url v0.2.28
       Fresh mime v0.0.10
       Fresh num_cpus v0.1.0
       Fresh time v0.1.21
       Fresh openssl-sys v0.5.2
       Fresh openssl v0.5.2
   Compiling fleet_client v0.0.1 (file:///Users/redacted/Code/fleet_client)
     Running `rustc src/lib.rs --crate-name fleet_client --crate-type lib -g -C metadata=143a9fc319f5ec12 -C extra-filename=-143a9fc319f5ec12 --out-dir /Users/redacted/Code/fleet_client/target/debug --emit=dep-info,link -L dependency=/Users/redacted/Code/fleet_client/target/debug -L dependency=/Users/redacted/Code/fleet_client/target/debug/deps --extern rustc_serialize=/Users/redacted/Code/fleet_client/target/debug/deps/librustc_serialize-1d1586c158359402.rlib --extern hyper=/Users/redacted/Code/fleet_client/target/debug/deps/libhyper-66ff64b8b88b65f5.rlib -L native=/Users/redacted/Code/fleet_client/target/debug/build/num_cpus-676e37b6735f47f5/out -L native=/usr/lib -L native=/Users/redacted/Code/fleet_client/target/debug/build/openssl-sys-1f3a6cfa99423735/out -L native=/Users/redacted/Code/fleet_client/target/debug/build/time-4d69cc17d95288c1/out`
       Fresh cookie v0.1.16
       Fresh hyper v0.3.3
src/client.rs:44:40: 44:75 error: internal compiler error: cat_expr Errd
src/client.rs:44                         current_state: UnitStates::from_str(current_state),
                                                        ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libsyntax/diagnostic.rs:130

stack backtrace:
   1:        0x11338dea4 - sys::backtrace::write::h78053bbfb550960cgjD
   2:        0x1133b9358 - panicking::on_panic::hc4e82bbd2a46726dR8I
   3:        0x1132d676e - rt::unwind::begin_unwind_inner::h5e8e19c0097ac0cdBRI
   4:        0x112addcbe - rt::unwind::begin_unwind::h8176364159252686369
   5:        0x112addc6b - diagnostic::SpanHandler::span_bug::hdcd6dc2adc048426AaB
   6:        0x1102dbcec - session::Session::span_bug::hd2e8d90e6e32fe5bTHn
   7:        0x10fa774db - check::regionck::visit_expr::h67ec4d109373a14f2Dd
   8:        0x10fa7f099 - visit::walk_expr::h4641389526047920594
   9:        0x10fa76783 - check::regionck::visit_expr::h67ec4d109373a14f2Dd
  10:        0x10fa76783 - check::regionck::visit_expr::h67ec4d109373a14f2Dd
  11:        0x10fa723c5 - check::regionck::Rcx<'a, 'tcx>::visit_fn_body::h91b65695e5e98981fgd
  12:        0x10fa7ef02 - visit::walk_expr::h4641389526047920594
  13:        0x10fa7638d - check::regionck::visit_expr::h67ec4d109373a14f2Dd
  14:        0x10fa7eaf7 - visit::walk_expr::h4641389526047920594
  15:        0x10fa75ccf - check::regionck::visit_expr::h67ec4d109373a14f2Dd
  16:        0x10fa7eaf7 - visit::walk_expr::h4641389526047920594
  17:        0x10fa75ccf - check::regionck::visit_expr::h67ec4d109373a14f2Dd
  18:        0x10fa7eacc - visit::walk_expr::h4641389526047920594
  19:        0x10fa76c9b - check::regionck::visit_expr::h67ec4d109373a14f2Dd
  20:        0x10fa76783 - check::regionck::visit_expr::h67ec4d109373a14f2Dd
  21:        0x10fa7eecb - visit::walk_expr::h4641389526047920594
  22:        0x10fa76353 - check::regionck::visit_expr::h67ec4d109373a14f2Dd
  23:        0x10fa723c5 - check::regionck::Rcx<'a, 'tcx>::visit_fn_body::h91b65695e5e98981fgd
  24:        0x10fb032da - check::check_bare_fn::h4d8e30b5b0012b734jn
  25:        0x10fb06eef - check::check_method_body::h31690ecdf0c794faRRn
  26:        0x10faff9f3 - check::check_item::h4b4a2b730b65c17cMCn
  27:        0x10fb01432 - visit::walk_item::h173672359968486163
  28:        0x10fbd278d - check_crate::closure.35881
  29:        0x10fbcd3da - check_crate::h4e5a2f60ebec7dccblC
  30:        0x10f90b417 - driver::phase_3_run_analysis_passes::h00707ed732905e83rGa
  31:        0x10f8f1816 - driver::compile_input::h22976530edbb8120Rba
  32:        0x10f9ab313 - run_compiler::h6f6a07f7c4e7d147s2b
  33:        0x10f9a8ea5 - thunk::F.Invoke<A, R>::invoke::h5399241532089372082
  34:        0x10f9a8267 - rt::unwind::try::try_fn::h14095088624371570858
  35:        0x11343f468 - rust_try_inner
  36:        0x11343f455 - rust_try
  37:        0x10f9a8605 - thunk::F.Invoke<A, R>::invoke::h14770910994046267349
  38:        0x1133a425d - sys::thread::create::thread_start::h3a4e0027cd5d22bdqPH
  39:     0x7fff8f118267 - _pthread_body
  40:     0x7fff8f1181e4 - _pthread_start

Could not compile `fleet_client`.

Caused by:
  Process didn't exit successfully: `rustc src/lib.rs --crate-name fleet_client --crate-type lib -g -C metadata=143a9fc319f5ec12 -C extra-filename=-143a9fc319f5ec12 --out-dir /Users/redacted/Code/fleet_client/target/debug --emit=dep-info,link -L dependency=/Users/redacted/Code/fleet_client/target/debug -L dependency=/Users/redacted/Code/fleet_client/target/debug/deps --extern rustc_serialize=/Users/redacted/Code/fleet_client/target/debug/deps/librustc_serialize-1d1586c158359402.rlib --extern hyper=/Users/redacted/Code/fleet_client/target/debug/deps/libhyper-66ff64b8b88b65f5.rlib -L native=/Users/redacted/Code/fleet_client/target/debug/build/num_cpus-676e37b6735f47f5/out -L native=/usr/lib -L native=/Users/redacted/Code/fleet_client/target/debug/build/openssl-sys-1f3a6cfa99423735/out -L native=/Users/redacted/Code/fleet_client/target/debug/build/time-4d69cc17d95288c1/out` (exit code: 101)
