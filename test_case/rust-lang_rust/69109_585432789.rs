plain
2020-02-12T21:37:14.3288970Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-02-12T21:37:15.9446050Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-02-12T21:37:29.5527509Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-02-12T21:37:34.8686716Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-02-12T21:37:44.6964828Z error[E0609]: no field `cache_hits` on type `&mut ty::query::plumbing::QueryCache<'_, Q>`
2020-02-12T21:37:44.6965979Z    --> src/librustc/ty/query/plumbing.rs:396:36
2020-02-12T21:37:44.6966480Z     |
2020-02-12T21:37:44.6967042Z 396 |         let cache_hits = &mut lock.cache_hits;
2020-02-12T21:37:44.6968181Z     |
2020-02-12T21:37:44.6968181Z     |
2020-02-12T21:37:44.6968732Z     = note: available fields are: `results`, `active`, `jobs`
2020-02-12T21:37:48.1299151Z error: aborting due to previous error
2020-02-12T21:37:48.1300019Z 
2020-02-12T21:37:48.1300599Z For more information about this error, try `rustc --explain E0609`.
2020-02-12T21:37:48.1547843Z error: could not compile `rustc`.
---
2020-02-12T21:38:02.3351008Z   local time: Wed Feb 12 21:38:02 UTC 2020
2020-02-12T21:38:02.8944766Z   network time: Wed, 12 Feb 2020 21:38:02 GMT
2020-02-12T21:38:02.8957564Z == end clock drift check ==
2020-02-12T21:38:03.3339510Z 
2020-02-12T21:38:03.3421248Z ##[error]Bash exited with code '1'.
2020-02-12T21:38:03.3455092Z ##[section]Starting: Checkout rust-lang/rust@try to s
2020-02-12T21:38:03.3456821Z ==============================================================================
2020-02-12T21:38:03.3456909Z Task         : Get sources
2020-02-12T21:38:03.3457007Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
