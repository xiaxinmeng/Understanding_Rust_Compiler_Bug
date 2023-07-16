plain
   Compiling askama_shared v0.12.0
   Compiling askama_derive v0.11.0
   Compiling askama v0.11.0
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_middle/src/hir/map/mod.rs:218:52

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (22706eeee 2022-01-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z binary-dep-depinfo -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=v0 -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C llvm-args=-import-instr-limit=10 --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_middle/src/hir/map/mod.rs:218:52
stack backtrace:
   0:     0x7f6e512d9b0c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7df7094cf3c284c8
   1:     0x7f6e5134125e - core::fmt::write::h4852797239f1f752
   2:     0x7f6e512c86e5 - std::io::Write::write_fmt::hf102962402bd6e9f
   3:     0x7f6e512dd3ac - std::panicking::default_hook::{{closure}}::h1586c00dab3f8d4d
   4:     0x7f6e512dce13 - std::panicking::default_hook::h49dc21191a430d8c
   5:     0x7f6e51ddf121 - rustc_driver[84c39a3f67574bbc]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f6e512ddcd3 - std::panicking::rust_panic_with_hook::h1d765cb1a2ca49ab
   7:     0x7f6e512dd6f9 - std::panicking::begin_panic_handler::{{closure}}::h3b541f40270d8a8a
   8:     0x7f6e512da004 - std::sys_common::backtrace::__rust_end_short_backtrace::h15f8250af7dc5123
   9:     0x7f6e512dd679 - rust_begin_unwind
  10:     0x7f6e5128bc53 - core::panicking::panic_fmt::hde8e94acffdc309d
  11:     0x7f6e5128bb9d - core::panicking::panic::h1f13069bcfb625ee
  12:     0x7f6e545fbb54 - <rustc_middle[5423e03d603da3d7]::hir::map::Map>::span_if_local
  13:     0x7f6e544eebde - <rustc_middle[5423e03d603da3d7]::hir::provide::{closure#7} as core[aaad0e172f51323f]::ops::function::FnOnce<(rustc_middle[5423e03d603da3d7]::ty::context::TyCtxt, rustc_span[2d67ee9d7068c609]::def_id::DefId)>>::call_once
  14:     0x7f6e533854a1 - rustc_query_system[30a68183c7890c97]::query::plumbing::try_execute_query::<rustc_query_impl[2677635d79aa9137]::plumbing::QueryCtxt, rustc_query_system[30a68183c7890c97]::query::caches::DefaultCache<rustc_span[2d67ee9d7068c609]::def_id::DefId, rustc_span[2d67ee9d7068c609]::span_encoding::Span>>
  15:     0x7f6e5348d8cd - rustc_query_system[30a68183c7890c97]::query::plumbing::get_query::<rustc_query_impl[2677635d79aa9137]::queries::def_span, rustc_query_impl[2677635d79aa9137]::plumbing::QueryCtxt>
  16:     0x7f6e535316c6 - <rustc_span[2d67ee9d7068c609]::def_id::DefId as rustc_query_impl[2677635d79aa9137]::keys::Key>::default_span
  17:     0x7f6e53544bfd - rustc_query_impl[2677635d79aa9137]::make_query::opt_def_kind
  18:     0x7f6e5331fbef - <rustc_query_system[30a68183c7890c97]::query::plumbing::QueryState<rustc_middle[5423e03d603da3d7]::dep_graph::dep_node::DepKind, rustc_span[2d67ee9d7068c609]::def_id::DefId>>::try_collect_active_jobs::<rustc_query_impl[2677635d79aa9137]::plumbing::QueryCtxt>
  19:     0x7f6e53581197 - <rustc_query_impl[2677635d79aa9137]::Queries>::try_collect_active_jobs
  20:     0x7f6e53530f84 - <rustc_query_impl[2677635d79aa9137]::plumbing::QueryCtxt>::try_print_query_stack
  21:     0x7f6e51f3f7d4 - rustc_interface[d5650ecda41a4c1]::interface::try_print_query_stack
  22:     0x7f6e51ddfa5c - rustc_driver[84c39a3f67574bbc]::report_ice
  23:     0x7f6e512ddcd3 - std::panicking::rust_panic_with_hook::h1d765cb1a2ca49ab
  24:     0x7f6e512dd6f9 - std::panicking::begin_panic_handler::{{closure}}::h3b541f40270d8a8a
  25:     0x7f6e512da004 - std::sys_common::backtrace::__rust_end_short_backtrace::h15f8250af7dc5123
  26:     0x7f6e512dd679 - rust_begin_unwind
  27:     0x7f6e5128bc53 - core::panicking::panic_fmt::hde8e94acffdc309d
  28:     0x7f6e5128bb9d - core::panicking::panic::h1f13069bcfb625ee
  29:     0x7f6e545f60d7 - <rustc_middle[5423e03d603da3d7]::hir::map::Map>::opt_def_kind
  30:     0x7f6e544eeed8 - <rustc_middle[5423e03d603da3d7]::hir::provide::{closure#9} as core[aaad0e172f51323f]::ops::function::FnOnce<(rustc_middle[5423e03d603da3d7]::ty::context::TyCtxt, rustc_span[2d67ee9d7068c609]::def_id::DefId)>>::call_once
  31:     0x7f6e53379cb0 - rustc_query_system[30a68183c7890c97]::query::plumbing::try_execute_query::<rustc_query_impl[2677635d79aa9137]::plumbing::QueryCtxt, rustc_query_system[30a68183c7890c97]::query::caches::DefaultCache<rustc_span[2d67ee9d7068c609]::def_id::DefId, core[aaad0e172f51323f]::option::Option<rustc_hir[aa2c9ea7716e9edf]::def::DefKind>>>
  32:     0x7f6e5342cdab - rustc_query_system[30a68183c7890c97]::query::plumbing::get_query::<rustc_query_impl[2677635d79aa9137]::queries::opt_def_kind, rustc_query_impl[2677635d79aa9137]::plumbing::QueryCtxt>
  33:     0x7f6e53b73454 - <rustc_metadata[438247d42201b8b6]::rmeta::encoder::EncodeContext>::encode_crate_root
  34:     0x7f6e53b8716a - rustc_metadata[438247d42201b8b6]::rmeta::encoder::encode_metadata_impl
  35:     0x7f6e53c105a1 - rustc_data_structures[59c8e2fd51341b32]::sync::join::<rustc_metadata[438247d42201b8b6]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[438247d42201b8b6]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[438247d42201b8b6]::rmeta::encoder::EncodedMetadata, ()>
  36:     0x7f6e53b86a3e - rustc_metadata[438247d42201b8b6]::rmeta::encoder::encode_metadata
  37:     0x7f6e51ffcf0c - <rustc_interface[d5650ecda41a4c1]::passes::QueryContext>::enter::<<rustc_interface[d5650ecda41a4c1]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[aaad0e172f51323f]::result::Result<alloc[ca740e15bbb10628]::boxed::Box<dyn core[aaad0e172f51323f]::any::Any>, rustc_errors[48d649485b17f5a5]::ErrorReported>>
  38:     0x7f6e51f82cf8 - <rustc_interface[d5650ecda41a4c1]::queries::Queries>::ongoing_codegen
  39:     0x7f6e51e29c47 - <rustc_interface[d5650ecda41a4c1]::interface::Compiler>::enter::<rustc_driver[84c39a3f67574bbc]::run_compiler::{closure#1}::{closure#2}, core[aaad0e172f51323f]::result::Result<core[aaad0e172f51323f]::option::Option<rustc_interface[d5650ecda41a4c1]::queries::Linker>, rustc_errors[48d649485b17f5a5]::ErrorReported>>
  40:     0x7f6e51dba316 - rustc_span[2d67ee9d7068c609]::with_source_map::<core[aaad0e172f51323f]::result::Result<(), rustc_errors[48d649485b17f5a5]::ErrorReported>, rustc_interface[d5650ecda41a4c1]::interface::create_compiler_and_run<core[aaad0e172f51323f]::result::Result<(), rustc_errors[48d649485b17f5a5]::ErrorReported>, rustc_driver[84c39a3f67574bbc]::run_compiler::{closure#1}>::{closure#1}>
  41:     0x7f6e51e2b72f - rustc_interface[d5650ecda41a4c1]::interface::create_compiler_and_run::<core[aaad0e172f51323f]::result::Result<(), rustc_errors[48d649485b17f5a5]::ErrorReported>, rustc_driver[84c39a3f67574bbc]::run_compiler::{closure#1}>
  42:     0x7f6e51dec414 - std[eedf4c3dfa358ba6]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d5650ecda41a4c1]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[d5650ecda41a4c1]::interface::run_compiler<core[aaad0e172f51323f]::result::Result<(), rustc_errors[48d649485b17f5a5]::ErrorReported>, rustc_driver[84c39a3f67574bbc]::run_compiler::{closure#1}>::{closure#0}, core[aaad0e172f51323f]::result::Result<(), rustc_errors[48d649485b17f5a5]::ErrorReported>>::{closure#0}, core[aaad0e172f51323f]::result::Result<(), rustc_errors[48d649485b17f5a5]::ErrorReported>>
  43:     0x7f6e51dbd991 - std[eedf4c3dfa358ba6]::panic::catch_unwind::<core[aaad0e172f51323f]::panic::unwind_safe::AssertUnwindSafe<<std[eedf4c3dfa358ba6]::thread::Builder>::spawn_unchecked_<rustc_interface[d5650ecda41a4c1]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[d5650ecda41a4c1]::interface::run_compiler<core[aaad0e172f51323f]::result::Result<(), rustc_errors[48d649485b17f5a5]::ErrorReported>, rustc_driver[84c39a3f67574bbc]::run_compiler::{closure#1}>::{closure#0}, core[aaad0e172f51323f]::result::Result<(), rustc_errors[48d649485b17f5a5]::ErrorReported>>::{closure#0}, core[aaad0e172f51323f]::result::Result<(), rustc_errors[48d649485b17f5a5]::ErrorReported>>::{closure#1}::{closure#0}>, core[aaad0e172f51323f]::result::Result<(), rustc_errors[48d649485b17f5a5]::ErrorReported>>
  44:     0x7f6e51dee18a - <<std[eedf4c3dfa358ba6]::thread::Builder>::spawn_unchecked_<rustc_interface[d5650ecda41a4c1]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[d5650ecda41a4c1]::interface::run_compiler<core[aaad0e172f51323f]::result::Result<(), rustc_errors[48d649485b17f5a5]::ErrorReported>, rustc_driver[84c39a3f67574bbc]::run_compiler::{closure#1}>::{closure#0}, core[aaad0e172f51323f]::result::Result<(), rustc_errors[48d649485b17f5a5]::ErrorReported>>::{closure#0}, core[aaad0e172f51323f]::result::Result<(), rustc_errors[48d649485b17f5a5]::ErrorReported>>::{closure#1} as core[aaad0e172f51323f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7f6e512e9583 - std::sys::unix::thread::Thread::new::thread_start::h1cd3b2d1f63d6469
  46:     0x7f6e4b656609 - start_thread
  47:     0x7f6e5114d293 - clone
  48:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (22706eeee 2022-01-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z binary-dep-depinfo -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=v0 -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C llvm-args=-import-instr-limit=10 --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
rustc exited with signal: 6 (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustdoc --edition=2021 src/librustdoc/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C metadata=f5e2e3dff3aaa32b -C extra-filename=-f5e2e3dff3aaa32b --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/release/deps --extern arrayvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libarrayvec-44411c80c7d94965.rmeta --extern askama=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libaskama-7832ce18e45e4a2c.rmeta --extern atty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libatty-12ea59cd8afb2768.rmeta --extern itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-3c648cc709f6363d.rmeta --extern minifier=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libminifier-c8bd3e5d143c8c34.rmeta --extern pulldown_cmark=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libpulldown_cmark-64c0c197a792935f.rmeta --extern rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/librayon-ca1898fe2c0b8f2d.rmeta --extern regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libregex-17dd8d9c8d912c36.rmeta --extern rustdoc_json_types=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/librustdoc_json_types-81804c997602d8e8.rmeta --extern serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libserde-ea7a39908eadfe06.rmeta --extern serde_json=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libserde_json-f8a327c501bb63c4.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libsmallvec-bdca991e9c873f18.rmeta --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtempfile-4c62facaf60c74f4.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtracing-54fc098d6642491e.rmeta --extern tracing_subscriber=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_subscriber-251156ced33d2872.rmeta --extern tracing_tree=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_tree-ad31bff0b5951ec4.rmeta -Csymbol-mangling-version=v0 -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Ztls-model=initial-exec -Cllvm-args=-import-instr-limit=10 -Z binary-dep-depinfo` (exit status: 254)

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml"
expected success, got: exit status: 101

