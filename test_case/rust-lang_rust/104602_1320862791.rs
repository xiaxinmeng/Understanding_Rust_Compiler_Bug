plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.82
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: internal compiler error: compiler/rustc_middle/src/middle/privacy.rs:203:17: fully private item in the table DefId(0:1512 ~ core[3b09]::intrinsics::{use#11}): Restricted(DefId(0:1495 ~ core[3b09]::intrinsics))
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:968:33
stack backtrace:
stack backtrace:
   0:     0x7f24486264a2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6f63aca0d818ff3d
   1:     0x7f2448694608 - core::fmt::write::h0cd9f5419c66d611
   2:     0x7f24486170c1 - std::io::Write::write_fmt::hdbe174a0534139e1
   3:     0x7f2448626265 - std::sys_common::backtrace::print::h8337b838189f3756
   4:     0x7f2448629577 - std::panicking::default_hook::{{closure}}::h40a27f353e96fa92
   5:     0x7f24486292d5 - std::panicking::default_hook::h7a5f0283402423a0
   6:     0x7f2448fe5b24 - rustc_driver[33805c906714b4ec]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f2448629e83 - std::panicking::rust_panic_with_hook::hec8edde2808e982b
   8:     0x7f244ba3c7c3 - std[f70911762a7dc09b]::panicking::begin_panic::<rustc_errors[f00c99bb6f3a048b]::ExplicitBug>::{closure#0}
   9:     0x7f244ba3c426 - std[f70911762a7dc09b]::sys_common::backtrace::__rust_end_short_backtrace::<std[f70911762a7dc09b]::panicking::begin_panic<rustc_errors[f00c99bb6f3a048b]::ExplicitBug>::{closure#0}, !>
  10:     0x7f2448f88526 - std[f70911762a7dc09b]::panicking::begin_panic::<rustc_errors[f00c99bb6f3a048b]::ExplicitBug>
  11:     0x7f244ba3c416 - std[f70911762a7dc09b]::panic::panic_any::<rustc_errors[f00c99bb6f3a048b]::ExplicitBug>
  12:     0x7f244ba353f1 - <rustc_errors[f00c99bb6f3a048b]::HandlerInner>::span_bug::<rustc_span[8615aa9fcc15d8c5]::span_encoding::Span, &alloc[a6df30274c58c997]::string::String>
  13:     0x7f244ba35207 - <rustc_errors[f00c99bb6f3a048b]::Handler>::span_bug::<rustc_span[8615aa9fcc15d8c5]::span_encoding::Span, &alloc[a6df30274c58c997]::string::String>
  14:     0x7f244bba7a14 - rustc_middle[7be20166cc633508]::ty::context::tls::with_context_opt::<rustc_middle[7be20166cc633508]::ty::context::tls::with_opt<rustc_middle[7be20166cc633508]::util::bug::opt_span_bug_fmt<rustc_span[8615aa9fcc15d8c5]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7f244bba8b89 - rustc_middle[7be20166cc633508]::util::bug::opt_span_bug_fmt::<rustc_span[8615aa9fcc15d8c5]::span_encoding::Span>
  16:     0x7f2448f8e1c7 - rustc_middle[7be20166cc633508]::util::bug::span_bug_fmt::<rustc_span[8615aa9fcc15d8c5]::span_encoding::Span>
  17:     0x7f244bbc9154 - <rustc_middle[7be20166cc633508]::middle::privacy::EffectiveVisibilities>::check_invariants
  18:     0x7f24496ec58b - rustc_privacy[6a86c43375f77618]::effective_visibilities
  19:     0x7f244ac99d8a - rustc_query_system[be8adce5b7c83ed2]::query::plumbing::try_execute_query::<rustc_query_impl[d94912f1f7b2d54e]::plumbing::QueryCtxt, rustc_query_system[be8adce5b7c83ed2]::query::caches::DefaultCache<(), &rustc_middle[7be20166cc633508]::middle::privacy::EffectiveVisibilities>>
  20:     0x7f244ad44eb0 - rustc_query_system[be8adce5b7c83ed2]::query::plumbing::get_query::<rustc_query_impl[d94912f1f7b2d54e]::queries::effective_visibilities, rustc_query_impl[d94912f1f7b2d54e]::plumbing::QueryCtxt>
  21:     0x7f244a9256fa - <rustc_query_impl[d94912f1f7b2d54e]::Queries as rustc_middle[7be20166cc633508]::ty::query::QueryEngine>::effective_visibilities
  22:     0x7f2449f697eb - rustc_passes[666026c91860f341]::stability::check_unused_or_stable_features
  23:     0x7f244910a5d0 - <rustc_session[1ff47ef0d4d6d0ce]::session::Session>::time::<(), rustc_interface[e7d8156fe52b0687]::passes::analysis::{closure#0}::{closure#2}::{closure#0}>
  24:     0x7f24491030e5 - std[f70911762a7dc09b]::panic::catch_unwind::<core[867cfca19013d5a]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[e7d8156fe52b0687]::passes::analysis::{closure#0}::{closure#2}>, ()>
  25:     0x7f244910b2c6 - <rustc_session[1ff47ef0d4d6d0ce]::session::Session>::time::<(), rustc_interface[e7d8156fe52b0687]::passes::analysis::{closure#0}>
  26:     0x7f2449132186 - rustc_interface[e7d8156fe52b0687]::passes::analysis
  27:     0x7f244ac91fc0 - rustc_query_system[be8adce5b7c83ed2]::query::plumbing::try_execute_query::<rustc_query_impl[d94912f1f7b2d54e]::plumbing::QueryCtxt, rustc_query_system[be8adce5b7c83ed2]::query::caches::DefaultCache<(), core[867cfca19013d5a]::result::Result<(), rustc_errors[f00c99bb6f3a048b]::ErrorGuaranteed>>>
  28:     0x7f244ad6f200 - rustc_query_system[be8adce5b7c83ed2]::query::plumbing::get_query::<rustc_query_impl[d94912f1f7b2d54e]::queries::analysis, rustc_query_impl[d94912f1f7b2d54e]::plumbing::QueryCtxt>
  29:     0x7f244a8f7aca - <rustc_query_impl[d94912f1f7b2d54e]::Queries as rustc_middle[7be20166cc633508]::ty::query::QueryEngine>::analysis
  30:     0x7f244903b788 - <rustc_interface[e7d8156fe52b0687]::passes::QueryContext>::enter::<rustc_driver[33805c906714b4ec]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[867cfca19013d5a]::result::Result<(), rustc_errors[f00c99bb6f3a048b]::ErrorGuaranteed>>
  31:     0x7f244904f77b - <rustc_interface[e7d8156fe52b0687]::interface::Compiler>::enter::<rustc_driver[33805c906714b4ec]::run_compiler::{closure#1}::{closure#2}, core[867cfca19013d5a]::result::Result<core[867cfca19013d5a]::option::Option<rustc_interface[e7d8156fe52b0687]::queries::Linker>, rustc_errors[f00c99bb6f3a048b]::ErrorGuaranteed>>
  32:     0x7f2448fe71e2 - rustc_span[8615aa9fcc15d8c5]::with_source_map::<core[867cfca19013d5a]::result::Result<(), rustc_errors[f00c99bb6f3a048b]::ErrorGuaranteed>, rustc_interface[e7d8156fe52b0687]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[f00c99bb6f3a048b]::ErrorGuaranteed>, rustc_driver[33805c906714b4ec]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  33:     0x7f2449042ccc - <scoped_tls[b64ea83672690cf8]::ScopedKey<rustc_span[8615aa9fcc15d8c5]::SessionGlobals>>::set::<rustc_interface[e7d8156fe52b0687]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[f00c99bb6f3a048b]::ErrorGuaranteed>, rustc_driver[33805c906714b4ec]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[f00c99bb6f3a048b]::ErrorGuaranteed>>
  34:     0x7f244903eaca - std[f70911762a7dc09b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[e7d8156fe52b0687]::util::run_in_thread_pool_with_globals<rustc_interface[e7d8156fe52b0687]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[f00c99bb6f3a048b]::ErrorGuaranteed>, rustc_driver[33805c906714b4ec]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[f00c99bb6f3a048b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[f00c99bb6f3a048b]::ErrorGuaranteed>>
  35:     0x7f2449048096 - std[f70911762a7dc09b]::panic::catch_unwind::<core[867cfca19013d5a]::panic::unwind_safe::AssertUnwindSafe<<std[f70911762a7dc09b]::thread::Builder>::spawn_unchecked_<rustc_interface[e7d8156fe52b0687]::util::run_in_thread_pool_with_globals<rustc_interface[e7d8156fe52b0687]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[f00c99bb6f3a048b]::ErrorGuaranteed>, rustc_driver[33805c906714b4ec]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[f00c99bb6f3a048b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[f00c99bb6f3a048b]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[867cfca19013d5a]::result::Result<(), rustc_errors[f00c99bb6f3a048b]::ErrorGuaranteed>>
  36:     0x7f2448ff4339 - <<std[f70911762a7dc09b]::thread::Builder>::spawn_unchecked_<rustc_interface[e7d8156fe52b0687]::util::run_in_thread_pool_with_globals<rustc_interface[e7d8156fe52b0687]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[f00c99bb6f3a048b]::ErrorGuaranteed>, rustc_driver[33805c906714b4ec]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[f00c99bb6f3a048b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[f00c99bb6f3a048b]::ErrorGuaranteed>>::{closure#1} as core[867cfca19013d5a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  37:     0x7f2448636d9e - std::sys::unix::thread::Thread::new::thread_start::h8e0f0b30fe6295ff
  38:     0x7f24483cbb43 - <unknown>
  39:     0x7f244845da00 - <unknown>
  40:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-nightly (45f885284 2022-11-19) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [effective_visibilities] checking effective visibilities
#1 [analysis] running analysis passes on this crate
error: could not compile `core`
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:03:49
