plain
   Compiling clippy_lints v0.1.67 (/checkout/src/tools/clippy/clippy_lints)
[RUSTC-TIMING] clippy_utils test:false 10.705
[RUSTC-TIMING] clippy_lints test:false 48.348
[RUSTC-TIMING] cargo_clippy test:false 1.398
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:46 ~ clippy_driver[5216]::track_files), const_param_did: None }) (after phase change to runtime-optimized) at bb14[0]:
                                use of local _24, which has no storage here
   |
99 | }
   | ^
   |
   |
   = note: delayed at compiler/rustc_const_eval/src/transform/validate.rs:88:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:46 ~ clippy_driver[5216]::track_files), const_param_did: None }) (after phase change to runtime-optimized) at bb15[0]:
                                use of local _24, which has no storage here
   |
99 | }
   | ^
   |
   |
   = note: delayed at compiler/rustc_const_eval/src/transform/validate.rs:88:36

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:46 ~ clippy_driver[5216]::track_files), const_param_did: None }) (after phase change to runtime-optimized) at bb16[0]:
                                use of local _24, which has no storage here
   |
99 | }
   | ^
   |
   |
   = note: delayed at compiler/rustc_const_eval/src/transform/validate.rs:88:36

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1594:13
stack backtrace:
   0:     0x7f02b96ffe00 - std::backtrace_rs::backtrace::libunwind::trace::he7b388c72d5c5af1
                               at /rustc/39a4ef785f01329d4de9263d2e9153cdaaacc768/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f02b96ffe00 - std::backtrace_rs::backtrace::trace_unsynchronized::h52d9c8b4170038f9
                               at /rustc/39a4ef785f01329d4de9263d2e9153cdaaacc768/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f02b96ffe00 - std::sys_common::backtrace::_print_fmt::h4604b7ab2bad60f0
                               at /rustc/39a4ef785f01329d4de9263d2e9153cdaaacc768/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f02b96ffe00 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::heda5c74b056684cf
                               at /rustc/39a4ef785f01329d4de9263d2e9153cdaaacc768/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f02b975fe5e - core::fmt::write::hcb131ddd0c15f718
                               at /rustc/39a4ef785f01329d4de9263d2e9153cdaaacc768/library/core/src/fmt/mod.rs:1208:17
   5:     0x7f02b96f1b05 - std::io::Write::write_fmt::hfd2fdcf6193cb169
                               at /rustc/39a4ef785f01329d4de9263d2e9153cdaaacc768/library/std/src/io/mod.rs:1682:15
   6:     0x7f02b96ffbc5 - std::sys_common::backtrace::_print::h54f67baad2cc4371
                               at /rustc/39a4ef785f01329d4de9263d2e9153cdaaacc768/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f02b96ffbc5 - std::sys_common::backtrace::print::h13f8b74aa4081790
                               at /rustc/39a4ef785f01329d4de9263d2e9153cdaaacc768/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f02b97028ff - std::panicking::default_hook::{{closure}}::hab44665ca251453a
                               at /rustc/39a4ef785f01329d4de9263d2e9153cdaaacc768/library/std/src/panicking.rs:267:22
   9:     0x7f02b970263a - std::panicking::default_hook::h0cdeefaf73461997
                               at /rustc/39a4ef785f01329d4de9263d2e9153cdaaacc768/library/std/src/panicking.rs:286:9
  10:     0x7f02ba461b36 - rustc_driver[fe45e96554122331]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f02b97030c9 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::ha2057d61f04dcd53
                               at /rustc/39a4ef785f01329d4de9263d2e9153cdaaacc768/library/alloc/src/boxed.rs:2024:9
  12:     0x7f02b97030c9 - std::panicking::rust_panic_with_hook::h932d260e67858f12
                               at /rustc/39a4ef785f01329d4de9263d2e9153cdaaacc768/library/std/src/panicking.rs:692:13
  13:     0x7f02bffbb8a3 - std[6d6c9314907cb00a]::panicking::begin_panic::<rustc_errors[247b68d232213581]::ExplicitBug>::{closure#0}
  14:     0x7f02bffb9046 - std[6d6c9314907cb00a]::sys_common::backtrace::__rust_end_short_backtrace::<std[6d6c9314907cb00a]::panicking::begin_panic<rustc_errors[247b68d232213581]::ExplicitBug>::{closure#0}, !>
  15:     0x7f02ba2f9056 - std[6d6c9314907cb00a]::panicking::begin_panic::<rustc_errors[247b68d232213581]::ExplicitBug>
  16:     0x7f02bffa9a26 - std[6d6c9314907cb00a]::panic::panic_any::<rustc_errors[247b68d232213581]::ExplicitBug>
  17:     0x7f02bffb1a7a - <rustc_errors[247b68d232213581]::HandlerInner>::flush_delayed::<alloc[967626bc9549ee4e]::vec::Vec<rustc_errors[247b68d232213581]::diagnostic::Diagnostic>, &str>
  18:     0x7f02bffadae2 - <rustc_errors[247b68d232213581]::HandlerInner as core[ae65f184c67bc919]::ops::drop::Drop>::drop
  19:     0x7f02ba3dae28 - core[ae65f184c67bc919]::ptr::drop_in_place::<rustc_session[ffc7ee4522bbc933]::parse::ParseSess>
  20:     0x7f02ba3dc628 - core[ae65f184c67bc919]::ptr::drop_in_place::<rustc_session[ffc7ee4522bbc933]::session::Session>
  21:     0x7f02ba3ca069 - core[ae65f184c67bc919]::ptr::drop_in_place::<rustc_interface[f380b338ebc0772e]::interface::Compiler>
  22:     0x7f02ba3c5919 - rustc_span[7df3da3bdec3cb]::with_source_map::<core[ae65f184c67bc919]::result::Result<(), rustc_errors[247b68d232213581]::ErrorGuaranteed>, rustc_interface[f380b338ebc0772e]::interface::run_compiler<core[ae65f184c67bc919]::result::Result<(), rustc_errors[247b68d232213581]::ErrorGuaranteed>, rustc_driver[fe45e96554122331]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  23:     0x7f02ba42d028 - <scoped_tls[27cd29d9afbae785]::ScopedKey<rustc_span[7df3da3bdec3cb]::SessionGlobals>>::set::<rustc_interface[f380b338ebc0772e]::interface::run_compiler<core[ae65f184c67bc919]::result::Result<(), rustc_errors[247b68d232213581]::ErrorGuaranteed>, rustc_driver[fe45e96554122331]::run_compiler::{closure#1}>::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[247b68d232213581]::ErrorGuaranteed>>
  24:     0x7f02ba3ecd30 - std[6d6c9314907cb00a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f380b338ebc0772e]::util::run_in_thread_pool_with_globals<rustc_interface[f380b338ebc0772e]::interface::run_compiler<core[ae65f184c67bc919]::result::Result<(), rustc_errors[247b68d232213581]::ErrorGuaranteed>, rustc_driver[fe45e96554122331]::run_compiler::{closure#1}>::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[247b68d232213581]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[247b68d232213581]::ErrorGuaranteed>>
  25:     0x7f02ba3cc434 - <<std[6d6c9314907cb00a]::thread::Builder>::spawn_unchecked_<rustc_interface[f380b338ebc0772e]::util::run_in_thread_pool_with_globals<rustc_interface[f380b338ebc0772e]::interface::run_compiler<core[ae65f184c67bc919]::result::Result<(), rustc_errors[247b68d232213581]::ErrorGuaranteed>, rustc_driver[fe45e96554122331]::run_compiler::{closure#1}>::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[247b68d232213581]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[247b68d232213581]::ErrorGuaranteed>>::{closure#1} as core[ae65f184c67bc919]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  26:     0x7f02b970cdc3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h1b45accef3e95402
                               at /rustc/39a4ef785f01329d4de9263d2e9153cdaaacc768/library/alloc/src/boxed.rs:1990:9
  27:     0x7f02b970cdc3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hd6ffcefa71bd9c61
                               at /rustc/39a4ef785f01329d4de9263d2e9153cdaaacc768/library/alloc/src/boxed.rs:1990:9
  28:     0x7f02b970cdc3 - std::sys::unix::thread::Thread::new::thread_start::h79691a19e695532b
                               at /rustc/39a4ef785f01329d4de9263d2e9153cdaaacc768/library/std/src/sys/unix/thread.rs:108:17
  29:     0x7f02b9440609 - start_thread
  30:     0x7f02b9580133 - clone
  31:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-nightly (39a4ef785 2022-11-22) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -C linker=riscv64-unknown-linux-gnu-gcc -C symbol-mangling-version=v0 -Z unstable-options -Z dual-proc-macros -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z binary-dep-depinfo -Z tls-model=initial-exec
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
[RUSTC-TIMING] clippy_driver test:false 2.524
error: could not compile `clippy`
[TIMING] tool::ToolBuild { compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, target: riscv64gc-unknown-linux-gnu, tool: "clippy-driver", path: "src/tools/clippy", mode: ToolRustc, is_optional_tool: true, source_type: InTree, extra_features: [] } -- 53.951
thread 'main' panicked at 'clippy expected to build - essential tool', dist.rs:1149:14
[TIMING] tool::Clippy { compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, target: riscv64gc-unknown-linux-gnu, extra_features: [] } -- 0.000
Build completed unsuccessfully in 0:20:36
