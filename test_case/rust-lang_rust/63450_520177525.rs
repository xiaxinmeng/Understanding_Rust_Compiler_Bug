plain
2019-08-10T19:50:07.1655105Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-10T19:50:07.1857435Z ##[command]git config gc.auto 0
2019-08-10T19:50:07.1924243Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-10T19:50:07.1988057Z ##[command]git config --get-all http.proxy
2019-08-10T19:50:07.2132070Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63450/merge:refs/remotes/pull/63450/merge
---
2019-08-10T19:50:41.4913531Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-10T19:50:41.4913565Z 
2019-08-10T19:50:41.4913795Z   git checkout -b <new-branch-name>
2019-08-10T19:50:41.4913828Z 
2019-08-10T19:50:41.4913895Z HEAD is now at 208c7e6ee Merge c5c155f29b9596ee5f083ec9b8297e0092bd1059 into be3fb0cd2cc408eb4cc9c1d71f9cedb2c974dcd9
2019-08-10T19:50:41.5073390Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-10T19:50:41.5076454Z ==============================================================================
2019-08-10T19:50:41.5076521Z Task         : Bash
2019-08-10T19:50:41.5076592Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-10T20:25:06.3310776Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-08-10T20:25:11.5975489Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-08-10T20:25:32.0175378Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-08-10T20:27:07.1248937Z    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-08-10T20:28:01.6844385Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:01.6844926Z    --> src/librustc/ty/query/plumbing.rs:350:5
2019-08-10T20:28:01.6845206Z     |
2019-08-10T20:28:01.6845623Z 350 | /     pub(super) fn get_query<Q: QueryDescription<'tcx>>(self, span: Span, key: Q::Key) -> Q::Value {
2019-08-10T20:28:01.6846079Z 351 | |         debug!("ty::query::get_query<{}>(key={:?}, span={:?})",
2019-08-10T20:28:01.6846435Z 352 | |                Q::NAME.as_str(),
2019-08-10T20:28:01.6847110Z 353 | |                key,
2019-08-10T20:28:01.6848244Z 431 | |         result
2019-08-10T20:28:01.6848575Z 432 | |     }
2019-08-10T20:28:01.6848575Z 432 | |     }
2019-08-10T20:28:01.6851848Z     | |_____^ type validation failed: encountered 144, but expected a valid enum discriminant
2019-08-10T20:28:01.6857166Z     |
2019-08-10T20:28:01.6896748Z     = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-10T20:28:01.7209817Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:01.7209817Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:01.7215142Z     --> src/librustc/ty/query/plumbing.rs:956:13
2019-08-10T20:28:01.7223207Z 693  | /     macro_rules! define_queries {
2019-08-10T20:28:01.7223207Z 693  | /     macro_rules! define_queries {
2019-08-10T20:28:01.7227190Z 694  | |         (<$tcx:tt> $($category:tt {
2019-08-10T20:28:01.7232060Z 695  | |             $($(#[$attr:meta])* [$($modifiers:tt)*] fn $name:ident: $node:ident($K:ty) -> $V:ty,)*
2019-08-10T20:28:01.7235675Z 696  | |         },)*) => {
2019-08-10T20:28:01.7239451Z 697  | /             define_queries_inner! { <$tcx>
2019-08-10T20:28:01.7258082Z 698  |                   $($( $(#[$attr])* category<$category> [$($modifiers)*] fn $name: $node($K) -> $V,)*)*
2019-08-10T20:28:01.7259024Z      | |_____________- in this macro invocation (#3)
2019-08-10T20:28:01.7259382Z 700  | |         }
2019-08-10T20:28:01.7259716Z 701  | |     }
2019-08-10T20:28:01.7259716Z 701  | |     }
2019-08-10T20:28:01.7260082Z      | |_____- in this expansion of `define_queries!` (#2)
2019-08-10T20:28:01.7260363Z 702  | 
2019-08-10T20:28:01.7260739Z 703  | /     macro_rules! define_queries_inner {
2019-08-10T20:28:01.7261073Z 704  | |         (<$tcx:tt>
2019-08-10T20:28:01.7261441Z 705  | |          $($(#[$attr:meta])* category<$category:tt>
2019-08-10T20:28:01.7261873Z 706  | |             [$($modifiers:tt)*] fn $name:ident: $node:ident($K:ty) -> $V:ty,)*) => {
2019-08-10T20:28:01.7262151Z ...    |
2019-08-10T20:28:01.7262740Z 956  | |                 const NAME: QueryName = QueryName::$name;
2019-08-10T20:28:01.7263300Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 144, but expected a valid enum discriminant
2019-08-10T20:28:01.7263929Z 1087 | |         }
2019-08-10T20:28:01.7264266Z 1088 | |     }
2019-08-10T20:28:01.7264266Z 1088 | |     }
2019-08-10T20:28:01.7264659Z      | |_____- in this expansion of `define_queries_inner!` (#3)
2019-08-10T20:28:01.7265235Z     ::: src/librustc/query/mod.rs:32:1
2019-08-10T20:28:01.7265479Z      |
2019-08-10T20:28:01.7265479Z      |
2019-08-10T20:28:01.7265813Z 32   |     / rustc_queries! {
2019-08-10T20:28:01.7266166Z 33   |     |     Other {
2019-08-10T20:28:01.7266589Z 34   |     |         /// Records the type of every item.
2019-08-10T20:28:01.7267161Z 35   |     |         query type_of(key: DefId) -> Ty<'tcx> {
2019-08-10T20:28:01.7268308Z 1106 |     |     }
2019-08-10T20:28:01.7268638Z 1107 |     | }
2019-08-10T20:28:01.7268981Z      |     | -
2019-08-10T20:28:01.7269295Z      |     | |
2019-08-10T20:28:01.7269295Z      |     | |
2019-08-10T20:28:01.7269666Z      |     |_in this expansion of `rustc_query_append!` (#1)
2019-08-10T20:28:01.7270243Z      | 
2019-08-10T20:28:01.7270547Z     ::: src/librustc/ty/query/mod.rs:101:1
2019-08-10T20:28:01.7270799Z      |
2019-08-10T20:28:01.7270799Z      |
2019-08-10T20:28:01.7271161Z 101  | /     rustc_query_append! { [define_queries!][ <'tcx>
2019-08-10T20:28:01.7271509Z 102  | |         Other {
2019-08-10T20:28:01.7271919Z 103  | |             /// Runs analysis passes on the crate.
2019-08-10T20:28:01.7272362Z 104  | |             [eval_always] fn analysis: Analysis(CrateNum) -> Result<(), ErrorReported>,
2019-08-10T20:28:01.7273126Z 106  | |     ]}
2019-08-10T20:28:01.7273650Z      | |______- in this macro invocation (#1)
2019-08-10T20:28:01.7274044Z      |
2019-08-10T20:28:01.7274044Z      |
2019-08-10T20:28:01.7274591Z      = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-10T20:28:01.9136017Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:01.9136017Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:01.9136578Z     --> src/librustc/ty/query/plumbing.rs:1169:1
2019-08-10T20:28:01.9136850Z      |
2019-08-10T20:28:01.9137628Z 1169 | / pub fn force_from_dep_node(tcx: TyCtxt<'_>, dep_node: &DepNode) -> bool {
2019-08-10T20:28:01.9138136Z 1170 | |     use crate::dep_graph::RecoverKey;
2019-08-10T20:28:01.9138443Z 1171 | |
2019-08-10T20:28:01.9138841Z 1172 | |     // We must avoid ever having to call force_from_dep_node() for a
2019-08-10T20:28:01.9139709Z 1244 | |     true
2019-08-10T20:28:01.9140040Z 1245 | | }
2019-08-10T20:28:01.9140040Z 1245 | | }
2019-08-10T20:28:01.9140435Z      | |_^ type validation failed: encountered 174, but expected a valid enum discriminant
2019-08-10T20:28:01.9140688Z      |
2019-08-10T20:28:01.9141197Z      = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-10T20:28:03.2467495Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.2467495Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.2468940Z    --> src/librustc/ty/query/plumbing.rs:618:5
2019-08-10T20:28:03.2469262Z     |
2019-08-10T20:28:03.2469675Z 618 | /     fn force_query<Q: QueryDescription<'tcx>>(self, key: Q::Key, span: Span, dep_node: DepNode) {
2019-08-10T20:28:03.2473041Z 619 | |         profq_msg!(
2019-08-10T20:28:03.2479206Z 620 | |             self,
2019-08-10T20:28:03.2484206Z 621 | |             ProfileQueriesMsg::QueryBegin(span.data(),
2019-08-10T20:28:03.2504432Z ...   |
2019-08-10T20:28:03.2505375Z 634 | |         self.force_query_with_job::<Q>(key, job, dep_node);
2019-08-10T20:28:03.2505964Z 635 | |     }
2019-08-10T20:28:03.2506705Z     | |_____^ type validation failed: encountered 128, but expected a valid enum discriminant
2019-08-10T20:28:03.2507834Z     |
2019-08-10T20:28:03.2508768Z     = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-10T20:28:03.2509591Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.2509591Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.2510139Z     --> src/librustc/ty/query/plumbing.rs:956:13
2019-08-10T20:28:03.2511268Z 693  | /     macro_rules! define_queries {
2019-08-10T20:28:03.2511268Z 693  | /     macro_rules! define_queries {
2019-08-10T20:28:03.2511866Z 694  | |         (<$tcx:tt> $($category:tt {
2019-08-10T20:28:03.2512514Z 695  | |             $($(#[$attr:meta])* [$($modifiers:tt)*] fn $name:ident: $node:ident($K:ty) -> $V:ty,)*
2019-08-10T20:28:03.2513106Z 696  | |         },)*) => {
2019-08-10T20:28:03.2513691Z 697  | /             define_queries_inner! { <$tcx>
2019-08-10T20:28:03.2514293Z 698  |                   $($( $(#[$attr])* category<$category> [$($modifiers)*] fn $name: $node($K) -> $V,)*)*
2019-08-10T20:28:03.2515688Z      | |_____________- in this macro invocation (#3)
2019-08-10T20:28:03.2516252Z 700  | |         }
2019-08-10T20:28:03.2516789Z 701  | |     }
2019-08-10T20:28:03.2516789Z 701  | |     }
2019-08-10T20:28:03.2517371Z      | |_____- in this expansion of `define_queries!` (#2)
2019-08-10T20:28:03.2518326Z 702  | 
2019-08-10T20:28:03.2518925Z 703  | /     macro_rules! define_queries_inner {
2019-08-10T20:28:03.2519575Z 704  | |         (<$tcx:tt>
2019-08-10T20:28:03.2520189Z 705  | |          $($(#[$attr:meta])* category<$category:tt>
2019-08-10T20:28:03.2520801Z 706  | |             [$($modifiers:tt)*] fn $name:ident: $node:ident($K:ty) -> $V:ty,)*) => {
2019-08-10T20:28:03.2521289Z ...    |
2019-08-10T20:28:03.2521849Z 956  | |                 const NAME: QueryName = QueryName::$name;
2019-08-10T20:28:03.2522589Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 128, but expected a valid enum discriminant
2019-08-10T20:28:03.2523592Z 1087 | |         }
2019-08-10T20:28:03.2524309Z 1088 | |     }
2019-08-10T20:28:03.2524309Z 1088 | |     }
2019-08-10T20:28:03.2524971Z      | |_____- in this expansion of `define_queries_inner!` (#3)
2019-08-10T20:28:03.2526062Z     ::: src/librustc/query/mod.rs:32:1
2019-08-10T20:28:03.2526506Z      |
2019-08-10T20:28:03.2526506Z      |
2019-08-10T20:28:03.2527042Z 32   |     / rustc_queries! {
2019-08-10T20:28:03.2527917Z 33   |     |     Other {
2019-08-10T20:28:03.2528619Z 34   |     |         /// Records the type of every item.
2019-08-10T20:28:03.2529207Z 35   |     |         query type_of(key: DefId) -> Ty<'tcx> {
2019-08-10T20:28:03.2530838Z 1106 |     |     }
2019-08-10T20:28:03.2531375Z 1107 |     | }
2019-08-10T20:28:03.2532178Z      |     | -
2019-08-10T20:28:03.2532747Z      |     | |
2019-08-10T20:28:03.2532747Z      |     | |
2019-08-10T20:28:03.2534764Z      |     |_in this expansion of `rustc_query_append!` (#1)
2019-08-10T20:28:03.2535927Z      | 
2019-08-10T20:28:03.2536397Z     ::: src/librustc/ty/query/mod.rs:101:1
2019-08-10T20:28:03.2536837Z      |
2019-08-10T20:28:03.2536837Z      |
2019-08-10T20:28:03.2538577Z 101  | /     rustc_query_append! { [define_queries!][ <'tcx>
2019-08-10T20:28:03.2539340Z 102  | |         Other {
2019-08-10T20:28:03.2540053Z 103  | |             /// Runs analysis passes on the crate.
2019-08-10T20:28:03.2543693Z 104  | |             [eval_always] fn analysis: Analysis(CrateNum) -> Result<(), ErrorReported>,
2019-08-10T20:28:03.2544397Z 106  | |     ]}
2019-08-10T20:28:03.2544740Z      | |______- in this macro invocation (#1)
2019-08-10T20:28:03.2544988Z      |
2019-08-10T20:28:03.2544988Z      |
2019-08-10T20:28:03.2546623Z      = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-10T20:28:03.2594791Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.2594791Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.2595214Z    --> src/librustc/ty/query/plumbing.rs:618:5
2019-08-10T20:28:03.2595480Z     |
2019-08-10T20:28:03.2595911Z 618 | /     fn force_query<Q: QueryDescription<'tcx>>(self, key: Q::Key, span: Span, dep_node: DepNode) {
2019-08-10T20:28:03.2596256Z 619 | |         profq_msg!(
2019-08-10T20:28:03.2596601Z 620 | |             self,
2019-08-10T20:28:03.2596984Z 621 | |             ProfileQueriesMsg::QueryBegin(span.data(),
2019-08-10T20:28:03.2597256Z ...   |
2019-08-10T20:28:03.2597645Z 634 | |         self.force_query_with_job::<Q>(key, job, dep_node);
2019-08-10T20:28:03.2598398Z 635 | |     }
2019-08-10T20:28:03.2598799Z     | |_____^ type validation failed: encountered 129, but expected a valid enum discriminant
2019-08-10T20:28:03.2599072Z     |
2019-08-10T20:28:03.2599556Z     = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-10T20:28:03.2608864Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.2608864Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.2610560Z     --> src/librustc/ty/query/plumbing.rs:956:13
2019-08-10T20:28:03.2611206Z 693  | /     macro_rules! define_queries {
2019-08-10T20:28:03.2611206Z 693  | /     macro_rules! define_queries {
2019-08-10T20:28:03.2611590Z 694  | |         (<$tcx:tt> $($category:tt {
2019-08-10T20:28:03.2612014Z 695  | |             $($(#[$attr:meta])* [$($modifiers:tt)*] fn $name:ident: $node:ident($K:ty) -> $V:ty,)*
2019-08-10T20:28:03.2612372Z 696  | |         },)*) => {
2019-08-10T20:28:03.2612756Z 697  | /             define_queries_inner! { <$tcx>
2019-08-10T20:28:03.2613170Z 698  |                   $($( $(#[$attr])* category<$category> [$($modifiers)*] fn $name: $node($K) -> $V,)*)*
2019-08-10T20:28:03.2614266Z      | |_____________- in this macro invocation (#3)
2019-08-10T20:28:03.2614618Z 700  | |         }
2019-08-10T20:28:03.2614952Z 701  | |     }
2019-08-10T20:28:03.2614952Z 701  | |     }
2019-08-10T20:28:03.2615306Z      | |_____- in this expansion of `define_queries!` (#2)
2019-08-10T20:28:03.2615741Z 702  | 
2019-08-10T20:28:03.2616144Z 703  | /     macro_rules! define_queries_inner {
2019-08-10T20:28:03.2616496Z 704  | |         (<$tcx:tt>
2019-08-10T20:28:03.2616867Z 705  | |          $($(#[$attr:meta])* category<$category:tt>
2019-08-10T20:28:03.2617268Z 706  | |             [$($modifiers:tt)*] fn $name:ident: $node:ident($K:ty) -> $V:ty,)*) => {
2019-08-10T20:28:03.2617896Z ...    |
2019-08-10T20:28:03.2618337Z 956  | |                 const NAME: QueryName = QueryName::$name;
2019-08-10T20:28:03.2618861Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 129, but expected a valid enum discriminant
2019-08-10T20:28:03.2619494Z 1087 | |         }
2019-08-10T20:28:03.2619819Z 1088 | |     }
2019-08-10T20:28:03.2619819Z 1088 | |     }
2019-08-10T20:28:03.2620367Z      | |_____- in this expansion of `define_queries_inner!` (#3)
2019-08-10T20:28:03.2620999Z     ::: src/librustc/query/mod.rs:32:1
2019-08-10T20:28:03.2621254Z      |
2019-08-10T20:28:03.2621254Z      |
2019-08-10T20:28:03.2621614Z 32   |     / rustc_queries! {
2019-08-10T20:28:03.2621950Z 33   |     |     Other {
2019-08-10T20:28:03.2622334Z 34   |     |         /// Records the type of every item.
2019-08-10T20:28:03.2622711Z 35   |     |         query type_of(key: DefId) -> Ty<'tcx> {
2019-08-10T20:28:03.2623343Z 1106 |     |     }
2019-08-10T20:28:03.2623685Z 1107 |     | }
2019-08-10T20:28:03.2624010Z      |     | -
2019-08-10T20:28:03.2624340Z      |     | |
2019-08-10T20:28:03.2624340Z      |     | |
2019-08-10T20:28:03.2624693Z      |     |_in this expansion of `rustc_query_append!` (#1)
2019-08-10T20:28:03.2625277Z      | 
2019-08-10T20:28:03.2625561Z     ::: src/librustc/ty/query/mod.rs:101:1
2019-08-10T20:28:03.2625798Z      |
2019-08-10T20:28:03.2625798Z      |
2019-08-10T20:28:03.2626269Z 101  | /     rustc_query_append! { [define_queries!][ <'tcx>
2019-08-10T20:28:03.2626637Z 102  | |         Other {
2019-08-10T20:28:03.2627068Z 103  | |             /// Runs analysis passes on the crate.
2019-08-10T20:28:03.2627515Z 104  | |             [eval_always] fn analysis: Analysis(CrateNum) -> Result<(), ErrorReported>,
2019-08-10T20:28:03.2628874Z 106  | |     ]}
2019-08-10T20:28:03.2629515Z      | |______- in this macro invocation (#1)
2019-08-10T20:28:03.2629808Z      |
2019-08-10T20:28:03.2629808Z      |
2019-08-10T20:28:03.2630312Z      = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-10T20:28:03.2704648Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.2704648Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.2705008Z    --> src/librustc/ty/query/plumbing.rs:618:5
2019-08-10T20:28:03.2705552Z     |
2019-08-10T20:28:03.2705992Z 618 | /     fn force_query<Q: QueryDescription<'tcx>>(self, key: Q::Key, span: Span, dep_node: DepNode) {
2019-08-10T20:28:03.2706348Z 619 | |         profq_msg!(
2019-08-10T20:28:03.2706709Z 620 | |             self,
2019-08-10T20:28:03.2707076Z 621 | |             ProfileQueriesMsg::QueryBegin(span.data(),
2019-08-10T20:28:03.2707346Z ...   |
2019-08-10T20:28:03.2708094Z 634 | |         self.force_query_with_job::<Q>(key, job, dep_node);
2019-08-10T20:28:03.2708452Z 635 | |     }
2019-08-10T20:28:03.2708858Z     | |_____^ type validation failed: encountered 130, but expected a valid enum discriminant
2019-08-10T20:28:03.2709113Z     |
2019-08-10T20:28:03.2709606Z     = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-10T20:28:03.2725443Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.2725443Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.2725818Z     --> src/librustc/ty/query/plumbing.rs:956:13
2019-08-10T20:28:03.2726454Z 693  | /     macro_rules! define_queries {
2019-08-10T20:28:03.2726454Z 693  | /     macro_rules! define_queries {
2019-08-10T20:28:03.2726854Z 694  | |         (<$tcx:tt> $($category:tt {
2019-08-10T20:28:03.2727421Z 695  | |             $($(#[$attr:meta])* [$($modifiers:tt)*] fn $name:ident: $node:ident($K:ty) -> $V:ty,)*
2019-08-10T20:28:03.2728276Z 696  | |         },)*) => {
2019-08-10T20:28:03.2728721Z 697  | /             define_queries_inner! { <$tcx>
2019-08-10T20:28:03.2729128Z 698  |                   $($( $(#[$attr])* category<$category> [$($modifiers)*] fn $name: $node($K) -> $V,)*)*
2019-08-10T20:28:03.2729858Z      | |_____________- in this macro invocation (#3)
2019-08-10T20:28:03.2730217Z 700  | |         }
2019-08-10T20:28:03.2730540Z 701  | |     }
2019-08-10T20:28:03.2730540Z 701  | |     }
2019-08-10T20:28:03.2730910Z      | |_____- in this expansion of `define_queries!` (#2)
2019-08-10T20:28:03.2731185Z 702  | 
2019-08-10T20:28:03.2731531Z 703  | /     macro_rules! define_queries_inner {
2019-08-10T20:28:03.2732060Z 704  | |         (<$tcx:tt>
2019-08-10T20:28:03.2732474Z 705  | |          $($(#[$attr:meta])* category<$category:tt>
2019-08-10T20:28:03.2732883Z 706  | |             [$($modifiers:tt)*] fn $name:ident: $node:ident($K:ty) -> $V:ty,)*) => {
2019-08-10T20:28:03.2733362Z ...    |
2019-08-10T20:28:03.2733800Z 956  | |                 const NAME: QueryName = QueryName::$name;
2019-08-10T20:28:03.2734311Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 130, but expected a valid enum discriminant
2019-08-10T20:28:03.2734956Z 1087 | |         }
2019-08-10T20:28:03.2735285Z 1088 | |     }
2019-08-10T20:28:03.2735285Z 1088 | |     }
2019-08-10T20:28:03.2735655Z      | |_____- in this expansion of `define_queries_inner!` (#3)
2019-08-10T20:28:03.2736212Z     ::: src/librustc/query/mod.rs:32:1
2019-08-10T20:28:03.2736557Z      |
2019-08-10T20:28:03.2736557Z      |
2019-08-10T20:28:03.2736893Z 32   |     / rustc_queries! {
2019-08-10T20:28:03.2737242Z 33   |     |     Other {
2019-08-10T20:28:03.2738079Z 34   |     |         /// Records the type of every item.
2019-08-10T20:28:03.2738610Z 35   |     |         query type_of(key: DefId) -> Ty<'tcx> {
2019-08-10T20:28:03.2739289Z 1106 |     |     }
2019-08-10T20:28:03.2739613Z 1107 |     | }
2019-08-10T20:28:03.2740006Z      |     | -
2019-08-10T20:28:03.2740338Z      |     | |
2019-08-10T20:28:03.2740338Z      |     | |
2019-08-10T20:28:03.2740689Z      |     |_in this expansion of `rustc_query_append!` (#1)
2019-08-10T20:28:03.2741273Z      | 
2019-08-10T20:28:03.2741554Z     ::: src/librustc/ty/query/mod.rs:101:1
2019-08-10T20:28:03.2741806Z      |
2019-08-10T20:28:03.2741806Z      |
2019-08-10T20:28:03.2742166Z 101  | /     rustc_query_append! { [define_queries!][ <'tcx>
2019-08-10T20:28:03.2742494Z 102  | |         Other {
2019-08-10T20:28:03.2743071Z 103  | |             /// Runs analysis passes on the crate.
2019-08-10T20:28:03.2743525Z 104  | |             [eval_always] fn analysis: Analysis(CrateNum) -> Result<(), ErrorReported>,
2019-08-10T20:28:03.2744275Z 106  | |     ]}
2019-08-10T20:28:03.2744650Z      | |______- in this macro invocation (#1)
2019-08-10T20:28:03.2744938Z      |
2019-08-10T20:28:03.2744938Z      |
2019-08-10T20:28:03.2745455Z      = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-10T20:28:03.2830827Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.2830827Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.2831223Z    --> src/librustc/ty/query/plumbing.rs:618:5
2019-08-10T20:28:03.2831493Z     |
2019-08-10T20:28:03.2831903Z 618 | /     fn force_query<Q: QueryDescription<'tcx>>(self, key: Q::Key, span: Span, dep_node: DepNode) {
2019-08-10T20:28:03.2832278Z 619 | |         profq_msg!(
2019-08-10T20:28:03.2832609Z 620 | |             self,
2019-08-10T20:28:03.2832989Z 621 | |             ProfileQueriesMsg::QueryBegin(span.data(),
2019-08-10T20:28:03.2833282Z ...   |
2019-08-10T20:28:03.2833834Z 634 | |         self.force_query_with_job::<Q>(key, job, dep_node);
2019-08-10T20:28:03.2842174Z 635 | |     }
2019-08-10T20:28:03.2842615Z     | |_____^ type validation failed: encountered 132, but expected a valid enum discriminant
2019-08-10T20:28:03.2842868Z     |
2019-08-10T20:28:03.2843372Z     = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-10T20:28:03.2843802Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.2843802Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.2844128Z     --> src/librustc/ty/query/plumbing.rs:956:13
2019-08-10T20:28:03.2844723Z 693  | /     macro_rules! define_queries {
2019-08-10T20:28:03.2844723Z 693  | /     macro_rules! define_queries {
2019-08-10T20:28:03.2845100Z 694  | |         (<$tcx:tt> $($category:tt {
2019-08-10T20:28:03.2845522Z 695  | |             $($(#[$attr:meta])* [$($modifiers:tt)*] fn $name:ident: $node:ident($K:ty) -> $V:ty,)*
2019-08-10T20:28:03.2846102Z 696  | |         },)*) => {
2019-08-10T20:28:03.2846463Z 697  | /             define_queries_inner! { <$tcx>
2019-08-10T20:28:03.2846890Z 698  |                   $($( $(#[$attr])* category<$category> [$($modifiers)*] fn $name: $node($K) -> $V,)*)*
2019-08-10T20:28:03.2847612Z      | |_____________- in this macro invocation (#3)
2019-08-10T20:28:03.2848654Z 700  | |         }
2019-08-10T20:28:03.2849015Z 701  | |     }
2019-08-10T20:28:03.2849015Z 701  | |     }
2019-08-10T20:28:03.2849395Z      | |_____- in this expansion of `define_queries!` (#2)
2019-08-10T20:28:03.2849670Z 702  | 
2019-08-10T20:28:03.2850039Z 703  | /     macro_rules! define_queries_inner {
2019-08-10T20:28:03.2850377Z 704  | |         (<$tcx:tt>
2019-08-10T20:28:03.2850742Z 705  | |          $($(#[$attr:meta])* category<$category:tt>
2019-08-10T20:28:03.2851160Z 706  | |             [$($modifiers:tt)*] fn $name:ident: $node:ident($K:ty) -> $V:ty,)*) => {
2019-08-10T20:28:03.2851451Z ...    |
2019-08-10T20:28:03.2851836Z 956  | |                 const NAME: QueryName = QueryName::$name;
2019-08-10T20:28:03.2857028Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 132, but expected a valid enum discriminant
2019-08-10T20:28:03.2858461Z 1087 | |         }
2019-08-10T20:28:03.2858801Z 1088 | |     }
2019-08-10T20:28:03.2858801Z 1088 | |     }
2019-08-10T20:28:03.2859186Z      | |_____- in this expansion of `define_queries_inner!` (#3)
2019-08-10T20:28:03.2859739Z     ::: src/librustc/query/mod.rs:32:1
2019-08-10T20:28:03.2859998Z      |
2019-08-10T20:28:03.2859998Z      |
2019-08-10T20:28:03.2860339Z 32   |     / rustc_queries! {
2019-08-10T20:28:03.2860696Z 33   |     |     Other {
2019-08-10T20:28:03.2861119Z 34   |     |         /// Records the type of every item.
2019-08-10T20:28:03.2861529Z 35   |     |         query type_of(key: DefId) -> Ty<'tcx> {
2019-08-10T20:28:03.2862409Z 1106 |     |     }
2019-08-10T20:28:03.2862764Z 1107 |     | }
2019-08-10T20:28:03.2863138Z      |     | -
2019-08-10T20:28:03.2863486Z      |     | |
2019-08-10T20:28:03.2863486Z      |     | |
2019-08-10T20:28:03.2863871Z      |     |_in this expansion of `rustc_query_append!` (#1)
2019-08-10T20:28:03.2864523Z      | 
2019-08-10T20:28:03.2864832Z     ::: src/librustc/ty/query/mod.rs:101:1
2019-08-10T20:28:03.2865174Z      |
2019-08-10T20:28:03.2865174Z      |
2019-08-10T20:28:03.2865560Z 101  | /     rustc_query_append! { [define_queries!][ <'tcx>
2019-08-10T20:28:03.2865943Z 102  | |         Other {
2019-08-10T20:28:03.2866342Z 103  | |             /// Runs analysis passes on the crate.
2019-08-10T20:28:03.2866775Z 104  | |             [eval_always] fn analysis: Analysis(CrateNum) -> Result<(), ErrorReported>,
2019-08-10T20:28:03.2867519Z 106  | |     ]}
2019-08-10T20:28:03.2868603Z      | |______- in this macro invocation (#1)
2019-08-10T20:28:03.2868884Z      |
2019-08-10T20:28:03.2868884Z      |
2019-08-10T20:28:03.2869513Z      = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-10T20:28:03.2917356Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.2917356Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.2918076Z    --> src/librustc/ty/query/plumbing.rs:618:5
2019-08-10T20:28:03.2918438Z     |
2019-08-10T20:28:03.2928818Z 618 | /     fn force_query<Q: QueryDescription<'tcx>>(self, key: Q::Key, span: Span, dep_node: DepNode) {
2019-08-10T20:28:03.2929409Z 619 | |         profq_msg!(
2019-08-10T20:28:03.2929774Z 620 | |             self,
2019-08-10T20:28:03.2930172Z 621 | |             ProfileQueriesMsg::QueryBegin(span.data(),
2019-08-10T20:28:03.2930453Z ...   |
2019-08-10T20:28:03.2930818Z 634 | |         self.force_query_with_job::<Q>(key, job, dep_node);
2019-08-10T20:28:03.2931153Z 635 | |     }
2019-08-10T20:28:03.2931822Z     | |_____^ type validation failed: encountered 133, but expected a valid enum discriminant
2019-08-10T20:28:03.2932076Z     |
2019-08-10T20:28:03.2932592Z     = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-10T20:28:03.2933056Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.2933056Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.2933587Z     --> src/librustc/ty/query/plumbing.rs:956:13
2019-08-10T20:28:03.2934225Z 693  | /     macro_rules! define_queries {
2019-08-10T20:28:03.2934225Z 693  | /     macro_rules! define_queries {
2019-08-10T20:28:03.2934579Z 694  | |         (<$tcx:tt> $($category:tt {
2019-08-10T20:28:03.2935024Z 695  | |             $($(#[$attr:meta])* [$($modifiers:tt)*] fn $name:ident: $node:ident($K:ty) -> $V:ty,)*
2019-08-10T20:28:03.2935370Z 696  | |         },)*) => {
2019-08-10T20:28:03.2935725Z 697  | /             define_queries_inner! { <$tcx>
2019-08-10T20:28:03.2936150Z 698  |                   $($( $(#[$attr])* category<$category> [$($modifiers)*] fn $name: $node($K) -> $V,)*)*
2019-08-10T20:28:03.2936921Z      | |_____________- in this macro invocation (#3)
2019-08-10T20:28:03.2937444Z 700  | |         }
2019-08-10T20:28:03.2942700Z 701  | |     }
2019-08-10T20:28:03.2942700Z 701  | |     }
2019-08-10T20:28:03.2943147Z      | |_____- in this expansion of `define_queries!` (#2)
2019-08-10T20:28:03.2943423Z 702  | 
2019-08-10T20:28:03.2944048Z 703  | /     macro_rules! define_queries_inner {
2019-08-10T20:28:03.2944450Z 704  | |         (<$tcx:tt>
2019-08-10T20:28:03.2944822Z 705  | |          $($(#[$attr:meta])* category<$category:tt>
2019-08-10T20:28:03.2945259Z 706  | |             [$($modifiers:tt)*] fn $name:ident: $node:ident($K:ty) -> $V:ty,)*) => {
2019-08-10T20:28:03.2945552Z ...    |
2019-08-10T20:28:03.2945940Z 956  | |                 const NAME: QueryName = QueryName::$name;
2019-08-10T20:28:03.2946429Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 133, but expected a valid enum discriminant
2019-08-10T20:28:03.2947408Z 1087 | |         }
2019-08-10T20:28:03.2947958Z 1088 | |     }
2019-08-10T20:28:03.2947958Z 1088 | |     }
2019-08-10T20:28:03.2948447Z      | |_____- in this expansion of `define_queries_inner!` (#3)
2019-08-10T20:28:03.2949011Z     ::: src/librustc/query/mod.rs:32:1
2019-08-10T20:28:03.2949253Z      |
2019-08-10T20:28:03.2949253Z      |
2019-08-10T20:28:03.2949587Z 32   |     / rustc_queries! {
2019-08-10T20:28:03.2949955Z 33   |     |     Other {
2019-08-10T20:28:03.2950324Z 34   |     |         /// Records the type of every item.
2019-08-10T20:28:03.2950708Z 35   |     |         query type_of(key: DefId) -> Ty<'tcx> {
2019-08-10T20:28:03.2951332Z 1106 |     |     }
2019-08-10T20:28:03.2951661Z 1107 |     | }
2019-08-10T20:28:03.2952004Z      |     | -
2019-08-10T20:28:03.2952332Z      |     | |
2019-08-10T20:28:03.2952332Z      |     | |
2019-08-10T20:28:03.2952699Z      |     |_in this expansion of `rustc_query_append!` (#1)
2019-08-10T20:28:03.2953453Z      | 
2019-08-10T20:28:03.2953877Z     ::: src/librustc/ty/query/mod.rs:101:1
2019-08-10T20:28:03.2954122Z      |
2019-08-10T20:28:03.2954122Z      |
2019-08-10T20:28:03.2954480Z 101  | /     rustc_query_append! { [define_queries!][ <'tcx>
2019-08-10T20:28:03.2954826Z 102  | |         Other {
2019-08-10T20:28:03.2955254Z 103  | |             /// Runs analysis passes on the crate.
2019-08-10T20:28:03.2955693Z 104  | |             [eval_always] fn analysis: Analysis(CrateNum) -> Result<(), ErrorReported>,
2019-08-10T20:28:03.2956454Z 106  | |     ]}
2019-08-10T20:28:03.2956846Z      | |______- in this macro invocation (#1)
2019-08-10T20:28:03.2957122Z      |
2019-08-10T20:28:03.2957122Z      |
2019-08-10T20:28:03.2957961Z      = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-10T20:28:03.2984566Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.2984566Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.2984950Z    --> src/librustc/ty/query/plumbing.rs:618:5
2019-08-10T20:28:03.2985232Z     |
2019-08-10T20:28:03.2985651Z 618 | /     fn force_query<Q: QueryDescription<'tcx>>(self, key: Q::Key, span: Span, dep_node: DepNode) {
2019-08-10T20:28:03.2986013Z 619 | |         profq_msg!(
2019-08-10T20:28:03.2995734Z 620 | |             self,
2019-08-10T20:28:03.2996326Z 621 | |             ProfileQueriesMsg::QueryBegin(span.data(),
2019-08-10T20:28:03.2996621Z ...   |
2019-08-10T20:28:03.2996990Z 634 | |         self.force_query_with_job::<Q>(key, job, dep_node);
2019-08-10T20:28:03.2997340Z 635 | |     }
2019-08-10T20:28:03.2997736Z     | |_____^ type validation failed: encountered 134, but expected a valid enum discriminant
2019-08-10T20:28:03.2998482Z     |
2019-08-10T20:28:03.2998997Z     = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-10T20:28:03.2999474Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.2999474Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.2999778Z     --> src/librustc/ty/query/plumbing.rs:956:13
2019-08-10T20:28:03.3000535Z 693  | /     macro_rules! define_queries {
2019-08-10T20:28:03.3000535Z 693  | /     macro_rules! define_queries {
2019-08-10T20:28:03.3000946Z 694  | |         (<$tcx:tt> $($category:tt {
2019-08-10T20:28:03.3001464Z 695  | |             $($(#[$attr:meta])* [$($modifiers:tt)*] fn $name:ident: $node:ident($K:ty) -> $V:ty,)*
2019-08-10T20:28:03.3001822Z 696  | |         },)*) => {
2019-08-10T20:28:03.3002176Z 697  | /             define_queries_inner! { <$tcx>
2019-08-10T20:28:03.3002609Z 698  |                   $($( $(#[$attr])* category<$category> [$($modifiers)*] fn $name: $node($K) -> $V,)*)*
2019-08-10T20:28:03.3003319Z      | |_____________- in this macro invocation (#3)
2019-08-10T20:28:03.3003646Z 700  | |         }
2019-08-10T20:28:03.3003987Z 701  | |     }
2019-08-10T20:28:03.3003987Z 701  | |     }
2019-08-10T20:28:03.3004480Z      | |_____- in this expansion of `define_queries!` (#2)
2019-08-10T20:28:03.3004753Z 702  | 
2019-08-10T20:28:03.3005129Z 703  | /     macro_rules! define_queries_inner {
2019-08-10T20:28:03.3005463Z 704  | |         (<$tcx:tt>
2019-08-10T20:28:03.3005850Z 705  | |          $($(#[$attr:meta])* category<$category:tt>
2019-08-10T20:28:03.3006256Z 706  | |             [$($modifiers:tt)*] fn $name:ident: $node:ident($K:ty) -> $V:ty,)*) => {
2019-08-10T20:28:03.3006549Z ...    |
2019-08-10T20:28:03.3006936Z 956  | |                 const NAME: QueryName = QueryName::$name;
2019-08-10T20:28:03.3007429Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 134, but expected a valid enum discriminant
2019-08-10T20:28:03.3008705Z 1087 | |         }
2019-08-10T20:28:03.3009052Z 1088 | |     }
2019-08-10T20:28:03.3009052Z 1088 | |     }
2019-08-10T20:28:03.3009420Z      | |_____- in this expansion of `define_queries_inner!` (#3)
2019-08-10T20:28:03.3009985Z     ::: src/librustc/query/mod.rs:32:1
2019-08-10T20:28:03.3010222Z      |
2019-08-10T20:28:03.3010222Z      |
2019-08-10T20:28:03.3010679Z 32   |     / rustc_queries! {
2019-08-10T20:28:03.3011089Z 33   |     |     Other {
2019-08-10T20:28:03.3011462Z 34   |     |         /// Records the type of every item.
2019-08-10T20:28:03.3011853Z 35   |     |         query type_of(key: DefId) -> Ty<'tcx> {
2019-08-10T20:28:03.3012468Z 1106 |     |     }
2019-08-10T20:28:03.3012806Z 1107 |     | }
2019-08-10T20:28:03.3013131Z      |     | -
2019-08-10T20:28:03.3013677Z      |     | |
2019-08-10T20:28:03.3013677Z      |     | |
2019-08-10T20:28:03.3014068Z      |     |_in this expansion of `rustc_query_append!` (#1)
2019-08-10T20:28:03.3014620Z      | 
2019-08-10T20:28:03.3014922Z     ::: src/librustc/ty/query/mod.rs:101:1
2019-08-10T20:28:03.3015317Z      |
2019-08-10T20:28:03.3015317Z      |
2019-08-10T20:28:03.3015676Z 101  | /     rustc_query_append! { [define_queries!][ <'tcx>
2019-08-10T20:28:03.3016033Z 102  | |         Other {
2019-08-10T20:28:03.3016450Z 103  | |             /// Runs analysis passes on the crate.
2019-08-10T20:28:03.3017034Z 104  | |             [eval_always] fn analysis: Analysis(CrateNum) -> Result<(), ErrorReported>,
2019-08-10T20:28:03.3018160Z 106  | |     ]}
2019-08-10T20:28:03.3018538Z      | |______- in this macro invocation (#1)
2019-08-10T20:28:03.3018786Z      |
2019-08-10T20:28:03.3018786Z      |
2019-08-10T20:28:03.3019305Z      = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-08-10T20:28:03.3039656Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.3039656Z error[E0080]: it is undefined behavior to use this value
2019-08-10T20:28:03.3040051Z    --> src/librustc/ty/query/plumbing.rs:618:5
2019-08-10T20:28:03.3040308Z     |
2019-08-10T20:28:03.3040715Z 618 | /     fn force_query<Q: QueryDescription<'tcx>>(self, key: Q::Key, span: Span, dep_node: DepNode) {
2019-08-10T20:28:03.3041147Z 619 | |         profq_msg!(
2019-08-10T20:28:03.3041643Z 620 | |             self,
2019-08-10T20:28:03.3042098Z 621 | |             ProfileQueriesMsg::QueryBegin(span.data(),
2019-08-10T20:28:03.3042372Z ...   |
2019-08-10T20:28:03.3042734Z 634 | |         self.force_query_with_job::<Q>(key, job, dep_node);
2019-08-10T20:28:03.3052327Z 635 | |     }
2019-08-10T20:28:03.3052777Z     | |_____^ type validation failed: encountered 135, but expected a valid enum discriminant
2019-08-10T20:28:03.3053053Z     |
2019-08-10T20:28:03.3053774Z     = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
