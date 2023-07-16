
  69: rustc_session::utils::<impl rustc_session::session::Session>::time
  70: rustc_typeck::check_crate
  71: rustc_middle::ty::context::tls::enter_global
  72: rustc_interface::interface::create_compiler_and_run
  73: rustdoc::core::run_core
...

note: compiler flags: -Z treat-err-as-bug

query stack during panic:
#0 [typeck] type-checking `f`
#1 [mir_built] building MIR for `f`
#2 [unsafety_check_result] unsafety-checking `f`
#3 [mir_const] processing MIR for `f`
#4 [mir_validated] processing `f`
#5 [mir_borrowck] borrow-checking `f`
#6 [type_of] computing type of `f::{{opaque}}#0`
#7 [check_mod_item_types] checking item types in top-level module
end of query stack
