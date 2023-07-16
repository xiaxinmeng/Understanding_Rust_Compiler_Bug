plain
travis_time:end:056ccc8b:start=1544226758627332461,finish=1544226761032561273,duration=2405228812
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:12] 
[00:55:12] running 120 tests
[00:55:15] i..ii...iii..iiii.....i...i..........i..iii.............i.....i.....ii...i..i.ii..............i...ii 100/120
[00:55:15] ..ii.i.....iiii.....
[00:55:15] 
[00:55:15]  finished in 3.627
[00:55:15] travis_fold:end:test_codegen

---
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:17] 
[00:55:17] running 95 tests
nux-gnu/test/incremental/hashes/let_expressions/let_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/auxiliary"
[00:55:30] ------------------------------------------
[00:55:30] 
[00:55:30] ------------------------------------------
[00:55:30] stderr:
[00:55:30] stderr:
[00:55:30] ------------------------------------------
[00:55:30] {"message":"`MirOptimized(change_mutability_of_reference_type)` should be clean but is not","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/let_expressions.rs","byte_start":2231,"byte_end":2301,"line_start":86,"line_end":88,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub fn change_mutability_of_reference_type() {","highlight_start":1,"highlight_end":47},{"text":"    let _x: &mut u64;","highlight_start":1,"highlight_end":22},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `MirOptimized(change_mutability_of_reference_type)` should be clean but is not\n  --> /checkout/src/test/incremental/hashes/let_expressions.rs:86:1\n   |\nLL | / pub fn change_mutability_of_reference_ttravis_time:end:05a9e850:start=1544226769399648101,finish=1544230100508153661,duration=3331108505560
travis_time:start:187aad80
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Dec  8 00:48:20 UTC 2018
Sat, 08 Dec 2018 00:48:20 GMT
---
travis_time:end:0b2f6f86:start=1544230102064849243,finish=1544230102071227728,duration=6378485
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03cf1925
$ ln -s . checkout &&
