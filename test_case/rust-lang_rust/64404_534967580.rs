plain
2019-09-25T09:40:04.5191055Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-25T09:40:04.5402478Z ##[command]git config gc.auto 0
2019-09-25T09:40:04.5490285Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-25T09:40:04.5557827Z ##[command]git config --get-all http.proxy
2019-09-25T09:40:04.5722778Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64404/merge:refs/remotes/pull/64404/merge
---
2019-09-25T10:44:53.6491736Z .................................................................................................... 1500/9043
2019-09-25T10:45:00.0259911Z .................................................................................................... 1600/9043
2019-09-25T10:45:12.9217556Z .........................................................................i...............i.......... 1700/9043
2019-09-25T10:45:20.2445066Z .................................................................................................... 1800/9043
2019-09-25T10:45:29.3018117Z ................................................................iiiii............................... 1900/9043
2019-09-25T10:45:49.2864121Z .................................................................................................... 2100/9043
2019-09-25T10:45:51.8767032Z .................................................................................................... 2200/9043
2019-09-25T10:45:55.2251002Z .................................................................................................... 2300/9043
2019-09-25T10:46:03.9741043Z .................................................................................................... 2400/9043
---
2019-09-25T10:49:11.5245074Z .......................................................i...............i............................ 4700/9043
2019-09-25T10:49:21.1494655Z .................................................................................................... 4800/9043
2019-09-25T10:49:30.0980514Z .................................................................................................... 4900/9043
2019-09-25T10:49:37.9022229Z .................................................................................................... 5000/9043
2019-09-25T10:49:47.9114939Z ..........................................ii.ii..................................................... 5100/9043
2019-09-25T10:49:58.5067614Z .................................................................................................... 5300/9043
2019-09-25T10:50:09.5192836Z .................................................................................................... 5400/9043
2019-09-25T10:50:17.3535948Z .......i............................................................................................ 5500/9043
2019-09-25T10:50:23.0990007Z .................................................................................................... 5600/9043
2019-09-25T10:50:23.0990007Z .................................................................................................... 5600/9043
2019-09-25T10:50:35.3800897Z .................................................................................................... 5700/9043
2019-09-25T10:50:48.7221825Z ..ii...i..ii...........i............................................................................ 5800/9043
2019-09-25T10:51:11.0539099Z .................................................................................................... 6000/9043
2019-09-25T10:51:18.3212342Z .................................................................................................... 6100/9043
2019-09-25T10:51:18.3212342Z .................................................................................................... 6100/9043
2019-09-25T10:51:32.8119508Z ....i..ii........................................................................................... 6200/9043
2019-09-25T10:51:52.6918771Z ................................................................i................................... 6400/9043
2019-09-25T10:51:55.0473345Z .................................................................................................... 6500/9043
2019-09-25T10:51:57.6665282Z ....................................i............................................................... 6600/9043
2019-09-25T10:52:01.8752585Z .................................................................................................... 6700/9043
---
2019-09-25T10:56:12.7876615Z 61 
2019-09-25T10:56:12.7876653Z 
2019-09-25T10:56:12.7876679Z 
2019-09-25T10:56:12.7876756Z The actual stderr differed from the expected stderr.
2019-09-25T10:56:12.7877527Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns/regions-nested-fns.stderr
2019-09-25T10:56:12.7877819Z To update references, rerun the tests and pass the `--bless` flag
2019-09-25T10:56:12.7878119Z To only update this specific test, also pass `--test-args regions/regions-nested-fns.rs`
2019-09-25T10:56:12.7878200Z error: 1 errors occurred comparing output.
2019-09-25T10:56:12.7878246Z status: exit code: 1
2019-09-25T10:56:12.7878246Z status: exit code: 1
2019-09-25T10:56:12.7879242Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-nested-fns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns/auxiliary" "-A" "unused"
2019-09-25T10:56:12.7879933Z ------------------------------------------
2019-09-25T10:56:12.7879966Z 
2019-09-25T10:56:12.7880174Z ------------------------------------------
2019-09-25T10:56:12.7880232Z stderr:
2019-09-25T10:56:12.7880232Z stderr:
2019-09-25T10:56:12.7880436Z ------------------------------------------
2019-09-25T10:56:12.7880486Z error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
2019-09-25T10:56:12.7880734Z   --> /checkout/src/test/ui/regions/regions-nested-fns.rs:5:18
2019-09-25T10:56:12.7880782Z    |
2019-09-25T10:56:12.7880824Z LL |     let mut ay = &y; //~ ERROR E0495
2019-09-25T10:56:12.7880923Z    |
2019-09-25T10:56:12.7880923Z    |
2019-09-25T10:56:12.7880972Z note: first, the lifetime cannot outlive the anonymous lifetime #2 defined on the body at 7:58...
2019-09-25T10:56:12.7881285Z    |
2019-09-25T10:56:12.7881285Z    |
2019-09-25T10:56:12.7881515Z LL |       ignore::<Box<dyn for<'z> FnMut(&'z isize)>>(Box::new(|z| {
2019-09-25T10:56:12.7881565Z    |  __________________________________________________________^
2019-09-25T10:56:12.7881624Z LL | |         ay = x;
2019-09-25T10:56:12.7881664Z LL | |         ay = &y;
2019-09-25T10:56:12.7881705Z LL | |         ay = z;
2019-09-25T10:56:12.7881760Z LL | |     }));
2019-09-25T10:56:12.7881844Z note: ...so that reference does not outlive borrowed content
2019-09-25T10:56:12.7882093Z   --> /checkout/src/test/ui/regions/regions-nested-fns.rs:10:14
2019-09-25T10:56:12.7882137Z    |
2019-09-25T10:56:12.7882137Z    |
2019-09-25T10:56:12.7882174Z LL |         ay = z;
2019-09-25T10:56:12.7882214Z    |              ^
2019-09-25T10:56:12.7882281Z note: but, the lifetime must be valid for the anonymous lifetime #2 defined on the body at 13:72...
2019-09-25T10:56:12.7882575Z    |
2019-09-25T10:56:12.7882575Z    |
2019-09-25T10:56:12.7882831Z LL |       ignore::< Box<dyn for<'z> FnMut(&'z isize) -> &'z isize>>(Box::new(|z| {
2019-09-25T10:56:12.7882884Z    |  ________________________________________________________________________^
2019-09-25T10:56:12.7882932Z LL | |         if false { return x; } //~ ERROR E0312
2019-09-25T10:56:12.7882993Z LL | |         if false { return ay; }
2019-09-25T10:56:12.7883034Z LL | |         return z;
2019-09-25T10:56:12.7883073Z LL | |     }));
2019-09-25T10:56:12.7883170Z    = note: ...so that the types are compatible:
2019-09-25T10:56:12.7883170Z    = note: ...so that the types are compatible:
2019-09-25T10:56:12.7883213Z            expected &isize
2019-09-25T10:56:12.7883254Z               found &isize
2019-09-25T10:56:12.7883300Z 
2019-09-25T10:56:12.7883344Z error[E0312]: lifetime of reference outlives lifetime of borrowed content...
2019-09-25T10:56:12.7883830Z    |
2019-09-25T10:56:12.7883830Z    |
2019-09-25T10:56:12.7883872Z LL |         if false { return x; } //~ ERROR E0312
2019-09-25T10:56:12.7884044Z    |
2019-09-25T10:56:12.7884044Z    |
2019-09-25T10:56:12.7884280Z note: ...the reference is valid for the anonymous lifetime #2 defined on the body at 13:72...
2019-09-25T10:56:12.7884559Z    |
2019-09-25T10:56:12.7884559Z    |
2019-09-25T10:56:12.7884813Z LL |       ignore::< Box<dyn for<'z> FnMut(&'z isize) -> &'z isize>>(Box::new(|z| {
2019-09-25T10:56:12.7884865Z    |  ________________________________________________________________________^
2019-09-25T10:56:12.7884913Z LL | |         if false { return x; } //~ ERROR E0312
2019-09-25T10:56:12.7884972Z LL | |         if false { return ay; }
2019-09-25T10:56:12.7885014Z LL | |         return z;
2019-09-25T10:56:12.7885053Z LL | |     }));
2019-09-25T10:56:12.7885109Z    | |_____^
2019-09-25T10:56:12.7885576Z note: ...but the borrowed content is only valid for the lifetime 'x as defined on the function body at 3:11
2019-09-25T10:56:12.7885919Z    |
2019-09-25T10:56:12.7885919Z    |
2019-09-25T10:56:12.7886723Z LL | fn nested<'x>(x: &'x isize) {
2019-09-25T10:56:12.7886806Z 
2019-09-25T10:56:12.7886870Z error: aborting due to 2 previous errors
2019-09-25T10:56:12.7886898Z 
2019-09-25T10:56:12.7886942Z Some errors have detailed explanations: E0312, E0495.
---
2019-09-25T10:56:12.7888894Z 110 
2019-09-25T10:56:12.7888920Z 
2019-09-25T10:56:12.7888946Z 
2019-09-25T10:56:12.7888990Z The actual stderr differed from the expected stderr.
2019-09-25T10:56:12.7889307Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-static-method/wf-static-method.stderr
2019-09-25T10:56:12.7889553Z To update references, rerun the tests and pass the `--bless` flag
2019-09-25T10:56:12.7890157Z To only update this specific test, also pass `--test-args wf/wf-static-method.rs`
2019-09-25T10:56:12.7890260Z error: 1 errors occurred comparing output.
2019-09-25T10:56:12.7890301Z status: exit code: 1
2019-09-25T10:56:12.7890301Z status: exit code: 1
2019-09-25T10:56:12.7891154Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/wf/wf-static-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-static-method" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-static-method/auxiliary" "-A" "unused"
2019-09-25T10:56:12.7891486Z ------------------------------------------
2019-09-25T10:56:12.7891517Z 
2019-09-25T10:56:12.7891721Z ------------------------------------------
2019-09-25T10:56:12.7891760Z stderr:
2019-09-25T10:56:12.7891760Z stderr:
2019-09-25T10:56:12.7891974Z ------------------------------------------
2019-09-25T10:56:12.7892038Z error[E0312]: lifetime of reference outlives lifetime of borrowed content...
2019-09-25T10:56:12.7892258Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:17:9
2019-09-25T10:56:12.7892355Z LL |         u //~ ERROR E0312
2019-09-25T10:56:12.7892395Z    |         ^
2019-09-25T10:56:12.7892447Z    |
2019-09-25T10:56:12.7892447Z    |
2019-09-25T10:56:12.7892685Z note: ...the reference is valid for the lifetime 'a as defined on the impl at 14:6...
2019-09-25T10:56:12.7892899Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:14:6
2019-09-25T10:56:12.7892956Z    |
2019-09-25T10:56:12.7893160Z LL | impl<'a, 'b> Foo<'a, 'b, Evil<'a, 'b>> for () {
2019-09-25T10:56:12.7893201Z    |      ^^
2019-09-25T10:56:12.7893447Z note: ...but the borrowed content is only valid for the lifetime 'b as defined on the impl at 14:10
2019-09-25T10:56:12.7893694Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:14:10
2019-09-25T10:56:12.7893738Z    |
2019-09-25T10:56:12.7893940Z LL | impl<'a, 'b> Foo<'a, 'b, Evil<'a, 'b>> for () {
2019-09-25T10:56:12.7894226Z 
2019-09-25T10:56:12.7894265Z error[E0478]: lifetime bound not satisfied
2019-09-25T10:56:12.7894540Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:26:18
2019-09-25T10:56:12.7894582Z    |
2019-09-25T10:56:12.7894582Z    |
2019-09-25T10:56:12.7894625Z LL |         let me = Self::make_me(); //~ ERROR lifetime bound not satisfied
2019-09-25T10:56:12.7894723Z    |
2019-09-25T10:56:12.7894723Z    |
2019-09-25T10:56:12.7894964Z note: lifetime parameter instantiated with the lifetime 'b as defined on the impl at 23:10
2019-09-25T10:56:12.7895180Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:23:10
2019-09-25T10:56:12.7895236Z    |
2019-09-25T10:56:12.7895448Z LL | impl<'a, 'b> Foo<'a, 'b, ()> for IndirectEvil<'a, 'b> {
2019-09-25T10:56:12.7895490Z    |          ^^
2019-09-25T10:56:12.7895743Z note: but lifetime parameter must outlive the lifetime 'a as defined on the impl at 23:6
2019-09-25T10:56:12.7895970Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:23:6
2019-09-25T10:56:12.7896018Z    |
2019-09-25T10:56:12.7896842Z LL | impl<'a, 'b> Foo<'a, 'b, ()> for IndirectEvil<'a, 'b> {
2019-09-25T10:56:12.7896923Z 
2019-09-25T10:56:12.7896923Z 
2019-09-25T10:56:12.7896969Z error[E0312]: lifetime of reference outlives lifetime of borrowed content...
2019-09-25T10:56:12.7897230Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:33:9
2019-09-25T10:56:12.7897318Z LL |         u //~ ERROR E0312
2019-09-25T10:56:12.7897376Z    |         ^
2019-09-25T10:56:12.7897417Z    |
2019-09-25T10:56:12.7897417Z    |
2019-09-25T10:56:12.7897679Z note: ...the reference is valid for the lifetime 'a as defined on the impl at 31:6...
2019-09-25T10:56:12.7897936Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:31:6
2019-09-25T10:56:12.7897983Z    |
2019-09-25T10:56:12.7898190Z LL | impl<'a, 'b> Evil<'a, 'b> {
2019-09-25T10:56:12.7898234Z    |      ^^
2019-09-25T10:56:12.7898536Z note: ...but the borrowed content is only valid for the lifetime 'b as defined on the impl at 31:10
2019-09-25T10:56:12.7898786Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:31:10
2019-09-25T10:56:12.7898832Z    |
2019-09-25T10:56:12.7899054Z LL | impl<'a, 'b> Evil<'a, 'b> {
2019-09-25T10:56:12.7899129Z 
2019-09-25T10:56:12.7899129Z 
2019-09-25T10:56:12.7899411Z error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'b` due to conflicting requirements
2019-09-25T10:56:12.7899671Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:41:5
2019-09-25T10:56:12.7899717Z    |
2019-09-25T10:56:12.7899925Z LL |     <()>::static_evil(b) //~ ERROR cannot infer an appropriate lifetime
2019-09-25T10:56:12.7900021Z    |
2019-09-25T10:56:12.7900021Z    |
2019-09-25T10:56:12.7900270Z note: first, the lifetime cannot outlive the lifetime 'b as defined on the function body at 40:13...
2019-09-25T10:56:12.7900505Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:40:13
2019-09-25T10:56:12.7900546Z    |
2019-09-25T10:56:12.7900762Z LL | fn evil<'a, 'b>(b: &'b u32) -> &'a u32 {
2019-09-25T10:56:12.7900864Z note: ...so that reference does not outlive borrowed content
2019-09-25T10:56:12.7901079Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:41:23
2019-09-25T10:56:12.7901134Z    |
2019-09-25T10:56:12.7901134Z    |
2019-09-25T10:56:12.7901176Z LL |     <()>::static_evil(b) //~ ERROR cannot infer an appropriate lifetime
2019-09-25T10:56:12.7901219Z    |                       ^
2019-09-25T10:56:12.7901469Z note: but, the lifetime must be valid for the lifetime 'a as defined on the function body at 40:9...
2019-09-25T10:56:12.7901702Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:40:9
2019-09-25T10:56:12.7901743Z    |
2019-09-25T10:56:12.7901940Z LL | fn evil<'a, 'b>(b: &'b u32) -> &'a u32 {
2019-09-25T10:56:12.7902042Z note: ...so that reference does not outlive borrowed content
2019-09-25T10:56:12.7902254Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:41:5
2019-09-25T10:56:12.7902490Z    |
2019-09-25T10:56:12.7902490Z    |
2019-09-25T10:56:12.7902541Z LL |     <()>::static_evil(b) //~ ERROR cannot infer an appropriate lifetime
2019-09-25T10:56:12.7902610Z 
2019-09-25T10:56:12.7902610Z 
2019-09-25T10:56:12.7902922Z error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'b` due to conflicting requirements
2019-09-25T10:56:12.7903141Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:45:5
2019-09-25T10:56:12.7903181Z    |
2019-09-25T10:56:12.7903237Z LL |     <IndirectEvil>::static_evil(b)
2019-09-25T10:56:12.7903314Z    |
2019-09-25T10:56:12.7903314Z    |
2019-09-25T10:56:12.7903579Z note: first, the lifetime cannot outlive the lifetime 'b as defined on the function body at 44:22...
2019-09-25T10:56:12.7903797Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:44:22
2019-09-25T10:56:12.7903839Z    |
2019-09-25T10:56:12.7904061Z LL | fn indirect_evil<'a, 'b>(b: &'b u32) -> &'a u32 {
2019-09-25T10:56:12.7904164Z note: ...so that reference does not outlive borrowed content
2019-09-25T10:56:12.7904397Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:45:33
2019-09-25T10:56:12.7904438Z    |
2019-09-25T10:56:12.7904438Z    |
2019-09-25T10:56:12.7904476Z LL |     <IndirectEvil>::static_evil(b)
2019-09-25T10:56:12.7904517Z    |                                 ^
2019-09-25T10:56:12.7904784Z note: but, the lifetime must be valid for the lifetime 'a as defined on the function body at 44:18...
2019-09-25T10:56:12.7905000Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:44:18
2019-09-25T10:56:12.7905040Z    |
2019-09-25T10:56:12.7905265Z LL | fn indirect_evil<'a, 'b>(b: &'b u32) -> &'a u32 {
2019-09-25T10:56:12.7905348Z note: ...so that reference does not outlive borrowed content
2019-09-25T10:56:12.7905755Z   --> /checkout/src/test/ui/wf/wf-static-method.rs:45:5
2019-09-25T10:56:12.7905974Z    |
2019-09-25T10:56:12.7905974Z    |
2019-09-25T10:56:12.7906021Z LL |     <IndirectEvil>::static_evil(b)
2019-09-25T10:56:12.7906812Z 
2019-09-25T10:56:12.7906856Z error: aborting due to 5 previous errors
2019-09-25T10:56:12.7906883Z 
2019-09-25T10:56:12.7906944Z Some errors have detailed explanations: E0312, E0478, E0495.
---
2019-09-25T10:56:12.7925973Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-09-25T10:56:12.7926730Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-25T10:56:12.7942972Z 
2019-09-25T10:56:12.7943097Z 
2019-09-25T10:56:12.7945246Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-25T10:56:12.7945615Z 
2019-09-25T10:56:12.7945646Z 
2019-09-25T10:56:12.7957207Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-25T10:56:12.7957699Z Build completed unsuccessfully in 1:08:47
2019-09-25T10:56:12.7957699Z Build completed unsuccessfully in 1:08:47
2019-09-25T10:56:12.8014163Z == clock drift check ==
2019-09-25T10:56:12.8047461Z   local time: Wed Sep 25 10:56:12 UTC 2019
2019-09-25T10:56:12.9521459Z   network time: Wed, 25 Sep 2019 10:56:12 GMT
2019-09-25T10:56:12.9521581Z == end clock drift check ==
2019-09-25T10:56:13.7209569Z ##[error]Bash exited with code '1'.
2019-09-25T10:56:13.7250153Z ##[section]Starting: Checkout
2019-09-25T10:56:13.7252743Z ==============================================================================
2019-09-25T10:56:13.7252795Z Task         : Get sources
2019-09-25T10:56:13.7252851Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
