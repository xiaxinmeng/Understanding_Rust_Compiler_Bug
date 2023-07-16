plain
2020-01-04T03:47:31.8170212Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-04T03:47:31.8356236Z ##[command]git config gc.auto 0
2020-01-04T03:47:31.8413609Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-04T03:47:31.8466925Z ##[command]git config --get-all http.proxy
2020-01-04T03:47:31.8580957Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67854/merge:refs/remotes/pull/67854/merge
---
2020-01-04T04:10:32.8604645Z    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2020-01-04T04:10:35.8953621Z    Compiling arena v0.0.0 (/checkout/src/libarena)
2020-01-04T04:10:38.4506693Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2020-01-04T04:10:48.1918314Z    Compiling rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-01-04T04:10:48.4305959Z error: Prefer FxHashSet over HashSet, it has better performance
2020-01-04T04:10:48.4306843Z     --> src/librustc_span/symbol.rs:347:9
2020-01-04T04:10:48.4307301Z      |
2020-01-04T04:10:48.4307821Z 23   | / symbols! {
2020-01-04T04:10:48.4308418Z 24   | |     // After modifying this list adjust `is_special`, `is_used_keyword`/`is_unused_keyword`,
2020-01-04T04:10:48.4309222Z 25   | |     // this should be rarely necessary though if the keywords are kept in alphabetic order.
2020-01-04T04:10:48.4309750Z 26   | |     Keywords {
2020-01-04T04:10:48.4310673Z 347  | |         HashSet,
2020-01-04T04:10:48.4311130Z      | |         ^^^^^^^
2020-01-04T04:10:48.4311568Z ...    |
2020-01-04T04:10:48.4312025Z 793  | |     }
2020-01-04T04:10:48.4312025Z 793  | |     }
2020-01-04T04:10:48.4312504Z 794  | | }
2020-01-04T04:10:48.4313008Z      | |_- in this expansion of `symbols!`
2020-01-04T04:10:48.4313400Z ...
2020-01-04T04:10:48.4313860Z 1054 |       symbols!();
2020-01-04T04:10:48.4314352Z      |       ----------- in this macro invocation
2020-01-04T04:10:48.4314754Z      |
2020-01-04T04:10:48.4315236Z      = note: `-D rustc::default-hash-types` implied by `-D warnings`
2020-01-04T04:10:48.4315705Z      = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary
2020-01-04T04:10:48.4315928Z 
2020-01-04T04:10:48.4479456Z error: Prefer FxHashMap over HashMap, it has better performance
2020-01-04T04:10:48.4480506Z     --> src/librustc_span/symbol.rs:348:9
2020-01-04T04:10:48.4480917Z      |
2020-01-04T04:10:48.4481324Z 23   | / symbols! {
2020-01-04T04:10:48.4481791Z 24   | |     // After modifying this list adjust `is_special`, `is_used_keyword`/`is_unused_keyword`,
2020-01-04T04:10:48.4483226Z 25   | |     // this should be rarely necessary though if the keywords are kept in alphabetic order.
2020-01-04T04:10:48.4483860Z 26   | |     Keywords {
2020-01-04T04:10:48.4485125Z 348  | |         HashMap,
2020-01-04T04:10:48.4485613Z      | |         ^^^^^^^
2020-01-04T04:10:48.4486499Z ...    |
2020-01-04T04:10:48.4487404Z 793  | |     }
---
2020-01-04T04:10:49.8436525Z   local time: Sat Jan  4 04:10:49 UTC 2020
2020-01-04T04:10:49.8436561Z   network time: Sat, 04 Jan 2020 04:10:49 GMT
2020-01-04T04:10:49.8436635Z == end clock drift check ==
2020-01-04T04:10:50.7100571Z 
2020-01-04T04:10:50.7196582Z ##[error]Bash exited with code '1'.
2020-01-04T04:10:50.7230733Z ##[section]Starting: Checkout
2020-01-04T04:10:50.7232165Z ==============================================================================
2020-01-04T04:10:50.7232225Z Task         : Get sources
2020-01-04T04:10:50.7232262Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
