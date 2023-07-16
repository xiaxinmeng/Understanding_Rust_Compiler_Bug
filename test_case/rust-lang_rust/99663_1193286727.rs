
warning: Error finalizing incremental compilation session directory `/home/waffle/projects/repos/teloxide-core/target/debug/incremental/teloxide_core-18q92qydgl9s2/s-gby0l5k2b7-uchhu9-working`: No such file or directory (os error 2)

error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: ty_is_local invoked on closure or generator: requests::json::inner0::Send<_>
  |
  = note: delayed at compiler/rustc_trait_selection/src/traits/coherence.rs:707:31

error: internal compiler error: ty_is_local invoked on closure or generator: requests::json::inner1::SendRef<_>
  |
  = note: delayed at compiler/rustc_trait_selection/src/traits/coherence.rs:707:31

error: internal compiler error: ty_is_local invoked on closure or generator: requests::multipart::inner0::Send<_>
  |
  = note: delayed at compiler/rustc_trait_selection/src/traits/coherence.rs:707:31

error: internal compiler error: ty_is_local invoked on closure or generator: requests::multipart::inner1::SendRef<_>
  |
  = note: delayed at compiler/rustc_trait_selection/src/traits/coherence.rs:707:31

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1426:13
stack backtrace:
   0:     0x7f6baaaa2a60 - std::backtrace_rs::backtrace::libunwind::trace::hf2beb4832a93d58c
                               at /rustc/62b272d25c5bb8b6bb8ac73797d82b8b9a1eabda/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   1:     0x7f6baaaa2a60 - std::backtrace_rs::backtrace::trace_unsynchronized::h3982da4fbb41ae41
                               at /rustc/62b272d25c5bb8b6bb8ac73797d82b8b9a1eabda/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f6baaaa2a60 - std::sys_common::backtrace::_print_fmt::h5267f0008a8fc426
                               at /rustc/62b272d25c5bb8b6bb8ac73797d82b8b9a1eabda/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f6baaaa2a60 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h71e6048dc8b53372
                               at /rustc/62b272d25c5bb8b6bb8ac73797d82b8b9a1eabda/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f6baaafc03c - core::fmt::write::h44fac1ed2c2bc452
                               at /rustc/62b272d25c5bb8b6bb8ac73797d82b8b9a1eabda/library/core/src/fmt/mod.rs:1198:17
   5:     0x7f6baaa94265 - std::io::Write::write_fmt::hdf9bd6769e922d0d
                               at /rustc/62b272d25c5bb8b6bb8ac73797d82b8b9a1eabda/library/std/src/io/mod.rs:1672:15
   6:     0x7f6baaaa56f1 - std::sys_common::backtrace::_print::h788796696fb30551
                               at /rustc/62b272d25c5bb8b6bb8ac73797d82b8b9a1eabda/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f6baaaa56f1 - std::sys_common::backtrace::print::h40448107ebb1926c
                               at /rustc/62b272d25c5bb8b6bb8ac73797d82b8b9a1eabda/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f6baaaa56f1 - std::panicking::default_hook::{{closure}}::h1ab3d8de4ebf8726
                               at /rustc/62b272d25c5bb8b6bb8ac73797d82b8b9a1eabda/library/std/src/panicking.rs:295:22
   9:     0x7f6baaaa53c3 - std::panicking::default_hook::he77b437669b8a585
                               at /rustc/62b272d25c5bb8b6bb8ac73797d82b8b9a1eabda/library/std/src/panicking.rs:314:9
  10:     0x7f6bab3504e4 - rustc_driver[675ea44f3a6668c]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f6baaaa5ec6 - std::panicking::rust_panic_with_hook::hb4404628327391c1
                               at /rustc/62b272d25c5bb8b6bb8ac73797d82b8b9a1eabda/library/std/src/panicking.rs:702:17
  12:     0x7f6bac50d1f1 - std[4ad147cfb1c6141d]::panicking::begin_panic::<rustc_errors[9bc3d6868954527]::ExplicitBug>::{closure#0}
  13:     0x7f6bac50cf96 - std[4ad147cfb1c6141d]::sys_common::backtrace::__rust_end_short_backtrace::<std[4ad147cfb1c6141d]::panicking::begin_panic<rustc_errors[9bc3d6868954527]::ExplicitBug>::{closure#0}, !>
  14:     0x7f6bac506a16 - std[4ad147cfb1c6141d]::panicking::begin_panic::<rustc_errors[9bc3d6868954527]::ExplicitBug>
  15:     0x7f6bac5097c6 - std[4ad147cfb1c6141d]::panic::panic_any::<rustc_errors[9bc3d6868954527]::ExplicitBug>
  16:     0x7f6baddebb73 - <rustc_errors[9bc3d6868954527]::HandlerInner as core[6322eef8c4bd95ac]::ops::drop::Drop>::drop
  17:     0x7f6bad51ba68 - core[6322eef8c4bd95ac]::ptr::drop_in_place::<rustc_session[5d29067c91fd7252]::parse::ParseSess>
  18:     0x7f6bad51e6d3 - <alloc[c193065513c74202]::rc::Rc<rustc_session[5d29067c91fd7252]::session::Session> as core[6322eef8c4bd95ac]::ops::drop::Drop>::drop
  19:     0x7f6bad5009fd - core[6322eef8c4bd95ac]::ptr::drop_in_place::<rustc_interface[a4f70ba529f15626]::interface::Compiler>
  20:     0x7f6bad4feae4 - rustc_span[d928a03ea8d1c0cd]::with_source_map::<core[6322eef8c4bd95ac]::result::Result<(), rustc_errors[9bc3d6868954527]::ErrorGuaranteed>, rustc_interface[a4f70ba529f15626]::interface::create_compiler_and_run<core[6322eef8c4bd95ac]::result::Result<(), rustc_errors[9bc3d6868954527]::ErrorGuaranteed>, rustc_driver[675ea44f3a6668c]::run_compiler::{closure#1}>::{closure#1}>
  21:     0x7f6bad51b000 - rustc_interface[a4f70ba529f15626]::interface::create_compiler_and_run::<core[6322eef8c4bd95ac]::result::Result<(), rustc_errors[9bc3d6868954527]::ErrorGuaranteed>, rustc_driver[675ea44f3a6668c]::run_compiler::{closure#1}>
  22:     0x7f6bad52f062 - <scoped_tls[90013e0bb440dd89]::ScopedKey<rustc_span[d928a03ea8d1c0cd]::SessionGlobals>>::set::<rustc_interface[a4f70ba529f15626]::interface::run_compiler<core[6322eef8c4bd95ac]::result::Result<(), rustc_errors[9bc3d6868954527]::ErrorGuaranteed>, rustc_driver[675ea44f3a6668c]::run_compiler::{closure#1}>::{closure#0}, core[6322eef8c4bd95ac]::result::Result<(), rustc_errors[9bc3d6868954527]::ErrorGuaranteed>>
  23:     0x7f6bad500e3f - std[4ad147cfb1c6141d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a4f70ba529f15626]::util::run_in_thread_pool_with_globals<rustc_interface[a4f70ba529f15626]::interface::run_compiler<core[6322eef8c4bd95ac]::result::Result<(), rustc_errors[9bc3d6868954527]::ErrorGuaranteed>, rustc_driver[675ea44f3a6668c]::run_compiler::{closure#1}>::{closure#0}, core[6322eef8c4bd95ac]::result::Result<(), rustc_errors[9bc3d6868954527]::ErrorGuaranteed>>::{closure#0}, core[6322eef8c4bd95ac]::result::Result<(), rustc_errors[9bc3d6868954527]::ErrorGuaranteed>>
  24:     0x7f6bad51b459 - <<std[4ad147cfb1c6141d]::thread::Builder>::spawn_unchecked_<rustc_interface[a4f70ba529f15626]::util::run_in_thread_pool_with_globals<rustc_interface[a4f70ba529f15626]::interface::run_compiler<core[6322eef8c4bd95ac]::result::Result<(), rustc_errors[9bc3d6868954527]::ErrorGuaranteed>, rustc_driver[675ea44f3a6668c]::run_compiler::{closure#1}>::{closure#0}, core[6322eef8c4bd95ac]::result::Result<(), rustc_errors[9bc3d6868954527]::ErrorGuaranteed>>::{closure#0}, core[6322eef8c4bd95ac]::result::Result<(), rustc_errors[9bc3d6868954527]::ErrorGuaranteed>>::{closure#1} as core[6322eef8c4bd95ac]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  25:     0x7f6baaaaf913 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h8c262efc0d3bf7b6
                               at /rustc/62b272d25c5bb8b6bb8ac73797d82b8b9a1eabda/library/alloc/src/boxed.rs:1935:9
  26:     0x7f6baaaaf913 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h1282f3bacc4e8c5e
                               at /rustc/62b272d25c5bb8b6bb8ac73797d82b8b9a1eabda/library/alloc/src/boxed.rs:1935:9
  27:     0x7f6baaaaf913 - std::sys::unix::thread::Thread::new::thread_start::h1bf757eaa35d6ff5
                               at /rustc/62b272d25c5bb8b6bb8ac73797d82b8b9a1eabda/library/std/src/sys/unix/thread.rs:108:17
  28:     0x7f6baa68c54d - <unknown>
  29:     0x7f6baa711874 - clone
  30:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (62b272d25 2022-07-21) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C debuginfo=2 -C linker=clang -C incremental -C link-arg=-fuse-ld=/usr/bin/mold

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
