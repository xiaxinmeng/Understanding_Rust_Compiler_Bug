plain
 Documenting rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
[RUSTC-TIMING] chalk_solve test:false 3.700
    Checking chalk-engine v0.80.0
[RUSTC-TIMING] chalk_engine test:false 1.149
thread 'rustc' panicked at 'DefId(43:399 ~ ena[5ef4]::unify::InPlaceUnificationTable) does not have a "constness"', compiler/rustc_metadata/src/rmeta/decoder/cstore_impl.rs:190:1
stack backtrace:
   0:     0x7fd935fd549b - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h88f8d46b11879331
   1:     0x7fd93603a728 - core::fmt::write::hc5e5aea5d0717917
   2:     0x7fd935fc7171 - std::io::Write::write_fmt::h1f2688dd827a6b59
   3:     0x7fd935fd8488 - std::panicking::default_hook::{{closure}}::he8217ad69616fa6a
   4:     0x7fd935fd81e5 - std::panicking::default_hook::h8699e353c28dd00a
   5:     0x7fd936935f76 - rustc_driver[f758b7caec8134b7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fd935fd8c30 - std::panicking::rust_panic_with_hook::h99ee3864bba5a3ae
   7:     0x7fd935fd8a57 - std::panicking::begin_panic_handler::{{closure}}::h25727327c73652d4
   8:     0x7fd935fd59bc - std::sys_common::backtrace::__rust_end_short_backtrace::hbc2cae323ee33586
   9:     0x7fd935fd8722 - rust_begin_unwind
  10:     0x7fd935f8e873 - core::panicking::panic_fmt::hf1da691d4b12f166
  11:     0x7fd9391099f0 - rustc_metadata[6ed90a05e15ac46b]::rmeta::decoder::cstore_impl::provide_extern::constness::{closure#0}
  12:     0x7fd939109908 - rustc_metadata[6ed90a05e15ac46b]::rmeta::decoder::cstore_impl::provide_extern::constness
  13:     0x7fd938a4b40e - rustc_query_system[90dcc62980754d1e]::query::plumbing::try_execute_query::<rustc_query_impl[34ec5fda12fbff4e]::plumbing::QueryCtxt, rustc_query_system[90dcc62980754d1e]::query::caches::DefaultCache<rustc_span[2f200fd7f919d5d]::def_id::DefId, rustc_hir[6d66a3b5f51d73ce]::hir::Constness>>
  14:     0x7fd938b68489 - rustc_query_system[90dcc62980754d1e]::query::plumbing::get_query::<rustc_query_impl[34ec5fda12fbff4e]::queries::constness, rustc_query_impl[34ec5fda12fbff4e]::plumbing::QueryCtxt>
  15:     0x7fd93861bfce - <rustc_query_impl[34ec5fda12fbff4e]::Queries as rustc_middle[9e1bcfe7e5a6848]::ty::query::QueryEngine>::constness
  16:     0x7fd936ee2e99 - rustc_ty_utils[b0d1ae63d81584af]::ty::param_env
  17:     0x7fd938a4e9ce - rustc_query_system[90dcc62980754d1e]::query::plumbing::try_execute_query::<rustc_query_impl[34ec5fda12fbff4e]::plumbing::QueryCtxt, rustc_query_system[90dcc62980754d1e]::query::caches::DefaultCache<rustc_span[2f200fd7f919d5d]::def_id::DefId, rustc_middle[9e1bcfe7e5a6848]::ty::ParamEnv>>
  18:     0x7fd938b68e43 - rustc_query_system[90dcc62980754d1e]::query::plumbing::get_query::<rustc_query_impl[34ec5fda12fbff4e]::queries::param_env, rustc_query_impl[34ec5fda12fbff4e]::plumbing::QueryCtxt>
  19:     0x7fd93864345e - <rustc_query_impl[34ec5fda12fbff4e]::Queries as rustc_middle[9e1bcfe7e5a6848]::ty::query::QueryEngine>::param_env
  20:     0x55d3430aa440 - rustdoc[be159fae8510b392]::html::render::print_item::document_type_layout
  21:     0x55d34309b5d4 - rustdoc[be159fae8510b392]::html::render::print_item::print_item
  22:     0x55d3430fa1dd - <rustdoc[be159fae8510b392]::html::render::context::Context>::render_item
  23:     0x55d343102989 - <rustdoc[be159fae8510b392]::html::render::context::Context as rustdoc[be159fae8510b392]::formats::renderer::FormatRenderer>::item
  24:     0x55d3430f7b01 - rustdoc[be159fae8510b392]::formats::renderer::run_format::<rustdoc[be159fae8510b392]::html::render::context::Context>
  25:     0x55d342f1e815 - rustdoc[be159fae8510b392]::run_renderer::<rustdoc[be159fae8510b392]::html::render::context::Context>
  26:     0x55d343004b08 - <rustc_session[178c579968b582e7]::session::Session>::time::<core[67963b6b87f865d9]::result::Result<(), rustc_errors[4344af38e2bbf759]::ErrorGuaranteed>, rustdoc[be159fae8510b392]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#1}>
  27:     0x55d34305f2e6 - <rustc_interface[823d12167b9838a1]::passes::QueryContext>::enter::<rustdoc[be159fae8510b392]::main_options::{closure#0}::{closure#0}::{closure#1}, core[67963b6b87f865d9]::result::Result<(), rustc_errors[4344af38e2bbf759]::ErrorGuaranteed>>
  28:     0x55d342f22237 - <rustc_interface[823d12167b9838a1]::interface::Compiler>::enter::<rustdoc[be159fae8510b392]::main_options::{closure#0}::{closure#0}, core[67963b6b87f865d9]::result::Result<(), rustc_errors[4344af38e2bbf759]::ErrorGuaranteed>>
  29:     0x55d343231a31 - rustc_span[2f200fd7f919d5d]::with_source_map::<core[67963b6b87f865d9]::result::Result<(), rustc_errors[4344af38e2bbf759]::ErrorGuaranteed>, rustc_interface[823d12167b9838a1]::interface::create_compiler_and_run<core[67963b6b87f865d9]::result::Result<(), rustc_errors[4344af38e2bbf759]::ErrorGuaranteed>, rustdoc[be159fae8510b392]::main_options::{closure#0}>::{closure#1}>
  30:     0x55d342f77517 - rustc_interface[823d12167b9838a1]::interface::create_compiler_and_run::<core[67963b6b87f865d9]::result::Result<(), rustc_errors[4344af38e2bbf759]::ErrorGuaranteed>, rustdoc[be159fae8510b392]::main_options::{closure#0}>
  31:     0x55d342f1faf2 - rustdoc[be159fae8510b392]::main_options
  32:     0x55d342f23a4b - <scoped_tls[b2e0916a3a421c16]::ScopedKey<rustc_span[2f200fd7f919d5d]::SessionGlobals>>::set::<rustdoc[be159fae8510b392]::main_args::{closure#0}, core[67963b6b87f865d9]::result::Result<(), rustc_errors[4344af38e2bbf759]::ErrorGuaranteed>>
  33:     0x55d3430cc070 - std[9ddd439261b3c3ce]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[823d12167b9838a1]::util::run_in_thread_pool_with_globals<rustdoc[be159fae8510b392]::main_args::{closure#0}, core[67963b6b87f865d9]::result::Result<(), rustc_errors[4344af38e2bbf759]::ErrorGuaranteed>>::{closure#0}, core[67963b6b87f865d9]::result::Result<(), rustc_errors[4344af38e2bbf759]::ErrorGuaranteed>>
  34:     0x55d342f7a109 - <<std[9ddd439261b3c3ce]::thread::Builder>::spawn_unchecked_<rustc_interface[823d12167b9838a1]::util::run_in_thread_pool_with_globals<rustdoc[be159fae8510b392]::main_args::{closure#0}, core[67963b6b87f865d9]::result::Result<(), rustc_errors[4344af38e2bbf759]::ErrorGuaranteed>>::{closure#0}, core[67963b6b87f865d9]::result::Result<(), rustc_errors[4344af38e2bbf759]::ErrorGuaranteed>>::{closure#1} as core[67963b6b87f865d9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  35:     0x7fd935fe4ab5 - std::sys::unix::thread::Thread::new::thread_start::h4419d4e49fef9f21
  36:     0x7fd935ee3609 - start_thread
  37:     0x7fd935cb7133 - clone
  38:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0 (13aca0038 2022-10-11) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -Z unstable-options -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z unstable-options -Z normalize-docs -Z force-unstable-if-unmarked -Z unstable-options
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [constness] checking if item is const: `ena::unify::InPlaceUnificationTable`
#1 [param_env] computing normalized predicates of `ena::unify::InPlaceUnificationTable`
[RUSTC-TIMING] rustc_data_structures test:false 1.876
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
 Documenting rustc_span v0.0.0 (/checkout/compiler/rustc_span)
 Documenting rustc_span v0.0.0 (/checkout/compiler/rustc_span)
 Documenting rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
error: could not document `rustc_data_structures`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_data_structures compiler/rustc_data_structures/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature, "rayon", "rayon-core", "rustc_use_parallel_compiler")' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=bc31fed8714bfeb5 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern arrayvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libarrayvec-5be777cbbfe87ffd.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-f62606bf9137bc51.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libcfg_if-032c67636302d603.rmeta --extern ena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libena-219956d74ca08406.rmeta --extern indexmap=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libindexmap-2019a3c547f9f98e.rmeta --extern jobserver_crate=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-01c157212a7caac1.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-a9314e22742b0186.rmeta --extern measureme=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libmeasureme-4a85727ea7163e3f.rmeta --extern memmap2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libmemmap2-699f751da0786ece.rmeta --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-44ad259624e2337b.rmeta --extern rustc_hash=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hash-1b0b464a21ad4d5e.rmeta --extern rustc_graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_graphviz-6af4e6a05c14ab69.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-832d41334b3ce965.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-8ceec63f1948ac4f.so --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-4ee7b6b0d2580c56.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-4526497b6a1d6db4.rmeta --extern stable_deref_trait=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libstable_deref_trait-90df04b941e143c9.rmeta --extern stacker=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libstacker-f4b8e256976c8edb.rmeta --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-717eb93730b0756a.rmeta --extern thin_vec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libthin_vec-953f157ddfa62bc1.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-9204d540dbdaf4dc.rmeta --extern-html-root-url 'arrayvec=https://docs.rs/arrayvec/0.7.0/' --extern-html-root-url 'bitflags=https://docs.rs/bitflags/1.3.2/' --extern-html-root-url 'cfg_if=https://docs.rs/cfg-if/0.1.10/' --extern-html-root-url 'ena=https://docs.rs/ena/0.14.0/' --extern-html-root-url 'indexmap=https://docs.rs/indexmap/1.9.1/' --extern-html-root-url 'jobserver=https://docs.rs/jobserver/0.1.24/' --extern-html-root-url 'libc=https://docs.rs/libc/0.2.131/' --extern-html-root-url 'measureme=https://docs.rs/measureme/10.1.0/' --extern-html-root-url 'memmap2=https://docs.rs/memmap2/0.2.1/' --extern-html-root-url 'parking_lot=https://docs.rs/parking_lot/0.11.2/' --extern-html-root-url 'rustc_hash=https://docs.rs/rustc-hash/1.1.0/' --extern-html-root-url 'smallvec=https://docs.rs/smallvec/1.8.1/' --extern-html-root-url 'stable_deref_trait=https://docs.rs/stable_deref_trait/1.2.0/' --extern-html-root-url 'stacker=https://docs.rs/stacker/0.1.14/' --extern-html-root-url 'tempfile=https://docs.rs/tempfile/3.2.0/' --extern-html-root-url 'thin_vec=https://docs.rs/thin-vec/0.2.8/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.35/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.66.0
  (13aca0038
  2022-10-11)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 101)
[RUSTC-TIMING] rustc_type_ir test:false 0.597
[RUSTC-TIMING] serde test:false 5.001
[RUSTC-TIMING] rustc_span test:false 2.076
Build completed unsuccessfully in 0:19:54
