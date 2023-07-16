plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between a7f375789bab1a4e4a291c963081a8ca7d2b6bd7 and 7b925daae4a5d52d4a47616038cea3b5619265c4
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling askama_shared v0.12.0
   Compiling askama_derive v0.11.0
   Compiling askama v0.11.0
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_middle/src/hir/map/mod.rs:215:52

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (7b925daae 2022-01-26) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z binary-dep-depinfo -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=v0 -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_middle/src/hir/map/mod.rs:215:52
   0:     0x7f4803f5895c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5734ea179c7e42a0
   1:     0x7f4803fbff9e - core::fmt::write::h4852797239f1f752
   1:     0x7f4803fbff9e - core::fmt::write::h4852797239f1f752
   2:     0x7f4803f47525 - std::io::Write::write_fmt::hcdf65abe620eac1b
   3:     0x7f4803f5c1fc - std::panicking::default_hook::{{closure}}::he079a526240560d5
   4:     0x7f4803f5bc63 - std::panicking::default_hook::hef7a23379fec980f
   5:     0x7f48053c9a9a - rustc_driver[5120c2b6e417fd9c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f4803f5cb23 - std::panicking::rust_panic_with_hook::h4837515df51b84f7
   7:     0x7f4803f5c549 - std::panicking::begin_panic_handler::{{closure}}::he15a17969a5bfb42
   8:     0x7f4803f58e54 - std::sys_common::backtrace::__rust_end_short_backtrace::h0d94dd52daa54791
   9:     0x7f4803f5c4c9 - rust_begin_unwind
  10:     0x7f4803f0a8a3 - core::panicking::panic_fmt::hde8e94acffdc309d
  11:     0x7f4803f0a7ed - core::panicking::panic::h1f13069bcfb625ee
  12:     0x7f480b451072 - <rustc_middle[fc0e383fcfae3ac4]::hir::map::Map>::span_if_local
  13:     0x7f480b34a1ce - <rustc_middle[fc0e383fcfae3ac4]::hir::provide::{closure#7} as core[aaad0e172f51323f]::ops::function::FnOnce<(rustc_middle[fc0e383fcfae3ac4]::ty::context::TyCtxt, rustc_span[8a4ccc0ec4fe0dc6]::def_id::DefId)>>::call_once
  14:     0x7f4809dabbf2 - rustc_query_system[7ec4a006836b71ad]::query::plumbing::try_execute_query::<rustc_query_impl[bd1549ea260ec54]::plumbing::QueryCtxt, rustc_query_system[7ec4a006836b71ad]::query::caches::DefaultCache<rustc_span[8a4ccc0ec4fe0dc6]::def_id::DefId, rustc_span[8a4ccc0ec4fe0dc6]::span_encoding::Span>>
  15:     0x7f4809ebeaad - rustc_query_system[7ec4a006836b71ad]::query::plumbing::get_query::<rustc_query_impl[bd1549ea260ec54]::queries::def_span, rustc_query_impl[bd1549ea260ec54]::plumbing::QueryCtxt>
  16:     0x7f4809f4bf45 - <rustc_span[8a4ccc0ec4fe0dc6]::def_id::DefId as rustc_query_impl[bd1549ea260ec54]::keys::Key>::default_span
  17:     0x7f4809f67d2b - rustc_query_impl[bd1549ea260ec54]::make_query::opt_def_kind
  18:     0x7f4809d38454 - <rustc_query_system[7ec4a006836b71ad]::query::plumbing::QueryState<rustc_middle[fc0e383fcfae3ac4]::dep_graph::dep_node::DepKind, rustc_span[8a4ccc0ec4fe0dc6]::def_id::DefId>>::try_collect_active_jobs::<rustc_query_impl[bd1549ea260ec54]::plumbing::QueryCtxt>
  19:     0x7f4809fb4937 - <rustc_query_impl[bd1549ea260ec54]::Queries>::try_collect_active_jobs
  20:     0x7f4809f4b6a4 - <rustc_query_impl[bd1549ea260ec54]::plumbing::QueryCtxt>::try_print_query_stack
  21:     0x7f480559deb1 - rustc_interface[a838c00c7590dfbc]::interface::try_print_query_stack
  22:     0x7f48053ca577 - rustc_driver[5120c2b6e417fd9c]::report_ice
  23:     0x7f4803f5cb23 - std::panicking::rust_panic_with_hook::h4837515df51b84f7
  24:     0x7f4803f5c549 - std::panicking::begin_panic_handler::{{closure}}::he15a17969a5bfb42
  25:     0x7f4803f58e54 - std::sys_common::backtrace::__rust_end_short_backtrace::h0d94dd52daa54791
  26:     0x7f4803f5c4c9 - rust_begin_unwind
  27:     0x7f4803f0a8a3 - core::panicking::panic_fmt::hde8e94acffdc309d
  28:     0x7f4803f0a7ed - core::panicking::panic::h1f13069bcfb625ee
  29:     0x7f480b44b100 - <rustc_middle[fc0e383fcfae3ac4]::hir::map::Map>::opt_def_kind
  30:     0x7f480b34a698 - <rustc_middle[fc0e383fcfae3ac4]::hir::provide::{closure#9} as core[aaad0e172f51323f]::ops::function::FnOnce<(rustc_middle[fc0e383fcfae3ac4]::ty::context::TyCtxt, rustc_span[8a4ccc0ec4fe0dc6]::def_id::DefId)>>::call_once
  31:     0x7f4809d9da35 - rustc_query_system[7ec4a006836b71ad]::query::plumbing::try_execute_query::<rustc_query_impl[bd1549ea260ec54]::plumbing::QueryCtxt, rustc_query_system[7ec4a006836b71ad]::query::caches::DefaultCache<rustc_span[8a4ccc0ec4fe0dc6]::def_id::DefId, core[aaad0e172f51323f]::option::Option<rustc_hir[63ef48af24379d5d]::def::DefKind>>>
  32:     0x7f4809e58f6b - rustc_query_system[7ec4a006836b71ad]::query::plumbing::get_query::<rustc_query_impl[bd1549ea260ec54]::queries::opt_def_kind, rustc_query_impl[bd1549ea260ec54]::plumbing::QueryCtxt>
  33:     0x7f480a6dab5e - <rustc_metadata[c05445375d97d38c]::rmeta::encoder::EncodeContext>::encode_crate_root
  34:     0x7f480a6f6e74 - rustc_metadata[c05445375d97d38c]::rmeta::encoder::encode_metadata_impl
  35:     0x7f480a7b3c01 - rustc_data_structures[e955639fcb14ac5b]::sync::join::<rustc_metadata[c05445375d97d38c]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[c05445375d97d38c]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[c05445375d97d38c]::rmeta::encoder::EncodedMetadata, ()>
  36:     0x7f480a6f6572 - rustc_metadata[c05445375d97d38c]::rmeta::encoder::encode_metadata
  37:     0x7f4805676c51 - <rustc_interface[a838c00c7590dfbc]::passes::QueryContext>::enter::<<rustc_interface[a838c00c7590dfbc]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[aaad0e172f51323f]::result::Result<alloc[ca740e15bbb10628]::boxed::Box<dyn core[aaad0e172f51323f]::any::Any>, rustc_errors[94d1d1c5f09d9309]::ErrorReported>>
  38:     0x7f48055c6a98 - <rustc_interface[a838c00c7590dfbc]::queries::Queries>::ongoing_codegen
  39:     0x7f480542c8d5 - <rustc_interface[a838c00c7590dfbc]::interface::Compiler>::enter::<rustc_driver[5120c2b6e417fd9c]::run_compiler::{closure#1}::{closure#2}, core[aaad0e172f51323f]::result::Result<core[aaad0e172f51323f]::option::Option<rustc_interface[a838c00c7590dfbc]::queries::Linker>, rustc_errors[94d1d1c5f09d9309]::ErrorReported>>
  40:     0x7f480538bc23 - rustc_span[8a4ccc0ec4fe0dc6]::with_source_map::<core[aaad0e172f51323f]::result::Result<(), rustc_errors[94d1d1c5f09d9309]::ErrorReported>, rustc_interface[a838c00c7590dfbc]::interface::create_compiler_and_run<core[aaad0e172f51323f]::result::Result<(), rustc_errors[94d1d1c5f09d9309]::ErrorReported>, rustc_driver[5120c2b6e417fd9c]::run_compiler::{closure#1}>::{closure#1}>
  41:     0x7f48054627a4 - rustc_interface[a838c00c7590dfbc]::interface::create_compiler_and_run::<core[aaad0e172f51323f]::result::Result<(), rustc_errors[94d1d1c5f09d9309]::ErrorReported>, rustc_driver[5120c2b6e417fd9c]::run_compiler::{closure#1}>
  42:     0x7f48053dbb31 - std[bf6e9d6bddd82d9e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a838c00c7590dfbc]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[a838c00c7590dfbc]::interface::run_compiler<core[aaad0e172f51323f]::result::Result<(), rustc_errors[94d1d1c5f09d9309]::ErrorReported>, rustc_driver[5120c2b6e417fd9c]::run_compiler::{closure#1}>::{closure#0}, core[aaad0e172f51323f]::result::Result<(), rustc_errors[94d1d1c5f09d9309]::ErrorReported>>::{closure#0}, core[aaad0e172f51323f]::result::Result<(), rustc_errors[94d1d1c5f09d9309]::ErrorReported>>
  43:     0x7f48053de0c0 - <<std[bf6e9d6bddd82d9e]::thread::Builder>::spawn_unchecked_<rustc_interface[a838c00c7590dfbc]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[a838c00c7590dfbc]::interface::run_compiler<core[aaad0e172f51323f]::result::Result<(), rustc_errors[94d1d1c5f09d9309]::ErrorReported>, rustc_driver[5120c2b6e417fd9c]::run_compiler::{closure#1}>::{closure#0}, core[aaad0e172f51323f]::result::Result<(), rustc_errors[94d1d1c5f09d9309]::ErrorReported>>::{closure#0}, core[aaad0e172f51323f]::result::Result<(), rustc_errors[94d1d1c5f09d9309]::ErrorReported>>::{closure#1} as core[aaad0e172f51323f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  44:     0x7f4803f683a3 - std::sys::unix::thread::Thread::new::thread_start::h4d5ddf72d3b03844
  45:     0x7f48036bf6ba - start_thread
  46:     0x7f4803be051d - clone
  47:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (7b925daae 2022-01-26) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z binary-dep-depinfo -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=v0 -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
rustc exited with signal: 6 (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustdoc --edition=2021 src/librustdoc/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C metadata=5aa6e3de524c1a57 -C extra-filename=-5aa6e3de524c1a57 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/release/deps --extern arrayvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libarrayvec-44411c80c7d94965.rmeta --extern askama=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libaskama-7832ce18e45e4a2c.rmeta --extern atty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libatty-b961cf91cf8c3373.rmeta --extern itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-3c648cc709f6363d.rmeta --extern minifier=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libminifier-a1ba5a6fe70fbf70.rmeta --extern pulldown_cmark=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libpulldown_cmark-64c0c197a792935f.rmeta --extern rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/librayon-84ea2b4fa7f3f327.rmeta --extern regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libregex-17dd8d9c8d912c36.rmeta --extern rustdoc_json_types=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/librustdoc_json_types-81804c997602d8e8.rmeta --extern serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libserde-ea7a39908eadfe06.rmeta --extern serde_json=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libserde_json-f8a327c501bb63c4.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libsmallvec-bdca991e9c873f18.rmeta --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtempfile-182111a9bee03e11.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtracing-54fc098d6642491e.rmeta --extern tracing_subscriber=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_subscriber-7d9e08855213d3e2.rmeta --extern tracing_tree=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/libtracing_tree-d2b2df284b5cf88b.rmeta -Csymbol-mangling-version=v0 -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Ztls-model=initial-exec -Z binary-dep-depinfo` (exit status: 254)

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml"
expected success, got: exit status: 101

