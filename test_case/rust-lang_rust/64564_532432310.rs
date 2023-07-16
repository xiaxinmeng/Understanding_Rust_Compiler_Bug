plain
2019-09-17T22:13:37.4169490Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-17T22:13:37.4411747Z ##[command]git config gc.auto 0
2019-09-17T22:13:37.4473175Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-17T22:13:37.4528625Z ##[command]git config --get-all http.proxy
2019-09-17T22:13:37.4670199Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64564/merge:refs/remotes/pull/64564/merge
---
2019-09-17T22:49:23.2172133Z    Compiling arena v0.0.0 (/checkout/src/libarena)
2019-09-17T22:49:33.1020412Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-09-17T22:49:41.5974811Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-09-17T22:50:02.2506228Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-09-17T22:50:26.7031608Z error[E0520]: `cache_on_disk` specializes an item from a parent `impl`, but that item is not marked `default`
2019-09-17T22:50:26.7033157Z     --> src/librustc/query/mod.rs:31:1
2019-09-17T22:50:26.7034090Z      |
2019-09-17T22:50:26.7034714Z 31   |   rustc_queries! {
2019-09-17T22:50:26.7035323Z      |  _^
2019-09-17T22:50:26.7036488Z      | |
2019-09-17T22:50:26.7037490Z 32   | |     Other {
2019-09-17T22:50:26.7038283Z 33   | |         /// Records the type of every item.
2019-09-17T22:50:26.7038283Z 33   | |         /// Records the type of every item.
2019-09-17T22:50:26.7038973Z 34   | |         query type_of(key: DefId) -> Ty<'tcx> {
2019-09-17T22:50:26.7040527Z 1136 | |     }
2019-09-17T22:50:26.7041171Z 1137 | | }
2019-09-17T22:50:26.7041778Z      | | ^
2019-09-17T22:50:26.7042535Z      | |_|
2019-09-17T22:50:26.7042535Z      | |_|
2019-09-17T22:50:26.7043468Z      | |_cannot specialize default item `cache_on_disk`
2019-09-17T22:50:26.7044704Z      | 
2019-09-17T22:50:26.7045272Z     ::: /checkout/src/librustc_macros/src/lib.rs:22:1
2019-09-17T22:50:26.7045957Z      |
2019-09-17T22:50:26.7045957Z      |
2019-09-17T22:50:26.7046881Z 22   | / pub fn symbols(input: TokenStream) -> TokenStream {
2019-09-17T22:50:26.7048261Z 23   | |     symbols::symbols(input)
2019-09-17T22:50:26.7048959Z 24   | | }
2019-09-17T22:50:26.7050180Z      | |_- in this expansion of `rustc_queries!`
2019-09-17T22:50:26.7053032Z     ::: src/librustc/ty/query/config.rs:67:1
2019-09-17T22:50:26.7054821Z      |
2019-09-17T22:50:26.7054821Z      |
2019-09-17T22:50:26.7055481Z 67   | / impl<'tcx, M: QueryAccessors<'tcx, Key = DefId>> QueryDescription<'tcx> for M {
2019-09-17T22:50:26.7056039Z 68   | |     default fn describe(tcx: TyCtxt<'_>, def_id: DefId) -> Cow<'static, str> {
2019-09-17T22:50:26.7056512Z 69   | |         if !tcx.sess.verbose() {
2019-09-17T22:50:26.7057943Z 70   | |             format!("processing `{}`", tcx.def_path_str(def_id)).into()
2019-09-17T22:50:26.7059738Z 75   | |     }
2019-09-17T22:50:26.7060777Z 76   | | }
2019-09-17T22:50:26.7060777Z 76   | | }
2019-09-17T22:50:26.7061568Z      | |_- parent `impl` is here
2019-09-17T22:50:26.7090710Z      |
2019-09-17T22:50:26.7091419Z      = note: to specialize, `cache_on_disk` in the parent `impl` must be marked `default`
2019-09-17T22:50:26.7091751Z 
2019-09-17T22:50:26.7908505Z error[E0520]: `try_load_from_disk` specializes an item from a parent `impl`, but that item is not marked `default`
2019-09-17T22:50:26.7909942Z     --> src/librustc/query/mod.rs:31:1
2019-09-17T22:50:26.7910551Z      |
2019-09-17T22:50:26.7911153Z 31   |   rustc_queries! {
2019-09-17T22:50:26.7911592Z      |  _^
2019-09-17T22:50:26.7912890Z      | |
2019-09-17T22:50:26.7913366Z 32   | |     Other {
2019-09-17T22:50:26.7913873Z 33   | |         /// Records the type of every item.
2019-09-17T22:50:26.7913873Z 33   | |         /// Records the type of every item.
2019-09-17T22:50:26.7914424Z 34   | |         query type_of(key: DefId) -> Ty<'tcx> {
2019-09-17T22:50:26.7915451Z 1136 | |     }
2019-09-17T22:50:26.7915892Z 1137 | | }
2019-09-17T22:50:26.7916480Z      | | ^
2019-09-17T22:50:26.7917629Z      | |_|
2019-09-17T22:50:26.7917629Z      | |_|
2019-09-17T22:50:26.7918280Z      | |_cannot specialize default item `try_load_from_disk`
2019-09-17T22:50:26.7919207Z      | 
2019-09-17T22:50:26.7919675Z     ::: /checkout/src/librustc_macros/src/lib.rs:22:1
2019-09-17T22:50:26.7920086Z      |
2019-09-17T22:50:26.7920086Z      |
2019-09-17T22:50:26.7920597Z 22   | / pub fn symbols(input: TokenStream) -> TokenStream {
2019-09-17T22:50:26.7921254Z 23   | |     symbols::symbols(input)
2019-09-17T22:50:26.7921833Z 24   | | }
2019-09-17T22:50:26.7922309Z      | |_- in this expansion of `rustc_queries!`
2019-09-17T22:50:26.7923228Z     ::: src/librustc/ty/query/config.rs:67:1
2019-09-17T22:50:26.7923644Z      |
2019-09-17T22:50:26.7923644Z      |
2019-09-17T22:50:26.7924162Z 67   | / impl<'tcx, M: QueryAccessors<'tcx, Key = DefId>> QueryDescription<'tcx> for M {
2019-09-17T22:50:26.7924844Z 68   | |     default fn describe(tcx: TyCtxt<'_>, def_id: DefId) -> Cow<'static, str> {
2019-09-17T22:50:26.7925387Z 69   | |         if !tcx.sess.verbose() {
2019-09-17T22:50:26.7925975Z 70   | |             format!("processing `{}`", tcx.def_path_str(def_id)).into()
2019-09-17T22:50:26.7926878Z 75   | |     }
2019-09-17T22:50:26.7928220Z 76   | | }
2019-09-17T22:50:26.7928220Z 76   | | }
2019-09-17T22:50:26.7928815Z      | |_- parent `impl` is here
2019-09-17T22:50:26.7929331Z      |
2019-09-17T22:50:26.7930273Z      = note: to specialize, `try_load_from_disk` in the parent `impl` must be marked `default`
2019-09-17T22:50:46.1237470Z error: aborting due to 2 previous errors
2019-09-17T22:50:46.1241886Z 
2019-09-17T22:50:46.1248780Z For more information about this error, try `rustc --explain E0520`.
2019-09-17T22:50:46.4325251Z error: Could not compile `rustc`.
---
2019-09-17T22:51:50.7086399Z == clock drift check ==
2019-09-17T22:51:50.7101861Z   local time: Tue Sep 17 22:51:50 UTC 2019
2019-09-17T22:51:50.8619860Z   network time: Tue, 17 Sep 2019 22:51:50 GMT
2019-09-17T22:51:50.8620211Z == end clock drift check ==
2019-09-17T22:51:51.9441780Z ##[error]Bash exited with code '1'.
2019-09-17T22:51:51.9493040Z ##[section]Starting: Checkout
2019-09-17T22:51:51.9495557Z ==============================================================================
2019-09-17T22:51:51.9495607Z Task         : Get sources
2019-09-17T22:51:51.9495668Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
