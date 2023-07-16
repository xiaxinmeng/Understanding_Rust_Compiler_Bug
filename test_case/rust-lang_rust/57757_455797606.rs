plain
travis_time:end:17111523:start=1547917464764535568,finish=1547917467032854866,duration=2268319298
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:06:57]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:27] error[E0282]: type annotations needed
[00:07:27]    --> src/librustc/ty/query/config.rs:78:47
[00:07:27]     |
[00:07:27] 78  |           impl_uncacheable_query!($query, |tcx, key| key.describe_as_module(tcx); $($rest)*);
[00:07:27]     |                                                 ^^^ consider giving this closure parameter a type
[00:07:27] ...
[00:07:27] 97  | / impl_uncacheable_query! {
[00:07:27] 98  | |     check_mod_attrs, as_module("checking attributes in {}");
[00:07:27] 99  | |     check_mod_unstable_api_usage, as_module("checking for unstable API usage in {}");
[00:07:27] 100 | |     check_mod_loops, as_module("checking loops in {}");
[00:07:27] ...   |
[00:07:27] 223 | |     dllimport_foreign_items, "wasm import module map";
[00:07:27]     | |_- in this macro invocation
[00:07:27]     |
[00:07:27]     = note: type must be known at this point
[00:07:27] 
[00:07:27] 
[00:07:27] error[E0282]: type annotations needed
[00:07:27]    --> src/librustc/ty/query/config.rs:78:47
[00:07:27]     |
[00:07:27] 78  |           impl_uncacheable_query!($query, |tcx, key| key.describe_as_module(tcx); $($rest)*);
[00:07:27]     |                                                 ^^^ consider giving this closure parameter a type
[00:07:27] ...
[00:07:27] 97  | / impl_uncacheable_query! {
[00:07:27] 98  | |     check_mod_attrs, as_module("checking attributes in {}");
[00:07:27] 99  | |     check_mod_unstable_api_usage, as_module("checking for unstable API usage in {}");
[00:07:27] 100 | |     check_mod_loops, as_module("checking loops in {}");
[00:07:27] ...   |
[00:07:27] 223 | |     dllimport_foreign_items, "wasm import module map";
[00:07:27]     | |_- in this macro invocation
[00:07:27]     |
[00:07:27]     = note: type must be known at this point
[00:07:27] 
[00:07:27] 
[00:07:27] error[E0282]: type annotations needed
[00:07:27]    --> src/librustc/ty/query/config.rs:78:47
[00:07:27]     |
[00:07:27] 78  |           impl_uncacheable_query!($query, |tcx, key| key.describe_as_module(tcx); $($rest)*);
[00:07:27]     |                                                 ^^^ consider giving this closure parameter a type
[00:07:27] ...
[00:07:27] 97  | / impl_uncacheable_query! {
[00:07:27] 98  | |     check_mod_attrs, as_module("checking attributes in {}");
[00:07:27] 99  | |     check_mod_unstable_api_usage, as_module("checking for unstable API usage in {}");
[00:07:27] 100 | |     check_mod_loops, as_module("checking loops in {}");
[00:07:27] ...   |
[00:07:27] 223 | |     dllimport_foreign_items, "wasm import module map";
[00:07:27]     | |_- in this macro invocation
[00:07:27]     |
[00:07:27]     = note: type must be known at this point
[00:07:27] 
[00:07:27] 
[00:07:27] error[E0282]: type annotations needed
[00:07:27]    --> src/librustc/ty/query/config.rs:78:47
[00:07:27]     |
[00:07:27] 78  |           impl_uncacheable_query!($query, |tcx, key| key.describe_as_module(tcx); $($rest)*);
[00:07:27]     |                                                 ^^^ consider giving this closure parameter a type
[00:07:27] ...
[00:07:27] 97  | / impl_uncacheable_query! {
[00:07:27] 98  | |     check_mod_attrs, as_module("checking attributes in {}");
[00:07:27] 99  | |     check_mod_unstable_api_usage, as_module("checking for unstable API usage in {}");
[00:07:27] 100 | |     check_mod_loops, as_module("checking loops in {}");
[00:07:27] ...   |
[00:07:27] 223 | |     dllimport_foreign_items, "wasm import module map";
[00:07:27]     | |_- in this macro invocation
[00:07:27]     |
[00:07:27]     = note: type must be known at this point
[00:07:27] 
[00:07:27] 
[00:07:27] error[E0282]: type annotations needed
[00:07:27]    --> src/librustc/ty/query/config.rs:78:47
[00:07:27]     |
[00:07:27] 78  |           impl_uncacheable_query!($query, |tcx, key| key.describe_as_module(tcx); $($rest)*);
[00:07:27]     |                                                 ^^^ consider giving this closure parameter a type
[00:07:27] ...
[00:07:27] 97  | / impl_uncacheable_query! {
[00:07:27] 98  | |     check_mod_attrs, as_module("checking attributes in {}");
[00:07:27] 99  | |     check_mod_unstable_api_usage, as_module("checking for unstable API usage in {}");
[00:07:27] 100 | |     check_mod_loops, as_module("checking loops in {}");
[00:07:27] ...   |
[00:07:27] 223 | |     dllimport_foreign_items, "wasm import module map";
[00:07:27]     | |_- in this macro invocation
[00:07:27]     |
[00:07:27]     = note: type must be known at this point
[00:07:27] 
[00:07:27] 
[00:07:27] error[E0282]: type annotations needed
[00:07:27]    --> src/librustc/ty/query/config.rs:78:47
[00:07:27]     |
[00:07:27] 78  |           impl_uncacheable_query!($query, |tcx, key| key.describe_as_module(tcx); $($rest)*);
[00:07:27]     |                                                 ^^^ consider giving this closure parameter a type
[00:07:27] ...
[00:07:27] 97  | / impl_uncacheable_query! {
[00:07:27] 98  | |     check_mod_attrs, as_module("checking attributes in {}");
[00:07:27] 99  | |     check_mod_unstable_api_usage, as_module("checking for unstable API usage in {}");
[00:07:27] 100 | |     check_mod_loops, as_module("checking loops in {}");
[00:07:27] ...   |
[00:07:27] 223 | |     dllimport_foreign_items, "wasm import module map";
[00:07:27]     | |_- in this macro invocation
[00:07:27]     |
[00:07:27]     = note: type must be known at this point
[00:07:27] 
[00:07:27] 
[00:07:27] error[E0282]: type annotations needed
[00:07:27]    --> src/librustc/ty/query/config.rs:78:47
[00:07:27]     |
[00:07:27] 78  |           impl_uncacheable_query!($query, |tcx, key| key.describe_as_module(tcx); $($rest)*);
[00:07:27]     |                                                 ^^^ consider giving this closure parameter a type
[00:07:27] ...
[00:07:27] 97  | / impl_uncacheable_query! {
[00:07:27] 98  | |     check_mod_attrs, as_module("checking attributes in {}");
[00:07:27] 99  | |     check_mod_unstable_api_usage, as_module("checking for unstable API usage in {}");
[00:07:27] 100 | |     check_mod_loops, as_module("checking loops in {}");
[00:07:27] ...   |
[00:07:27] 223 | |     dllimport_foreign_items, "wasm import module map";
[00:07:27]     | |_- in this macro invocation
[00:07:27]     |
[00:07:27]     = note: type must be known at this point
[00:07:27] 
[00:07:27] 
[00:07:27] error[E0282]: type annotations needed
[00:07:27]    --> src/librustc/ty/query/config.rs:78:47
[00:07:27]     |
[00:07:27] 78  |           impl_uncacheable_query!($query, |tcx, key| key.describe_as_module(tcx); $($rest)*);
[00:07:27]     |                                                 ^^^ consider giving this closure parameter a type
[00:07:27] ...
[00:07:27] 97  | / impl_uncacheable_query! {
[00:07:27] 98  | |     check_mod_attrs, as_module("checking attributes in {}");
[00:07:27] 99  | |     check_mod_unstable_api_usage, as_module("checking for unstable API usage in {}");
[00:07:27] 100 | |     check_mod_loops, as_module("checking loops in {}");
[00:07:27] ...   |
[00:07:27] 223 | |     dllimport_foreign_items, "wasm import module map";
[00:07:27]     | |_- in this macro invocation
[00:07:27]     |
[00:07:27]     = note: type must be known at this point
[00:07:27] 
---
[00:07:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:30] expected success, got: exit code: 101
[00:07:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:30] Build completed unsuccessfully in 0:03:55
[00:07:30] make: *** [all] Error 1
[00:07:30] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1127d9a7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Jan 19 17:12:08 UTC 2019
---
travis_time:end:09823674:start=1547917928723672431,finish=1547917928731625825,duration=7953394
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0546dbf0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:29b331c7
$ dmesg | grep -i kill
