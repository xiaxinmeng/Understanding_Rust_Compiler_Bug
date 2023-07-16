plain
failures:

---- [ui] src/test/ui/restrictions/impl-restriction.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/restrictions/impl-restriction.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/restrictions/impl-restriction" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/restrictions/impl-restriction/auxiliary" "--crate-type=lib"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_metadata/src/rmeta/decoder.rs:457:33
stack backtrace:
   0:     0x7f1e0504d215 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0b4203fa508073aa
   1:     0x7f1e050bd468 - core::fmt::write::h99bd4e8f70a258f5
   2:     0x7f1e0503f2d1 - std::io::Write::write_fmt::hb899f7b7567a87b1
   3:     0x7f1e0504d021 - std::sys_common::backtrace::print::h84c790995c017e75
   4:     0x7f1e05050404 - std::panicking::default_hook::{{closure}}::h321f3f6dd8ef98bf
   5:     0x7f1e050500ca - std::panicking::default_hook::h3d5cac77e0cb8609
   6:     0x7f1e05aa11b2 - rustc_driver[f7eaf44a912ff2a2]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f1e05050b74 - std::panicking::rust_panic_with_hook::h63632cc5f61b6488
   8:     0x7f1e0505089a - std::panicking::begin_panic_handler::{{closure}}::hae3bdb6fb0c3f339
   9:     0x7f1e0504d734 - std::sys_common::backtrace::__rust_end_short_backtrace::hdc6d95cc14b293ce
  10:     0x7f1e05050582 - rust_begin_unwind
  11:     0x7f1e05000fd3 - core::panicking::panic_fmt::h22d423bcc14ae370
  12:     0x7f1e0500106d - core::panicking::panic::h5e7c718f0c7ba99a
  13:     0x7f1e07dde1f4 - <rustc_span[c3cb5f0f286ca043]::hygiene::SyntaxContext as rustc_serialize[7ec8be6eeab145df]::serialize::Decodable<rustc_metadata[4cbfd0758a7edbc6]::rmeta::decoder::DecodeContext>>::decode
  14:     0x7f1e07dde87a - <rustc_span[c3cb5f0f286ca043]::span_encoding::Span as rustc_serialize[7ec8be6eeab145df]::serialize::Decodable<rustc_metadata[4cbfd0758a7edbc6]::rmeta::decoder::DecodeContext>>::decode
  15:     0x7f1e07e1d0cf - <rustc_middle[7edb81460b5b18ef]::ty::Restriction as rustc_serialize[7ec8be6eeab145df]::serialize::Decodable<rustc_metadata[4cbfd0758a7edbc6]::rmeta::decoder::DecodeContext>>::decode
  16:     0x7f1e07e4c26c - <rustc_metadata[4cbfd0758a7edbc6]::rmeta::LazyValue<rustc_middle[7edb81460b5b18ef]::ty::Restriction>>::decode::<rustc_metadata[4cbfd0758a7edbc6]::creader::CrateMetadataRef>
  17:     0x7f1e07e9fc60 - <rustc_metadata[4cbfd0758a7edbc6]::creader::CrateMetadataRef>::get_impl_restriction
  18:     0x7f1e07dc6faf - rustc_metadata[4cbfd0758a7edbc6]::rmeta::decoder::cstore_impl::provide_extern::impl_restriction
  19:     0x7f1e07834301 - rustc_query_system[a0c4981a01644b4c]::query::plumbing::try_execute_query::<rustc_query_impl[5541e8ddaec983b6]::queries::impl_restriction, rustc_query_impl[5541e8ddaec983b6]::plumbing::QueryCtxt>
  20:     0x7f1e0790d183 - rustc_query_system[a0c4981a01644b4c]::query::plumbing::get_query::<rustc_query_impl[5541e8ddaec983b6]::queries::impl_restriction, rustc_query_impl[5541e8ddaec983b6]::plumbing::QueryCtxt, rustc_middle[7edb81460b5b18ef]::dep_graph::dep_node::DepKind>
  21:     0x7f1e07585143 - <rustc_query_impl[5541e8ddaec983b6]::Queries as rustc_middle[7edb81460b5b18ef]::ty::query::QueryEngine>::impl_restriction
  22:     0x7f1e05fa2778 - <rustc_restrictions[b73af9ec22cddcd6]::impl_restriction::ImplOfRestrictedTraitVisitor as rustc_hir[d04a52597f0232ed]::intravisit::Visitor>::visit_item
  23:     0x7f1e05fa7e0a - rustc_hir[d04a52597f0232ed]::intravisit::walk_mod::<rustc_restrictions[b73af9ec22cddcd6]::impl_restriction::ImplOfRestrictedTraitVisitor>
  24:     0x7f1e05fa2e6d - rustc_restrictions[b73af9ec22cddcd6]::impl_restriction::check_impl_restriction
  25:     0x7f1e07866121 - rustc_query_system[a0c4981a01644b4c]::query::plumbing::try_execute_query::<rustc_query_impl[5541e8ddaec983b6]::queries::check_impl_restriction, rustc_query_impl[5541e8ddaec983b6]::plumbing::QueryCtxt>
  26:     0x7f1e07923a1f - rustc_query_system[a0c4981a01644b4c]::query::plumbing::get_query::<rustc_query_impl[5541e8ddaec983b6]::queries::check_impl_restriction, rustc_query_impl[5541e8ddaec983b6]::plumbing::QueryCtxt, rustc_middle[7edb81460b5b18ef]::dep_graph::dep_node::DepKind>
  27:     0x7f1e075858ba - <rustc_query_impl[5541e8ddaec983b6]::Queries as rustc_middle[7edb81460b5b18ef]::ty::query::QueryEngine>::check_impl_restriction
  28:     0x7f1e05be8724 - <rustc_session[7974a37c3abe02]::session::Session>::time::<(), rustc_interface[891b2acbb9f61f03]::passes::analysis::{closure#5}::{closure#3}::{closure#0}>
  29:     0x7f1e05bdf825 - std[1a4cad301affc0b6]::panicking::try::<(), core[41e20aa9658f2512]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[891b2acbb9f61f03]::passes::analysis::{closure#5}::{closure#3}>>
  30:     0x7f1e05bf618c - rustc_interface[891b2acbb9f61f03]::passes::analysis
  31:     0x7f1e078959fe - rustc_query_system[a0c4981a01644b4c]::query::plumbing::try_execute_query::<rustc_query_impl[5541e8ddaec983b6]::queries::analysis, rustc_query_impl[5541e8ddaec983b6]::plumbing::QueryCtxt>
  32:     0x7f1e079489b1 - rustc_query_system[a0c4981a01644b4c]::query::plumbing::get_query::<rustc_query_impl[5541e8ddaec983b6]::queries::analysis, rustc_query_impl[5541e8ddaec983b6]::plumbing::QueryCtxt, rustc_middle[7edb81460b5b18ef]::dep_graph::dep_node::DepKind>
  33:     0x7f1e0752802a - <rustc_query_impl[5541e8ddaec983b6]::Queries as rustc_middle[7edb81460b5b18ef]::ty::query::QueryEngine>::analysis
  34:     0x7f1e05afa80c - <rustc_interface[891b2acbb9f61f03]::passes::QueryContext>::enter::<rustc_driver[f7eaf44a912ff2a2]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[41e20aa9658f2512]::result::Result<(), rustc_errors[c2271daa90db8ca0]::ErrorGuaranteed>>
  35:     0x7f1e05b1459a - <rustc_interface[891b2acbb9f61f03]::interface::Compiler>::enter::<rustc_driver[f7eaf44a912ff2a2]::run_compiler::{closure#1}::{closure#2}, core[41e20aa9658f2512]::result::Result<core[41e20aa9658f2512]::option::Option<rustc_interface[891b2acbb9f61f03]::queries::Linker>, rustc_errors[c2271daa90db8ca0]::ErrorGuaranteed>>
  36:     0x7f1e05aa297c - rustc_span[c3cb5f0f286ca043]::with_source_map::<core[41e20aa9658f2512]::result::Result<(), rustc_errors[c2271daa90db8ca0]::ErrorGuaranteed>, rustc_interface[891b2acbb9f61f03]::interface::run_compiler<core[41e20aa9658f2512]::result::Result<(), rustc_errors[c2271daa90db8ca0]::ErrorGuaranteed>, rustc_driver[f7eaf44a912ff2a2]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  37:     0x7f1e05b05b1a - <scoped_tls[3f5746808b72b9bd]::ScopedKey<rustc_span[c3cb5f0f286ca043]::SessionGlobals>>::set::<rustc_interface[891b2acbb9f61f03]::interface::run_compiler<core[41e20aa9658f2512]::result::Result<(), rustc_errors[c2271daa90db8ca0]::ErrorGuaranteed>, rustc_driver[f7eaf44a912ff2a2]::run_compiler::{closure#1}>::{closure#0}, core[41e20aa9658f2512]::result::Result<(), rustc_errors[c2271daa90db8ca0]::ErrorGuaranteed>>
  38:     0x7f1e05afdd89 - std[1a4cad301affc0b6]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[891b2acbb9f61f03]::util::run_in_thread_pool_with_globals<rustc_interface[891b2acbb9f61f03]::interface::run_compiler<core[41e20aa9658f2512]::result::Result<(), rustc_errors[c2271daa90db8ca0]::ErrorGuaranteed>, rustc_driver[f7eaf44a912ff2a2]::run_compiler::{closure#1}>::{closure#0}, core[41e20aa9658f2512]::result::Result<(), rustc_errors[c2271daa90db8ca0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[41e20aa9658f2512]::result::Result<(), rustc_errors[c2271daa90db8ca0]::ErrorGuaranteed>>
  39:     0x7f1e05afae26 - std[1a4cad301affc0b6]::panic::catch_unwind::<core[41e20aa9658f2512]::panic::unwind_safe::AssertUnwindSafe<<std[1a4cad301affc0b6]::thread::Builder>::spawn_unchecked_<rustc_interface[891b2acbb9f61f03]::util::run_in_thread_pool_with_globals<rustc_interface[891b2acbb9f61f03]::interface::run_compiler<core[41e20aa9658f2512]::result::Result<(), rustc_errors[c2271daa90db8ca0]::ErrorGuaranteed>, rustc_driver[f7eaf44a912ff2a2]::run_compiler::{closure#1}>::{closure#0}, core[41e20aa9658f2512]::result::Result<(), rustc_errors[c2271daa90db8ca0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[41e20aa9658f2512]::result::Result<(), rustc_errors[c2271daa90db8ca0]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[41e20aa9658f2512]::result::Result<(), rustc_errors[c2271daa90db8ca0]::ErrorGuaranteed>>
  40:     0x7f1e05ab1355 - <<std[1a4cad301affc0b6]::thread::Builder>::spawn_unchecked_<rustc_interface[891b2acbb9f61f03]::util::run_in_thread_pool_with_globals<rustc_interface[891b2acbb9f61f03]::interface::run_compiler<core[41e20aa9658f2512]::result::Result<(), rustc_errors[c2271daa90db8ca0]::ErrorGuaranteed>, rustc_driver[f7eaf44a912ff2a2]::run_compiler::{closure#1}>::{closure#0}, core[41e20aa9658f2512]::result::Result<(), rustc_errors[c2271daa90db8ca0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[41e20aa9658f2512]::result::Result<(), rustc_errors[c2271daa90db8ca0]::ErrorGuaranteed>>::{closure#1} as core[41e20aa9658f2512]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7f1e0505d97e - std::sys::unix::thread::Thread::new::thread_start::hfa88be40bdb4424f
  42:     0x7f1e04df2b43 - <unknown>
  43:     0x7f1e04e84a00 - <unknown>
  44:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.0-nightly (e7d062081 2023-01-05) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
query stack during panic:
#0 [impl_restriction] computing impl restriction for `external_impl_restriction::TopLevel`
#1 [check_impl_restriction] checking impl restrictions
#2 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/stats/hir-stats.rs stdout ----
---- [ui] src/test/ui/stats/hir-stats.rs stdout ----
diff of stderr:

14 ast-stats-1 Arm                       96 ( 1.3%)             2            48
15 ast-stats-1 ForeignItem               96 ( 1.3%)             1            96
16 ast-stats-1 - Fn                        96 ( 1.3%)             1
- ast-stats-1 FieldDef                 160 ( 2.2%)             2            80
- ast-stats-1 Stmt                     160 ( 2.2%)             5            32
+ ast-stats-1 Stmt                     160 ( 2.1%)             5            32
19 ast-stats-1 - Local                     32 ( 0.4%)             1
20 ast-stats-1 - MacCall                   32 ( 0.4%)             1
21 ast-stats-1 - Expr                      96 ( 1.3%)             3

- ast-stats-1 Param                    160 ( 2.2%)             4            40
+ ast-stats-1 Param                    160 ( 2.1%)             4            40
23 ast-stats-1 FnDecl                   200 ( 2.7%)             5            40
+ ast-stats-1 FieldDef                 208 ( 2.8%)             2           104
24 ast-stats-1 Variant                  240 ( 3.2%)             2           120
25 ast-stats-1 GenericBound             288 ( 3.9%)             4            72
26 ast-stats-1 - Trait                    288 ( 3.9%)             4

28 ast-stats-1 AssocItem                416 ( 5.6%)             4           104
29 ast-stats-1 - Type                     208 ( 2.8%)             2
30 ast-stats-1 - Fn                       208 ( 2.8%)             2
- ast-stats-1 GenericParam             480 ( 6.5%)             5            96
- ast-stats-1 Expr                     576 ( 7.8%)             8            72
+ ast-stats-1 GenericParam             480 ( 6.4%)             5            96
+ ast-stats-1 Expr                     576 ( 7.7%)             8            72
33 ast-stats-1 - Path                      72 ( 1.0%)             1
34 ast-stats-1 - Match                     72 ( 1.0%)             1
35 ast-stats-1 - Struct                    72 ( 1.0%)             1

39 ast-stats-1 - Struct                    88 ( 1.2%)             1
40 ast-stats-1 - Wild                      88 ( 1.2%)             1
41 ast-stats-1 - Ident                    440 ( 5.9%)             5
- ast-stats-1 PathSegment              720 ( 9.7%)            30            24
- ast-stats-1 Ty                       896 (12.1%)            14            64
+ ast-stats-1 PathSegment              720 ( 9.6%)            30            24
+ ast-stats-1 Ty                       896 (12.0%)            14            64
44 ast-stats-1 - Ptr                       64 ( 0.9%)             1
45 ast-stats-1 - Ref                       64 ( 0.9%)             1
46 ast-stats-1 - ImplicitSelf             128 ( 1.7%)             2

47 ast-stats-1 - Path                     640 ( 8.6%)            10
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- ast-stats-1 Item                   1_656 (22.3%)             9           184
+ ast-stats-1 Item                   1_656 (22.2%)             9           184
49 ast-stats-1 - Trait                    184 ( 2.5%)             1
50 ast-stats-1 - Enum                     184 ( 2.5%)             1
51 ast-stats-1 - ForeignMod               184 ( 2.5%)             1

52 ast-stats-1 - Impl                     184 ( 2.5%)             1
- ast-stats-1 - Fn                       368 ( 5.0%)             2
+ ast-stats-1 - Fn                       368 ( 4.9%)             2
54 ast-stats-1 - Use                      552 ( 7.4%)             3
55 ast-stats-1 ----------------------------------------------------------------
- ast-stats-1 Total                  7_416
+ ast-stats-1 Total                  7_464
57 ast-stats-1
58 ast-stats-2 POST EXPANSION AST STATS
59 ast-stats-2 Name                Accumulated Size         Count     Item Size

72 ast-stats-2 Attribute                128 ( 1.6%)             4            32
73 ast-stats-2 - DocComment                32 ( 0.4%)             1
74 ast-stats-2 - Normal                    96 ( 1.2%)             3
- ast-stats-2 FieldDef                 160 ( 2.0%)             2            80
76 ast-stats-2 Stmt                     160 ( 2.0%)             5            32
77 ast-stats-2 - Local                     32 ( 0.4%)             1
78 ast-stats-2 - Semi                      32 ( 0.4%)             1

79 ast-stats-2 - Expr                      96 ( 1.2%)             3
80 ast-stats-2 Param                    160 ( 2.0%)             4            40
81 ast-stats-2 FnDecl                   200 ( 2.5%)             5            40
- ast-stats-2 Variant                  240 ( 3.0%)             2           120
- ast-stats-2 GenericBound             288 ( 3.6%)             4            72
- ast-stats-2 - Trait                    288 ( 3.6%)             4
- ast-stats-2 Block                    288 ( 3.6%)             6            48
+ ast-stats-2 FieldDef                 208 ( 2.5%)             2           104
+ ast-stats-2 Variant                  240 ( 2.9%)             2           120
+ ast-stats-2 GenericBound             288 ( 3.5%)             4            72
+ ast-stats-2 - Trait                    288 ( 3.5%)             4
+ ast-stats-2 Block                    288 ( 3.5%)             6            48
86 ast-stats-2 AssocItem                416 ( 5.1%)             4           104
- ast-stats-2 - Type                     208 ( 2.6%)             2
- ast-stats-2 - Fn                       208 ( 2.6%)             2
+ ast-stats-2 - Type                     208 ( 2.5%)             2
+ ast-stats-2 - Fn                       208 ( 2.5%)             2
89 ast-stats-2 GenericParam             480 ( 5.9%)             5            96
- ast-stats-2 Pat                      616 ( 7.6%)             7            88
+ ast-stats-2 Pat                      616 ( 7.5%)             7            88
91 ast-stats-2 - Struct                    88 ( 1.1%)             1
92 ast-stats-2 - Wild                      88 ( 1.1%)             1
93 ast-stats-2 - Ident                    440 ( 5.4%)             5

- ast-stats-2 Expr                     648 ( 8.0%)             9            72
+ ast-stats-2 Expr                     648 ( 7.9%)             9            72
95 ast-stats-2 - Path                      72 ( 0.9%)             1
96 ast-stats-2 - Match                     72 ( 0.9%)             1
97 ast-stats-2 - Struct                    72 ( 0.9%)             1

98 ast-stats-2 - InlineAsm                 72 ( 0.9%)             1
99 ast-stats-2 - Lit                      144 ( 1.8%)             2
- ast-stats-2 - Block                    216 ( 2.7%)             3
- ast-stats-2 PathSegment              792 ( 9.8%)            33            24
+ ast-stats-2 - Block                    216 ( 2.6%)             3
+ ast-stats-2 PathSegment              792 ( 9.7%)            33            24
102 ast-stats-2 Ty                       896 (11.0%)            14            64
103 ast-stats-2 - Ptr                       64 ( 0.8%)             1
104 ast-stats-2 - Ref                       64 ( 0.8%)             1

105 ast-stats-2 - ImplicitSelf             128 ( 1.6%)             2
- ast-stats-2 - Path                     640 ( 7.9%)            10
- ast-stats-2 Item                   2_024 (25.0%)            11           184
+ ast-stats-2 - Path                     640 ( 7.8%)            10
+ ast-stats-2 Item                   2_024 (24.8%)            11           184
108 ast-stats-2 - Trait                    184 ( 2.3%)             1
109 ast-stats-2 - Enum                     184 ( 2.3%)             1
110 ast-stats-2 - ExternCrate              184 ( 2.3%)             1

111 ast-stats-2 - ForeignMod               184 ( 2.3%)             1
112 ast-stats-2 - Impl                     184 ( 2.3%)             1
113 ast-stats-2 - Fn                       368 ( 4.5%)             2
- ast-stats-2 - Use                      736 ( 9.1%)             4
+ ast-stats-2 - Use                      736 ( 9.0%)             4
115 ast-stats-2 ----------------------------------------------------------------
- ast-stats-2 Total                  8_112
+ ast-stats-2 Total                  8_160
117 ast-stats-2
118 hir-stats HIR STATS
119 hir-stats Name                Accumulated Size         Count     Item Size

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stats/hir-stats/hir-stats.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args stats/hir-stats.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/stats/hir-stats.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stats/hir-stats" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stats/hir-stats/auxiliary" "-Zhir-stats"
stdout: none
--- stderr -------------------------------
ast-stats-1 PRE EXPANSION AST STATS
ast-stats-1 Name                Accumulated Size         Count     Item Size
ast-stats-1 ----------------------------------------------------------------
ast-stats-1 ExprField                 48 ( 0.6%)             1            48
ast-stats-1 GenericArgs               56 ( 0.8%)             1            56
ast-stats-1 - AngleBracketed            56 ( 0.8%)             1
ast-stats-1 Crate                     56 ( 0.8%)             1            56
ast-stats-1 Attribute                 64 ( 0.9%)             2            32
ast-stats-1 - Normal                    32 ( 0.4%)             1
ast-stats-1 - DocComment                32 ( 0.4%)             1
ast-stats-1 Local                     72 ( 1.0%)             1            72
ast-stats-1 WherePredicate            72 ( 1.0%)             1            72
ast-stats-1 - BoundPredicate            72 ( 1.0%)             1
ast-stats-1 Arm                       96 ( 1.3%)             2            48
ast-stats-1 ForeignItem               96 ( 1.3%)             1            96
ast-stats-1 - Fn                        96 ( 1.3%)             1
ast-stats-1 Stmt                     160 ( 2.1%)             5            32
ast-stats-1 - Local                     32 ( 0.4%)             1
ast-stats-1 - MacCall                   32 ( 0.4%)             1
ast-stats-1 - Expr                      96 ( 1.3%)             3
ast-stats-1 Param                    160 ( 2.1%)             4            40
ast-stats-1 FnDecl                   200 ( 2.7%)             5            40
ast-stats-1 FieldDef                 208 ( 2.8%)             2           104
ast-stats-1 Variant                  240 ( 3.2%)             2           120
ast-stats-1 GenericBound             288 ( 3.9%)             4            72
ast-stats-1 - Trait                    288 ( 3.9%)             4
ast-stats-1 Block                    288 ( 3.9%)             6            48
ast-stats-1 AssocItem                416 ( 5.6%)             4           104
ast-stats-1 - Type                     208 ( 2.8%)             2
ast-stats-1 - Fn                       208 ( 2.8%)             2
ast-stats-1 GenericParam             480 ( 6.4%)             5            96
ast-stats-1 Expr                     576 ( 7.7%)             8            72
ast-stats-1 - Path                      72 ( 1.0%)             1
ast-stats-1 - Match                     72 ( 1.0%)             1
ast-stats-1 - Struct                    72 ( 1.0%)             1
ast-stats-1 - Lit                      144 ( 1.9%)             2
ast-stats-1 - Block                    216 ( 2.9%)             3
ast-stats-1 Pat                      616 ( 8.3%)             7            88
ast-stats-1 - Struct                    88 ( 1.2%)             1
ast-stats-1 - Wild                      88 ( 1.2%)             1
ast-stats-1 - Ident                    440 ( 5.9%)             5
ast-stats-1 PathSegment              720 ( 9.6%)            30            24
ast-stats-1 Ty                       896 (12.0%)            14            64
ast-stats-1 - Ptr                       64 ( 0.9%)             1
ast-stats-1 - Ref                       64 ( 0.9%)             1
ast-stats-1 - ImplicitSelf             128 ( 1.7%)             2
ast-stats-1 - Path                     640 ( 8.6%)            10
ast-stats-1 Item                   1_656 (22.2%)             9           184
ast-stats-1 - Trait                    184 ( 2.5%)             1
ast-stats-1 - Enum                     184 ( 2.5%)             1
ast-stats-1 - ForeignMod               184 ( 2.5%)             1
ast-stats-1 - Impl                     184 ( 2.5%)             1
ast-stats-1 - Fn                       368 ( 4.9%)             2
ast-stats-1 - Use                      552 ( 7.4%)             3
ast-stats-1 ----------------------------------------------------------------
ast-stats-1 Total                  7_464
ast-stats-1
ast-stats-2 POST EXPANSION AST STATS
ast-stats-2 Name                Accumulated Size         Count     Item Size
ast-stats-2 ----------------------------------------------------------------
ast-stats-2 ExprField                 48 ( 0.6%)             1            48
ast-stats-2 GenericArgs               56 ( 0.7%)             1            56
ast-stats-2 - AngleBracketed            56 ( 0.7%)             1
ast-stats-2 Crate                     56 ( 0.7%)             1            56
ast-stats-2 Local                     72 ( 0.9%)             1            72
ast-stats-2 WherePredicate            72 ( 0.9%)             1            72
ast-stats-2 - BoundPredicate            72 ( 0.9%)             1
ast-stats-2 Arm                       96 ( 1.2%)             2            48
ast-stats-2 ForeignItem               96 ( 1.2%)             1            96
ast-stats-2 - Fn                        96 ( 1.2%)             1
ast-stats-2 InlineAsm                120 ( 1.5%)             1           120
ast-stats-2 Attribute                128 ( 1.6%)             4            32
ast-stats-2 - DocComment                32 ( 0.4%)             1
ast-stats-2 - Normal                    96 ( 1.2%)             3
ast-stats-2 Stmt                     160 ( 2.0%)             5            32
ast-stats-2 - Local                     32 ( 0.4%)             1
ast-stats-2 - Semi                      32 ( 0.4%)             1
ast-stats-2 - Expr                      96 ( 1.2%)             3
ast-stats-2 Param                    160 ( 2.0%)             4            40
ast-stats-2 FnDecl                   200 ( 2.5%)             5            40
ast-stats-2 FieldDef                 208 ( 2.5%)             2           104
ast-stats-2 Variant                  240 ( 2.9%)             2           120
ast-stats-2 GenericBound             288 ( 3.5%)             4            72
ast-stats-2 - Trait                    288 ( 3.5%)             4
ast-stats-2 Block                    288 ( 3.5%)             6            48
ast-stats-2 AssocItem                416 ( 5.1%)             4           104
ast-stats-2 - Type                     208 ( 2.5%)             2
ast-stats-2 - Fn                       208 ( 2.5%)             2
ast-stats-2 GenericParam             480 ( 5.9%)             5            96
ast-stats-2 Pat                      616 ( 7.5%)             7            88
ast-stats-2 - Struct                    88 ( 1.1%)             1
ast-stats-2 - Wild                      88 ( 1.1%)             1
ast-stats-2 - Ident                    440 ( 5.4%)             5
ast-stats-2 Expr                     648 ( 7.9%)             9            72
ast-stats-2 - Path                      72 ( 0.9%)             1
ast-stats-2 - Match                     72 ( 0.9%)             1
ast-stats-2 - Struct                    72 ( 0.9%)             1
ast-stats-2 - InlineAsm                 72 ( 0.9%)             1
ast-stats-2 - Lit                      144 ( 1.8%)             2
ast-stats-2 - Block                    216 ( 2.6%)             3
ast-stats-2 PathSegment              792 ( 9.7%)            33            24
ast-stats-2 Ty                       896 (11.0%)            14            64
ast-stats-2 - Ptr                       64 ( 0.8%)             1
ast-stats-2 - Ref                       64 ( 0.8%)             1
ast-stats-2 - ImplicitSelf             128 ( 1.6%)             2
ast-stats-2 - Path                     640 ( 7.8%)            10
ast-stats-2 Item                   2_024 (24.8%)            11           184
ast-stats-2 - Trait                    184 ( 2.3%)             1
ast-stats-2 - Enum                     184 ( 2.3%)             1
ast-stats-2 - ExternCrate              184 ( 2.3%)             1
ast-stats-2 - ForeignMod               184 ( 2.3%)             1
ast-stats-2 - Impl                     184 ( 2.3%)             1
ast-stats-2 - Fn                       368 ( 4.5%)             2
ast-stats-2 - Use                      736 ( 9.0%)             4
ast-stats-2 ----------------------------------------------------------------
ast-stats-2 Total                  8_160
hir-stats HIR STATS
hir-stats HIR STATS
hir-stats Name                Accumulated Size         Count     Item Size
hir-stats ----------------------------------------------------------------
hir-stats ForeignItemRef            24 ( 0.3%)             1            24
hir-stats Lifetime                  24 ( 0.3%)             1            24
hir-stats Mod                       32 ( 0.4%)             1            32
hir-stats ExprField                 40 ( 0.4%)             1            40
hir-stats TraitItemRef              56 ( 0.6%)             2            28
hir-stats Local                     64 ( 0.7%)             1            64
hir-stats Param                     64 ( 0.7%)             2            32
hir-stats InlineAsm                 72 ( 0.8%)             1            72
hir-stats ImplItemRef               72 ( 0.8%)             2            36
hir-stats Body                      96 ( 1.1%)             3            32
hir-stats FieldDef                  96 ( 1.1%)             2            48
hir-stats Arm                       96 ( 1.1%)             2            48
hir-stats Stmt                      96 ( 1.1%)             3            32
hir-stats - Local                     32 ( 0.4%)             1
hir-stats - Semi                      32 ( 0.4%)             1
hir-stats - Expr                      32 ( 0.4%)             1
hir-stats FnDecl                   120 ( 1.3%)             3            40
hir-stats Attribute                128 ( 1.4%)             4            32
hir-stats GenericArg               128 ( 1.4%)             4            32
hir-stats - Type                      32 ( 0.4%)             1
hir-stats - Lifetime                  96 ( 1.1%)             3
hir-stats GenericArgs              144 ( 1.6%)             3            48
hir-stats Variant                  176 ( 1.9%)             2            88
hir-stats GenericBound             192 ( 2.1%)             4            48
hir-stats - Trait                    192 ( 2.1%)             4
hir-stats WherePredicate           192 ( 2.1%)             3            64
hir-stats - BoundPredicate           192 ( 2.1%)             3
hir-stats Block                    288 ( 3.2%)             6            48
hir-stats Pat                      360 ( 4.0%)             5            72
hir-stats - Wild                      72 ( 0.8%)             1
hir-stats - Struct                    72 ( 0.8%)             1
hir-stats - Binding                  216 ( 2.4%)             3
hir-stats GenericParam             400 ( 4.4%)             5            80
hir-stats Generics                 560 ( 6.2%)            10            56
hir-stats Ty                       720 ( 8.0%)            15            48
hir-stats - Ptr                       48 ( 0.5%)             1
hir-stats - Ref                       48 ( 0.5%)             1
hir-stats - Path                     624 ( 6.9%)            13
hir-stats Expr                     768 ( 8.5%)            12            64
hir-stats - Path                      64 ( 0.7%)             1
hir-stats - Struct                    64 ( 0.7%)             1
hir-stats - Match                     64 ( 0.7%)             1
hir-stats - InlineAsm                 64 ( 0.7%)             1
hir-stats - Lit                      128 ( 1.4%)             2
hir-stats - Block                    384 ( 4.2%)             6
hir-stats Item                     880 ( 9.7%)            11            80
hir-stats - Trait                     80 ( 0.9%)             1
hir-stats - Enum                      80 ( 0.9%)             1
hir-stats - ExternCrate               80 ( 0.9%)             1
hir-stats - ForeignMod                80 ( 0.9%)             1
hir-stats - Impl                      80 ( 0.9%)             1
hir-stats - Fn                       160 ( 1.8%)             2
hir-stats - Use                      320 ( 3.5%)             4
hir-stats Path                   1_240 (13.7%)            31            40
hir-stats PathSegment            1_920 (21.2%)            40            48
hir-stats ----------------------------------------------------------------
hir-stats
------------------------------------------


