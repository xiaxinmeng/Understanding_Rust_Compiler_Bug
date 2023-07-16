plain
2020-01-13T14:41:11.2016488Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-13T14:41:11.2024338Z ##[command]git config gc.auto 0
2020-01-13T14:41:11.2026002Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-13T14:41:11.2027573Z ##[command]git config --get-all http.proxy
2020-01-13T14:41:11.2029787Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67940/merge:refs/remotes/pull/67940/merge
---
2020-01-13T16:10:02.3290534Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/rustc_mir/transform/fn.mir_built.html"
2020-01-13T16:10:02.3293264Z 
2020-01-13T16:10:02.3294104Z     ┌── mir/construction.md:14:42 ───
2020-01-13T16:10:02.3294584Z     │
2020-01-13T16:10:02.3295107Z  14 │ The lowering is triggered by calling the [`mir_built`] query.
2020-01-13T16:10:02.3296505Z     │
2020-01-13T16:10:02.3296768Z 
2020-01-13T16:10:02.3297486Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/rustc_mir/hair/index.html"
2020-01-13T16:10:02.3297746Z 
2020-01-13T16:10:02.3297746Z 
2020-01-13T16:10:02.3298807Z     ┌── mir/construction.md:16:36 ───
2020-01-13T16:10:02.3299346Z     │
2020-01-13T16:10:02.3299815Z  16 │ between [HIR] and [MIR] called the [HAIR] that is only used during the lowering.
2020-01-13T16:10:02.3300785Z     │
2020-01-13T16:10:02.3300939Z 
2020-01-13T16:10:02.3301575Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/rustc_mir/hair/index.html"
2020-01-13T16:10:02.3301746Z 
---
2020-01-13T16:10:02.3307272Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/rustc_mir/hair/index.html"
2020-01-13T16:10:02.3307458Z 
2020-01-13T16:10:02.3307797Z     ┌── mir/construction.md:22:5 ───
2020-01-13T16:10:02.3308117Z     │
2020-01-13T16:10:02.3308535Z  22 │ The [HAIR] has datatypes that mirror the [HIR] datatypes, but instead of e.g. `-x`
2020-01-13T16:10:02.3309643Z     │
2020-01-13T16:10:02.3309807Z 
2020-01-13T16:10:02.3310206Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/rustc_mir/hair/index.html"
2020-01-13T16:10:02.3310357Z 
---
2020-01-13T16:10:02.3319152Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/rustc_mir/hair/cx/expr/trait.ToBorrowKind.html"
2020-01-13T16:10:02.3319310Z 
2020-01-13T16:10:02.3319650Z     ┌── borrow_check/two_phase_borrows.md:61:36 ───
2020-01-13T16:10:02.3319991Z     │
2020-01-13T16:10:02.3320406Z  61 │ after type checking, which is then [converted] to a [`BorrowKind`] during MIR
2020-01-13T16:10:02.3321132Z     │
2020-01-13T16:10:02.3321279Z 
2020-01-13T16:10:02.3321923Z error: Unable to retrieve "http://www.cs.bgu.ac.il/%7Ehendlerd/papers/p280-hendler.pdf": https://www.cs.bgu.ac.il/%7Ehendlerd/papers/p280-hendler.pdf: error trying to connect: error:1416F086:SSL routines:tls_process_server_certificate:certificate verify failed:ssl/statem/statem_clnt.c:1915: (unable to get local issuer certificate)
2020-01-13T16:10:02.3322103Z 
---
2020-01-13T16:32:52.4950171Z 
2020-01-13T16:32:52.4950823Z If you do intend to update 'rustc-guide', please check the error messages above and
2020-01-13T16:32:52.4951124Z commit another update.
2020-01-13T16:32:52.4951305Z 
2020-01-13T16:32:52.4951727Z If you do NOT intend to update 'rustc-guide', please ensure you did not accidentally
2020-01-13T16:32:52.4952294Z change the submodule at 'src/doc/rustc-guide'. You may ask your reviewer for the
2020-01-13T16:32:52.4952559Z proper steps.
2020-01-13T16:32:52.5015274Z   local time: Mon Jan 13 16:32:52 UTC 2020
2020-01-13T16:32:52.5233232Z   network time: Mon, 13 Jan 2020 16:32:52 GMT
2020-01-13T16:32:52.5235447Z == end clock drift check ==
2020-01-13T16:32:53.2778976Z 
2020-01-13T16:32:53.2778976Z 
2020-01-13T16:32:53.2868370Z ##[error]Bash exited with code '1'.
2020-01-13T16:32:53.2905412Z ##[section]Starting: Checkout
2020-01-13T16:32:53.2906878Z ==============================================================================
2020-01-13T16:32:53.2906924Z Task         : Get sources
2020-01-13T16:32:53.2906980Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
