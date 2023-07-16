plain
2020-01-05T20:39:07.7352867Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-05T20:39:07.7370267Z ##[command]git config gc.auto 0
2020-01-05T20:39:07.7375347Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-05T20:39:07.7379853Z ##[command]git config --get-all http.proxy
2020-01-05T20:39:07.7385460Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67854/merge:refs/remotes/pull/67854/merge
---
2020-01-05T21:09:43.8016330Z    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2020-01-05T21:09:48.2014841Z    Compiling arena v0.0.0 (/checkout/src/libarena)
2020-01-05T21:09:50.9994892Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2020-01-05T21:10:04.3363773Z    Compiling rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-01-05T21:10:04.6663005Z error: Prefer FxHashSet over HashSet, it has better performance
2020-01-05T21:10:04.6663480Z     --> src/librustc_span/symbol.rs:350:9
2020-01-05T21:10:04.6663730Z      |
2020-01-05T21:10:04.6664029Z 26   | / symbols! {
2020-01-05T21:10:04.6664488Z 27   | |     // After modifying this list adjust `is_special`, `is_used_keyword`/`is_unused_keyword`,
2020-01-05T21:10:04.6664906Z 28   | |     // this should be rarely necessary though if the keywords are kept in alphabetic order.
2020-01-05T21:10:04.6665377Z 29   | |     Keywords {
2020-01-05T21:10:04.6666200Z 350  | |         HashSet,
2020-01-05T21:10:04.6666438Z      | |         ^^^^^^^
2020-01-05T21:10:04.6666635Z ...    |
2020-01-05T21:10:04.6666887Z 796  | |     }
2020-01-05T21:10:04.6666887Z 796  | |     }
2020-01-05T21:10:04.6667138Z 797  | | }
2020-01-05T21:10:04.6667393Z      | |_- in this expansion of `symbols!`
2020-01-05T21:10:04.6667583Z ...
2020-01-05T21:10:04.6667804Z 1057 |       symbols!();
2020-01-05T21:10:04.6668072Z      |       ----------- in this macro invocation
2020-01-05T21:10:04.6668277Z      |
2020-01-05T21:10:04.6668516Z      = note: `-D rustc::default-hash-types` implied by `-D warnings`
2020-01-05T21:10:04.6668763Z      = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary
2020-01-05T21:10:04.6668818Z 
2020-01-05T21:10:04.6908099Z error: Prefer FxHashMap over HashMap, it has better performance
2020-01-05T21:10:04.6908370Z     --> src/librustc_span/symbol.rs:351:9
2020-01-05T21:10:04.6908585Z      |
2020-01-05T21:10:04.6908820Z 26   | / symbols! {
2020-01-05T21:10:04.6909117Z 27   | |     // After modifying this list adjust `is_special`, `is_used_keyword`/`is_unused_keyword`,
2020-01-05T21:10:04.6909441Z 28   | |     // this should be rarely necessary though if the keywords are kept in alphabetic order.
2020-01-05T21:10:04.6909683Z 29   | |     Keywords {
2020-01-05T21:10:04.6910502Z 351  | |         HashMap,
2020-01-05T21:10:04.6910737Z      | |         ^^^^^^^
2020-01-05T21:10:04.6910951Z ...    |
2020-01-05T21:10:04.6911688Z 796  | |     }
---
2020-01-05T21:10:05.5752583Z   local time: Sun Jan  5 21:10:05 UTC 2020
2020-01-05T21:10:05.8332762Z   network time: Sun, 05 Jan 2020 21:10:05 GMT
2020-01-05T21:10:05.8333427Z == end clock drift check ==
2020-01-05T21:10:07.3582496Z 
2020-01-05T21:10:07.3690670Z ##[error]Bash exited with code '1'.
2020-01-05T21:10:07.3735558Z ##[section]Starting: Checkout
2020-01-05T21:10:07.3737730Z ==============================================================================
2020-01-05T21:10:07.3737813Z Task         : Get sources
2020-01-05T21:10:07.3737849Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
