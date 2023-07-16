plain
[00:51:56] ....................................................................................................
[00:51:59] ....................................................................................................
[00:52:01] ....................................................................................................
[00:52:04] ....................................................................................................
[00:52:08] ....................................................FFF...............F.F..................i........
[00:52:13] ....................................................................................i...............
[00:52:16] .............................i......................................................................
[00:52:16] .............................i......................................................................
[00:52:20] ....................................F...............................................................
[00:52:27] .......................................................i..ii........................................
[00:52:31] ....................................................................................................
[00:52:35] ....................................................................................................
[00:52:37] ....................................................................................................
[00:52:37] ....................................................................................................
[00:52:40] .......................................i............................................................
[00:52:43] ....................................................................................................
[00:52:46] ....................................................................................................
[00:52:48] ....................................................................................................
let bindings in constants are unstable (see issue #48821) [E0658]'
[00:52:50] 
[00:52:50] error: /checkout/src/test/compile-fail/const-block-non-item-statement.rs:12: unexpected error: '12:11: 12:27: statements in constants are unstable (see issue #48821) [E0658]'
[00:52:50] error: 2 unexpected errors found, 0 expected errors not found
[00:52:50] status: exit code: 1
[00:52:50] status: exit code: 1
[00:52:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/const-block-non-item-statement.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-block-non-item-statement/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-block-non-item-statement/auxiliary" "-A" "unused"
[00:52:50]     Error {
[00:52:50]         line_num: 12,
[00:52:50]         kind: Some(
[00:52:50]             Error
[00:52:50]             Error
[00:52:50]         ),
[00:52:50]         msg: "12:11: 12:27: let bindings in constants are unstable (see issue #48821) [E0658]"
[00:52:50]     Error {
[00:52:50]         line_num: 12,
[00:52:50]         kind: Some(
[00:52:50]             Error
[00:52:50]             Error
[00:52:50]         ),
[00:52:50]         msg: "12:11: 12:27: statements in constants are unstable (see issue #48821) [E0658]"
[00:52:50] ]
[00:52:50] 
[00:52:50] 
[00:52:50] thread '[compile-fail] compile-fail/const-blonst-fn-destructuring-arg.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-fn-destructuring-arg/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/const-fn-destructuring-arg/auxiliary" "-A" "unused"
[00:52:50]     Error {
[00:52:50]         line_num: 17,
[00:52:50]         kind: Some(
[00:52:50]             Error
[00:52:50]             Error
[00:52:50]         ),
[00:52:50]         msg: "17:13: 17:14: let bindings in constant functions are unstable (see issue #48821) [E0658]"
[00:52:50]     Error {
[00:52:50]         line_num: 17,
[00:52:50]         kind: Some(
[00:52:50]             Error
[00:52:50]             Error
[00:52:50]         ),
[00:52:50]         msg: "17:13: 17:14: statements in constant functions are unstable (see issue #48821) [E0658]"
[00:52:50]     Error {
[00:52:50]         line_num: 19,
[00:52:50]         kind: Some(
[00:52:50]             Error
[00:52:50]             Error
[00:52:50]         ),
[00:52:50]         msg: "19:13: 19:14: let bindings in constant functions are unstable (see issue #48821) [E0658]"
[00:52:50]     Error {
[00:52:50]         line_num: 19,
[00:52:50]         kind: Some(
[00:52:50]             Error
[00:52:50]             Error
[00:52:50]         ),
[00:52:50]         msg: "19:13: 19:14: statements in constant functions are unstable (see issue #48821) [E0658]"
[00:52:50] ]
[00:52:50] 
[00
travis_time:end:1898ce38:start=1532209820443612393,finish=1532212991509718810,duration=3171066106417
