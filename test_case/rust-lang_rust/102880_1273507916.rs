plain
 Documenting rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
[RUSTC-TIMING] chalk_solve test:false 3.406
    Checking chalk-engine v0.80.0
[RUSTC-TIMING] chalk_engine test:false 1.069
thread 'rustc' panicked at 'DefId(43:399 ~ ena[f086]::unify::InPlaceUnificationTable) does not have a "constness"', compiler/rustc_metadata/src/rmeta/decoder/cstore_impl.rs:190:1
stack backtrace:
   0:     0x7fdbcffc446b - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb49611583bf71f7b
   1:     0x7fdbd0029738 - core::fmt::write::h6ee7e26a109c33aa
   2:     0x7fdbcffb6131 - std::io::Write::write_fmt::hd4cbd9270947e242
   3:     0x7fdbcffc7458 - std::panicking::default_hook::{{closure}}::hd224148075aa8d35
   4:     0x7fdbcffc71b5 - std::panicking::default_hook::h1081cde78e85ec01
   5:     0x7fdbd09239d6 - rustc_driver[78d191d799fd2ae]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fdbcffc7c00 - std::panicking::rust_panic_with_hook::ha8958ce8ec3d153e
   7:     0x7fdbcffc7a27 - std::panicking::begin_panic_handler::{{closure}}::h671d5837f31f0468
   8:     0x7fdbcffc498c - std::sys_common::backtrace::__rust_end_short_backtrace::h3a25c4386963f845
   9:     0x7fdbcffc76f2 - rust_begin_unwind
  10:     0x7fdbcff7d873 - core::panicking::panic_fmt::h43d08ae55328314e
  11:     0x7fdbd30f9670 - rustc_metadata[31a7c854177ab081]::rmeta::decoder::cstore_impl::provide_extern::constness::{closure#0}
  12:     0x7fdbd30f9588 - rustc_metadata[31a7c854177ab081]::rmeta::decoder::cstore_impl::provide_extern::constness
  13:     0x7fdbd2a48a6e - rustc_query_system[e2806fb84f1114e]::query::plumbing::try_execute_query::<rustc_query_impl[3d45336e449c71a5]::plumbing::QueryCtxt, rustc_query_system[e2806fb84f1114e]::query::caches::DefaultCache<rustc_span[59de404deb05021d]::def_id::DefId, rustc_hir[2ae5c6e06ec995f4]::hir::Constness>>
  14:     0x7fdbd2b62529 - rustc_query_system[e2806fb84f1114e]::query::plumbing::get_query::<rustc_query_impl[3d45336e449c71a5]::queries::constness, rustc_query_impl[3d45336e449c71a5]::plumbing::QueryCtxt>
  15:     0x7fdbd26164de - <rustc_query_impl[3d45336e449c71a5]::Queries as rustc_middle[1a57ba83450cd241]::ty::query::QueryEngine>::constness
  16:     0x7fdbd0ed9919 - rustc_ty_utils[ac8a6b8ec5780aee]::ty::param_env
  17:     0x7fdbd2a454ce - rustc_query_system[e2806fb84f1114e]::query::plumbing::try_execute_query::<rustc_query_impl[3d45336e449c71a5]::plumbing::QueryCtxt, rustc_query_system[e2806fb84f1114e]::query::caches::DefaultCache<rustc_span[59de404deb05021d]::def_id::DefId, rustc_middle[1a57ba83450cd241]::ty::ParamEnv>>
  18:     0x7fdbd2b62ee3 - rustc_query_system[e2806fb84f1114e]::query::plumbing::get_query::<rustc_query_impl[3d45336e449c71a5]::queries::param_env, rustc_query_impl[3d45336e449c71a5]::plumbing::QueryCtxt>
  19:     0x7fdbd263d96e - <rustc_query_impl[3d45336e449c71a5]::Queries as rustc_middle[1a57ba83450cd241]::ty::query::QueryEngine>::param_env
  20:     0x55964428df30 - rustdoc[442fb1f417a20dd3]::html::render::print_item::document_type_layout
  21:     0x55964427eba4 - rustdoc[442fb1f417a20dd3]::html::render::print_item::print_item
  22:     0x5596442e165d - <rustdoc[442fb1f417a20dd3]::html::render::context::Context>::render_item
  23:     0x5596442e9e09 - <rustdoc[442fb1f417a20dd3]::html::render::context::Context as rustdoc[442fb1f417a20dd3]::formats::renderer::FormatRenderer>::item
  24:     0x5596442def81 - rustdoc[442fb1f417a20dd3]::formats::renderer::run_format::<rustdoc[442fb1f417a20dd3]::html::render::context::Context>
  25:     0x559644102ef5 - rustdoc[442fb1f417a20dd3]::run_renderer::<rustdoc[442fb1f417a20dd3]::html::render::context::Context>
  26:     0x5596441c12e8 - <rustc_session[2a8e65fc69fae0d0]::session::Session>::time::<core[1308c75ca0c11975]::result::Result<(), rustc_errors[65af6d72b4fc7102]::ErrorGuaranteed>, rustdoc[442fb1f417a20dd3]::main_args::{closure#1}::{closure#0}::{closure#1}::{closure#1}>
  27:     0x55964429a8c6 - <rustc_interface[5964167f59f8b63d]::passes::QueryContext>::enter::<rustdoc[442fb1f417a20dd3]::main_args::{closure#1}::{closure#0}::{closure#1}, core[1308c75ca0c11975]::result::Result<(), rustc_errors[65af6d72b4fc7102]::ErrorGuaranteed>>
  28:     0x5596441c3527 - <rustc_interface[5964167f59f8b63d]::interface::Compiler>::enter::<rustdoc[442fb1f417a20dd3]::main_args::{closure#1}::{closure#0}, core[1308c75ca0c11975]::result::Result<(), rustc_errors[65af6d72b4fc7102]::ErrorGuaranteed>>
  29:     0x55964435f7d0 - rustc_span[59de404deb05021d]::with_source_map::<core[1308c75ca0c11975]::result::Result<(), rustc_errors[65af6d72b4fc7102]::ErrorGuaranteed>, rustc_interface[5964167f59f8b63d]::interface::run_compiler<core[1308c75ca0c11975]::result::Result<(), rustc_errors[65af6d72b4fc7102]::ErrorGuaranteed>, rustdoc[442fb1f417a20dd3]::main_args::{closure#1}>::{closure#0}::{closure#1}>
  30:     0x559644109224 - <scoped_tls[a4c2d6a7b1d3149a]::ScopedKey<rustc_span[59de404deb05021d]::SessionGlobals>>::set::<rustc_interface[5964167f59f8b63d]::interface::run_compiler<core[1308c75ca0c11975]::result::Result<(), rustc_errors[65af6d72b4fc7102]::ErrorGuaranteed>, rustdoc[442fb1f417a20dd3]::main_args::{closure#1}>::{closure#0}, core[1308c75ca0c11975]::result::Result<(), rustc_errors[65af6d72b4fc7102]::ErrorGuaranteed>>
  31:     0x5596443161c0 - std[cb77ffe630b00e3c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[5964167f59f8b63d]::util::run_in_thread_pool_with_globals<rustc_interface[5964167f59f8b63d]::interface::run_compiler<core[1308c75ca0c11975]::result::Result<(), rustc_errors[65af6d72b4fc7102]::ErrorGuaranteed>, rustdoc[442fb1f417a20dd3]::main_args::{closure#1}>::{closure#0}, core[1308c75ca0c11975]::result::Result<(), rustc_errors[65af6d72b4fc7102]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1308c75ca0c11975]::result::Result<(), rustc_errors[65af6d72b4fc7102]::ErrorGuaranteed>>
  32:     0x5596442282cf - <<std[cb77ffe630b00e3c]::thread::Builder>::spawn_unchecked_<rustc_interface[5964167f59f8b63d]::util::run_in_thread_pool_with_globals<rustc_interface[5964167f59f8b63d]::interface::run_compiler<core[1308c75ca0c11975]::result::Result<(), rustc_errors[65af6d72b4fc7102]::ErrorGuaranteed>, rustdoc[442fb1f417a20dd3]::main_args::{closure#1}>::{closure#0}, core[1308c75ca0c11975]::result::Result<(), rustc_errors[65af6d72b4fc7102]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1308c75ca0c11975]::result::Result<(), rustc_errors[65af6d72b4fc7102]::ErrorGuaranteed>>::{closure#1} as core[1308c75ca0c11975]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  33:     0x7fdbcffd3a85 - std::sys::unix::thread::Thread::new::thread_start::hb9095a7c6917e861
  34:     0x7fdbcfed2609 - start_thread
  35:     0x7fdbcfca6133 - clone
  36:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (c2afc4ed1 2022-10-10) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -Z unstable-options -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z unstable-options -Z normalize-docs -Z force-unstable-if-unmarked -Z unstable-options
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [constness] checking if item is const: `ena::unify::InPlaceUnificationTable`
#1 [param_env] computing normalized predicates of `ena::unify::InPlaceUnificationTable`
error: could not document `rustc_data_structures`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name rustc_data_structures compiler/rustc_data_structures/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'values(feature, "rayon", "rayon-core", "rustc_use_parallel_compiler")' --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=d29617a99500b2a0 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern arrayvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libarrayvec-36cba89a866f411f.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-f431309c6512d3c1.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libcfg_if-81b70c7223774baf.rmeta --extern ena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libena-4c646b37d359f3a4.rmeta --extern indexmap=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libindexmap-3c5cba8a01449ac3.rmeta --extern jobserver_crate=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-c4dcd6610bea6cea.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-824c9523f3f925a3.rmeta --extern measureme=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libmeasureme-c1ba77a208105e28.rmeta --extern memmap2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libmemmap2-3dec9791c8fecb50.rmeta --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-a87d5e048ee9160c.rmeta --extern rustc_hash=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hash-f1a4f1e7e2fbbe87.rmeta --extern rustc_graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_graphviz-0154684f1f480fec.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-cbde38ee61e2f73b.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-84619777c5d61a2a.so --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-d3e125dfcd1acfa6.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-514c0d16dcc4ec4a.rmeta --extern stable_deref_trait=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libstable_deref_trait-df4e4df27fd85e53.rmeta --extern stacker=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libstacker-876a82a81a6563a1.rmeta --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-11419f8c313443e3.rmeta --extern thin_vec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libthin_vec-4789dd33035fa11d.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-6f6f97066f4e8df8.rmeta --extern-html-root-url 'arrayvec=https://docs.rs/arrayvec/0.7.0/' --extern-html-root-url 'bitflags=https://docs.rs/bitflags/1.3.2/' --extern-html-root-url 'cfg_if=https://docs.rs/cfg-if/0.1.10/' --extern-html-root-url 'ena=https://docs.rs/ena/0.14.0/' --extern-html-root-url 'indexmap=https://docs.rs/indexmap/1.9.1/' --extern-html-root-url 'jobserver=https://docs.rs/jobserver/0.1.24/' --extern-html-root-url 'libc=https://docs.rs/libc/0.2.131/' --extern-html-root-url 'measureme=https://docs.rs/measureme/10.1.0/' --extern-html-root-url 'memmap2=https://docs.rs/memmap2/0.2.1/' --extern-html-root-url 'parking_lot=https://docs.rs/parking_lot/0.11.2/' --extern-html-root-url 'rustc_hash=https://docs.rs/rustc-hash/1.1.0/' --extern-html-root-url 'smallvec=https://docs.rs/smallvec/1.8.1/' --extern-html-root-url 'stable_deref_trait=https://docs.rs/stable_deref_trait/1.2.0/' --extern-html-root-url 'stacker=https://docs.rs/stacker/0.1.14/' --extern-html-root-url 'tempfile=https://docs.rs/tempfile/3.2.0/' --extern-html-root-url 'thin_vec=https://docs.rs/thin-vec/0.2.8/' --extern-html-root-url 'tracing=https://docs.rs/tracing/0.1.35/' -Zunstable-options -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.66.0-nightly
  (c2afc4ed1
  2022-10-10)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --generate-link-to-definition --extern-html-root-url 'ena=https://docs.rs/ena/latest/'` (exit status: 101)
[RUSTC-TIMING] rustc_data_structures test:false 1.752
[RUSTC-TIMING] serde test:false 4.557
Build completed unsuccessfully in 0:19:38
