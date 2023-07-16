plain
   Compiling rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
   Compiling rustc_const_eval v0.0.0 (/checkout/compiler/rustc_const_eval)
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
   Compiling rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3202 ~ rustc_metadata[5df2]::rmeta::encoder::{impl#17}::encode_info_for_mod::{closure#0}), const_param_did: None }) (end of phase transition to Optimized) at bb1[3]:
                                Out of bounds field field[0] for PlaceTy { ty: [generator@compiler/rustc_metadata/src/rmeta/encoder.rs:1130:73: 1149:14], variant_index: None }
    --> compiler/rustc_metadata/src/rmeta/encoder.rs:1131:32
     |
1131 |                 for item_id in md.item_ids {
     |
     = note: delayed at compiler/rustc_const_eval/src/transform/validate.rs:127:36


error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3202 ~ rustc_metadata[5df2]::rmeta::encoder::{impl#17}::encode_info_for_mod::{closure#0}), const_param_did: None }) (end of phase transition to Optimized) at bb5[6]:
                                Out of bounds field field[1] for PlaceTy { ty: [generator@compiler/rustc_metadata/src/rmeta/encoder.rs:1130:73: 1149:14], variant_index: None }
    --> compiler/rustc_metadata/src/rmeta/encoder.rs:1132:27
     |
1132 |                     match tcx.hir().item(*item_id).kind {
     |
     = note: delayed at compiler/rustc_const_eval/src/transform/validate.rs:127:36


error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3202 ~ rustc_metadata[5df2]::rmeta::encoder::{impl#17}::encode_info_for_mod::{closure#0}), const_param_did: None }) (end of phase transition to Optimized) at bb17[7]:
                                Out of bounds field field[1] for PlaceTy { ty: [generator@compiler/rustc_metadata/src/rmeta/encoder.rs:1130:73: 1149:14], variant_index: None }
    --> compiler/rustc_metadata/src/rmeta/encoder.rs:1143:30
     |
1143 |                         _ if tcx.def_key(item_id.def_id.to_def_id()).get_opt_name().is_some() => {
     |
     = note: delayed at compiler/rustc_const_eval/src/transform/validate.rs:127:36


error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3202 ~ rustc_metadata[5df2]::rmeta::encoder::{impl#17}::encode_info_for_mod::{closure#0}), const_param_did: None }) (end of phase transition to Optimized) at bb0[0]:
                                Out of bounds field field[0] for PlaceTy { ty: [generator@compiler/rustc_metadata/src/rmeta/encoder.rs:1130:73: 1149:14], variant_index: None }
    --> compiler/rustc_metadata/src/rmeta/encoder.rs:1130:73
     |
1130 |               record!(self.tables.children[def_id] <- iter_from_generator(|| {
     |  _________________________________________________________________________^
1131 | |                 for item_id in md.item_ids {
1132 | |                     match tcx.hir().item(*item_id).kind {
1133 | |                         // Foreign items are planted into their parent modules
1148 | |                 }
1149 | |             }));
     | |_____________^
     |
     |
     = note: delayed at compiler/rustc_const_eval/src/transform/validate.rs:127:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:3202 ~ rustc_metadata[5df2]::rmeta::encoder::{impl#17}::encode_info_for_mod::{closure#0}), const_param_did: None }) (end of phase transition to Optimized) at bb0[0]:
                                Out of bounds field field[1] for PlaceTy { ty: [generator@compiler/rustc_metadata/src/rmeta/encoder.rs:1130:73: 1149:14], variant_index: None }
    --> compiler/rustc_metadata/src/rmeta/encoder.rs:1130:73
     |
1130 |               record!(self.tables.children[def_id] <- iter_from_generator(|| {
     |  _________________________________________________________________________^
1131 | |                 for item_id in md.item_ids {
1132 | |                     match tcx.hir().item(*item_id).kind {
1133 | |                         // Foreign items are planted into their parent modules
1148 | |                 }
1149 | |             }));
     | |_____________^
     |
     |
     = note: delayed at compiler/rustc_const_eval/src/transform/validate.rs:127:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1701 ~ rustc_metadata[5df2]::rmeta::encoder::{impl#17}::encode_info_for_mod), const_param_did: None }) (end of phase transition to Optimized) at bb42[7]:
                                Out of bounds field field[0] for PlaceTy { ty: [generator@compiler/rustc_metadata/src/rmeta/encoder.rs:1130:73: 1149:14], variant_index: None }
    --> compiler/rustc_metadata/src/rmeta/encoder.rs:1130:73
     |
1130 |               record!(self.tables.children[def_id] <- iter_from_generator(|| {
     |  _________________________________________________________________________^
1131 | |                 for item_id in md.item_ids {
1132 | |                     match tcx.hir().item(*item_id).kind {
1133 | |                         // Foreign items are planted into their parent modules
1148 | |                 }
1149 | |             }));
     | |_____________^
     |
     |
     = note: delayed at compiler/rustc_const_eval/src/transform/validate.rs:127:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:1701 ~ rustc_metadata[5df2]::rmeta::encoder::{impl#17}::encode_info_for_mod), const_param_did: None }) (end of phase transition to Optimized) at bb42[8]:
                                Out of bounds field field[1] for PlaceTy { ty: [generator@compiler/rustc_metadata/src/rmeta/encoder.rs:1130:73: 1149:14], variant_index: None }
    --> compiler/rustc_metadata/src/rmeta/encoder.rs:1130:73
     |
1130 |               record!(self.tables.children[def_id] <- iter_from_generator(|| {
     |  _________________________________________________________________________^
1131 | |                 for item_id in md.item_ids {
1132 | |                     match tcx.hir().item(*item_id).kind {
1133 | |                         // Foreign items are planted into their parent modules
1148 | |                 }
1149 | |             }));
     | |_____________^
     |
     |
     = note: delayed at compiler/rustc_const_eval/src/transform/validate.rs:127:36

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1369:13
stack backtrace:
   0:     0x7f4796ca0d02 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha288dc719902e24d
   1:     0x7f4796d08648 - core::fmt::write::h42234c3e51154f4c
   2:     0x7f4796c91051 - std::io::Write::write_fmt::h74fbb9643da2d185
   3:     0x7f4796ca4046 - std::panicking::default_hook::{{closure}}::h3bfe018301273550
   4:     0x7f4796ca3c3d - std::panicking::default_hook::h4173afa32faa81d9
   5:     0x7f47977f3e91 - rustc_driver[87b42345fa270eee]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f4796ca49e0 - std::panicking::rust_panic_with_hook::h59cc3082e9104592
   7:     0x7f479a2bf1b3 - std[eba90a372f7a1edd]::panicking::begin_panic::<rustc_errors[984494b0cf0e650]::ExplicitBug>::{closure#0}
   8:     0x7f479a2bc366 - std[eba90a372f7a1edd]::sys_common::backtrace::__rust_end_short_backtrace::<std[eba90a372f7a1edd]::panicking::begin_panic<rustc_errors[984494b0cf0e650]::ExplicitBug>::{closure#0}, !>
   9:     0x7f47977381f6 - std[eba90a372f7a1edd]::panicking::begin_panic::<rustc_errors[984494b0cf0e650]::ExplicitBug>
  10:     0x7f479a304456 - std[eba90a372f7a1edd]::panic::panic_any::<rustc_errors[984494b0cf0e650]::ExplicitBug>
  11:     0x7f479a308c37 - <rustc_errors[984494b0cf0e650]::HandlerInner as core[10878fb91fc84a80]::ops::drop::Drop>::drop
  12:     0x7f4797779af2 - core[10878fb91fc84a80]::ptr::drop_in_place::<rustc_session[2a929b385c5bc398]::parse::ParseSess>
  13:     0x7f479777ed75 - <alloc[24f448636cd10183]::rc::Rc<rustc_session[2a929b385c5bc398]::session::Session> as core[10878fb91fc84a80]::ops::drop::Drop>::drop
  14:     0x7f479776f78c - core[10878fb91fc84a80]::ptr::drop_in_place::<rustc_interface[fc3bf7b819dbb0d8]::interface::Compiler>
  15:     0x7f479776c0b4 - rustc_span[5c736203a6ab5594]::with_source_map::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_interface[fc3bf7b819dbb0d8]::interface::create_compiler_and_run<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f479778ae7f - <scoped_tls[5ed4b67c3b198af5]::ScopedKey<rustc_span[5c736203a6ab5594]::SessionGlobals>>::set::<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  17:     0x7f47977df869 - std[eba90a372f7a1edd]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  18:     0x7f479779e4d1 - std[eba90a372f7a1edd]::panicking::try::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<<std[eba90a372f7a1edd]::thread::Builder>::spawn_unchecked_<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  19:     0x7f47977e1ae2 - <<std[eba90a372f7a1edd]::thread::Builder>::spawn_unchecked_<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7f4796cb13e3 - std::sys::unix::thread::Thread::new::thread_start::h6bc2e8f9e4e3d29f
  21:     0x7f4791201609 - start_thread
  22:     0x7f4796b14163 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.62.0-nightly (7c167840c 2022-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -Z tls-model=initial-exec -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
