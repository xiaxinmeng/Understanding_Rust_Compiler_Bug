plain
2019-11-19T02:49:00.2371922Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-19T02:49:00.2595533Z ##[command]git config gc.auto 0
2019-11-19T02:49:00.2657414Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-19T02:49:00.2725167Z ##[command]git config --get-all http.proxy
2019-11-19T02:49:00.2868785Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66536/merge:refs/remotes/pull/66536/merge
---
2019-11-19T02:56:02.1730914Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-11-19T02:56:03.7233720Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-11-19T02:56:05.1028313Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-11-19T02:56:11.8392823Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2019-11-19T02:56:14.1411503Z error[E0432]: unresolved import `ty::query::job::QueryResult`
2019-11-19T02:56:14.1412394Z     --> src/librustc/ty/query/plumbing.rs:690:13
2019-11-19T02:56:14.1468529Z 673  | /     macro_rules! define_queries {
2019-11-19T02:56:14.1468529Z 673  | /     macro_rules! define_queries {
2019-11-19T02:56:14.1469518Z 674  | |         (<$tcx:tt> $($category:tt {
2019-11-19T02:56:14.1470276Z 675  | |             $($(#[$attr:meta])* [$($modifiers:tt)*] fn $name:ident: $node:ident($K:ty) -> $V:ty,)*
2019-11-19T02:56:14.1470855Z 676  | |         },)*) => {
2019-11-19T02:56:14.1471424Z 677  | /             define_queries_inner! { <$tcx>
2019-11-19T02:56:14.1472575Z 678  |                   $($( $(#[$attr])* category<$category> [$($modifiers)*] fn $name: $node($K) -> $V,)*)*
2019-11-19T02:56:14.1473958Z      | |_____________- in this macro invocation (#3)
2019-11-19T02:56:14.1474614Z 680  | |         }
2019-11-19T02:56:14.1475227Z 681  | |     }
2019-11-19T02:56:14.1475227Z 681  | |     }
2019-11-19T02:56:14.1476183Z      | |_____- in this expansion of `define_queries!` (#2)
2019-11-19T02:56:14.1476684Z 682  | 
2019-11-19T02:56:14.1477421Z 683  | /     macro_rules! define_queries_inner {
2019-11-19T02:56:14.1477966Z 684  | |         (<$tcx:tt>
2019-11-19T02:56:14.1478516Z 685  | |          $($(#[$attr:meta])* category<$category:tt>
2019-11-19T02:56:14.1479111Z 686  | |             [$($modifiers:tt)*] fn $name:ident: $node:ident($K:ty) -> $V:ty,)*) => {
2019-11-19T02:56:14.1480007Z ...    |
2019-11-19T02:56:14.1480581Z 690  | |             use ty::query::job::QueryResult;
2019-11-19T02:56:14.1481182Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `QueryResult` in `ty::query::job`
2019-11-19T02:56:14.1483071Z 1076 | |         }
2019-11-19T02:56:14.1506365Z 1077 | |     }
2019-11-19T02:56:14.1506365Z 1077 | |     }
2019-11-19T02:56:14.1507198Z      | |_____- in this expansion of `define_queries_inner!` (#3)
2019-11-19T02:56:14.1508589Z     ::: src/librustc/query/mod.rs:31:1
2019-11-19T02:56:14.1509116Z      |
2019-11-19T02:56:14.1509116Z      |
2019-11-19T02:56:14.1509664Z 31   |     / rustc_queries! {
2019-11-19T02:56:14.1510187Z 32   |     |     Other {
2019-11-19T02:56:14.1510744Z 33   |     |         /// Records the type of every item.
2019-11-19T02:56:14.1511298Z 34   |     |         query type_of(key: DefId) -> Ty<'tcx> {
2019-11-19T02:56:14.1513116Z 1131 |     |     }
2019-11-19T02:56:14.1513793Z 1132 |     | }
2019-11-19T02:56:14.1514425Z      |     | -
2019-11-19T02:56:14.1515031Z      |     | |
2019-11-19T02:56:14.1515031Z      |     | |
2019-11-19T02:56:14.1515686Z      |     |_in this expansion of `rustc_query_append!` (#1)
2019-11-19T02:56:14.1516846Z      | 
2019-11-19T02:56:14.1518049Z     ::: src/librustc/ty/query/mod.rs:98:1
2019-11-19T02:56:14.1518550Z      |
2019-11-19T02:56:14.1518550Z      |
2019-11-19T02:56:14.1519021Z 98   | /     rustc_query_append! { [define_queries!][ <'tcx>
2019-11-19T02:56:14.1519464Z 99   | |         Other {
2019-11-19T02:56:14.1519912Z 100  | |             /// Runs analysis passes on the crate.
2019-11-19T02:56:14.1520358Z 101  | |             [eval_always] fn analysis: Analysis(CrateNum) -> Result<(), ErrorReported>,
2019-11-19T02:56:14.1521160Z 103  | |     ]}
2019-11-19T02:56:14.1522181Z      | |______- in this macro invocation (#1)
2019-11-19T02:56:14.1522428Z 
2019-11-19T02:56:14.3068741Z     Checking syntax_expand v0.0.0 (/checkout/src/libsyntax_expand)
---
2019-11-19T02:56:35.0289073Z   local time: Tue Nov 19 02:56:35 UTC 2019
2019-11-19T02:56:35.1164293Z   network time: Tue, 19 Nov 2019 02:56:35 GMT
2019-11-19T02:56:35.1170090Z == end clock drift check ==
2019-11-19T02:56:36.2345280Z 
2019-11-19T02:56:36.2464373Z ##[error]Bash exited with code '1'.
2019-11-19T02:56:36.2494137Z ##[section]Starting: Checkout
2019-11-19T02:56:36.2495903Z ==============================================================================
2019-11-19T02:56:36.2495961Z Task         : Get sources
2019-11-19T02:56:36.2496027Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
