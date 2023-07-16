plain
2020-01-31T07:08:22.4499390Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-01-31T07:08:24.3231932Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-01-31T07:08:46.3196047Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-01-31T07:09:01.1971460Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-01-31T07:09:16.9765290Z error[E0412]: cannot find type `PhantomData` in this scope
2020-01-31T07:09:16.9766448Z   --> src/librustc/ty/query/job.rs:85:12
2020-01-31T07:09:16.9766929Z    |
2020-01-31T07:09:16.9767497Z 85 |     dummy: PhantomData<QueryLatch<'tcx>>,
2020-01-31T07:09:16.9769089Z    |
2020-01-31T07:09:16.9769703Z help: possible candidates are found in other modules, you can import them into scope
2020-01-31T07:09:16.9770184Z    |
2020-01-31T07:09:16.9770696Z 1  | use core::marker::PhantomData;
2020-01-31T07:09:16.9770696Z 1  | use core::marker::PhantomData;
2020-01-31T07:09:16.9771153Z    |
2020-01-31T07:09:16.9771937Z 1  | use std::marker::PhantomData;
2020-01-31T07:09:16.9772418Z    |
2020-01-31T07:09:16.9775936Z 
2020-01-31T07:09:16.9987490Z error[E0425]: cannot find value `PhantomData` in this scope
2020-01-31T07:09:16.9989232Z   --> src/librustc/ty/query/job.rs:97:20
2020-01-31T07:09:16.9989746Z    |
2020-01-31T07:09:16.9990300Z 97 |             dummy: PhantomData,
2020-01-31T07:09:16.9991412Z    |
2020-01-31T07:09:16.9991989Z help: possible candidates are found in other modules, you can import them into scope
2020-01-31T07:09:16.9992443Z    |
2020-01-31T07:09:16.9992984Z 1  | use core::marker::PhantomData;
---
2020-01-31T07:09:46.2145077Z   local time: Fri Jan 31 07:09:46 UTC 2020
2020-01-31T07:09:46.9226763Z   network time: Fri, 31 Jan 2020 07:09:46 GMT
2020-01-31T07:09:46.9227114Z == end clock drift check ==
2020-01-31T07:09:47.2857182Z 
2020-01-31T07:09:47.2965292Z ##[error]Bash exited with code '1'.
2020-01-31T07:09:47.3038499Z ##[section]Starting: Checkout rust-lang/rust@try to s
2020-01-31T07:09:47.3040808Z ==============================================================================
2020-01-31T07:09:47.3040923Z Task         : Get sources
2020-01-31T07:09:47.3041019Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
