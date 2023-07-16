plain
2020-03-16T10:46:19.3153616Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-16T10:46:22.3793866Z error: unnecessary parentheses around type
2020-03-16T10:46:22.3795511Z     --> src/librustc/query/mod.rs:78:20
2020-03-16T10:46:22.3796198Z      |
2020-03-16T10:46:22.3796946Z 38   | / rustc_queries! {
2020-03-16T10:46:22.3797855Z 39   | |     Other {
2020-03-16T10:46:22.3799148Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-03-16T10:46:22.3800576Z 41   | |             desc { "trigger a delay span bug" }
2020-03-16T10:46:22.3801424Z ...    |
2020-03-16T10:46:22.3802461Z 78   | |             storage(caches::LocalDenseDefIdCacheSelector<&'tcx HirOwner<'tcx>>)
2020-03-16T10:46:22.3805077Z ...    |
2020-03-16T10:46:22.3805838Z 1257 | |     }
2020-03-16T10:46:22.3806692Z 1258 | | }
2020-03-16T10:46:22.3806692Z 1258 | | }
2020-03-16T10:46:22.3807639Z      | |_- in this expansion of `rustc_query_append!`
2020-03-16T10:46:22.3809013Z     ::: src/librustc/ty/query/mod.rs:110:1
2020-03-16T10:46:22.3809637Z      |
2020-03-16T10:46:22.3809637Z      |
2020-03-16T10:46:22.3810404Z 110  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-03-16T10:46:22.3812273Z      |
2020-03-16T10:46:22.3813031Z      = note: `-D unused-parens` implied by `-D warnings`
2020-03-16T10:46:22.3813412Z 
2020-03-16T10:46:22.3847681Z error: unnecessary parentheses around type
2020-03-16T10:46:22.3847681Z error: unnecessary parentheses around type
2020-03-16T10:46:22.3848561Z     --> src/librustc/query/mod.rs:87:20
2020-03-16T10:46:22.3849169Z      |
2020-03-16T10:46:22.3849932Z 38   | / rustc_queries! {
2020-03-16T10:46:22.3850831Z 39   | |     Other {
2020-03-16T10:46:22.3851914Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-03-16T10:46:22.3853094Z 41   | |             desc { "trigger a delay span bug" }
2020-03-16T10:46:22.3853892Z ...    |
2020-03-16T10:46:22.3854962Z 87   | |             storage(caches::LocalDenseDefIdCacheSelector<&'tcx HirOwnerItems<'tcx>>)
2020-03-16T10:46:22.3857875Z ...    |
2020-03-16T10:46:22.3858650Z 1257 | |     }
2020-03-16T10:46:22.3859497Z 1258 | | }
2020-03-16T10:46:22.3859497Z 1258 | | }
2020-03-16T10:46:22.3860521Z      | |_- in this expansion of `rustc_query_append!`
2020-03-16T10:46:22.3861979Z     ::: src/librustc/ty/query/mod.rs:110:1
2020-03-16T10:46:22.3862587Z      |
2020-03-16T10:46:22.3862587Z      |
2020-03-16T10:46:22.3863368Z 110  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-03-16T10:46:22.3864940Z 
2020-03-16T10:46:36.2374925Z error: aborting due to 2 previous errors
2020-03-16T10:46:36.2375218Z 
2020-03-16T10:46:42.4615748Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
2020-03-16T10:47:07.3362261Z   local time: Mon Mar 16 10:47:07 UTC 2020
2020-03-16T10:47:07.9478792Z   network time: Mon, 16 Mar 2020 10:47:07 GMT
2020-03-16T10:47:07.9479250Z == end clock drift check ==
2020-03-16T10:47:08.5047517Z 
2020-03-16T10:47:08.5147377Z ##[error]Bash exited with code '1'.
2020-03-16T10:47:08.5238686Z ##[section]Starting: Checkout rust-lang/rust@try to s
2020-03-16T10:47:08.5244791Z ==============================================================================
2020-03-16T10:47:08.5245212Z Task         : Get sources
2020-03-16T10:47:08.5245649Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
