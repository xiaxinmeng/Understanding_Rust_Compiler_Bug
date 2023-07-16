plain
Successfully built 712b86606ffd
Successfully tagged rust-ci:latest
Built container sha256:712b86606ffdd17187f344e8509fac226a6ebc57f3f56746e6084b0edb4cc2dd
Uploading finished image to https://ci-caches.rust-lang.org/docker/a27a2e8501e6bda8d9ec9572725b52c65accf3f919588efe2ef5cb584fdeae418747b3be4fa090dfcc3ff7ae8714c60234c3f32ed53a403a0831a7e22eb564d1
upload failed: - to s3://rust-lang-ci-sccache2/docker/a27a2e8501e6bda8d9ec9572725b52c65accf3f919588efe2ef5cb584fdeae418747b3be4fa090dfcc3ff7ae8714c60234c3f32ed53a403a0831a7e22eb564d1 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---
failures:

---- [ui] src/test/ui/structs/struct-record-suggestion.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/structs/struct-record-suggestion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/struct-record-suggestion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/struct-record-suggestion/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/structs/struct-record-suggestion.rs:10:20
   |
   |
LL |     let q = A { c: 5..Default::default() };
   |                    ^^^^^^^^^^^^^^^^^^^^^ expected `u64`, found struct `std::ops::Range`
   = note: expected type `u64`
            found struct `std::ops::Range<{integer}>`

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_hir_typeck/src/expr.rs:1855:22
   0:     0x7fcedb3a75fe - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1e2e35fa4e15c19e
   1:     0x7fcedb4103d8 - core::fmt::write::h6551f8c86e18ae3f
   2:     0x7fcedb399061 - std::io::Write::write_fmt::h3ae719e8dae6b929
   3:     0x7fcedb3a7401 - std::sys_common::backtrace::print::h7f08b722a84b3a2f
   3:     0x7fcedb3a7401 - std::sys_common::backtrace::print::h7f08b722a84b3a2f
   4:     0x7fcedb3aa794 - std::panicking::default_hook::{{closure}}::h66b3fdf646765cf6
   5:     0x7fcedb3aa459 - std::panicking::default_hook::h9a30b9f98a14e108
   6:     0x7fcedbd953d4 - rustc_driver[629040cd15fec58d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fcedb3aaee4 - std::panicking::rust_panic_with_hook::h2f1510898354ca45
   8:     0x7fcedb3aac09 - std::panicking::begin_panic_handler::{{closure}}::haa913d7fa6ed1734
   9:     0x7fcedb3a7b34 - std::sys_common::backtrace::__rust_end_short_backtrace::h67daab02455a8c79
  10:     0x7fcedb3aa912 - rust_begin_unwind
  11:     0x7fcedb35c903 - core::panicking::panic_fmt::hb75190da7d625ab9
  12:     0x7fcedb35c9dd - core::panicking::panic::h4ab729da960e2762
  13:     0x7fcedc29ff54 - <rustc_hir_typeck[40ce5a9c2da9d3c6]::fn_ctxt::FnCtxt>::check_expr_struct_fields
  14:     0x7fcedc307e6e - <rustc_hir_typeck[40ce5a9c2da9d3c6]::fn_ctxt::FnCtxt>::check_expr_kind
  15:     0x7fcedc29a2c1 - <rustc_hir_typeck[40ce5a9c2da9d3c6]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  16:     0x7fcedc302d32 - <rustc_hir_typeck[40ce5a9c2da9d3c6]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  17:     0x7fcedc2ba960 - <rustc_hir_typeck[40ce5a9c2da9d3c6]::fn_ctxt::FnCtxt>::check_decl
  18:     0x7fcedc2bacab - <rustc_hir_typeck[40ce5a9c2da9d3c6]::fn_ctxt::FnCtxt>::check_stmt
  19:     0x7fcedc2bb3d7 - <rustc_hir_typeck[40ce5a9c2da9d3c6]::fn_ctxt::FnCtxt>::check_block_with_expected
  20:     0x7fcedc303c9b - <rustc_hir_typeck[40ce5a9c2da9d3c6]::fn_ctxt::FnCtxt>::check_expr_kind
  21:     0x7fcedc29a2c1 - <rustc_hir_typeck[40ce5a9c2da9d3c6]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  22:     0x7fcedc302d32 - <rustc_hir_typeck[40ce5a9c2da9d3c6]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  23:     0x7fcedc29bb2c - <rustc_hir_typeck[40ce5a9c2da9d3c6]::fn_ctxt::FnCtxt>::check_return_expr
  24:     0x7fcedc40c312 - rustc_hir_typeck[40ce5a9c2da9d3c6]::check::check_fn
  25:     0x7fcedc42fb13 - <rustc_hir_typeck[40ce5a9c2da9d3c6]::inherited::InheritedBuilder>::enter::<rustc_hir_typeck[40ce5a9c2da9d3c6]::typeck_with_fallback<rustc_hir_typeck[40ce5a9c2da9d3c6]::typeck::{closure#0}>::{closure#1}, &rustc_middle[696199d31819883f]::ty::context::TypeckResults>
  26:     0x7fcedc456f21 - rustc_hir_typeck[40ce5a9c2da9d3c6]::typeck
  27:     0x7fceddac29c9 - rustc_query_system[209ae96016cb9745]::query::plumbing::try_execute_query::<rustc_query_impl[8dca69c4326f267f]::plumbing::QueryCtxt, rustc_query_system[209ae96016cb9745]::query::caches::DefaultCache<rustc_span[45dc2fb2ef168f3b]::def_id::LocalDefId, &rustc_middle[696199d31819883f]::ty::context::TypeckResults>>
  28:     0x7fceddbf364d - rustc_query_system[209ae96016cb9745]::query::plumbing::get_query::<rustc_query_impl[8dca69c4326f267f]::queries::typeck, rustc_query_impl[8dca69c4326f267f]::plumbing::QueryCtxt>
  29:     0x7fcedd75f120 - <rustc_query_impl[8dca69c4326f267f]::Queries as rustc_middle[696199d31819883f]::ty::query::QueryEngine>::typeck
  30:     0x7fcedc363b26 - <core[ec80980db2017715]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c58917a02625142d]::sync::par_for_each_in<&[rustc_span[45dc2fb2ef168f3b]::def_id::LocalDefId], <rustc_middle[696199d31819883f]::hir::map::Map>::par_body_owners<rustc_hir_typeck[40ce5a9c2da9d3c6]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[ec80980db2017715]::ops::function::FnOnce<()>>::call_once
  31:     0x7fcedc497ef6 - std[857e21b9fc1c7c4a]::panicking::try::<(), core[ec80980db2017715]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[c58917a02625142d]::sync::par_for_each_in<&[rustc_span[45dc2fb2ef168f3b]::def_id::LocalDefId], <rustc_middle[696199d31819883f]::hir::map::Map>::par_body_owners<rustc_hir_typeck[40ce5a9c2da9d3c6]::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  32:     0x7fcedc352ead - rustc_data_structures[c58917a02625142d]::sync::par_for_each_in::<&[rustc_span[45dc2fb2ef168f3b]::def_id::LocalDefId], <rustc_middle[696199d31819883f]::hir::map::Map>::par_body_owners<rustc_hir_typeck[40ce5a9c2da9d3c6]::typeck_item_bodies::{closure#0}>::{closure#0}>
  33:     0x7fcedc4567d7 - rustc_hir_typeck[40ce5a9c2da9d3c6]::typeck_item_bodies
  34:     0x7fceddb196fe - rustc_query_system[209ae96016cb9745]::query::plumbing::try_execute_query::<rustc_query_impl[8dca69c4326f267f]::plumbing::QueryCtxt, rustc_query_system[209ae96016cb9745]::query::caches::DefaultCache<(), ()>>
  35:     0x7fceddbbaceb - rustc_query_system[209ae96016cb9745]::query::plumbing::get_query::<rustc_query_impl[8dca69c4326f267f]::queries::typeck_item_bodies, rustc_query_impl[8dca69c4326f267f]::plumbing::QueryCtxt>
  36:     0x7fcedd75e9ca - <rustc_query_impl[8dca69c4326f267f]::Queries as rustc_middle[696199d31819883f]::ty::query::QueryEngine>::typeck_item_bodies
  37:     0x7fcedc592d5b - <rustc_session[1c24b5136c8e147f]::session::Session>::time::<(), rustc_hir_analysis[a92072a637b65309]::check_crate::{closure#7}>
  38:     0x7fcedc6e2713 - rustc_hir_analysis[a92072a637b65309]::check_crate
  39:     0x7fcedbf01d21 - rustc_interface[a91bdfd16f7e728f]::passes::analysis
  40:     0x7fceddb0df3f - rustc_query_system[209ae96016cb9745]::query::plumbing::try_execute_query::<rustc_query_impl[8dca69c4326f267f]::plumbing::QueryCtxt, rustc_query_system[209ae96016cb9745]::query::caches::DefaultCache<(), core[ec80980db2017715]::result::Result<(), rustc_errors[13d263178e17cb1d]::ErrorGuaranteed>>>
  41:     0x7fceddbf39fb - rustc_query_system[209ae96016cb9745]::query::plumbing::get_query::<rustc_query_impl[8dca69c4326f267f]::queries::analysis, rustc_query_impl[8dca69c4326f267f]::plumbing::QueryCtxt>
  42:     0x7fcedd7359ea - <rustc_query_impl[8dca69c4326f267f]::Queries as rustc_middle[696199d31819883f]::ty::query::QueryEngine>::analysis
  43:     0x7fcedbdf3cbd - <rustc_interface[a91bdfd16f7e728f]::passes::QueryContext>::enter::<rustc_driver[629040cd15fec58d]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[ec80980db2017715]::result::Result<(), rustc_errors[13d263178e17cb1d]::ErrorGuaranteed>>
  44:     0x7fcedbe0772a - <rustc_interface[a91bdfd16f7e728f]::interface::Compiler>::enter::<rustc_driver[629040cd15fec58d]::run_compiler::{closure#1}::{closure#2}, core[ec80980db2017715]::result::Result<core[ec80980db2017715]::option::Option<rustc_interface[a91bdfd16f7e728f]::queries::Linker>, rustc_errors[13d263178e17cb1d]::ErrorGuaranteed>>
  45:     0x7fcedbd96b2e - rustc_span[45dc2fb2ef168f3b]::with_source_map::<core[ec80980db2017715]::result::Result<(), rustc_errors[13d263178e17cb1d]::ErrorGuaranteed>, rustc_interface[a91bdfd16f7e728f]::interface::run_compiler<core[ec80980db2017715]::result::Result<(), rustc_errors[13d263178e17cb1d]::ErrorGuaranteed>, rustc_driver[629040cd15fec58d]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  46:     0x7fcedbdfa35c - <scoped_tls[57ed0101742b44be]::ScopedKey<rustc_span[45dc2fb2ef168f3b]::SessionGlobals>>::set::<rustc_interface[a91bdfd16f7e728f]::interface::run_compiler<core[ec80980db2017715]::result::Result<(), rustc_errors[13d263178e17cb1d]::ErrorGuaranteed>, rustc_driver[629040cd15fec58d]::run_compiler::{closure#1}>::{closure#0}, core[ec80980db2017715]::result::Result<(), rustc_errors[13d263178e17cb1d]::ErrorGuaranteed>>
  47:     0x7fcedbdb5e69 - std[857e21b9fc1c7c4a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a91bdfd16f7e728f]::util::run_in_thread_pool_with_globals<rustc_interface[a91bdfd16f7e728f]::interface::run_compiler<core[ec80980db2017715]::result::Result<(), rustc_errors[13d263178e17cb1d]::ErrorGuaranteed>, rustc_driver[629040cd15fec58d]::run_compiler::{closure#1}>::{closure#0}, core[ec80980db2017715]::result::Result<(), rustc_errors[13d263178e17cb1d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[ec80980db2017715]::result::Result<(), rustc_errors[13d263178e17cb1d]::ErrorGuaranteed>>
  48:     0x7fcedbdfbcd8 - std[857e21b9fc1c7c4a]::panic::catch_unwind::<core[ec80980db2017715]::panic::unwind_safe::AssertUnwindSafe<<std[857e21b9fc1c7c4a]::thread::Builder>::spawn_unchecked_<rustc_interface[a91bdfd16f7e728f]::util::run_in_thread_pool_with_globals<rustc_interface[a91bdfd16f7e728f]::interface::run_compiler<core[ec80980db2017715]::result::Result<(), rustc_errors[13d263178e17cb1d]::ErrorGuaranteed>, rustc_driver[629040cd15fec58d]::run_compiler::{closure#1}>::{closure#0}, core[ec80980db2017715]::result::Result<(), rustc_errors[13d263178e17cb1d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[ec80980db2017715]::result::Result<(), rustc_errors[13d263178e17cb1d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[ec80980db2017715]::result::Result<(), rustc_errors[13d263178e17cb1d]::ErrorGuaranteed>>
  49:     0x7fcedbda703a - <<std[857e21b9fc1c7c4a]::thread::Builder>::spawn_unchecked_<rustc_interface[a91bdfd16f7e728f]::util::run_in_thread_pool_with_globals<rustc_interface[a91bdfd16f7e728f]::interface::run_compiler<core[ec80980db2017715]::result::Result<(), rustc_errors[13d263178e17cb1d]::ErrorGuaranteed>, rustc_driver[629040cd15fec58d]::run_compiler::{closure#1}>::{closure#0}, core[ec80980db2017715]::result::Result<(), rustc_errors[13d263178e17cb1d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[ec80980db2017715]::result::Result<(), rustc_errors[13d263178e17cb1d]::ErrorGuaranteed>>::{closure#1} as core[ec80980db2017715]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  50:     0x7fcedb3b72fe - std::sys::unix::thread::Thread::new::thread_start::h30d15556887a1e89
  51:     0x7fcedb153b43 - <unknown>
  52:     0x7fcedb1e5a00 - <unknown>
  53:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (65a9b5b24 2022-11-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `a`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
error[E0308]: mismatched types
  --> /checkout/src/test/ui/structs/struct-record-suggestion.rs:24:20
   |
   |
LL |     let q = B { b: 1..Default::default() };
   |                    ^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found struct `std::ops::Range`
   = note: expected type `u32`
            found struct `std::ops::Range<{integer}>`
            found struct `std::ops::Range<{integer}>`
help: to set the remaining fields from `Default::default()`, separate the last named field with a comma
   |
LL |     let q = B { b: 1,..Default::default() };

error[E0308]: mismatched types
  --> /checkout/src/test/ui/structs/struct-record-suggestion.rs:10:20
   |
   |
LL |     let q = A { c: 5..Default::default() };
   |                    ^^^^^^^^^^^^^^^^^^^^^ expected `u64`, found struct `std::ops::Range`
   = note: expected type `u64`
            found struct `std::ops::Range<{integer}>`

error: aborting due to 3 previous errors
