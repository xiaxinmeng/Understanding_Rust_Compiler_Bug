plain
.................................................................................................... 9300/11515
.................................................................................................... 9400/11515
..................................................................i......i.......................... 9500/11515
.................................................................................................... 9600/11515
.....iiiiiii..iiiiii.i.............................................................................. 9700/11515
.................................................................................................... 9900/11515
.................................................................................................... 10000/11515
.................................................................................................... 10100/11515
.................................................................................................... 10200/11515
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.076 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i......i..i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.27s

 finished in 2.348 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
failures:

---- [rustdoc] rustdoc/universal-impl-trait.rs stdout ----

error: rustdoc failed!
status: signal: 4
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/universal-impl-trait/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/universal-impl-trait" "/checkout/src/test/rustdoc/universal-impl-trait.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `DefId(0:35 ~ foo[8787]::much_universe::{opaque#1})`,
 right: `DefId(0:31 ~ foo[8787]::much_universe): Different parents for DefId(0:36 ~ foo[8787]::much_universe::{opaque#1}::{opaque#0})`', compiler/rustc_middle/src/hir/map/collector.rs:219:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (aa8973071 2021-03-01) running on x86_64-unknown-linux-gnu

query stack during panic:
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_query_system/src/query/plumbing.rs:246:47
stack backtrace:
   0:     0x7fe52dbc0fff - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h333da29b436d78d6
   1:     0x7fe52dc386ec - core::fmt::write::hf4433bc6cddbea6a
   2:     0x7fe52dbb3f05 - std::io::Write::write_fmt::h52ffeb70991ae796
   3:     0x7fe52dbc5632 - std::panicking::default_hook::{{closure}}::hf16dca9b28c042c1
   4:     0x7fe52dbc4fe7 - std::panicking::default_hook::hf1791697d850052e
   5:     0x7fe52e4ba39d - rustc_driver::report_ice::hcfb098e6b008582a
   6:     0x7fe52dbc5f63 - std::panicking::rust_panic_with_hook::h9abdb4d9ba4774fb
   7:     0x7fe52dbc5a77 - std::panicking::begin_panic_handler::{{closure}}::h4af8e8cf1f15274b
   8:     0x7fe52dbc14cc - std::sys_common::backtrace::__rust_end_short_backtrace::h6cf0c52d177f5a21
   9:     0x7fe52dbc5a09 - rust_begin_unwind
  10:     0x7fe52dc34a01 - core::panicking::panic_fmt::h1ece5642bfe79802
  11:     0x7fe52dc3494d - core::panicking::panic::hc30344b7e6c32a1a
  12:     0x7fe52f70791c - rustc_data_structures::cold_path::hbeadfcc15c5db82b
  13:     0x7fe52f1d4519 - rustc_query_system::query::plumbing::get_query_impl::h4bd10402ea12a052
  14:     0x7fe52f2f9a65 - rustc_query_system::query::plumbing::get_query::h59c4bb914fd08d77
  15:     0x7fe530aae7e1 - core::ops::function::FnOnce::call_once::h68166d78f834e723
  16:     0x7fe52f31d937 - rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::hir_owner>::compute::h5449219cdcc281db
  17:     0x7fe52f626710 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hf2ee10cab6022c1f
  18:     0x7fe52f4df588 - rustc_data_structures::stack::ensure_sufficient_stack::h430a68e119acf635
  19:     0x7fe52f276331 - rustc_query_system::query::plumbing::force_query_with_job::h2e90bb66fa57ac39
  20:     0x7fe52f1fd031 - rustc_query_system::query::plumbing::get_query_impl::ha46dde7f71e88fa3
  21:     0x7fe52f300d02 - rustc_query_system::query::plumbing::get_query::h8178121e3ea11e97
  22:     0x7fe52f6720fe - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::hir_owner::he916bd5ae84606cf
  23:     0x7fe530942482 - rustc_middle::hir::map::Map::find_entry::h1c97db9599fa12c1
  24:     0x7fe530945d73 - rustc_middle::hir::map::Map::opt_span::h2ba8fa4d0c602daf
  25:     0x7fe530946108 - rustc_middle::hir::map::Map::span_if_local::hd93f8eede318cf41
  26:     0x7fe530aaee5e - core::ops::function::FnOnce::call_once::h6deb8065f4f47714
  27:     0x7fe52f5bf9fb - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h0bf2c44bd3606efe
  28:     0x7fe52f4e72b1 - rustc_data_structures::stack::ensure_sufficient_stack::h8f5ed93779532b1a
  29:     0x7fe52f2718cd - rustc_query_system::query::plumbing::force_query_with_job::h0e2048f629381b09
  30:     0x7fe52f1bbb56 - rustc_query_system::query::plumbing::get_query_impl::h1b6661be24f2c214
  31:     0x7fe52f2ee4ef - rustc_query_system::query::plumbing::get_query::h12369638e59fd77c
  32:     0x7fe52f6ca944 - <rustc_span::def_id::DefId as rustc_query_impl::keys::Key>::default_span::h63bc0e9e3be51558
  33:     0x7fe52f6ca827 - <rustc_span::def_id::LocalDefId as rustc_query_impl::keys::Key>::default_span::h571418a71e4f0a48
  34:     0x7fe52f68ef04 - rustc_query_impl::make_query::index_hir::h2024e783d855d1ee
  35:     0x7fe52f4923fd - <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold::h0cbad439da2e2111
  36:     0x7fe52f3dc7b1 - <hashbrown::map::HashMap<K,V,S> as core::iter::traits::collect::Extend<(K,V)>>::extend::h56a79aa5f9349a5c
  37:     0x7fe52f2bc77c - rustc_query_system::query::plumbing::QueryState<D,K>::try_collect_active_jobs::h7fc1b034a9f80597
  38:     0x7fe52f66fa2c - rustc_query_impl::Queries::try_collect_active_jobs::h3147d496192258d9
  39:     0x7fe52f4cfee1 - rustc_query_system::query::job::print_query_stack::h367f74ed84258c08
  40:     0x7fe52e6181d6 - rustc_interface::interface::try_print_query_stack::hfe8cf7c87d314ccd
  41:     0x7fe52e4bac0a - rustc_driver::report_ice::hcfb098e6b008582a
  42:     0x7fe52dbc5f63 - std::panicking::rust_panic_with_hook::h9abdb4d9ba4774fb
  43:     0x7fe52dbc5aa7 - std::panicking::begin_panic_handler::{{closure}}::h4af8e8cf1f15274b
  44:     0x7fe52dbc14cc - std::sys_common::backtrace::__rust_end_short_backtrace::h6cf0c52d177f5a21
  45:     0x7fe52dbc5a09 - rust_begin_unwind
  46:     0x7fe52dc34a01 - core::panicking::panic_fmt::h1ece5642bfe79802
  47:     0x7fe52dc34d1e - core::panicking::assert_failed::inner::h78d67746a712c67b
  48:     0x7fe530af7c0b - core::panicking::assert_failed::h79ec5a04d4e3e18d
  49:     0x7fe5309d521a - rustc_middle::hir::map::collector::NodeCollector::insert_nested::h7db3de68cb8a7bd4
  50:     0x7fe530afc1ed - rustc_hir::intravisit::walk_fn::h8149a07de0ead18f
  51:     0x7fe5309d6041 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_fn::h091c9adfe5263708
  52:     0x7fe530afd560 - rustc_hir::intravisit::walk_item::h6d6acfc32c28ee14
  53:     0x7fe5309d55e3 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::h8a96763f554ebb5c
  54:     0x7fe5309d3e1f - rustc_middle::hir::map::collector::collect::h4bd7fcb79ee8b684
  55:     0x7fe530947365 - rustc_middle::hir::map::index_hir::h73dc9c0c2035a1f1
  56:     0x7fe52f5d6b56 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h3ff0c885c14a7f79
  57:     0x7fe52f4e7100 - rustc_data_structures::stack::ensure_sufficient_stack::h8dfea3c29df121ad
  58:     0x7fe52f27544c - rustc_query_system::query::plumbing::force_query_with_job::h26a7f582aa0db317
  59:     0x7fe52f22e92d - rustc_query_system::query::plumbing::get_query_impl::hf4b9725bb362be05
  60:     0x7fe52f303d15 - rustc_query_system::query::plumbing::get_query::h8f047f3fe08247bf
  61:     0x7fe530aae12a - core::ops::function::FnOnce::call_once::h2ea45230ee0a8994
  62:     0x7fe52f5f8fd6 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h95b07a4a6117707c
  63:     0x7fe52f4efb80 - rustc_data_structures::stack::ensure_sufficient_stack::he0751c07f70a2fd2
  64:     0x7fe52f2979dc - rustc_query_system::query::plumbing::force_query_with_job::he40fbb043f759b3e
  65:     0x7fe52f1d487d - rustc_query_system::query::plumbing::get_query_impl::h4bd10402ea12a052
  66:     0x7fe52f2f9a65 - rustc_query_system::query::plumbing::get_query::h59c4bb914fd08d77
  67:     0x7fe530942bc7 - rustc_middle::hir::map::Map::body::h87e33f5244177a40
  68:     0x7fe52ef34e1f - <rustc_resolve::late::lifetimes::LifetimeContext as rustc_hir::intravisit::Visitor>::visit_nested_body::h3d128ae84edc62ee
  69:     0x7fe52ef3631a - <rustc_resolve::late::lifetimes::LifetimeContext as rustc_hir::intravisit::Visitor>::visit_item::h3515a39f7ec71422
  70:     0x7fe52ef33d85 - rustc_resolve::late::lifetimes::resolve_lifetimes::h805dfdf0ae384e66
  71:     0x7fe52f324445 - rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::resolve_lifetimes>::compute::h220212b4ad929d42
  72:     0x7fe52f5ce914 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h2dd2d0d3417e86c5
  73:     0x7fe52f4f31a8 - rustc_data_structures::stack::ensure_sufficient_stack::hfd5fa461015503ac
  74:     0x7fe52f2906ae - rustc_query_system::query::plumbing::force_query_with_job::hb8a161be8133578f
  75:     0x7fe52f1dcd44 - rustc_query_system::query::plumbing::get_query_impl::h55bf1d4690ef3fb8
  76:     0x7fe52f31709e - rustc_query_system::query::plumbing::get_query::hfb3e6456c8603166
  77:     0x7fe52ef2ed0e - core::ops::function::FnOnce::call_once::h798ef9594e99b0fa
  78:     0x7fe52f611436 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hcab89313c1ce470b
  79:     0x7fe52f4f11b0 - rustc_data_structures::stack::ensure_sufficient_stack::hebd42a2ca81b3b12
  80:     0x7fe52f2850ac - rustc_query_system::query::plumbing::force_query_with_job::h7ce166366befcb32
  81:     0x7fe52f1ee2bd - rustc_query_system::query::plumbing::get_query_impl::h7a614bc07db230a6
  82:     0x7fe52f2fbec2 - rustc_query_system::query::plumbing::get_query::h67da3d5f933a6412
  83:     0x7fe530a2e6ff - rustc_middle::ty::context::TyCtxt::object_lifetime_defaults::h6610a21d4a09ecd9
  84:     0x7fe52ec62592 - rustc_typeck::collect::generics_of::hf1752c388434bb84
  85:     0x7fe52f31dbad - rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::generics_of>::compute::h264eb5c4daf569f6
  86:     0x7fe52f5c713f - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h1dc28ca3f915f1c4
  87:     0x7fe52f4e4f6e - rustc_data_structures::stack::ensure_sufficient_stack::h7aa3f7fc761be628
  88:     0x7fe52f2700e7 - rustc_query_system::query::plumbing::force_query_with_job::h0ac33d7583748621
  89:     0x7fe52f214b32 - rustc_query_system::query::plumbing::get_query_impl::hd35b597de9470015
  90:     0x7fe52f2efa4a - rustc_query_system::query::plumbing::get_query::h1d90f988dda3049b
  91:     0x7fe5309b7b5e - rustc_middle::ty::subst::<impl rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>::identity_for_item::h4073b445d47a81cc
  92:     0x7fe52ee11bc1 - rustc_typeck::collect::type_of::type_of::h064ad96cdde75af2
  93:     0x7fe52f5f2b8b - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h86fbdc7b62be35ce
  94:     0x7fe52f4dc462 - rustc_data_structures::stack::ensure_sufficient_stack::h2854ee3eb775ceaf
  95:     0x7fe52f27caed - rustc_query_system::query::plumbing::force_query_with_job::h563e86d9869d9f98
  96:     0x7fe52f20e65a - rustc_query_system::query::plumbing::get_query_impl::hc284aad8e7fc2369
  97:     0x7fe52f3030fa - rustc_query_system::query::plumbing::get_query::h89d4df4afb37c73b
  98:     0x7fe52ecc37e3 - rustc_middle::ty::util::<impl rustc_middle::ty::context::TyCtxt>::calculate_dtor::h04516c87dbe3ff1d
  99:     0x7fe52f61bd9b - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::he019577101b9450d
 100:     0x7fe52f4e1091 - rustc_data_structures::stack::ensure_sufficient_stack::h4f33dbaf2e530bcb
 101:     0x7fe52f28644d - rustc_query_system::query::plumbing::force_query_with_job::h82ff5b93ceba57ec
 102:     0x7fe52f1be706 - rustc_query_system::query::plumbing::get_query_impl::h222702bd3646650e
 103:     0x7fe52f2f5e2c - rustc_query_system::query::plumbing::get_query::h4399ff713e97899a
 104:     0x7fe530a8f55e - rustc_middle::ty::AdtDef::destructor::h413a3517a6a2edef
 105:     0x7fe52edf84cc - rustc_typeck::check::check::check_item_type::h94a63c6fc4a431d3
 106:     0x7fe52ee40200 - rustc_middle::hir::map::Map::visit_item_likes_in_module::hf4a732397edc755d
 107:     0x7fe52ee0600d - rustc_typeck::check::check::check_mod_item_types::h30913d18faccef66
 108:     0x7fe52f5e6412 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h669fdcee6f08a9c3
 109:     0x7fe52f4f0eb0 - rustc_data_structures::stack::ensure_sufficient_stack::he9b58df140bfab3e
 110:     0x7fe52f29a177 - rustc_query_system::query::plumbing::force_query_with_job::hf7c2b2b0ac07c476
 111:     0x7fe52f1cd775 - rustc_query_system::query::plumbing::get_query_impl::h42cd44f0d8213032
 112:     0x7fe52f2f794e - rustc_query_system::query::plumbing::get_query::h500ea720f2ca5a38
 113:     0x55a2317d7f25 - rustc_session::utils::<impl rustc_session::session::Session>::time::hc2a261ee3f4071e9
 114:     0x55a23180ee3f - rustdoc::core::run_global_ctxt::h9ac9da7279da5781
 115:     0x55a2317d7c7f - rustc_session::utils::<impl rustc_session::session::Session>::time::hb2a10dc67e6cbef7
 116:     0x55a2316e0a68 - rustc_interface::passes::QueryContext::enter::h946de78c76f8fc1f
 117:     0x55a231929253 - rustc_interface::interface::create_compiler_and_run::h3e9cac71c3adf557
 118:     0x55a23170e221 - rustdoc::main_options::h94b0e009b6adfbbe
 119:     0x55a23196dce5 - scoped_tls::ScopedKey<T>::set::h7488563af7d9518d
 120:     0x55a231913843 - rustc_span::with_session_globals::h4cedeee6d44a2552
 121:     0x55a231843b93 - std::sys_common::backtrace::__rust_begin_short_backtrace::he8fa09300e24e619
 122:     0x55a2319733e6 - std::panic::catch_unwind::h2a12a88781ee8b9b
 123:     0x55a23192d57a - core::ops::function::FnOnce::call_once{{vtable.shim}}::hb3ba52e47eccbedc
 124:     0x7fe52dbd75ba - std::sys::unix::thread::Thread::new::thread_start::h0d674cc134c80c83
 125:     0x7fe52d6f46db - start_thread
 126:     0x7fe52d07f71f - __clone
 127:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (aa8973071 2021-03-01) running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack
thread panicked while panicking. aborting.
------------------------------------------




failures:
    [rustdoc] rustdoc/universal-impl-trait.rs

test result: FAILED. 422 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out; finished in 30.00s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:10
