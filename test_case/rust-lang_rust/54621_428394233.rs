plain
[00:50:55] 
[00:50:55] running 4579 tests
[00:50:58] .................................................................................................... 100/4579
[00:51:01] .................................................................................................... 200/4579
[00:51:04] ........................................................................F........................... 300/4579
[00:51:07] ...............................................................F.........F.......................... 400/4579
[00:51:14] .......................i............................................................................ 600/4579
[00:51:20] .................................................................................................... 700/4579
[00:51:26] ...................................i...........i.................................................... 800/4579
[00:51:29] ......................................................iiiii......................................... 900/4579
---
[00:53:24] .................................................................................................... 4100/4579
[00:53:28] ................................................................i................................... 4200/4579
[00:53:32] .................................................................................................... 4300/4579
[00:53:35] .................................................................................................... 4400/4579
wck/borrowck-uninit-ref-chain.rs:39: unexpected error: '39:5: 39:14: cannot assign to `a.x` when `a` is not initialized [E0718]'
[00:53:41] 
[00:53:41] error in revision `mir`: /checkout/src/test/ui/borrowck/borrowck-uninit-ref-chain.rs:45: unexpected error: '45:5: 45:12: cannot assign to `a.x` when `a` is not initialized [E0718]'
[00:53:41] 
[00:53:41] error in revision `mir`: /checkout/src/test/ui/borrowck/borrowck-uninit-ref-chain.rs:50: unexpected error: '50:5: 50:14: cannot assign to `a.x` when `a` is not initialized [E0718]'
[00:53:41] 
[00:53:41] error in revision `mir`: 4 unexpected errors found, 0 expected errors not found
[00:53:41] status: exit code: 1
[00:53:41] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-uninit-ref-chain.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-uninit-ref-chain.mir/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-uninit-ref-chain.mir/auxiliary" "-A" "unused"
[00:53:41]     Error {
[00:53:41]         line_num: 34,
[00:53:41]         kind: Some(
[00:53:41]             Error
[00:53:41]             Error
[00:53:41]         ),
[00:53:41]         msg: "34:5: 34:12: cannot assign to `a.x` when `a` is not initialized [E0718]"
[00:53:41]     Error {
[00:53:41]     Error {
[00:53:41]       eckout/src/test/ui/borrowck/borrowck-use-in-index-lvalue.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-use-in-index-lvalue.mir/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-use-in-index-lvalue.mir/auxiliary" "-A" "unused"
[00:53:41]     Error {
[00:53:41]         line_num: 16,
[00:53:41]         kind: Some(
[00:53:41]             Error
[00:53:41]             Error
[00:53:41]         ),
[00:53:41]         msg: "16:5: 16:13: cannot assign to `w[..]` when `w` is not initialized [E0718]"
[00:53:41]     Error {
[00:53:41]         line_num: 20,
[00:53:41]         kind: Some(
[00:53:41]             Error
[00:53:41]             Error
[00:53:41]         ),
[00:53:41]         msg: "20:5: 20:13: cannot assign to `w[..]` when `w` is not initialized [E0718]"
[00:53:41] ]
[00:53:41] 
[00:53:41] thread '[ui] ui/borrowck/borrowck-use-in-index-lvalue.rs#mir' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1341:13
[00:53:41] 
