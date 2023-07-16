plain
travis_time:end:1733260b:start=1543850730845872634,finish=1543850788076333896,duration=57230461262
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:05:31]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:05:38]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:05:38]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:52]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:06]    Compiling rustc_query_macro v0.1.0 (/checkout/src/librustc_query_macro)
[00:07:18]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:19] group [Other] type_of
[00:07:19] QUERY Other { [  ] fn type_of : TypeOf ( DefId ) -> Ty < 'tcx > , } ,
[00:07:19] DEP NODE [  ] TypeOf ( DefId ) ,
[00:07:19] DEP NODE FORCE DepKind :: TypeOf => {
[00:07:19] force_ex ! ( $ tcx , type_of , def_id_ex ! ( $ dep_node ) ) ; }
[00:07:19] RETURNING macro_rules ! rustc_query_append {
[00:07:19] ( [ $ ( $ macro : tt ) * ] [ $ ( $ other : tt ) * ] ) => {
[00:07:19] $ ( $ macro ) * {
[00:07:19] $ ( $ other ) * Other { [  ] fn type_of : TypeOf ( DefId ) -> Ty < 'tcx > , }
[00:07:19] , } } } macro_rules ! rustc_dep_node_append {
[00:07:19] ( [ $ ( $ macro : tt ) * ] [ $ ( $ other : tt ) * ] ) => {
[00:07:19] $ ( $ macro ) * ( $ ( $ other ) * [  ] TypeOf ( DefId ) , ) ; } } macro_rules
[00:07:19] ! rustc_dep_node_force {
[00:07:19] ( [ $ dep_node : expr , $ tcx : expr ] $ ( $ other : tt ) * ) => {
[00:07:19] match $ dep_node . kind {
[00:07:19] $ ( $ other ) * DepKind :: TypeOf => {
[00:07:19] force_ex ! ( $ tcx , type_of , def_id_ex ! ( $ dep_node ) ) ; } } } }
[00:13:10]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:13:10]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:13:10] error[E0425]: cannot find value `TypeOfItem` in module `label_strs`
[00:13:10]   --> src/librustc_incremental/persist/dirty_clean.rs:50:17
[00:13:10]    |
[00:13:10] 50 |     label_strs::TypeOfItem,
[00:13:10]    |                 ^^^^^^^^^^ not found in `label_strs`
[00:13:10] 
[00:13:10] error[E0425]: cannot find value `TypeOfItem` in module `label_strs`
[00:13:10]   --> src/librustc_incremental/persist/dirty_clean.rs:59:17
[00:13:10]    |
[00:13:10] 59 |     label_strs::TypeOfItem,
[00:13:10]    |                 ^^^^^^^^^^ not found in `label_strs`
[00:13:10] 
[00:13:10] error[E0425]: cannot find value `TypeOfItem` in module `label_strs`
[00:13:10]   --> src/librustc_incremental/persist/dirty_clean.rs:94:17
[00:13:10]    |
[00:13:10] 94 |     label_strs::TypeOfItem,
[00:13:10]    |                 ^^^^^^^^^^ not found in `label_strs`
[00:13:11] error: aborting due to 3 previous errors
[00:13:11] 
[00:13:11] For more information about this error, try `rustc --explain E0425`.
[00:13:11] error: Could not compile `rustc_incremental`.
---
184272 ./obj/build/cache/2018-10-30
159708 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
153284 ./src/tools/clang
150356 ./obj/build/bootstrap/debug/incremental
134764 ./obj/build/bootstrap/debug/incremental/bootstrap-2oigu19tl6bg6
134760 ./obj/build/bootstrap/debug/incremental/bootstrap-2oigu19tl6bg6/s-f78zbczu1w-1iibz2p-35hdvoweuro4i
134004 ./.git/modules/src
115352 ./src/llvm/test/CodeGen
107892 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
107416 ./src/tools/lldb
---
travis_time:end:17a9aa98:start=1543851809405176344,finish=1543851809411059496,duration=5883152
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2680d436
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0048ec20
travis_time:start:0048ec20
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or direc
