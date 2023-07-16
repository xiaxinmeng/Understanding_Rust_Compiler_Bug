plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:e58f560dd5e72b61cf9aeebf432d862b23ac76a8)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.87
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Some(usize)`,
 right: `None`', compiler/rustc_mir_transform/src/upvar_to_local_prop.rs:363:9
   0:     0x7efd85400256 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h53e61b71c84f668e
   0:     0x7efd85400256 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h53e61b71c84f668e
   1:     0x7efd8546f778 - core::fmt::write::h370ceefc764332d1
   2:     0x7efd853f3d41 - std::io::Write::write_fmt::h29ae07842e655986
   3:     0x7efd85400025 - std::sys_common::backtrace::print::h60a6ac165bdd7b12
   4:     0x7efd85403467 - std::panicking::default_hook::{{closure}}::h5edace8ef4ec4fe3
   5:     0x7efd85403152 - std::panicking::default_hook::h0f63acebb79ef1ba
   6:     0x7efd85e540b5 - rustc_driver_impl[5d5e4192b7f53dbc]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7efd85403d60 - std::panicking::rust_panic_with_hook::h190cc8de795c3aef
   8:     0x7efd85403aa9 - std::panicking::begin_panic_handler::{{closure}}::hd42ae2c63e2914e5
   9:     0x7efd854007cc - std::sys_common::backtrace::__rust_end_short_backtrace::h24f6a86a9a516860
  11:     0x7efd853b9ff3 - core::panicking::panic_fmt::ha6d05d75fb2b7d9f
  11:     0x7efd853b9ff3 - core::panicking::panic_fmt::ha6d05d75fb2b7d9f
  12:     0x7efd853ba37f - core::panicking::assert_failed_inner::hc2e6cee167734903
  13:     0x7efd85c82f1b - core[78b8f9b0dd37335f]::panicking::assert_failed::<core[78b8f9b0dd37335f]::option::Option<rustc_middle[5382cdda0f518d20]::ty::Ty>, core[78b8f9b0dd37335f]::option::Option<rustc_middle[5382cdda0f518d20]::ty::Ty>>
  14:     0x7efd868b4f09 - <rustc_mir_transform[a15a9b3dcb3fc6e0]::upvar_to_local_prop::Walk as rustc_middle[5382cdda0f518d20]::mir::visit::Visitor>::visit_assign
  15:     0x7efd868b1b5b - <rustc_mir_transform[a15a9b3dcb3fc6e0]::upvar_to_local_prop::UpvarToLocalProp as rustc_middle[5382cdda0f518d20]::mir::MirPass>::run_pass
  16:     0x7efd86a41f78 - rustc_mir_transform[a15a9b3dcb3fc6e0]::pass_manager::run_passes_inner
  17:     0x7efd86900a5b - rustc_mir_transform[a15a9b3dcb3fc6e0]::run_analysis_to_runtime_passes
  18:     0x7efd86900226 - rustc_mir_transform[a15a9b3dcb3fc6e0]::mir_drops_elaborated_and_const_checked
  19:     0x7efd87eb4d80 - rustc_query_system[4861802a6be54dc9]::query::plumbing::try_execute_query::<rustc_query_impl[c6dad79d0c8f4c5c]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[c6dad79d0c8f4c5c]::plumbing::QueryCtxt>
  20:     0x7efd87a2e153 - <rustc_query_impl[c6dad79d0c8f4c5c]::Queries as rustc_middle[5382cdda0f518d20]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  21:     0x7efd86023312 - <rustc_session[9990726127e59c34]::session::Session>::time::<(), rustc_interface[e3be3f3d83090b5c]::passes::analysis::{closure#3}>
  22:     0x7efd85f63f07 - rustc_interface[e3be3f3d83090b5c]::passes::analysis
  23:     0x7efd87ecc17c - rustc_query_system[4861802a6be54dc9]::query::plumbing::try_execute_query::<rustc_query_impl[c6dad79d0c8f4c5c]::queries::analysis, rustc_query_impl[c6dad79d0c8f4c5c]::plumbing::QueryCtxt>
  24:     0x7efd87a211f7 - <rustc_query_impl[c6dad79d0c8f4c5c]::Queries as rustc_middle[5382cdda0f518d20]::ty::query::QueryEngine>::analysis
  25:     0x7efd85ea781c - <rustc_middle[5382cdda0f518d20]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[5d5e4192b7f53dbc]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[78b8f9b0dd37335f]::result::Result<(), rustc_span[225e5e04edcb4a36]::ErrorGuaranteed>>
  26:     0x7efd85ea6168 - rustc_span[225e5e04edcb4a36]::with_source_map::<core[78b8f9b0dd37335f]::result::Result<(), rustc_span[225e5e04edcb4a36]::ErrorGuaranteed>, rustc_interface[e3be3f3d83090b5c]::interface::run_compiler<core[78b8f9b0dd37335f]::result::Result<(), rustc_span[225e5e04edcb4a36]::ErrorGuaranteed>, rustc_driver_impl[5d5e4192b7f53dbc]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  27:     0x7efd85e92948 - <scoped_tls[98e817fa317f4d9a]::ScopedKey<rustc_span[225e5e04edcb4a36]::SessionGlobals>>::set::<rustc_interface[e3be3f3d83090b5c]::interface::run_compiler<core[78b8f9b0dd37335f]::result::Result<(), rustc_span[225e5e04edcb4a36]::ErrorGuaranteed>, rustc_driver_impl[5d5e4192b7f53dbc]::run_compiler::{closure#1}>::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_span[225e5e04edcb4a36]::ErrorGuaranteed>>
  28:     0x7efd85e61f70 - std[f283601f176d5ebe]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[e3be3f3d83090b5c]::util::run_in_thread_pool_with_globals<rustc_interface[e3be3f3d83090b5c]::interface::run_compiler<core[78b8f9b0dd37335f]::result::Result<(), rustc_span[225e5e04edcb4a36]::ErrorGuaranteed>, rustc_driver_impl[5d5e4192b7f53dbc]::run_compiler::{closure#1}>::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_span[225e5e04edcb4a36]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_span[225e5e04edcb4a36]::ErrorGuaranteed>>
  29:     0x7efd85e66cf3 - <<std[f283601f176d5ebe]::thread::Builder>::spawn_unchecked_<rustc_interface[e3be3f3d83090b5c]::util::run_in_thread_pool_with_globals<rustc_interface[e3be3f3d83090b5c]::interface::run_compiler<core[78b8f9b0dd37335f]::result::Result<(), rustc_span[225e5e04edcb4a36]::ErrorGuaranteed>, rustc_driver_impl[5d5e4192b7f53dbc]::run_compiler::{closure#1}>::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_span[225e5e04edcb4a36]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_span[225e5e04edcb4a36]::ErrorGuaranteed>>::{closure#1} as core[78b8f9b0dd37335f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  30:     0x7efd85410c2e - std::sys::unix::thread::Thread::new::thread_start::h6a9b2c451f20553b
  31:     0x7efd851abb43 - <unknown>
  32:     0x7efd8523da00 - <unknown>
  33:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.69.0-nightly (7d6cd1b87 2023-03-01) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [mir_drops_elaborated_and_const_checked] elaborating drops for `slice::index::<impl at library/core/src/slice/index.rs:349:1: 349:59>::get_unchecked`
#1 [analysis] running analysis passes on this crate
error: could not compile `core`
Build completed unsuccessfully in 0:04:33
cat: /tmp/toolstate/toolstates.json: No such file or directory
