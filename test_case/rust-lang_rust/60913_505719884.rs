plain
travis_time:end:1454de80:start=1561524483561524434,finish=1561524486319308399,duration=2757783965
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:14:53]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:14:53]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:14:54] error: format argument must be a string literal
[00:14:54]      | 
[00:14:54]     ::: <::log::macros::debug macros>:1:1
[00:14:54]      |
[00:14:54] 1    |   / ( target : $ target : expr , $ ( $ arg : tt ) * ) => (
[00:14:54] 2    |   | log ! ( target : $ target , $ crate :: Level :: Debug , $ ( $ arg ) * ) ; ) ;
[00:14:54] 3    |   | ( $ ( $ arg : tt ) * ) => (
[00:14:54] 4    |   | log ! ( $ crate :: Level :: Debug , $ ( $ arg ) * ) ; )
[00:14:54]      |   |_-----------------------------------------------------_- in this expansion of `debug!` (#1)
[00:14:54]      |     in this macro invocation (#2)
[00:14:54]      | 
[00:14:54]      | 
[00:14:54]     ::: <::log::macros::log macros>:1:1
[00:14:54]      |
[00:14:54] 1    | /   ( target : $ target : expr , $ lvl : expr , $ ( $ arg : tt ) + ) => (
[00:14:54] 2    | |   {
[00:14:54] 3    | |   let lvl = $ lvl ; if lvl <= $ crate :: STATIC_MAX_LEVEL && lvl <= $ crate ::
[00:14:54] 4    | |   max_level (  ) {
[00:14:54] 5    | |   $ crate :: __private_api_log (
[00:14:54] 6    | |   __log_format_args ! ( $ ( $ arg ) + ) , lvl , & (
[00:14:54]      | |   ------------------------------------- in this macro invocation (#4)
[00:14:54] 7    | |   $ target , __log_module_path ! (  ) , __log_file ! (  ) , __log_line ! (  ) )
[00:14:54] 8    | |   , ) ; } } ) ; ( $ lvl : expr , $ ( $ arg : tt ) + ) => (
[00:14:54] 9    | |   log ! ( target : __log_module_path ! (  ) , $ lvl , $ ( $ arg ) + ) )
[00:14:54]      | |   ------------------------------------------------------------------- -
[00:14:54]      | |___|___________________________________________________________________in this expansion of `log!` (#2)
[00:14:54]      |     |                                                                   in this expansion of `log!` (#3)
[00:14:54]      |     in this macro invocation (#3)
[00:14:54]      | 
[00:14:54]      | 
[00:14:54]     ::: <::log::macros::__log_format_args macros>:1:1
[00:14:54]      |
[00:14:54] 1    |     ( $ ( $ args : tt ) * ) => { format_args ! ( $ ( $ args ) * ) } ;
[00:14:54]      |     ----------------------------------------------------------------- in this expansion of `__log_format_args!` (#4)
[00:14:54]     --> src/librustc_mir/borrow_check/mod.rs:2252:33
[00:14:54] 2251 | /                               debug!(
[00:14:54] 2251 | /                               debug!(
[00:14:54] 2252 |                                     "upvar.mutability={:?} local_mutation_is_allowed={:?}" /
[00:14:54]      |  ___________________________________^
[00:14:54] 2253 | |                                   "place={:?} {:?}",
[00:14:54]      | |___________________________________________________^
[00:14:54] 2254 | |                                   upvar, is_local_mutation_allowed, place_base, place_projection
[00:14:54]      | |________________________________- in this macro invocation (#1)
[00:14:54] help: you might be missing a string literal to format with
[00:14:54]      |
[00:14:54]      |
[00:14:54] 2252 |                                 "{} {} {} {} {}", "upvar.mutability={:?} local_mutation_is_allowed={:?}" /
[00:14:54] 
[00:15:08] error: aborting due to previous error
[00:15:08] 
[00:15:08] error: Could not compile `rustc_mir`.
---
travis_time:end:0c574d04:start=1561525571603819068,finish=1561525571608591743,duration=4772675
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09e93082
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1d8a07ab
travis_time:start:1d8a07ab
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:25a52916
$ dmesg | grep -i kill
