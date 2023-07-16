
$ cargo build
   Compiling rustc_ice__serde_de_Error_custom__arguments v0.1.0 (~/Documents/rustc_ice__serde_de_Error_custom__arguments)
error: cannot find macro `arguments!` in this scope
  --> src/lib.rs:45:43
   |
45 |                     _ => D::Error::custom(arguments!("Unrecognized world number {}", v)),
   |                                           ^^^^^^^^^

error[E0401]: can't use type parameters from outer function; try using a local type parameter instead
  --> src/lib.rs:45:26
   |
45 |                     _ => D::Error::custom(arguments!("Unrecognized world number {}", v)),
   |                          ^^^^^^^^^^^^^^^^ use of type variable from outer function

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'assertion failed: resolution.depth == 0 || resolution.base_def != Def::Err', src/librustc_resolve/lib.rs:3060
stack backtrace:
   1:        0x108b0e1e9 - std::sys::imp::backtrace::tracing::imp::write::h9559bd7cb15d72ad
   2:        0x108b1ac9e - std::panicking::default_hook::{{closure}}::h4b3f2b69c9ce844d
   3:        0x108b1a8cb - std::panicking::default_hook::h61d415f2381a7336
   4:        0x108b1b117 - std::panicking::rust_panic_with_hook::h8e6300d8e8aca457
   5:        0x104cdf334 - std::panicking::begin_panic::h07bca7d56e8cc59f
   6:        0x104d6bf18 - rustc_resolve::Resolver::record_def::h76da3c01d36079d3
   7:        0x104d61972 - rustc_resolve::Resolver::smart_resolve_path_fragment::h5d76ea9688e88cd4
   8:        0x104d61341 - rustc_resolve::Resolver::smart_resolve_path::hf3c46d2a237088c2
   9:        0x104d6ac01 - rustc_resolve::Resolver::resolve_expr::h09b0dc21b3ab616b
  10:        0x104d6a310 - rustc_resolve::Resolver::resolve_expr::h09b0dc21b3ab616b
  11:        0x104d5f7a3 - rustc_resolve::Resolver::resolve_arm::h23d6d148db68a5b2
  12:        0x104d24eae - syntax::visit::walk_expr::h437c79100384b73b
  13:        0x104d6ac0c - rustc_resolve::Resolver::resolve_expr::h09b0dc21b3ab616b
  14:        0x104d5ffcd - rustc_resolve::Resolver::resolve_block::hbaa1b9cd3b39d13b
  15:        0x104d526d1 - <rustc_resolve::Resolver<'a> as syntax::visit::Visitor<'tcx>>::visit_fn::hc8ab95e4e9e1018f
  16:        0x104d22624 - syntax::visit::walk_impl_item::h8444851599ccdb5a
  17:        0x104d5ce0f - rustc_resolve::Resolver::with_type_parameter_rib::h3e6c8e0287344f15
  18:        0x104d5e0f1 - rustc_resolve::Resolver::with_current_self_type::h2d9f068bacea9de1
  19:        0x104d5ea2a - rustc_resolve::Resolver::with_self_rib::hc4caf49323a9c233
  20:        0x104d5e478 - rustc_resolve::Resolver::with_optional_trait_ref::ha2a0f64226efba95
  21:        0x104d5e7ae - rustc_resolve::Resolver::with_self_rib::h3225ea62b06fd17b
  22:        0x104d5b9b8 - rustc_resolve::Resolver::resolve_item::h4583f1200c5cff01
  23:        0x104d5ffb5 - rustc_resolve::Resolver::resolve_block::hbaa1b9cd3b39d13b
  24:        0x104d526d1 - <rustc_resolve::Resolver<'a> as syntax::visit::Visitor<'tcx>>::visit_fn::hc8ab95e4e9e1018f
  25:        0x104d22624 - syntax::visit::walk_impl_item::h8444851599ccdb5a
  26:        0x104d5ce0f - rustc_resolve::Resolver::with_type_parameter_rib::h3e6c8e0287344f15
  27:        0x104d5e0f1 - rustc_resolve::Resolver::with_current_self_type::h2d9f068bacea9de1
  28:        0x104d5ea2a - rustc_resolve::Resolver::with_self_rib::hc4caf49323a9c233
  29:        0x104d5e478 - rustc_resolve::Resolver::with_optional_trait_ref::ha2a0f64226efba95
  30:        0x104d5e7ae - rustc_resolve::Resolver::with_self_rib::h3225ea62b06fd17b
  31:        0x104d5b9b8 - rustc_resolve::Resolver::resolve_item::h4583f1200c5cff01
  32:        0x104d5524e - rustc_resolve::Resolver::resolve_crate::he366a34c39e14f9e
  33:        0x103ae09a1 - rustc_driver::driver::phase_2_configure_and_expand::hb928c8ce49deb9dc
  34:        0x103ad636d - rustc_driver::driver::compile_input::hd50a6918443232a0
  35:        0x103b1e21e - rustc_driver::run_compiler::h5151bfc01962b066
  36:        0x103a33928 - std::panicking::try::do_call::h9e86b95d9c931e3d
  37:        0x108b1dd6a - __rust_maybe_catch_panic
  38:        0x103a5c353 - <F as alloc::boxed::FnBox<A>>::call_box::ha55b035dd9316cf5
  39:        0x108b19dc4 - std::sys::imp::thread::Thread::new::thread_start::h80e9dc7cc1dfe0d2
  40:     0x7fff89bfa99c - _pthread_body
  41:     0x7fff89bfa919 - _pthread_start

error: Could not compile `rustc_ice__serde_de_Error_custom__arguments`.

To learn more, run the command again with --verbose.
