plain
travis_time:end:1de1a7d3:start=1556125891628190684,finish=1556125892380426432,duration=752235748
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:40]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:43] error[E0432]: unresolved import `crate::mir::Body`
[00:07:43]   --> src/librustc/ty/mod.rs:20:5
[00:07:43]    |
[00:07:43] 20 | use crate::mir::Body;
[00:07:43]    |     ^^^^^^^^^^^^^^^^ no `Body` in `mir`
[00:07:43] 
[00:07:50] error[E0412]: cannot find type `Body` in module `crate::mir`
[00:07:50]    --> src/librustc/query/mod.rs:119:45
[00:07:50]     |
[00:07:50] 119 |                 let mir: Option<crate::mir::Body<'tcx>> = tcx.queries.on_disk_cache
[00:07:50]     |                                             ^^^^ not found in `crate::mir`
[00:07:50]     |
[00:07:50] 1   | use crate::hir::Body;
[00:07:50]     |
[00:07:50] 
[00:07:50] 
[00:07:50] error[E0412]: cannot find type `Body` in module `mir`
[00:07:50]     --> src/librustc/query/mod.rs:100:55
[00:07:50]      |
[00:07:50] 31   |  / rustc_queries! {
[00:07:50] 33   |  |         /// Records the type of every item.
[00:07:50] 33   |  |         /// Records the type of every item.
[00:07:50] 34   |  |         query type_of(key: DefId) -> Ty<'tcx> {
[00:07:50] ...     |
[00:07:50] 100  |  |         query mir_built(_: DefId) -> &'tcx Steal<mir::Body<'tcx>> {}
[00:07:50]      |  |                                                       ^^^^ not found in `mir`
[00:07:50] 1071 |  |     }
[00:07:50] 1072 |  | }
[00:07:50] 1072 |  | }
[00:07:50]      |  |_- in this expansion of `rustc_query_append!`
[00:07:50]     ::: src/librustc/ty/query/mod.rs:101:1
[00:07:50]      |
[00:07:50]      |
[00:07:50] 101  | /  rustc_query_append! { [define_queries!][ <'tcx>
[00:07:50] 103  | |          /// Run analysis passes on the crate
[00:07:50] 103  | |          /// Run analysis passes on the crate
[00:07:50] 104  | |          [] fn analysis: Analysis(CrateNum) -> Result<(), ErrorReported>,
[00:07:50] 106  | |      },
[00:07:50] 107  | |  ]}
[00:07:50]      | |___- in this macro invocation
[00:07:50] help: possible candidate is found in another module, you can import it into scope
[00:07:50] help: possible candidate is found in another module, you can import it into scope
[00:07:50]      |
[00:07:50] 1    | use crate::hir::Body;
[00:07:50]      |
[00:07:50] 
[00:07:50] error[E0412]: cannot find type `Body` in module `mir`
[00:07:50]     --> src/librustc/query/mod.rs:106:55
[00:07:50]      |
[00:07:50] 31   |  / rustc_queries! {
[00:07:50] 33   |  |         /// Records the type of every item.
[00:07:50] 33   |  |         /// Records the type of every item.
[00:07:50] 34   |  |         query type_of(key: DefId) -> Ty<'tcx> {
[00:07:50] ...     |
[00:07:50] 106  |  |         query mir_const(_: DefId) -> &'tcx Steal<mir::Body<'tcx>> {
[00:07:50]      |  |                                                       ^^^^ not found in `mir`
[00:07:50] 1071 |  |     }
[00:07:50] 1072 |  | }
[00:07:50] 1072 |  | }
[00:07:50]      |  |_- in this expansion of `rustc_query_append!`
[00:07:50]     ::: src/librustc/ty/query/mod.rs:101:1
[00:07:50]      |
[00:07:50]      |
[00:07:50] 101  | /  rustc_query_append! { [define_queries!][ <'tcx>
[00:07:50] 103  | |          /// Run analysis passes on the crate
[00:07:50] 103  | |          /// Run analysis passes on the crate
[00:07:50] 104  | |          [] fn analysis: Analysis(CrateNum) -> Result<(), ErrorReported>,
[00:07:50] 106  | |      },
[00:07:50] 107  | |  ]}
[00:07:50]      | |___- in this macro invocation
[00:07:50] help: possible candidate is found in another module, you can import it into scope
[00:07:50] help: possible candidate is found in another module, you can import it into scope
[00:07:50]      |
[00:07:50] 1    | use crate::hir::Body;
[00:07:50]      |
[00:07:50] 
[00:07:50] error[E0412]: cannot find type `Body` in module `mir`
[00:07:50]     --> src/librustc/query/mod.rs:110:59
[00:07:50]      |
[00:07:50] 31   |  / rustc_queries! {
[00:07:50] 33   |  |         /// Records the type of every item.
[00:07:50] 33   |  |         /// Records the type of every item.
[00:07:50] 34   |  |         query type_of(key: DefId) -> Ty<'tcx> {
[00:07:50] ...     |
[00:07:50] 110  |  |         query mir_validated(_: DefId) -> &'tcx Steal<mir::Body<'tcx>> {
[00:07:50]      |  |                                                           ^^^^ not found in `mir`
[00:07:50] 1071 |  |     }
[00:07:50] 1072 |  | }
[00:07:50] 1072 |  | }
[00:07:50]      |  |_- in this expansion of `rustc_query_append!`
[00:07:50]     ::: src/librustc/ty/query/mod.rs:101:1
[00:07:50]      |
[00:07:50]      |
[00:07:50] 101  | /  rustc_query_append! { [define_queries!][ <'tcx>
[00:07:50] 103  | |          /// Run analysis passes on the crate
[00:07:50] 103  | |          /// Run analysis passes on the crate
[00:07:50] 104  | |          [] fn analysis: Analysis(CrateNum) -> Result<(), ErrorReported>,
[00:07:50] 106  | |      },
[00:07:50] 107  | |  ]}
[00:07:50]      | |___- in this macro invocation
[00:07:50] help: possible candidate is found in another module, you can import it into scope
[00:07:50] help: possible candidate is found in another module, you can import it into scope
[00:07:50]      |
[00:07:50] 1    | use crate::hir::Body;
[00:07:50]      |
[00:07:50] 
[00:07:51] error[E0412]: cannot find type `Body` in module `mir`
[00:07:51]     --> src/librustc/query/mod.rs:116:55
[00:07:51]      |
[00:07:51] 31   |  / rustc_queries! {
[00:07:51] 33   |  |         /// Records the type of every item.
[00:07:51] 33   |  |         /// Records the type of every item.
[00:07:51] 34   |  |         query type_of(key: DefId) -> Ty<'tcx> {
[00:07:51] ...     |
[00:07:51] 116  |  |         query optimized_mir(key: DefId) -> &'tcx mir::Body<'tcx> {
[00:07:51]      |  |                                                       ^^^^ not found in `mir`
[00:07:51] 1071 |  |     }
[00:07:51] 1072 |  | }
[00:07:51] 1072 |  | }
[00:07:51]      |  |_- in this expansion of `rustc_query_append!`
[00:07:51]     ::: src/librustc/ty/query/mod.rs:101:1
[00:07:51]      |
[00:07:51]      |
[00:07:51] 101  | /  rustc_query_append! { [define_queries!][ <'tcx>
[00:07:51] 103  | |          /// Run analysis passes on the crate
[00:07:51] 103  | |          /// Run analysis passes on the crate
[00:07:51] 104  | |          [] fn analysis: Analysis(CrateNum) -> Result<(), ErrorReported>,
[00:07:51] 106  | |      },
[00:07:51] 107  | |  ]}
[00:07:51]      | |___- in this macro invocation
[00:07:51] help: possible candidate is found in another module, you can import it into scope
[00:07:51] help: possible candidate is found in another module, you can import it into scope
[00:07:51]      |
[00:07:51] 1    | use crate::hir::Body;
[00:07:51]      |
[00:07:51] 
[00:07:51] error[E0412]: cannot find type `Body` in module `mir`
[00:07:51]     --> src/librustc/query/mod.rs:461:67
[00:07:51]      |
[00:07:51] 31   |  / rustc_queries! {
[00:07:51] 33   |  |         /// Records the type of every item.
[00:07:51] 33   |  |         /// Records the type of every item.
[00:07:51] 34   |  |         query type_of(key: DefId) -> Ty<'tcx> {
[00:07:51] ...     |
[00:07:51] 461  |  |         query mir_shims(key: ty::InstanceDef<'tcx>) -> &'tcx mir::Body<'tcx> {
[00:07:51]      |  |                                                                   ^^^^ not found in `mir`
[00:07:51] 1071 |  |     }
[00:07:51] 1072 |  | }
[00:07:51] 1072 |  | }
[00:07:51]      |  |_- in this expansion of `rustc_query_append!`
[00:07:51]     ::: src/librustc/ty/query/mod.rs:101:1
[00:07:51]      |
[00:07:51]      |
[00:07:51] 101  | /  rustc_query_append! { [define_queries!][ <'tcx>
[00:07:51] 103  | |          /// Run analysis passes on the crate
[00:07:51] 103  | |          /// Run analysis passes on the crate
[00:07:51] 104  | |          [] fn analysis: Analysis(CrateNum) -> Result<(), ErrorReported>,
[00:07:51] 106  | |      },
[00:07:51] 107  | |  ]}
[00:07:51]      | |___- in this macro invocation
[00:07:51] help: possible candidate is found in another module, you can import it into scope
[00:07:51] help: possible candidate is found in another module, you can import it into scope
[00:07:51]      |
[00:07:51] 1    | use crate::hir::Body;
[00:07:51]      |
[00:07:51] 
[00:07:51] error[E0412]: cannot find type `Mir` in this scope
[00:07:51]     --> src/librustc/ty/mod.rs:2998:34
[00:07:51]      |
[00:07:51] 2998 |                         -> &'gcx Mir<'gcx>
[00:07:51] help: possible candidate is found in another module, you can import it into scope
[00:07:51]      |
[00:07:51] 3    | use crate::mir::Mir;
[00:07:51]      |
---
travis_time:end:27342358:start=1556126404228290139,finish=1556126404233708038,duration=5417899
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:26ac02d8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:25ee054d
travis_time:start:25ee054d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:046ae4a0
$ dmesg | grep -i kill
