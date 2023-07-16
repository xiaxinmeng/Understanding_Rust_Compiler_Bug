plain
[00:50:36] ......................................................................i.............................
[00:51:01] ....................................................................................................
[00:51:15] ....................................................................................................
[00:51:40] .......................................ii...........................................................
[00:52:02] .i....ii....................................................i.ii....................................
[00:52:28] .....................................................................iiiiiii........................
[00:52:57] ....................................................................................................
[00:53:13] ....................................................................................................
[00:53:22] .............................................
[00:53:22] test result: ok. 3024 passed; 0 failed; 21 ignored; 0 measured; 0 filtered out
---
travis_time:start:test_run-fail
Check compiletest suite=run-fail mode=run-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:49] 
[00:55:49] running 143 tests
[00:55:59] FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF..FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
e(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/binop-fail-3/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'quux', /checkout/src/test/run-fail/binop-fail-3.rs:13:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/binop-fail-3.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/binop-fail-3.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/assert-ne-macro-panic.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/assert-ne-macro-panic/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/assert-ne-macro-panic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/borrowck-local-borrow.rs#ast stdout ----
[00:56:04] 
[00:56:04] error in revision `ast`: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] status: exit code: 101
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/borrowck-local-borrow.ast/a"
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'panic 1', /checkout/src/test/run-fail/borrowck-local-borrow.rs:18:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/borrowck-local-borrow.rs#ast' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] ---- [run-fail] run-fail/binop-panic.rs stdout ----
[00:56:04] 
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/binop-panic/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] bye
[00:56:04] bye
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'quux', /checkout/src/test/run-fail/binop-panic.rs:14:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/binop-panic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/binop-panic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/borrowck-local-borrow.rs#mir stdout ----
[00:56:04] 
[00:56:04] error in revision `mir`: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] status: exit code: 101
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/borrowck-local-borrow.mir/a"
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'panic 1', /checkout/src/test/run-fail/borrowck-local-borrow.rs:18:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/borrowck-local-borrow.rs#mir' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] ---- [run-fail] run-fail/bounds-check-no-overflow.rs stdout ----
[00:56:04] 
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/bounds-check-no-overflow/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/bounds-check-no-overflow.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/bug-811.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/bug-811/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'quux', /checkout/src/test/run-fail/bug-811.rs:33:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/bug-811.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/bug-811.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/bug-2470-bounds-check-overflow.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/bug-2470-bounds-check-overflow/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] ------------------------------------------
[00:56:04] ov1 base = 0x7fc451c20020
[00:56:04] ov1 idx = 0xff88a384004
[00:56:04] ov1 sizeof::<usize>() = 0x8
[00:56:04] ov1 idx * sizeof::<usize>() = 0x7fc451c20020
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 17560145248260', /checkout/src/libcore/slice/mod.rs:2076:10
---
[00:56:04] thread '[run-fail] run-fail/bug-2470-bounds-check-overflow.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/call-fn-never-arg.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/call-fn-never-arg/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'wowzers!', /checkout/src/test/run-fail/call-fn-never-arg.rs:23:9
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/call-fn-never-arg.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/call-fn-never-arg.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/cast-never.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/cast-never/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/cast-never.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/die-macro-expr.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/die-macro-expr/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/die-macro-expr.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/die-macro.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/die-macro/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/die-macro.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/die-macro-pure.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/die-macro-pure/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/die-macro-pure.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/diverging-closure.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/diverging-closure/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'oops', /checkout/src/test/run-fail/diverging-closure.rs:15:9
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/diverging-closure.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/diverging-closure.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/divide-by-zero.rs stdout ----
[00:56:04] 
[00:56:0456:04] thread '[run-fail] run-fail/dst-raw-slice.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/doublepanic.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/doublepanic/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'One', /checkout/src/test/run-fail/doublepanic.rs:15:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/doublepanic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/doublepanic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/explicit-panic.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/explicit-panic/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/explicit-panic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/expr-fn-panic.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/expr-fn-panic/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/expr-fn-panic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/explicit-panic-msg.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/explicit-panic-msg/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'quux', /checkout/src/test/run-fail/if-cond-bot.rs:14:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/if-cond-bot.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/if-cond-bot.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/issue-12920.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/issue-12920/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/issue-12920.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/hashmap-capacity-overflow.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/hashmap-capacity-overflow/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'capacity overflow', libstd/collections/hash/table.rs:760:58
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/hashmap-capacity-overflow.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/hashmap-capacity-overflow.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/issue-13202.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/issue-13202/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'bad input', /checkout/src/test/run-fail/issue-13202.rs:14:27
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/issue-13202.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/issue-13202.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/issue-18576.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/issue-18576/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'stop', /checkout/src/test/run-fail/issue-18576.rs:19:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/issue-18576.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/issue-18576.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/issue-20971.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/issue-20971/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'Hello, world!', /checkout/src/test/run-fail/issue-20971.rs:26:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/issue-20971.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/issue-20971.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/issue-23354-2.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/issue-23354-2/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'panic evaluated', /checkout/src/test/run-fail/issue-23354-2.rs:16:14
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/issue-23354-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/issue-23354-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/issue-23354.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/issue-23354/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'panic evaluated', /checkout/src/test/run-fail/issue-23354.rs:15:14
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/issue-23354.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/issue-23354.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/issue-2444.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/issue-2444/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/issue-2444.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/issue-29798.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/issue-29798/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/issue-29798.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/issue-2761.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/issue-2761/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'custom message', /checkout/src/test/run-fail/issue-2761.rs:14:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/issue-2761.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/issue-2761.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/issue-28934.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/issue-28934/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/issue-28934.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/issue-44216-add-instant.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/issue-44216-add-instant/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'overflow when adding duration to time', libcore/option.rs:960:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/issue-44216-add-instant.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/issue-44216-add-instant.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/issue-44216-add-system-time.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/issue-44216-add-system-time/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'overflow when adding duration to time', libcore/option.rs:960:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/issue-44216-add-system-time.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/issue-44216-add-system-time.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/issue-3029.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/issue-3029/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'so long', /checkout/src/test/run-fail/issue-3029.rs:20:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/issue-3029.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/issue-3029.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/issue-30380.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/issue-30380/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'panicking destructors ftw!', /checkout/src/test/run-fail/issue-30380.rs:25:13
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/issue-30380.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/issue-30380.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/issue-44216-sub-instant.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/issue-44216-sub-instant/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'overflow when subtracting duration from time', libcore/option.rs:960:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/issue-44216-sub-instant.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/issue-44216-sub-instant.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/issue-44216-sub-system-time.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/issue-44216-sub-system-time/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'overflow when subtracting duration from time', libcore/option.rs:960:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/issue-44216-sub-system-time.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/issue-44216-sub-system-time.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/issue-6458-1.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/issue-6458-1/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/issue-6458-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/issue-51345.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/issue-51345/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/issue-51345.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/issue-948.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/issue-948/a"
[00:56:04] std
[00:56:04] status: exit code: 101
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/match-bot-panic/a"
---
[00:56:04] thread '[run-fail] run-fail/match-bot-panic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/match-disc-bot.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/match-disc-bot/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'quux', /checkout/src/test/run-fail/match-disc-bot.rs:13:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/match-disc-bot.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/match-disc-bot.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/match-wildcards.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/match-wildcards/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'squirrelcupcake', /checkout/src/test/run-fail/match-wildcards.rs:15:13
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/match-wildcards.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/match-wildcards.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/meta-revision-ok.rs#bar stdout ----
[00:56:04] 
[00:56:04] error in revision `bar`: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] status: exit code: 101
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/meta-revision-ok.bar/a"
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'bar', /checkout/src/test/run-fail/meta-revision-ok.rs:24:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/meta-revision-ok.rs#bar' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] ---- [run-fail] run-fail/meta-revision-ok.rs#foo stdout ----
[00:56:04] 
[00:56:04] 
[00:56:04] error in revision `foo`: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] status: exit code: 101
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/meta-revision-ok.foo/a"
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'foo', /checkout/src/test/run-fail/meta-revision-ok.rs:20:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/meta-revision-ok.rs#foo' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] ---- [run-fail] run-fail/mir_codegen_calls_converging_drops.rs stdout ----
[00:56:04] 
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/mir_codegen_calls_converging_drops/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] converging_fn called
[00:56:04] 0 dropped
[00:56:04] thread 'main' panicked at 'exit', /checkout/src/test/run-fail/mir_codegen_calls_converging_drops.rs:33:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/mir_codegen_calls_converging_drops.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/mir_codegen_calls_converging_drops.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/mir_codegen_calls_diverging.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/mir_codegen_calls_diverging/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'diverging_fn called', /checkout/src/test/run-fail/mir_codegen_calls_diverging.rs:14:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/mir_codegen_calls_diverging.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
---
[00:56:04] thread '[run-fail] run-fail/mir_codegen_calls_diverging_drops.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/mir_codegen_no_landing_pads.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/mir_codegen_no_landing_pads/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'converging_fn called', /checkout/src/test/run-fail/mir_codegen_no_landing_pads.rs:25:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/mir_codegen_no_landing_pads.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/mir_codegen_no_landing_pads.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/mir_codegen_no_landing_pads_diverging.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/mir_codegen_no_landing_pads_diverging/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'diverging_fn called', /checkout/src/test/run-fail/mir_codegen_no_landing_pads_diverging.rs:25:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/mir_codegen_no_landing_pads_diverging.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/mir_codegen_no_landing_pads_diverging.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/mir_drop_panics.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/mir_drop_panics/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'panic 1', /checkout/src/test/run-fail/mir_drop_panics.rs:18:13
[00:56:04] drop 2
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/mir_drop_panics.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/mir_drop_panics.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] ruflow', /checkout/src/test/run-fail/overflowing-add.rs:17:14
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/overflowing-add.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/overflowing-add.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/overflowing-lsh-1.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-lsh-1/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'attempt to shift left with overflow', /checkout/src/test/run-fail/overflowing-lsh-1.rs:17:14
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/overflowing-lsh-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/overflowing-lsh-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/never-type-arg.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/never-type-arg/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'oh no!', /checkout/src/test/run-fail/never-type-arg.rs:26:20
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/never-type-arg.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/never-type-arg.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/overflowing-lsh-2.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-lsh-2/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'attempt to shift left with overflow', /checkout/src/test/run-fail/overflowing-lsh-2.rs:17:14
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/overflowing-lsh-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/overflowing-lsh-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/overflowing-lsh-3.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-lsh-3/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'attempt to shift left with overflow', /checkout/src/test/run-fail/overflowing-lsh-3.rs:17:14
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/overflowing-lsh-3.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/overflowing-lsh-3.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/overflowing-mul.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-mul/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'attempt to multiply with overflow', /checkout/src/test/run-fail/overflowing-mul.rs:17:13
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/overflowing-mul.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/overflowing-mul.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/overflowing-lsh-4.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-lsh-4/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'attempt to shift left with overflow', /checkout/src/test/run-fail/overflowing-lsh-4.rs:21:13
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/overflowing-lsh-4.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/overflowing-lsh-4.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/overflowing-neg.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-neg/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'attempt to negate with overflow', /checkout/src/test/run-fail/overflowing-neg.rs:17:14
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/overflowing-neg.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/overflowing-neg.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/overflowing-pow-signed.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-pow-signed/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'attempt to multiply with overflow', /checkout/src/libcore/num/mod.rs:1705:28
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/overflowing-pow-signed.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/overflowing-pow-signed.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/overflowing-pow-unsigned.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-pow-unsigned/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'attempt to multiply with overflow', /checkout/src/libcore/num/mod.rs:3334:24
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/overflowing-pow-unsigned.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/overflowing-pow-unsigned.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/overflowing-rsh-1.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-rsh-1/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/overflowing-rsh-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/overflowing-rsh-2.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-rsh-2/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/overflowing-rsh-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/overflowing-rsh-3.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-rsh-3/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/overflowing-rsh-3.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/overflowing-rsh-4.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-rsh-4/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/overflowing-rsh-4.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/overflowing-rsh-5.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-rsh-5/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/overflowing-rsh-5.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/overflowing-rsh-6.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-rsh-6/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/overflowing-rsh-6.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/overflowing-sub.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/overflowing-sub/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/overflowing-sub.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/panic-arg.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/panic-arg/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'woe', /checkout/src/test/run-fail/panic-arg.rs:17:7
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/panic-arg.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/panic-arg.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/panic-macro-any-wrapped.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/panic-macro-any-wrapped/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/panic-macro-any-wrapped.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/panic-macro-explicit.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/panic-macro-explicit/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/panic-macro-explicit.rs' panicked at`RUST_BACKTRACE=1` for a backtrace.
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/panic-macro-fmt.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/panic-macro-owned.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/panic-macro-owned/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'test-fail-owned', /checkout/src/test/run-fail/panic-macro-owned.rs:14:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/panic-macro-owned.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/panic-macro-owned.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/panic-macro-static.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/panic-macro-static/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/panic-macro-static.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/panic-main.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/panic-main/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'moop', /checkout/src/test/run-fail/panic-main.rs:13:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/panic-main.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/panic-main.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/panic-parens.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/panic-parens/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'oops', /checkout/src/test/run-fail/panic-parens.rs:16:12
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/panic-parens.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/panic-parens.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/panic-set-handler.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/panic-set-handler/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] greetings from the panic handler
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/panic-set-handler.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/panic-take-handler-nop.rs stdout ----
[00:56:04] ---- [run-fail] run-fail/panic-take-handler-nop.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_un-fail/promoted_div_by_zero.rs stdout ----
[00:56:04] 
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/promoted_div_by_zero/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'attempt to divide by zero', /checkout/src/test/run-fail/promoted_div_by_zero.rs:16:14
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/promoted_div_by_zero.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/promoted_div_by_zero.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/panic.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/panic/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'assertion failed: 1 == 2', /checkout/src/test/run-fail/panic.rs:13:5
[00:56:04] 
[00:56:04] -----------------------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
---
[00:56:04] thread '[run-fail] run-fail/panic-task-name-owned.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/promoted_overflow.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/promoted_overflow/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/promoted_overflow.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/return-never-coerce.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/return-never-coerce/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'aah!', /checkout/src/test/run-fail/return-never-coerce.rs:20:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/return-never-coerce.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/return-never-coerce.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/result-get-panic.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/result-get-panic/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "kitty"', libcore/result.rs:945:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/result-get-panic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/result-get-panic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/rfc-1937-termination-trait/termination-trait-for-never.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/rfc-1937-termination-trait/termination-trait-for-never/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'oh, dear', /checkout/src/test/run-fail/rfc-1937-termination-trait/termination-trait-for-never.rs:14:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/rfc-1937-termination-trait/termination-trait-for-never.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/rfc-1937-termination-trait/termination-trait-for-never.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/rhs-type.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/rhs-type/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'bye', /checkout/src/test/run-fail/rhs-type.rs:23:15
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/rhs-type.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/rhs-type.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/run-unexported-tests.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/run-unexported-tests/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] running 1 test
[00:56:04] test m::unexported ... FAILED
[00:56:04] 
[00:56:04] failures:
[00:56:04] 
[00:56:04] ---- m::unexported stdout ----
[00:56:04] thread 'm::unexported' panicked at 'runned an unexported test', /checkout/src/test/run-fail/run-unexported-tests.rs:20:9
[00:56:04] 
[00:56:04] 
[00:56:04] failures:
[00:56:04]     m::unexported
---
[00:56:04] thread '[run-fail] run-fail/run-unexported-tests.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/str-overrun.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/str-overrun/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/str-overrun.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/test-panic.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/test-panic/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] running 1 test
[00:56:04] test test_foo ... FAILED
[00:56:04] 
[00:56:04] failures:
[00:56:04] 
[00:56:04] ---- test_foo stdout ----
[00:56:04] thread 'test_foo' panicked at 'explicit panic', /checkout/src/test/run-fail/test-panic.rs:18:5
[00:56:04] 
[00:56:04] 
[00:56:04] failures:
[00:56:04]     test_foo
---
[00:56:04] thread '[run-fail] run-fail/test-panic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/test-should-fail-bad-message.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/test-should-fail-bad-message/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] running 1 test
[00:56:04] test test_foo ... FAILED
[00:56:04] 
[00:56:04] failures:
[00:56:04] 
[00:56:04] ---- test_foo stdout ----
[00:56:04] thread 'test_foo' panicked at 'blah', /checkout/src/test/run-fail/test-should-fail-bad-message.rs:19:5
[00:56:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:56:04] note: Panic did not include expected string 'foobar'
[00:56:04] failures:
[00:56:04]     test_foo
[00:56:04] 
[00:56:04] test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
---
[00:56:04] thread '[run-fail] run-fail/test-should-fail-bad-message.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/task-spawn-barefn.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/task-spawn-barefn/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread '<unnamed>' panicked at 'assertion failed: "Ensure that the child thread runs by panicking".is_empty()', /checkout/src/test/run-fail/task-spawn-barefn.rs:26:5
[00:56:04] thread 'main' panicked at 'explicit panic', /checkout/src/test/run-fail/task-spawn-barefn.rs:21:9
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/task-spawn-barefn.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/test-should-panic-no-message.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/test-should-panic-no-message/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] 
[00:56:04] ---- test_explicit stdout ----
[00:56:04] thread 'test_explicit' panicked at 'explicit panic', /checkout/src/test/run-fail/test-should-panic-no-message.rs:18:5
[00:56:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:56:04] note: Panic did not include expected string 'foo'
[00:56:04] failures:
[00:56:04]     test_explicit
[00:56:04] 
[00:56:04] test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
---
[00:56:04] thread '[run-fail] run-fail/test-should-panic-no-message.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/test-should-panic-bad-message.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/test-should-panic-bad-message/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] running 1 test
[00:56:04] test test_bar ... FAILED
[00:56:04] 
[00:56:04] failures:
[00:56:04] 
[00:56:04] ---- test_bar stdout ----
[00:56:04] thread 'test_bar' panicked at 'bar', /checkout/src/test/run-fail/test-should-panic-bad-message.rs:18:5
[00:56:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:56:04] note: Panic did not include expected string 'foo'
[00:56:04] failures:
[00:56:04]     test_bar
[00:56:04] 
[00:56:04] test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
---
[00:56:04] thread '[run-fail] run-fail/test-should-panic-bad-message.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/test-tasks-invalid-value.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/test-tasks-invalid-value/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'RUST_TEST_THREADS is `foo`, should be a positive integer.', libtest/lib.rs:1168:22
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/test-tasks-invalid-value.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/test-tasks-invalid-value.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/tls-exit-status.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/tls-exit-status/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'please have a nonzero exit status', /checkout/src/test/run-fail/tls-exit-status.rs:19:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/tls-exit-status.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/tls-exit-status.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/unimplemented-macro-panic.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/unimplemented-macro-panic/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'not yet implemented', /checkout/src/test/run-fail/unimplemented-macro-panic.rs:13:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/unimplemented-macro-panic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/unimplemented-macro-panic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/unique-panic.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/unique-panic/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/unique-panic.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/unreachable-fmt-msg.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/unreachable-fmt-msg/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'internal error: entered unreachable code: 6 is not prime', /checkout/src/test/run-fail/unreachable-fmt-msg.rs:13:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/unreachable-fmt-msg.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/unreachable-fmt-msg.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/unreachable-macro-panic.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/unreachable-macro-panic/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] stderr:
[00:56:04] ------------------------------------------
[00:56:04] thread 'main' panicked at 'internal error: entered unreachable code', /checkout/src/test/run-fail/unreachable-macro-panic.rs:13:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/unreachable-macro-panic.rs' panicked at 'explicit panic', tofail/unreachable.rs:13:5
[00:56:04] 
[00:56:04] ------------------------------------------
[00:56:04] 
[00:56:04] thread '[run-fail] run-fail/unreachable.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] thread '[run-fail] run-fail/unreachable.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/unwind-interleaved.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/unwind-interleaved/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/unwind-interleaved.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/unwind-rec.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/unwind-rec/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/unwind-rec.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/unwind-rec2.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/unwind-rec2/a"
[00:56:04] stdout:
[00:56:04] ------------------------------------------
[00:56:04] 
---
[00:56:04] thread '[run-fail] run-fail/unwind-rec2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:56:04] 
[00:56:04] ---- [run-fail] run-fail/unwind-unique.rs stdout ----
[00:56:04] 
[00:56:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:04] command: "/checkout/obj/build/x86_64-unknown-00:56:04]     [run-fail] run-fail/expr-fn-panic.rs
[00:56:04]     [run-fail] run-fail/expr-if-panic-fn.rs
[00:56:04]     [run-fail] run-fail/expr-if-panic.rs
[00:56:04]     [run-fail] run-fail/expr-match-panic-fn.rs
---
[00:56:04] test result: FAILED. 5 passed; 138 failed; 0 ignored; 0 measured; 0 filtered out
[00:56:04] 
[00:56:04] 
[00:56:04] 
[00:56:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:04] 
[00:56:04] 
[00:56:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:04] Build completed unsuccessfully in 0:14:42
[00:56:04] Build completed unsuccessfully in 0:14:42
[00:56:04] make: *** [check] Error 1
[00:56:04] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:195e68bd
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
