plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
running 14517 tests
..........................................ii............................................ 88/14517
.................................................................................iiiiiii 176/14517
i.iiiiiii....................i..................i....................................... 264/14517
.............................................F............................F............. 352/14517
........................................................................................ 528/14517
........................................................................................ 616/14517
........................................................................................ 704/14517
........................................................................................ 792/14517
---
.............................................................iii........................ 14432/14517
.....................................................................................
failures:

---- [ui] tests/ui/associated-inherent-types/bugs/fresh-inf-vars-leaked-in-itemctxt.rs stdout ----

1 error: the compiler unexpectedly panicked. this is a bug.
2 
3 query stack during panic:
3 query stack during panic:
- #0 [typeck] type-checking `main`
- #1 [typeck_item_bodies] type-checking all item bodies
+ #0 [collect_mod_item_types] collecting item types in top-level module
+ #1 [analysis] running analysis passes on this crate
7 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/bugs/fresh-inf-vars-leaked-in-itemctxt/fresh-inf-vars-leaked-in-itemctxt.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-inherent-types/bugs/fresh-inf-vars-leaked-in-itemctxt.rs`
error: 1 errors occurred comparing output.
status: exit status: 101
status: exit status: 101
command: RUST_BACKTRACE="0" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-inherent-types/bugs/fresh-inf-vars-leaked-in-itemctxt.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/bugs/fresh-inf-vars-leaked-in-itemctxt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/bugs/fresh-inf-vars-leaked-in-itemctxt/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'type variables should not be hashed: _#0t', /checkout/compiler/rustc_type_ir/src/lib.rs:696:17

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (d93c1b4f8 2023-02-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [collect_mod_item_types] collecting item types in top-level module
#1 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] tests/ui/associated-inherent-types/substitute-params-bad.rs stdout ----
---- [ui] tests/ui/associated-inherent-types/substitute-params-bad.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-inherent-types/substitute-params-bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/substitute-params-bad" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/substitute-params-bad/auxiliary"
stdout: none
--- stderr -------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  --> fake-test-src-base/associated-inherent-types/substitute-params-bad.rs:9:9
   |
   |
LL | impl<T, 'a> S<T> { //~ ERROR lifetime parameters must be declared prior to type and const parameters
   |     ----^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, T>`

thread 'rustc' panicked at 'type variables should not be hashed: _#0t', /checkout/compiler/rustc_type_ir/src/lib.rs:696:17
   0:     0x7f3d3c5f4085 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::heb0e8852bd19ee85
   1:     0x7f3d3c65d938 - core::fmt::write::heb41eecae69b11f7
   2:     0x7f3d3c5e6081 - std::io::Write::write_fmt::hd27913492ffdbf83
   3:     0x7f3d3c5f3e91 - std::sys_common::backtrace::print::h25ca77f95a580bd1
   3:     0x7f3d3c5f3e91 - std::sys_common::backtrace::print::h25ca77f95a580bd1
   4:     0x7f3d3c5f7094 - std::panicking::default_hook::{{closure}}::he030f17c80380b11
   5:     0x7f3d3c5f6d7a - std::panicking::default_hook::hfdce2c13af1b3185
   6:     0x7f3d3d0d83a5 - rustc_driver_impl[153df6c50b47abce]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f3d3c5f77b1 - std::panicking::rust_panic_with_hook::h61320c70d37d7b80
   8:     0x7f3d3c5f7529 - std::panicking::begin_panic_handler::{{closure}}::hc52ceadca6818beb
   9:     0x7f3d3c5f4556 - std::sys_common::backtrace::__rust_end_short_backtrace::h263068f58dedd7c5
  10:     0x7f3d3c5f7207 - rust_begin_unwind
  11:     0x7f3d3c5aa303 - core::panicking::panic_fmt::h4f248033457b0464
  12:     0x7f3d3ea8c3b5 - <rustc_type_ir[226dad98a8090d9e]::InferTy as rustc_data_structures[87ddd873efe58833]::stable_hasher::HashStable<rustc_query_system[7034e84388327a47]::ich::hcx::StableHashingContext>>::hash_stable
  13:     0x7f3d3eae6795 - <rustc_type_ir[226dad98a8090d9e]::ty_info::WithCachedTypeInfo<rustc_type_ir[226dad98a8090d9e]::sty::TyKind<rustc_middle[252e0fe9546aded8]::ty::context::TyCtxt>> as rustc_data_structures[87ddd873efe58833]::stable_hasher::HashStable<rustc_query_system[7034e84388327a47]::ich::hcx::StableHashingContext>>::hash_stable
  14:     0x7f3d3eb83607 - rustc_query_system[7034e84388327a47]::dep_graph::graph::hash_result::<rustc_middle[252e0fe9546aded8]::ty::Ty>
  15:     0x7f3d3ee014e9 - rustc_query_system[7034e84388327a47]::query::plumbing::try_execute_query::<rustc_query_impl[7203e75c7f142ce1]::queries::type_of, rustc_query_impl[7203e75c7f142ce1]::plumbing::QueryCtxt>
  16:     0x7f3d3eec8d53 - rustc_query_system[7034e84388327a47]::query::plumbing::get_query::<rustc_query_impl[7203e75c7f142ce1]::queries::type_of, rustc_query_impl[7203e75c7f142ce1]::plumbing::QueryCtxt, rustc_middle[252e0fe9546aded8]::dep_graph::dep_node::DepKind>
  17:     0x7f3d3ec84945 - <rustc_query_impl[7203e75c7f142ce1]::Queries as rustc_middle[252e0fe9546aded8]::ty::query::QueryEngine>::type_of
  18:     0x7f3d3d979d4e - <rustc_hir_analysis[795e4905e24f7845]::collect::CollectItemTypesVisitor as rustc_hir[ae7a4cbd74de252c]::intravisit::Visitor>::visit_item
  19:     0x7f3d3d8f8540 - <rustc_middle[252e0fe9546aded8]::hir::map::Map>::visit_item_likes_in_module::<rustc_hir_analysis[795e4905e24f7845]::collect::CollectItemTypesVisitor>
  20:     0x7f3d3d977f4d - rustc_hir_analysis[795e4905e24f7845]::collect::collect_mod_item_types
  21:     0x7f3d3edcd6f5 - rustc_query_system[7034e84388327a47]::query::plumbing::try_execute_query::<rustc_query_impl[7203e75c7f142ce1]::queries::collect_mod_item_types, rustc_query_impl[7203e75c7f142ce1]::plumbing::QueryCtxt>
  22:     0x7f3d3ee9ce20 - rustc_query_system[7034e84388327a47]::query::plumbing::get_query::<rustc_query_impl[7203e75c7f142ce1]::queries::collect_mod_item_types, rustc_query_impl[7203e75c7f142ce1]::plumbing::QueryCtxt, rustc_middle[252e0fe9546aded8]::dep_graph::dep_node::DepKind>
  23:     0x7f3d3ecaffb0 - <rustc_query_impl[7203e75c7f142ce1]::Queries as rustc_middle[252e0fe9546aded8]::ty::query::QueryEngine>::collect_mod_item_types
  24:     0x7f3d3d8f7dea - <rustc_middle[252e0fe9546aded8]::hir::map::Map>::for_each_module::<rustc_hir_analysis[795e4905e24f7845]::check_crate::{closure#0}::{closure#0}::{closure#0}>
  25:     0x7f3d3d837a2c - <rustc_session[410c5f86ae6edc37]::session::Session>::track_errors::<rustc_hir_analysis[795e4905e24f7845]::check_crate::{closure#0}, ()>
  26:     0x7f3d3d949dfb - rustc_hir_analysis[795e4905e24f7845]::check_crate
  27:     0x7f3d3d1a9558 - rustc_interface[33ffd95b0021b94e]::passes::analysis
  28:     0x7f3d3ee01e09 - rustc_query_system[7034e84388327a47]::query::plumbing::try_execute_query::<rustc_query_impl[7203e75c7f142ce1]::queries::analysis, rustc_query_impl[7203e75c7f142ce1]::plumbing::QueryCtxt>
  29:     0x7f3d3eec8e11 - rustc_query_system[7034e84388327a47]::query::plumbing::get_query::<rustc_query_impl[7203e75c7f142ce1]::queries::analysis, rustc_query_impl[7203e75c7f142ce1]::plumbing::QueryCtxt, rustc_middle[252e0fe9546aded8]::dep_graph::dep_node::DepKind>
  30:     0x7f3d3ec8670a - <rustc_query_impl[7203e75c7f142ce1]::Queries as rustc_middle[252e0fe9546aded8]::ty::query::QueryEngine>::analysis
  31:     0x7f3d3d0db1f0 - <rustc_middle[252e0fe9546aded8]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[153df6c50b47abce]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[59e709323dcc814e]::result::Result<(), rustc_errors[b30051fe490a3f71]::ErrorGuaranteed>>
  32:     0x7f3d3d124738 - <rustc_interface[33ffd95b0021b94e]::interface::Compiler>::enter::<rustc_driver_impl[153df6c50b47abce]::run_compiler::{closure#1}::{closure#2}, core[59e709323dcc814e]::result::Result<core[59e709323dcc814e]::option::Option<rustc_interface[33ffd95b0021b94e]::queries::Linker>, rustc_errors[b30051fe490a3f71]::ErrorGuaranteed>>
  33:     0x7f3d3d0d96af - rustc_span[d42fdd2842e708c0]::with_source_map::<core[59e709323dcc814e]::result::Result<(), rustc_errors[b30051fe490a3f71]::ErrorGuaranteed>, rustc_interface[33ffd95b0021b94e]::interface::run_compiler<core[59e709323dcc814e]::result::Result<(), rustc_errors[b30051fe490a3f71]::ErrorGuaranteed>, rustc_driver_impl[153df6c50b47abce]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  34:     0x7f3d3d0ebc9b - <scoped_tls[54d99e21998a0fd0]::ScopedKey<rustc_span[d42fdd2842e708c0]::SessionGlobals>>::set::<rustc_interface[33ffd95b0021b94e]::interface::run_compiler<core[59e709323dcc814e]::result::Result<(), rustc_errors[b30051fe490a3f71]::ErrorGuaranteed>, rustc_driver_impl[153df6c50b47abce]::run_compiler::{closure#1}>::{closure#0}, core[59e709323dcc814e]::result::Result<(), rustc_errors[b30051fe490a3f71]::ErrorGuaranteed>>
  35:     0x7f3d3d0e7ce9 - std[41036c90a52ee8a7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[33ffd95b0021b94e]::util::run_in_thread_pool_with_globals<rustc_interface[33ffd95b0021b94e]::interface::run_compiler<core[59e709323dcc814e]::result::Result<(), rustc_errors[b30051fe490a3f71]::ErrorGuaranteed>, rustc_driver_impl[153df6c50b47abce]::run_compiler::{closure#1}>::{closure#0}, core[59e709323dcc814e]::result::Result<(), rustc_errors[b30051fe490a3f71]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[59e709323dcc814e]::result::Result<(), rustc_errors[b30051fe490a3f71]::ErrorGuaranteed>>
  36:     0x7f3d3d126616 - std[41036c90a52ee8a7]::panicking::try::<core[59e709323dcc814e]::result::Result<(), rustc_errors[b30051fe490a3f71]::ErrorGuaranteed>, core[59e709323dcc814e]::panic::unwind_safe::AssertUnwindSafe<<std[41036c90a52ee8a7]::thread::Builder>::spawn_unchecked_<rustc_interface[33ffd95b0021b94e]::util::run_in_thread_pool_with_globals<rustc_interface[33ffd95b0021b94e]::interface::run_compiler<core[59e709323dcc814e]::result::Result<(), rustc_errors[b30051fe490a3f71]::ErrorGuaranteed>, rustc_driver_impl[153df6c50b47abce]::run_compiler::{closure#1}>::{closure#0}, core[59e709323dcc814e]::result::Result<(), rustc_errors[b30051fe490a3f71]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[59e709323dcc814e]::result::Result<(), rustc_errors[b30051fe490a3f71]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  37:     0x7f3d3d0e8dc5 - <<std[41036c90a52ee8a7]::thread::Builder>::spawn_unchecked_<rustc_interface[33ffd95b0021b94e]::util::run_in_thread_pool_with_globals<rustc_interface[33ffd95b0021b94e]::interface::run_compiler<core[59e709323dcc814e]::result::Result<(), rustc_errors[b30051fe490a3f71]::ErrorGuaranteed>, rustc_driver_impl[153df6c50b47abce]::run_compiler::{closure#1}>::{closure#0}, core[59e709323dcc814e]::result::Result<(), rustc_errors[b30051fe490a3f71]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[59e709323dcc814e]::result::Result<(), rustc_errors[b30051fe490a3f71]::ErrorGuaranteed>>::{closure#1} as core[59e709323dcc814e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7f3d3c603a2e - std::sys::unix::thread::Thread::new::thread_start::hce37cdebb6826ee1
  39:     0x7f3d3c39ab43 - <unknown>
  40:     0x7f3d3c42ca00 - <unknown>
  41:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.69.0-nightly (d93c1b4f8 2023-02-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [collect_mod_item_types] collecting item types in top-level module
#1 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------




failures:
    [ui] tests/ui/associated-inherent-types/bugs/fresh-inf-vars-leaked-in-itemctxt.rs

test result: FAILED. 14381 passed; 2 failed; 134 ignored; 0 measured; 0 filtered out; finished in 149.98s

Build completed unsuccessfully in 0:13:39
