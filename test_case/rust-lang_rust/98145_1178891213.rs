plain
   Compiling stable_deref_trait v1.2.0
   Compiling arrayvec v0.7.0
   Compiling remove_dir_all v0.5.3
   Compiling serde_derive v1.0.125
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:62 ~ once_cell[f219]::imp::{impl#4}::initialize::{closure#0}), const_param_did: None }) (end of phase transition to Optimized) at bb5[13]:
                                use of local _16, which has no storage here
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/once_cell-1.12.0/src/imp_std.rs:87:34
   |
87 |                         unsafe { *slot = Some(value) };
   |
   = note: delayed at compiler/rustc_const_eval/src/transform/validate.rs:129:36


error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:62 ~ once_cell[f219]::imp::{impl#4}::initialize::{closure#0}), const_param_did: None }) (end of phase transition to Optimized) at bb8[2]:
                                use of local _16, which has no storage here
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/once_cell-1.12.0/src/imp_std.rs:87:34
   |
87 |                         unsafe { *slot = Some(value) };
   |
   = note: delayed at compiler/rustc_const_eval/src/transform/validate.rs:129:36


error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:62 ~ once_cell[f219]::imp::{impl#4}::initialize::{closure#0}), const_param_did: None }) (end of phase transition to Optimized) at bb9[2]:
                                use of local _16, which has no storage here
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/once_cell-1.12.0/src/imp_std.rs:87:34
   |
87 |                         unsafe { *slot = Some(value) };
   |
   = note: delayed at compiler/rustc_const_eval/src/transform/validate.rs:129:36

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
   0:     0x7fd157028e32 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb97170048eb2feed
   1:     0x7fd157090a68 - core::fmt::write::hddd81543d4c11162
   2:     0x7fd157019221 - std::io::Write::write_fmt::hb10c7c89d9d85e51
   3:     0x7fd15702c149 - std::panicking::default_hook::{{closure}}::hf26c6e572bb06902
   4:     0x7fd15702bdea - std::panicking::default_hook::h5e275631e30996f3
   Compiling cpufeatures v0.2.1
   5:     0x7fd157b362d4 - rustc_driver[68d88b74f1d094c5]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fd15702c9af - std::panicking::rust_panic_with_hook::h88a63d00b2da51a4
   7:     0x7fd15a6d41a3 - std[cf5d045d49351115]::panicking::begin_panic::<rustc_errors[3792828eff10f32e]::ExplicitBug>::{closure#0}
   8:     0x7fd15a6d0706 - std[cf5d045d49351115]::sys_common::backtrace::__rust_end_short_backtrace::<std[cf5d045d49351115]::panicking::begin_panic<rustc_errors[3792828eff10f32e]::ExplicitBug>::{closure#0}, !>
   9:     0x7fd157aeea56 - std[cf5d045d49351115]::panicking::begin_panic::<rustc_errors[3792828eff10f32e]::ExplicitBug>
  10:     0x7fd15a6e8ec6 - std[cf5d045d49351115]::panic::panic_any::<rustc_errors[3792828eff10f32e]::ExplicitBug>
  11:     0x7fd15a6ed478 - <rustc_errors[3792828eff10f32e]::HandlerInner as core[990e9aac2c65c03a]::ops::drop::Drop>::drop
  12:     0x7fd157b45612 - core[990e9aac2c65c03a]::ptr::drop_in_place::<rustc_session[dfaee8f0762e9ce8]::parse::ParseSess>
  13:     0x7fd157b49d3a - <alloc[b31f836c82ae9902]::rc::Rc<rustc_session[dfaee8f0762e9ce8]::session::Session> as core[990e9aac2c65c03a]::ops::drop::Drop>::drop
  14:     0x7fd157baa39c - core[990e9aac2c65c03a]::ptr::drop_in_place::<rustc_interface[fb356c2aecd6de1b]::interface::Compiler>
  15:     0x7fd157ba8345 - rustc_span[4929ef0fa1883a40]::with_source_map::<core[990e9aac2c65c03a]::result::Result<(), rustc_errors[3792828eff10f32e]::ErrorGuaranteed>, rustc_interface[fb356c2aecd6de1b]::interface::create_compiler_and_run<core[990e9aac2c65c03a]::result::Result<(), rustc_errors[3792828eff10f32e]::ErrorGuaranteed>, rustc_driver[68d88b74f1d094c5]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7fd157b4c4de - <scoped_tls[56ec63818e29d8c5]::ScopedKey<rustc_span[4929ef0fa1883a40]::SessionGlobals>>::set::<rustc_interface[fb356c2aecd6de1b]::interface::run_compiler<core[990e9aac2c65c03a]::result::Result<(), rustc_errors[3792828eff10f32e]::ErrorGuaranteed>, rustc_driver[68d88b74f1d094c5]::run_compiler::{closure#1}>::{closure#0}, core[990e9aac2c65c03a]::result::Result<(), rustc_errors[3792828eff10f32e]::ErrorGuaranteed>>
  17:     0x7fd157ba4a79 - std[cf5d045d49351115]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[fb356c2aecd6de1b]::util::run_in_thread_pool_with_globals<rustc_interface[fb356c2aecd6de1b]::interface::run_compiler<core[990e9aac2c65c03a]::result::Result<(), rustc_errors[3792828eff10f32e]::ErrorGuaranteed>, rustc_driver[68d88b74f1d094c5]::run_compiler::{closure#1}>::{closure#0}, core[990e9aac2c65c03a]::result::Result<(), rustc_errors[3792828eff10f32e]::ErrorGuaranteed>>::{closure#0}, core[990e9aac2c65c03a]::result::Result<(), rustc_errors[3792828eff10f32e]::ErrorGuaranteed>>
   Compiling rustc_graphviz v0.0.0 (/checkout/compiler/rustc_graphviz)
  18:     0x7fd157b9c7e9 - <<std[cf5d045d49351115]::thread::Builder>::spawn_unchecked_<rustc_interface[fb356c2aecd6de1b]::util::run_in_thread_pool_with_globals<rustc_interface[fb356c2aecd6de1b]::interface::run_compiler<core[990e9aac2c65c03a]::result::Result<(), rustc_errors[3792828eff10f32e]::ErrorGuaranteed>, rustc_driver[68d88b74f1d094c5]::run_compiler::{closure#1}>::{closure#0}, core[990e9aac2c65c03a]::result::Result<(), rustc_errors[3792828eff10f32e]::ErrorGuaranteed>>::{closure#0}, core[990e9aac2c65c03a]::result::Result<(), rustc_errors[3792828eff10f32e]::ErrorGuaranteed>>::{closure#1} as core[990e9aac2c65c03a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  19:     0x7fd157039373 - std::sys::unix::thread::Thread::new::thread_start::hfdb91e9761b815b8
  20:     0x7fd151588609 - start_thread
  21:     0x7fd156e9b133 - clone
  22:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (c803fc6a7 2022-07-08) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -Z tls-model=initial-exec -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
