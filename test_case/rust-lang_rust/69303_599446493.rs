plain
2020-03-16T09:23:02.1000826Z ========================== Starting Command Output ===========================
2020-03-16T09:23:02.1005073Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a97f3e21-087c-44f0-b400-9445f7412c75.sh
2020-03-16T09:23:02.1005698Z 
2020-03-16T09:23:02.1010405Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-16T09:23:02.1030923Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69303/merge to s
2020-03-16T09:23:02.1034491Z Task         : Get sources
2020-03-16T09:23:02.1034816Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-16T09:23:02.1035132Z Version      : 1.0.0
2020-03-16T09:23:02.1035346Z Author       : Microsoft
---
2020-03-16T09:23:03.3028942Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-16T09:23:03.3043255Z ##[command]git config gc.auto 0
2020-03-16T09:23:03.3073297Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-16T09:23:03.3081111Z ##[command]git config --get-all http.proxy
2020-03-16T09:23:03.3092606Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69303/merge:refs/remotes/pull/69303/merge
---
2020-03-16T09:59:53.9683560Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-16T09:59:55.4816292Z error: unnecessary parentheses around type
2020-03-16T09:59:55.4817705Z     --> src/librustc/query/mod.rs:78:20
2020-03-16T09:59:55.4818581Z      |
2020-03-16T09:59:55.4819520Z 38   | / rustc_queries! {
2020-03-16T09:59:55.4820644Z 39   | |     Other {
2020-03-16T09:59:55.4821927Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-03-16T09:59:55.4823357Z 41   | |             desc { "trigger a delay span bug" }
2020-03-16T09:59:55.4825131Z ...    |
2020-03-16T09:59:55.4826150Z 78   | |             storage(caches::LocalDenseDefIdCacheSelector<&'tcx HirOwner<'tcx>>)
2020-03-16T09:59:55.4828769Z ...    |
2020-03-16T09:59:55.4829543Z 1257 | |     }
2020-03-16T09:59:55.4830375Z 1258 | | }
2020-03-16T09:59:55.4830375Z 1258 | | }
2020-03-16T09:59:55.4836994Z      | |_- in this expansion of `rustc_query_append!`
2020-03-16T09:59:55.4838741Z     ::: src/librustc/ty/query/mod.rs:110:1
2020-03-16T09:59:55.4839403Z      |
2020-03-16T09:59:55.4839403Z      |
2020-03-16T09:59:55.4840152Z 110  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-03-16T09:59:55.4845195Z      |
2020-03-16T09:59:55.4845936Z      = note: `-D unused-parens` implied by `-D warnings`
2020-03-16T09:59:55.4853038Z 
2020-03-16T09:59:55.4881376Z error: unnecessary parentheses around type
2020-03-16T09:59:55.4881376Z error: unnecessary parentheses around type
2020-03-16T09:59:55.4882130Z     --> src/librustc/query/mod.rs:87:20
2020-03-16T09:59:55.4882743Z      |
2020-03-16T09:59:55.4883466Z 38   | / rustc_queries! {
2020-03-16T09:59:55.4884339Z 39   | |     Other {
2020-03-16T09:59:55.4885446Z 40   | |         query trigger_delay_span_bug(key: DefId) -> () {
2020-03-16T09:59:55.4886574Z 41   | |             desc { "trigger a delay span bug" }
2020-03-16T09:59:55.4887548Z ...    |
2020-03-16T09:59:55.4888661Z 87   | |             storage(caches::LocalDenseDefIdCacheSelector<&'tcx HirOwnerItems<'tcx>>)
2020-03-16T09:59:55.4891299Z ...    |
2020-03-16T09:59:55.4892052Z 1257 | |     }
2020-03-16T09:59:55.4893063Z 1258 | | }
2020-03-16T09:59:55.4893063Z 1258 | | }
2020-03-16T09:59:55.4894007Z      | |_- in this expansion of `rustc_query_append!`
2020-03-16T09:59:55.4895373Z     ::: src/librustc/ty/query/mod.rs:110:1
2020-03-16T09:59:55.4895962Z      |
2020-03-16T09:59:55.4895962Z      |
2020-03-16T09:59:55.4896733Z 110  |   rustc_query_append! { [define_queries!][<'tcx>] }
2020-03-16T09:59:55.4902303Z 
2020-03-16T10:00:11.0334921Z error: aborting due to 2 previous errors
2020-03-16T10:00:11.0335975Z 
2020-03-16T10:00:11.0336717Z error: could not compile `rustc`.
---
2020-03-16T10:00:29.0422398Z   local time: Mon Mar 16 10:00:29 UTC 2020
2020-03-16T10:00:29.5975731Z   network time: Mon, 16 Mar 2020 10:00:29 GMT
2020-03-16T10:00:29.5976335Z == end clock drift check ==
2020-03-16T10:00:30.1296762Z 
2020-03-16T10:00:30.1382695Z ##[error]Bash exited with code '1'.
2020-03-16T10:00:30.1399191Z ##[section]Finishing: Run build
2020-03-16T10:00:30.1519428Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69303/merge to s
2020-03-16T10:00:30.1525348Z Task         : Get sources
2020-03-16T10:00:30.1525762Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-16T10:00:30.1526128Z Version      : 1.0.0
2020-03-16T10:00:30.1526465Z Author       : Microsoft
2020-03-16T10:00:30.1526465Z Author       : Microsoft
2020-03-16T10:00:30.1526882Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-16T10:00:30.1527369Z ==============================================================================
2020-03-16T10:00:30.5460283Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-16T10:00:30.5537887Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69303/merge to s
2020-03-16T10:00:30.5643817Z Cleaning up task key
2020-03-16T10:00:30.5645066Z Start cleaning up orphan processes.
2020-03-16T10:00:30.6947161Z Terminate orphan process: pid (3819) (python)
2020-03-16T10:00:30.7003760Z ##[section]Finishing: Finalize Job
