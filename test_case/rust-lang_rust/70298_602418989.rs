plain
2020-03-23T05:49:58.0843642Z ========================== Starting Command Output ===========================
2020-03-23T05:49:58.0847051Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/dbfbecc7-fa71-4675-a327-f5fc0f92af92.sh
2020-03-23T05:49:58.0847377Z 
2020-03-23T05:49:58.0850963Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-23T05:49:58.0871844Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70298/merge to s
2020-03-23T05:49:58.0875607Z Task         : Get sources
2020-03-23T05:49:58.0875928Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T05:49:58.0876240Z Version      : 1.0.0
2020-03-23T05:49:58.0876470Z Author       : Microsoft
---
2020-03-23T05:49:59.2146295Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-23T05:49:59.2157245Z ##[command]git config gc.auto 0
2020-03-23T05:49:59.2162627Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-23T05:49:59.2170416Z ##[command]git config --get-all http.proxy
2020-03-23T05:49:59.2181187Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70298/merge:refs/remotes/pull/70298/merge
---
2020-03-23T06:46:16.9753557Z .................................................................................................... 1700/9821
2020-03-23T06:46:21.0423168Z .................................................................................................... 1800/9821
2020-03-23T06:46:30.5185676Z ................................................................................i...............FF.. 1900/9821
2020-03-23T06:46:37.4715444Z .................................................................................................... 2000/9821
2020-03-23T06:46:43.7627186Z ......................................................................iiiii......................... 2100/9821
2020-03-23T06:47:02.9766312Z .................................................................................................... 2300/9821
2020-03-23T06:47:05.2903656Z .................................................................................................... 2400/9821
2020-03-23T06:47:07.7968200Z .................................................................................................... 2500/9821
2020-03-23T06:47:20.7305597Z .................................................................................................... 2600/9821
---
2020-03-23T06:50:01.3603560Z ............................................i...............i....................................... 5000/9821
2020-03-23T06:50:09.9070550Z .................................................................................................... 5100/9821
2020-03-23T06:50:16.0940282Z ........................................................................................i........... 5200/9821
2020-03-23T06:50:21.5224483Z ..............................................................F....................F................ 5300/9821
2020-03-23T06:50:31.5384196Z .......................................................................ii.ii........i...i........... 5400/9821
2020-03-23T06:50:39.1457230Z ...........i........................................................................................ 5600/9821
2020-03-23T06:50:47.6100528Z ................i................................................................................... 5700/9821
2020-03-23T06:50:53.7237344Z .................................ii...................................i............................. 5800/9821
2020-03-23T06:51:00.1260973Z .......................................................................................F............ 5900/9821
2020-03-23T06:51:00.1260973Z .......................................................................................F............ 5900/9821
2020-03-23T06:51:06.5619881Z .................................................................................................... 6000/9821
2020-03-23T06:51:15.4760685Z ................................................................ii...i...ii..........i.............. 6100/9821
2020-03-23T06:51:34.7600162Z .................................................................................................... 6300/9821
2020-03-23T06:51:41.6717476Z .................................................................................................... 6400/9821
2020-03-23T06:51:41.6717476Z .................................................................................................... 6400/9821
2020-03-23T06:51:48.5984272Z ...................................................................F..........................i..ii. 6500/9821
2020-03-23T06:52:12.1400555Z .................................................................................................... 6700/9821
2020-03-23T06:52:22.5139504Z .............................................................................................i...... 6800/9821
2020-03-23T06:52:24.5387282Z .................................................................................................... 6900/9821
2020-03-23T06:52:26.6277100Z .................................................................................................... 7000/9821
---
2020-03-23T06:54:08.2272977Z .................................................................................................... 7800/9821
2020-03-23T06:54:12.9110920Z .................................................................................................... 7900/9821
2020-03-23T06:54:18.8925018Z ...................................................................................i................ 8000/9821
2020-03-23T06:54:27.3877121Z .......................F............................................................................ 8100/9821
2020-03-23T06:54:33.9770112Z ................................iiiiiiiiii.i........................................................ 8200/9821
2020-03-23T06:54:47.5717514Z .................................................................................................... 8400/9821
2020-03-23T06:54:52.5208706Z .................................................................................................... 8500/9821
2020-03-23T06:55:07.1260339Z .................................................................................................... 8600/9821
2020-03-23T06:55:14.5874548Z .................................................................................................... 8700/9821
---
2020-03-23T06:57:07.5419056Z ---- [ui] ui/array-slice-vec/vec-matching-autoslice.rs stdout ----
2020-03-23T06:57:07.5419654Z 
2020-03-23T06:57:07.5420450Z error: test compilation failed although it shouldn't!
2020-03-23T06:57:07.5421063Z status: exit code: 1
2020-03-23T06:57:07.5423561Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/array-slice-vec/vec-matching-autoslice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/vec-matching-autoslice/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/vec-matching-autoslice/auxiliary"
2020-03-23T06:57:07.5426032Z ------------------------------------------
2020-03-23T06:57:07.5426501Z 
2020-03-23T06:57:07.5427232Z ------------------------------------------
2020-03-23T06:57:07.5427738Z stderr:
2020-03-23T06:57:07.5427738Z stderr:
2020-03-23T06:57:07.5428562Z ------------------------------------------
2020-03-23T06:57:07.5429447Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5430425Z   --> /checkout/src/test/ui/array-slice-vec/vec-matching-autoslice.rs:20:18
2020-03-23T06:57:07.5431046Z    |
2020-03-23T06:57:07.5431491Z LL |         ([_, _], 0.5) => panic!(),
2020-03-23T06:57:07.5432376Z 
2020-03-23T06:57:07.5433104Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5434072Z   --> /checkout/src/test/ui/array-slice-vec/vec-matching-autoslice.rs:20:18
2020-03-23T06:57:07.5434683Z    |
2020-03-23T06:57:07.5434683Z    |
2020-03-23T06:57:07.5435109Z LL |         ([_, _], 0.5) => panic!(),
2020-03-23T06:57:07.5436488Z 
2020-03-23T06:57:07.5436915Z error: aborting due to 2 previous errors
2020-03-23T06:57:07.5437310Z 
2020-03-23T06:57:07.5438331Z 
2020-03-23T06:57:07.5438331Z 
2020-03-23T06:57:07.5439180Z ------------------------------------------
2020-03-23T06:57:07.5439576Z 
2020-03-23T06:57:07.5439815Z 
2020-03-23T06:57:07.5440479Z ---- [ui] ui/binding/match-range.rs stdout ----
2020-03-23T06:57:07.5440856Z 
2020-03-23T06:57:07.5441485Z error: test compilation failed although it shouldn't!
2020-03-23T06:57:07.5441974Z status: exit code: 1
2020-03-23T06:57:07.5444222Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binding/match-range.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/match-range/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/match-range/auxiliary"
2020-03-23T06:57:07.5447326Z ------------------------------------------
2020-03-23T06:57:07.5447737Z 
2020-03-23T06:57:07.5448338Z ------------------------------------------
2020-03-23T06:57:07.5448743Z stderr:
---
2020-03-23T06:57:07.5508810Z 
2020-03-23T06:57:07.5508917Z 
2020-03-23T06:57:07.5509500Z ---- [ui] ui/borrowck/two-phase-reservation-sharing-interference-2.rs#migrate2015 stdout ----
2020-03-23T06:57:07.5509796Z 
2020-03-23T06:57:07.5510830Z error in revision `migrate2015`: /checkout/src/test/ui/borrowck/two-phase-reservation-sharing-interference-2.rs:40: unexpected error: '40:5: 40:6: cannot borrow `v` as mutable because it is also borrowed as immutable [E0502]'
2020-03-23T06:57:07.5511465Z 
2020-03-23T06:57:07.5512326Z error in revision `migrate2015`: /checkout/src/test/ui/borrowck/two-phase-reservation-sharing-interference-2.rs:40: expected warning not found: cannot borrow `v` as mutable
2020-03-23T06:57:07.5516149Z error in revision `migrate2015`: /checkout/src/test/ui/borrowck/two-phase-reservation-sharing-interference-2.rs:40: expected warning not found: may become a hard error in the future
2020-03-23T06:57:07.5516911Z 
2020-03-23T06:57:07.5517241Z error in revision `migrate2015`: 1 unexpected errors found, 2 expected errors not found
2020-03-23T06:57:07.5517625Z status: exit code: 1
2020-03-23T06:57:07.5517625Z status: exit code: 1
2020-03-23T06:57:07.5520356Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/two-phase-reservation-sharing-interference-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "migrate2015" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-reservation-sharing-interference-2.migrate2015" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-reservation-sharing-interference-2.migrate2015/auxiliary"
2020-03-23T06:57:07.5522349Z     Error {
2020-03-23T06:57:07.5522557Z         line_num: 40,
2020-03-23T06:57:07.5522805Z         kind: Some(
2020-03-23T06:57:07.5523016Z             Error,
2020-03-23T06:57:07.5523016Z             Error,
2020-03-23T06:57:07.5523199Z         ),
2020-03-23T06:57:07.5523589Z         msg: "40:5: 40:6: cannot borrow `v` as mutable because it is also borrowed as immutable [E0502]",
2020-03-23T06:57:07.5524094Z ]
2020-03-23T06:57:07.5524224Z 
2020-03-23T06:57:07.5524582Z not found errors (from test file): [
2020-03-23T06:57:07.5524830Z     Error {
2020-03-23T06:57:07.5524830Z     Error {
2020-03-23T06:57:07.5525050Z         line_num: 40,
2020-03-23T06:57:07.5525275Z         kind: Some(
2020-03-23T06:57:07.5525787Z             Warning,
2020-03-23T06:57:07.5525978Z         ),
2020-03-23T06:57:07.5526245Z         msg: "cannot borrow `v` as mutable",
2020-03-23T06:57:07.5526638Z     Error {
2020-03-23T06:57:07.5526859Z         line_num: 40,
2020-03-23T06:57:07.5527085Z         kind: Some(
2020-03-23T06:57:07.5527311Z             Warning,
---
2020-03-23T06:57:07.5529790Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-23T06:57:07.5530076Z 
2020-03-23T06:57:07.5530649Z ---- [ui] ui/borrowck/two-phase-reservation-sharing-interference-2.rs#migrate2018 stdout ----
2020-03-23T06:57:07.5530942Z 
2020-03-23T06:57:07.5532240Z error in revision `migrate2018`: /checkout/src/test/ui/borrowck/two-phase-reservation-sharing-interference-2.rs:40: unexpected error: '40:5: 40:6: cannot borrow `v` as mutable because it is also borrowed as immutable [E0502]'
2020-03-23T06:57:07.5532874Z 
2020-03-23T06:57:07.5533741Z error in revision `migrate2018`: /checkout/src/test/ui/borrowck/two-phase-reservation-sharing-interference-2.rs:40: expected warning not found: cannot borrow `v` as mutable
2020-03-23T06:57:07.5536505Z error in revision `migrate2018`: /checkout/src/test/ui/borrowck/two-phase-reservation-sharing-interference-2.rs:40: expected warning not found: may become a hard error in the future
2020-03-23T06:57:07.5537934Z 
2020-03-23T06:57:07.5538288Z error in revision `migrate2018`: 1 unexpected errors found, 2 expected errors not found
2020-03-23T06:57:07.5538653Z status: exit code: 1
2020-03-23T06:57:07.5538653Z status: exit code: 1
2020-03-23T06:57:07.5550195Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/two-phase-reservation-sharing-interference-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "migrate2018" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-reservation-sharing-interference-2.migrate2018" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-reservation-sharing-interference-2.migrate2018/auxiliary"
2020-03-23T06:57:07.5553794Z     Error {
2020-03-23T06:57:07.5554007Z         line_num: 40,
2020-03-23T06:57:07.5554234Z         kind: Some(
2020-03-23T06:57:07.5554463Z             Error,
2020-03-23T06:57:07.5554463Z             Error,
2020-03-23T06:57:07.5554646Z         ),
2020-03-23T06:57:07.5555018Z         msg: "40:5: 40:6: cannot borrow `v` as mutable because it is also borrowed as immutable [E0502]",
2020-03-23T06:57:07.5555542Z ]
2020-03-23T06:57:07.5555657Z 
2020-03-23T06:57:07.5555878Z not found errors (from test file): [
2020-03-23T06:57:07.5556156Z     Error {
2020-03-23T06:57:07.5556156Z     Error {
2020-03-23T06:57:07.5556998Z         line_num: 40,
2020-03-23T06:57:07.5557230Z         kind: Some(
2020-03-23T06:57:07.5557469Z             Warning,
2020-03-23T06:57:07.5557656Z         ),
2020-03-23T06:57:07.5557896Z         msg: "cannot borrow `v` as mutable",
2020-03-23T06:57:07.5558311Z     Error {
2020-03-23T06:57:07.5558510Z         line_num: 40,
2020-03-23T06:57:07.5558751Z         kind: Some(
2020-03-23T06:57:07.5558964Z             Warning,
---
2020-03-23T06:57:07.5561013Z thread '[ui] ui/borrowck/two-phase-reservation-sharing-interference-2.rs#migrate2018' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1436:13
2020-03-23T06:57:07.5561455Z 
2020-03-23T06:57:07.5562078Z ---- [ui] ui/borrowck/two-phase-reservation-sharing-interference-future-compat-lint.rs stdout ----
2020-03-23T06:57:07.5562388Z 
2020-03-23T06:57:07.5564069Z error: /checkout/src/test/ui/borrowck/two-phase-reservation-sharing-interference-future-compat-lint.rs:13: unexpected error: '13:9: 13:10: cannot borrow `v` as mutable because it is also borrowed as immutable [E0502]'
2020-03-23T06:57:07.5565465Z 
2020-03-23T06:57:07.5566643Z error: /checkout/src/test/ui/borrowck/two-phase-reservation-sharing-interference-future-compat-lint.rs:24: unexpected error: '24:9: 24:10: cannot borrow `v` as mutable because it is also borrowed as immutable [E0502]'
2020-03-23T06:57:07.5567216Z 
2020-03-23T06:57:07.5568049Z error: /checkout/src/test/ui/borrowck/two-phase-reservation-sharing-interference-future-compat-lint.rs:24: expected warning not found: cannot borrow `v` as mutable
2020-03-23T06:57:07.5569410Z error: /checkout/src/test/ui/borrowck/two-phase-reservation-sharing-interference-future-compat-lint.rs:24: expected warning not found: may become a hard error in the future
2020-03-23T06:57:07.5569913Z 
2020-03-23T06:57:07.5570754Z error: /checkout/src/test/ui/borrowck/two-phase-reservation-sharing-interference-future-compat-lint.rs:37: expected warning not found: may become a hard error in the future
2020-03-23T06:57:07.5571272Z 
2020-03-23T06:57:07.5571272Z 
2020-03-23T06:57:07.5571524Z error: 2 unexpected errors found, 3 expected errors not found
2020-03-23T06:57:07.5571825Z status: exit code: 1
2020-03-23T06:57:07.5574656Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/two-phase-reservation-sharing-interference-future-compat-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-reservation-sharing-interference-future-compat-lint" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-reservation-sharing-interference-future-compat-lint/auxiliary"
2020-03-23T06:57:07.5576743Z     Error {
2020-03-23T06:57:07.5576950Z         line_num: 13,
2020-03-23T06:57:07.5577196Z         kind: Some(
2020-03-23T06:57:07.5577405Z             Error,
2020-03-23T06:57:07.5577405Z             Error,
2020-03-23T06:57:07.5577587Z         ),
2020-03-23T06:57:07.5577978Z         msg: "13:9: 13:10: cannot borrow `v` as mutable because it is also borrowed as immutable [E0502]",
2020-03-23T06:57:07.5578504Z     Error {
2020-03-23T06:57:07.5578723Z         line_num: 24,
2020-03-23T06:57:07.5578948Z         kind: Some(
2020-03-23T06:57:07.5579156Z             Error,
2020-03-23T06:57:07.5579156Z             Error,
2020-03-23T06:57:07.5579338Z         ),
2020-03-23T06:57:07.5579733Z         msg: "24:9: 24:10: cannot borrow `v` as mutable because it is also borrowed as immutable [E0502]",
2020-03-23T06:57:07.5580238Z ]
2020-03-23T06:57:07.5580586Z 
2020-03-23T06:57:07.5580816Z not found errors (from test file): [
2020-03-23T06:57:07.5581054Z     Error {
2020-03-23T06:57:07.5581054Z     Error {
2020-03-23T06:57:07.5581273Z         line_num: 24,
2020-03-23T06:57:07.5581496Z         kind: Some(
2020-03-23T06:57:07.5581709Z             Warning,
2020-03-23T06:57:07.5581894Z         ),
2020-03-23T06:57:07.5582156Z         msg: "cannot borrow `v` as mutable",
2020-03-23T06:57:07.5582550Z     Error {
2020-03-23T06:57:07.5582767Z         line_num: 24,
2020-03-23T06:57:07.5582992Z         kind: Some(
2020-03-23T06:57:07.5583205Z             Warning,
---
2020-03-23T06:57:07.5586601Z thread '[ui] ui/borrowck/two-phase-reservation-sharing-interference-future-compat-lint.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1436:13
2020-03-23T06:57:07.5587048Z 
2020-03-23T06:57:07.5587560Z ---- [ui] ui/deduplicate-diagnostics-2.rs#deduplicate stdout ----
2020-03-23T06:57:07.5587800Z 
2020-03-23T06:57:07.5588327Z error in revision `deduplicate`: test compilation failed although it shouldn't!
2020-03-23T06:57:07.5588702Z status: exit code: 1
2020-03-23T06:57:07.5591074Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/deduplicate-diagnostics-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "deduplicate" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deduplicate-diagnostics-2.deduplicate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Z" "deduplicate-diagnostics=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deduplicate-diagnostics-2.deduplicate/auxiliary"
2020-03-23T06:57:07.5593013Z ------------------------------------------
2020-03-23T06:57:07.5593207Z 
2020-03-23T06:57:07.5593631Z ------------------------------------------
2020-03-23T06:57:07.5593857Z stderr:
2020-03-23T06:57:07.5593857Z stderr:
2020-03-23T06:57:07.5594264Z ------------------------------------------
2020-03-23T06:57:07.5594809Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5595397Z   --> /checkout/src/test/ui/deduplicate-diagnostics-2.rs:7:9
2020-03-23T06:57:07.5595791Z    |
2020-03-23T06:57:07.5596474Z LL |         1.0 => {} //~ WARNING floating-point types cannot be used in patterns
2020-03-23T06:57:07.5596956Z 
2020-03-23T06:57:07.5597457Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5598047Z   --> /checkout/src/test/ui/deduplicate-diagnostics-2.rs:11:9
2020-03-23T06:57:07.5598321Z    |
2020-03-23T06:57:07.5598321Z    |
2020-03-23T06:57:07.5598867Z LL |         2.0 => {} //~ WARNING floating-point types cannot be used in patterns
2020-03-23T06:57:07.5599328Z 
2020-03-23T06:57:07.5599538Z error: aborting due to 2 previous errors
2020-03-23T06:57:07.5599746Z 
2020-03-23T06:57:07.5599854Z 
2020-03-23T06:57:07.5599854Z 
2020-03-23T06:57:07.5600255Z ------------------------------------------
2020-03-23T06:57:07.5600448Z 
2020-03-23T06:57:07.5600574Z 
2020-03-23T06:57:07.5601039Z ---- [ui] ui/deduplicate-diagnostics-2.rs#duplicate stdout ----
2020-03-23T06:57:07.5601273Z 
2020-03-23T06:57:07.5601797Z error in revision `duplicate`: test compilation failed although it shouldn't!
2020-03-23T06:57:07.5602168Z status: exit code: 1
2020-03-23T06:57:07.5604889Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/deduplicate-diagnostics-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "duplicate" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deduplicate-diagnostics-2.duplicate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deduplicate-diagnostics-2.duplicate/auxiliary"
2020-03-23T06:57:07.5606827Z ------------------------------------------
2020-03-23T06:57:07.5607024Z 
2020-03-23T06:57:07.5607452Z ------------------------------------------
2020-03-23T06:57:07.5607691Z stderr:
2020-03-23T06:57:07.5607691Z stderr:
2020-03-23T06:57:07.5608112Z ------------------------------------------
2020-03-23T06:57:07.5608661Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5609248Z   --> /checkout/src/test/ui/deduplicate-diagnostics-2.rs:7:9
2020-03-23T06:57:07.5609525Z    |
2020-03-23T06:57:07.5610079Z LL |         1.0 => {} //~ WARNING floating-point types cannot be used in patterns
2020-03-23T06:57:07.5610544Z 
2020-03-23T06:57:07.5611000Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5611590Z   --> /checkout/src/test/ui/deduplicate-diagnostics-2.rs:11:9
2020-03-23T06:57:07.5611865Z    |
2020-03-23T06:57:07.5611865Z    |
2020-03-23T06:57:07.5612408Z LL |         2.0 => {} //~ WARNING floating-point types cannot be used in patterns
2020-03-23T06:57:07.5612868Z 
2020-03-23T06:57:07.5613305Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5613917Z   --> /checkout/src/test/ui/deduplicate-diagnostics-2.rs:7:9
2020-03-23T06:57:07.5614194Z    |
2020-03-23T06:57:07.5614194Z    |
2020-03-23T06:57:07.5614726Z LL |         1.0 => {} //~ WARNING floating-point types cannot be used in patterns
2020-03-23T06:57:07.5615207Z 
2020-03-23T06:57:07.5615642Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5616244Z   --> /checkout/src/test/ui/deduplicate-diagnostics-2.rs:11:9
2020-03-23T06:57:07.5616518Z    |
2020-03-23T06:57:07.5616518Z    |
2020-03-23T06:57:07.5617047Z LL |         2.0 => {} //~ WARNING floating-point types cannot be used in patterns
2020-03-23T06:57:07.5617527Z 
2020-03-23T06:57:07.5617736Z error: aborting due to 4 previous errors
2020-03-23T06:57:07.5617926Z 
2020-03-23T06:57:07.5618033Z 
2020-03-23T06:57:07.5618033Z 
2020-03-23T06:57:07.5618455Z ------------------------------------------
2020-03-23T06:57:07.5618646Z 
2020-03-23T06:57:07.5618752Z 
2020-03-23T06:57:07.5619234Z ---- [ui] ui/editions/edition-raw-pointer-method-2015.rs stdout ----
2020-03-23T06:57:07.5619637Z 
2020-03-23T06:57:07.5620666Z error: /checkout/src/test/ui/editions/edition-raw-pointer-method-2015.rs:9: unexpected error: '9:15: 9:22: the type of this value must be known to call a method on a raw pointer on it [E0699]'
2020-03-23T06:57:07.5621207Z 
2020-03-23T06:57:07.5622041Z error: /checkout/src/test/ui/editions/edition-raw-pointer-method-2015.rs:9: expected error not found: type annotations needed [tyvar_behind_raw_pointer]
2020-03-23T06:57:07.5623194Z error: /checkout/src/test/ui/editions/edition-raw-pointer-method-2015.rs:9: expected warning not found: this was previously accepted
2020-03-23T06:57:07.5623599Z 
2020-03-23T06:57:07.5623869Z error: 1 unexpected errors found, 2 expected errors not found
2020-03-23T06:57:07.5624168Z status: exit code: 1
2020-03-23T06:57:07.5624168Z status: exit code: 1
2020-03-23T06:57:07.5626557Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/editions/edition-raw-pointer-method-2015.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-raw-pointer-method-2015" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-raw-pointer-method-2015/auxiliary"
2020-03-23T06:57:07.5628414Z     Error {
2020-03-23T06:57:07.5628618Z         line_num: 9,
2020-03-23T06:57:07.5628843Z         kind: Some(
2020-03-23T06:57:07.5629071Z             Error,
2020-03-23T06:57:07.5629071Z             Error,
2020-03-23T06:57:07.5629255Z         ),
2020-03-23T06:57:07.5629642Z         msg: "9:15: 9:22: the type of this value must be known to call a method on a raw pointer on it [E0699]",
2020-03-23T06:57:07.5630181Z ]
2020-03-23T06:57:07.5630295Z 
2020-03-23T06:57:07.5630516Z not found errors (from test file): [
2020-03-23T06:57:07.5630773Z     Error {
2020-03-23T06:57:07.5630773Z     Error {
2020-03-23T06:57:07.5630972Z         line_num: 9,
2020-03-23T06:57:07.5631193Z         kind: Some(
2020-03-23T06:57:07.5631418Z             Error,
2020-03-23T06:57:07.5631601Z         ),
2020-03-23T06:57:07.5631887Z         msg: "type annotations needed [tyvar_behind_raw_pointer]",
2020-03-23T06:57:07.5632338Z     Error {
2020-03-23T06:57:07.5632536Z         line_num: 9,
2020-03-23T06:57:07.5632758Z         kind: Some(
2020-03-23T06:57:07.5632986Z             Warning,
---
2020-03-23T06:57:07.5638931Z error: /checkout/src/test/ui/feature-gate/issue-43106-gating-of-inline.rs:17: expected warning not found: this was previously accepted
2020-03-23T06:57:07.5639339Z 
2020-03-23T06:57:07.5640099Z error: 1 unexpected errors found, 2 expected errors not found
2020-03-23T06:57:07.5640441Z status: exit code: 1
2020-03-23T06:57:07.5643392Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gate/issue-43106-gating-of-inline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/issue-43106-gating-of-inline" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/issue-43106-gating-of-inline/auxiliary"
2020-03-23T06:57:07.5645540Z     Error {
2020-03-23T06:57:07.5645768Z         line_num: 17,
2020-03-23T06:57:07.5645994Z         kind: Some(
2020-03-23T06:57:07.5646203Z             Error,
---
2020-03-23T06:57:07.5647930Z         line_num: 17,
2020-03-23T06:57:07.5648154Z         kind: Some(
2020-03-23T06:57:07.5648360Z             Error,
2020-03-23T06:57:07.5648541Z         ),
2020-03-23T06:57:07.5648803Z         msg: "attribute must be of the form",
2020-03-23T06:57:07.5649199Z     Error {
2020-03-23T06:57:07.5649417Z         line_num: 17,
2020-03-23T06:57:07.5649640Z         kind: Some(
2020-03-23T06:57:07.5649851Z             Warning,
---
2020-03-23T06:57:07.5654849Z error: /checkout/src/test/ui/feature-gates/feature-gate-default_type_parameter_fallback.rs:8: expected warning not found: this was previously accepted
2020-03-23T06:57:07.5655293Z 
2020-03-23T06:57:07.5655546Z error: 0 unexpected errors found, 2 expected errors not found
2020-03-23T06:57:07.5655865Z status: exit code: 1
2020-03-23T06:57:07.5658346Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-default_type_parameter_fallback.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-default_type_parameter_fallback" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-default_type_parameter_fallback/auxiliary"
2020-03-23T06:57:07.5660239Z     Error {
2020-03-23T06:57:07.5660459Z         line_num: 3,
2020-03-23T06:57:07.5660681Z         kind: Some(
2020-03-23T06:57:07.5660894Z             Warning,
---
2020-03-23T06:57:07.5670637Z error: /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-exhaustive-fail.rs:17: unexpected error: '17:16: 17:35: floating-point types cannot be used in patterns'
2020-03-23T06:57:07.5671140Z 
2020-03-23T06:57:07.5671410Z error: 4 unexpected errors found, 0 expected errors not found
2020-03-23T06:57:07.5671708Z status: exit code: 1
2020-03-23T06:57:07.5674233Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-exhaustive-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/half-open-range-patterns/half-open-range-pats-exhaustive-fail" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/half-open-range-patterns/half-open-range-pats-exhaustive-fail/auxiliary"
2020-03-23T06:57:07.5676414Z     Error {
2020-03-23T06:57:07.5676619Z         line_num: 16,
2020-03-23T06:57:07.5676842Z         kind: Some(
2020-03-23T06:57:07.5677070Z             Error,
---
2020-03-23T06:57:07.5692396Z error: /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-hair-lower-empty.rs:47: unexpected error: '47:16: 47:39: floating-point types cannot be used in patterns'
2020-03-23T06:57:07.5692904Z 
2020-03-23T06:57:07.5693174Z error: 4 unexpected errors found, 0 expected errors not found
2020-03-23T06:57:07.5693474Z status: exit code: 1
2020-03-23T06:57:07.5696003Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-hair-lower-empty.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/half-open-range-patterns/half-open-range-pats-hair-lower-empty" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/half-open-range-patterns/half-open-range-pats-hair-lower-empty/auxiliary"
2020-03-23T06:57:07.5697968Z     Error {
2020-03-23T06:57:07.5698172Z         line_num: 44,
2020-03-23T06:57:07.5698397Z         kind: Some(
2020-03-23T06:57:07.5698605Z             Error,
---
2020-03-23T06:57:07.5707413Z ---- [ui] ui/half-open-range-patterns/half-open-range-pats-semantics.rs stdout ----
2020-03-23T06:57:07.5707772Z 
2020-03-23T06:57:07.5708259Z error: test compilation failed although it shouldn't!
2020-03-23T06:57:07.5708850Z status: exit code: 1
2020-03-23T06:57:07.5711290Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/half-open-range-patterns/half-open-range-pats-semantics/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/half-open-range-patterns/half-open-range-pats-semantics/auxiliary"
2020-03-23T06:57:07.5713197Z ------------------------------------------
2020-03-23T06:57:07.5713409Z 
2020-03-23T06:57:07.5713819Z ------------------------------------------
2020-03-23T06:57:07.5714070Z stderr:
2020-03-23T06:57:07.5714070Z stderr:
2020-03-23T06:57:07.5714482Z ------------------------------------------
2020-03-23T06:57:07.5715006Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5715711Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:46:46
2020-03-23T06:57:07.5716054Z    |
2020-03-23T06:57:07.5716380Z LL |     assert!(yes!(core::f32::NEG_INFINITY, ..=core::f32::NEG_INFINITY));
2020-03-23T06:57:07.5717104Z 
2020-03-23T06:57:07.5717547Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5718250Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:47:46
2020-03-23T06:57:07.5718590Z    |
2020-03-23T06:57:07.5718590Z    |
2020-03-23T06:57:07.5718865Z LL |     assert!(yes!(core::f32::NEG_INFINITY, ..=1.0f32));
2020-03-23T06:57:07.5719486Z 
2020-03-23T06:57:07.5719931Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5720612Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:48:29
2020-03-23T06:57:07.5720969Z    |
2020-03-23T06:57:07.5720969Z    |
2020-03-23T06:57:07.5721190Z LL |     assert!(yes!(1.5f32, ..=1.5f32));
2020-03-23T06:57:07.5721663Z 
2020-03-23T06:57:07.5722123Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5722800Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:49:31
2020-03-23T06:57:07.5723138Z    |
2020-03-23T06:57:07.5723138Z    |
2020-03-23T06:57:07.5723580Z LL |     assert!(!yes!(1.6f32, ..=-1.5f32));
2020-03-23T06:57:07.5724070Z 
2020-03-23T06:57:07.5724778Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5725562Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:52:46
2020-03-23T06:57:07.5725901Z    |
2020-03-23T06:57:07.5725901Z    |
2020-03-23T06:57:07.5726244Z LL |     assert!(yes!(core::f64::NEG_INFINITY, ..=core::f64::NEG_INFINITY));
2020-03-23T06:57:07.5726948Z 
2020-03-23T06:57:07.5727392Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5728093Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:53:46
2020-03-23T06:57:07.5728429Z    |
2020-03-23T06:57:07.5728429Z    |
2020-03-23T06:57:07.5728702Z LL |     assert!(yes!(core::f64::NEG_INFINITY, ..=1.0f64));
2020-03-23T06:57:07.5729311Z 
2020-03-23T06:57:07.5729752Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5730448Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:54:29
2020-03-23T06:57:07.5730998Z    |
2020-03-23T06:57:07.5730998Z    |
2020-03-23T06:57:07.5731222Z LL |     assert!(yes!(1.5f64, ..=1.5f64));
2020-03-23T06:57:07.5731713Z 
2020-03-23T06:57:07.5732203Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5732888Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:55:31
2020-03-23T06:57:07.5733245Z    |
2020-03-23T06:57:07.5733245Z    |
2020-03-23T06:57:07.5733668Z LL |     assert!(!yes!(1.6f64, ..=-1.5f64));
2020-03-23T06:57:07.5734175Z 
2020-03-23T06:57:07.5734614Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5735292Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:90:45
2020-03-23T06:57:07.5735647Z    |
2020-03-23T06:57:07.5735647Z    |
2020-03-23T06:57:07.5735917Z LL |     assert!(yes!(core::f32::NEG_INFINITY, ..1.0f32));
2020-03-23T06:57:07.5736515Z 
2020-03-23T06:57:07.5737041Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5737726Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:91:29
2020-03-23T06:57:07.5738064Z    |
2020-03-23T06:57:07.5738064Z    |
2020-03-23T06:57:07.5738303Z LL |     assert!(!yes!(1.5f32, ..1.5f32));
2020-03-23T06:57:07.5738775Z 
2020-03-23T06:57:07.5739215Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5739916Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:93:28
2020-03-23T06:57:07.5740252Z    |
2020-03-23T06:57:07.5740252Z    |
2020-03-23T06:57:07.5740465Z LL |     assert!(yes!(1.5f32, ..E32));
2020-03-23T06:57:07.5740937Z 
2020-03-23T06:57:07.5741378Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5742085Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:94:29
2020-03-23T06:57:07.5742427Z    |
2020-03-23T06:57:07.5742427Z    |
2020-03-23T06:57:07.5742647Z LL |     assert!(!yes!(1.6f32, ..1.5f32));
2020-03-23T06:57:07.5743138Z 
2020-03-23T06:57:07.5743579Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5744257Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:97:45
2020-03-23T06:57:07.5744614Z    |
2020-03-23T06:57:07.5744614Z    |
2020-03-23T06:57:07.5744885Z LL |     assert!(yes!(core::f64::NEG_INFINITY, ..1.0f64));
2020-03-23T06:57:07.5745488Z 
2020-03-23T06:57:07.5745930Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5746607Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:98:29
2020-03-23T06:57:07.5746962Z    |
2020-03-23T06:57:07.5746962Z    |
2020-03-23T06:57:07.5747182Z LL |     assert!(!yes!(1.5f64, ..1.5f64));
2020-03-23T06:57:07.5747663Z 
2020-03-23T06:57:07.5748126Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5748810Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:100:28
2020-03-23T06:57:07.5749149Z    |
2020-03-23T06:57:07.5749149Z    |
2020-03-23T06:57:07.5749382Z LL |     assert!(yes!(1.5f64, ..E64));
2020-03-23T06:57:07.5749830Z 
2020-03-23T06:57:07.5750287Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5750969Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:101:29
2020-03-23T06:57:07.5751309Z    |
2020-03-23T06:57:07.5751309Z    |
2020-03-23T06:57:07.5751526Z LL |     assert!(!yes!(1.6f64, ..1.5f64));
2020-03-23T06:57:07.5752016Z 
2020-03-23T06:57:07.5752455Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5753281Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:136:43
2020-03-23T06:57:07.5753682Z    |
2020-03-23T06:57:07.5753682Z    |
2020-03-23T06:57:07.5753999Z LL |     assert!(yes!(core::f32::NEG_INFINITY, core::f32::NEG_INFINITY..));
2020-03-23T06:57:07.5754706Z 
2020-03-23T06:57:07.5755190Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5755877Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:137:39
2020-03-23T06:57:07.5756236Z    |
2020-03-23T06:57:07.5756236Z    |
2020-03-23T06:57:07.5756544Z LL |     assert!(yes!(core::f32::INFINITY, core::f32::NEG_INFINITY..));
2020-03-23T06:57:07.5757225Z 
2020-03-23T06:57:07.5757670Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5758351Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:138:44
2020-03-23T06:57:07.5758723Z    |
2020-03-23T06:57:07.5758723Z    |
2020-03-23T06:57:07.5758996Z LL |     assert!(!yes!(core::f32::NEG_INFINITY, 1.0f32..));
2020-03-23T06:57:07.5760589Z 
2020-03-23T06:57:07.5761156Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5761849Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:139:39
2020-03-23T06:57:07.5762190Z    |
2020-03-23T06:57:07.5762190Z    |
2020-03-23T06:57:07.5762473Z LL |     assert!(yes!(core::f32::INFINITY, 1.0f32..));
2020-03-23T06:57:07.5763024Z 
2020-03-23T06:57:07.5763487Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5764173Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:140:48
2020-03-23T06:57:07.5764710Z    |
2020-03-23T06:57:07.5764710Z    |
2020-03-23T06:57:07.5765294Z LL |     assert!(!yes!(1.0f32 - core::f32::EPSILON, 1.0f32..));
2020-03-23T06:57:07.5765931Z 
2020-03-23T06:57:07.5766378Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5767085Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:141:26
2020-03-23T06:57:07.5767425Z    |
2020-03-23T06:57:07.5767425Z    |
2020-03-23T06:57:07.5767642Z LL |     assert!(yes!(1.0f32, 1.0f32..));
2020-03-23T06:57:07.5768122Z 
2020-03-23T06:57:07.5768564Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5769265Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:142:39
2020-03-23T06:57:07.5769605Z    |
2020-03-23T06:57:07.5769605Z    |
2020-03-23T06:57:07.5769866Z LL |     assert!(yes!(core::f32::INFINITY, 1.0f32..));
2020-03-23T06:57:07.5770437Z 
2020-03-23T06:57:07.5770877Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5771575Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:143:39
2020-03-23T06:57:07.5771938Z    |
2020-03-23T06:57:07.5771938Z    |
2020-03-23T06:57:07.5772237Z LL |     assert!(yes!(core::f32::INFINITY, core::f32::INFINITY..));
2020-03-23T06:57:07.5772896Z 
2020-03-23T06:57:07.5773341Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5774020Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:146:43
2020-03-23T06:57:07.5774379Z    |
2020-03-23T06:57:07.5774379Z    |
2020-03-23T06:57:07.5774695Z LL |     assert!(yes!(core::f64::NEG_INFINITY, core::f64::NEG_INFINITY..));
2020-03-23T06:57:07.5775383Z 
2020-03-23T06:57:07.5775847Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5776674Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:147:39
2020-03-23T06:57:07.5777086Z    |
2020-03-23T06:57:07.5777086Z    |
2020-03-23T06:57:07.5777416Z LL |     assert!(yes!(core::f64::INFINITY, core::f64::NEG_INFINITY..));
2020-03-23T06:57:07.5778079Z 
2020-03-23T06:57:07.5778589Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5779278Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:148:44
2020-03-23T06:57:07.5779617Z    |
2020-03-23T06:57:07.5779617Z    |
2020-03-23T06:57:07.5779909Z LL |     assert!(!yes!(core::f64::NEG_INFINITY, 1.0f64..));
2020-03-23T06:57:07.5780491Z 
2020-03-23T06:57:07.5780933Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5781639Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:149:39
2020-03-23T06:57:07.5781977Z    |
2020-03-23T06:57:07.5781977Z    |
2020-03-23T06:57:07.5782246Z LL |     assert!(yes!(core::f64::INFINITY, 1.0f64..));
2020-03-23T06:57:07.5782823Z 
2020-03-23T06:57:07.5783271Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5783971Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:150:48
2020-03-23T06:57:07.5784312Z    |
2020-03-23T06:57:07.5784312Z    |
2020-03-23T06:57:07.5784821Z LL |     assert!(!yes!(1.0f64 - core::f64::EPSILON, 1.0f64..));
2020-03-23T06:57:07.5785455Z 
2020-03-23T06:57:07.5785894Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5786574Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:151:26
2020-03-23T06:57:07.5786933Z    |
2020-03-23T06:57:07.5786933Z    |
2020-03-23T06:57:07.5787150Z LL |     assert!(yes!(1.0f64, 1.0f64..));
2020-03-23T06:57:07.5787628Z 
2020-03-23T06:57:07.5788083Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5788769Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:152:39
2020-03-23T06:57:07.5789127Z    |
2020-03-23T06:57:07.5789127Z    |
2020-03-23T06:57:07.5789388Z LL |     assert!(yes!(core::f64::INFINITY, 1.0f64..));
2020-03-23T06:57:07.5789940Z 
2020-03-23T06:57:07.5790399Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5791079Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:153:39
2020-03-23T06:57:07.5791417Z    |
2020-03-23T06:57:07.5791417Z    |
2020-03-23T06:57:07.5791736Z LL |     assert!(yes!(core::f64::INFINITY, core::f64::INFINITY..));
2020-03-23T06:57:07.5792376Z 
2020-03-23T06:57:07.5792835Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5793522Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:46:46
2020-03-23T06:57:07.5793864Z    |
2020-03-23T06:57:07.5793864Z    |
2020-03-23T06:57:07.5794206Z LL |     assert!(yes!(core::f32::NEG_INFINITY, ..=core::f32::NEG_INFINITY));
2020-03-23T06:57:07.5794909Z 
2020-03-23T06:57:07.5795354Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5796056Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:47:46
2020-03-23T06:57:07.5796393Z    |
2020-03-23T06:57:07.5796393Z    |
2020-03-23T06:57:07.5796665Z LL |     assert!(yes!(core::f32::NEG_INFINITY, ..=1.0f32));
2020-03-23T06:57:07.5797275Z 
2020-03-23T06:57:07.5797852Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5798557Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:48:29
2020-03-23T06:57:07.5798895Z    |
2020-03-23T06:57:07.5798895Z    |
2020-03-23T06:57:07.5799277Z LL |     assert!(yes!(1.5f32, ..=1.5f32));
2020-03-23T06:57:07.5799774Z 
2020-03-23T06:57:07.5800256Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5800938Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:49:31
2020-03-23T06:57:07.5801294Z    |
2020-03-23T06:57:07.5801294Z    |
2020-03-23T06:57:07.5801721Z LL |     assert!(!yes!(1.6f32, ..=-1.5f32));
2020-03-23T06:57:07.5802226Z 
2020-03-23T06:57:07.5802667Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5803345Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:52:46
2020-03-23T06:57:07.5803700Z    |
2020-03-23T06:57:07.5803700Z    |
2020-03-23T06:57:07.5804024Z LL |     assert!(yes!(core::f64::NEG_INFINITY, ..=core::f64::NEG_INFINITY));
2020-03-23T06:57:07.5804982Z 
2020-03-23T06:57:07.5805513Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5806197Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:53:46
2020-03-23T06:57:07.5806534Z    |
2020-03-23T06:57:07.5806534Z    |
2020-03-23T06:57:07.5806825Z LL |     assert!(yes!(core::f64::NEG_INFINITY, ..=1.0f64));
2020-03-23T06:57:07.5807414Z 
2020-03-23T06:57:07.5807874Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5808556Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:54:29
2020-03-23T06:57:07.5808894Z    |
2020-03-23T06:57:07.5808894Z    |
2020-03-23T06:57:07.5809113Z LL |     assert!(yes!(1.5f64, ..=1.5f64));
2020-03-23T06:57:07.5809605Z 
2020-03-23T06:57:07.5810044Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5810746Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:55:31
2020-03-23T06:57:07.5811088Z    |
2020-03-23T06:57:07.5811088Z    |
2020-03-23T06:57:07.5811514Z LL |     assert!(!yes!(1.6f64, ..=-1.5f64));
2020-03-23T06:57:07.5812020Z 
2020-03-23T06:57:07.5812459Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5813140Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:90:45
2020-03-23T06:57:07.5813495Z    |
2020-03-23T06:57:07.5813495Z    |
2020-03-23T06:57:07.5813767Z LL |     assert!(yes!(core::f32::NEG_INFINITY, ..1.0f32));
2020-03-23T06:57:07.5814370Z 
2020-03-23T06:57:07.5814809Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5815484Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:91:29
2020-03-23T06:57:07.5815839Z    |
2020-03-23T06:57:07.5815839Z    |
2020-03-23T06:57:07.5816057Z LL |     assert!(!yes!(1.5f32, ..1.5f32));
2020-03-23T06:57:07.5816539Z 
2020-03-23T06:57:07.5817000Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5817679Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:93:28
2020-03-23T06:57:07.5818015Z    |
2020-03-23T06:57:07.5818015Z    |
2020-03-23T06:57:07.5818246Z LL |     assert!(yes!(1.5f32, ..E32));
2020-03-23T06:57:07.5818695Z 
2020-03-23T06:57:07.5819155Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5819839Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:94:29
2020-03-23T06:57:07.5820176Z    |
2020-03-23T06:57:07.5820176Z    |
2020-03-23T06:57:07.5820414Z LL |     assert!(!yes!(1.6f32, ..1.5f32));
2020-03-23T06:57:07.5820884Z 
2020-03-23T06:57:07.5821321Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5822292Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:97:45
2020-03-23T06:57:07.5822641Z    |
2020-03-23T06:57:07.5822641Z    |
2020-03-23T06:57:07.5822910Z LL |     assert!(yes!(core::f64::NEG_INFINITY, ..1.0f64));
2020-03-23T06:57:07.5823515Z 
2020-03-23T06:57:07.5823994Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5824677Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:98:29
2020-03-23T06:57:07.5825036Z    |
2020-03-23T06:57:07.5825036Z    |
2020-03-23T06:57:07.5825255Z LL |     assert!(!yes!(1.5f64, ..1.5f64));
2020-03-23T06:57:07.5825745Z 
2020-03-23T06:57:07.5826189Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5826873Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:100:28
2020-03-23T06:57:07.5827233Z    |
2020-03-23T06:57:07.5827233Z    |
2020-03-23T06:57:07.5827458Z LL |     assert!(yes!(1.5f64, ..E64));
2020-03-23T06:57:07.5827906Z 
2020-03-23T06:57:07.5828370Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5829055Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:101:29
2020-03-23T06:57:07.5829395Z    |
2020-03-23T06:57:07.5829395Z    |
2020-03-23T06:57:07.5829634Z LL |     assert!(!yes!(1.6f64, ..1.5f64));
2020-03-23T06:57:07.5830106Z 
2020-03-23T06:57:07.5830563Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5831247Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:136:43
2020-03-23T06:57:07.5831586Z    |
2020-03-23T06:57:07.5831586Z    |
2020-03-23T06:57:07.5831923Z LL |     assert!(yes!(core::f32::NEG_INFINITY, core::f32::NEG_INFINITY..));
2020-03-23T06:57:07.5832611Z 
2020-03-23T06:57:07.5833067Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5833780Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:137:39
2020-03-23T06:57:07.5834119Z    |
2020-03-23T06:57:07.5834119Z    |
2020-03-23T06:57:07.5834427Z LL |     assert!(yes!(core::f32::INFINITY, core::f32::NEG_INFINITY..));
2020-03-23T06:57:07.5835108Z 
2020-03-23T06:57:07.5835547Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5836248Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:138:44
2020-03-23T06:57:07.5836636Z    |
2020-03-23T06:57:07.5836636Z    |
2020-03-23T06:57:07.5836910Z LL |     assert!(!yes!(core::f32::NEG_INFINITY, 1.0f32..));
2020-03-23T06:57:07.5837513Z 
2020-03-23T06:57:07.5837969Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5838660Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:139:39
2020-03-23T06:57:07.5839022Z    |
2020-03-23T06:57:07.5839022Z    |
2020-03-23T06:57:07.5839285Z LL |     assert!(yes!(core::f32::INFINITY, 1.0f32..));
2020-03-23T06:57:07.5839859Z 
2020-03-23T06:57:07.5840304Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5840986Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:140:48
2020-03-23T06:57:07.5841342Z    |
2020-03-23T06:57:07.5841342Z    |
2020-03-23T06:57:07.5841849Z LL |     assert!(!yes!(1.0f32 - core::f32::EPSILON, 1.0f32..));
2020-03-23T06:57:07.5842465Z 
2020-03-23T06:57:07.5842929Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5843614Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:141:26
2020-03-23T06:57:07.5843953Z    |
2020-03-23T06:57:07.5843953Z    |
2020-03-23T06:57:07.5844342Z LL |     assert!(yes!(1.0f32, 1.0f32..));
2020-03-23T06:57:07.5845082Z 
2020-03-23T06:57:07.5845635Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5846330Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:142:39
2020-03-23T06:57:07.5846670Z    |
2020-03-23T06:57:07.5846670Z    |
2020-03-23T06:57:07.5846931Z LL |     assert!(yes!(core::f32::INFINITY, 1.0f32..));
2020-03-23T06:57:07.5847503Z 
2020-03-23T06:57:07.5847946Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5848653Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:143:39
2020-03-23T06:57:07.5848992Z    |
2020-03-23T06:57:07.5848992Z    |
2020-03-23T06:57:07.5849292Z LL |     assert!(yes!(core::f32::INFINITY, core::f32::INFINITY..));
2020-03-23T06:57:07.5849966Z 
2020-03-23T06:57:07.5850413Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5851095Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:146:43
2020-03-23T06:57:07.5851455Z    |
2020-03-23T06:57:07.5851455Z    |
2020-03-23T06:57:07.5851772Z LL |     assert!(yes!(core::f64::NEG_INFINITY, core::f64::NEG_INFINITY..));
2020-03-23T06:57:07.5852478Z 
2020-03-23T06:57:07.5852917Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5853597Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:147:39
2020-03-23T06:57:07.5853957Z    |
2020-03-23T06:57:07.5853957Z    |
2020-03-23T06:57:07.5854263Z LL |     assert!(yes!(core::f64::INFINITY, core::f64::NEG_INFINITY..));
2020-03-23T06:57:07.5854944Z 
2020-03-23T06:57:07.5855392Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5856082Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:148:44
2020-03-23T06:57:07.5856440Z    |
2020-03-23T06:57:07.5856440Z    |
2020-03-23T06:57:07.5856712Z LL |     assert!(!yes!(core::f64::NEG_INFINITY, 1.0f64..));
2020-03-23T06:57:07.5857292Z 
2020-03-23T06:57:07.5857753Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5858435Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:149:39
2020-03-23T06:57:07.5858774Z    |
2020-03-23T06:57:07.5858774Z    |
2020-03-23T06:57:07.5859054Z LL |     assert!(yes!(core::f64::INFINITY, 1.0f64..));
2020-03-23T06:57:07.5859606Z 
2020-03-23T06:57:07.5860045Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5860750Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:150:48
2020-03-23T06:57:07.5861101Z    |
2020-03-23T06:57:07.5861101Z    |
2020-03-23T06:57:07.5861611Z LL |     assert!(!yes!(1.0f64 - core::f64::EPSILON, 1.0f64..));
2020-03-23T06:57:07.5862251Z 
2020-03-23T06:57:07.5862691Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5863395Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:151:26
2020-03-23T06:57:07.5863734Z    |
2020-03-23T06:57:07.5863734Z    |
2020-03-23T06:57:07.5863951Z LL |     assert!(yes!(1.0f64, 1.0f64..));
2020-03-23T06:57:07.5864428Z 
2020-03-23T06:57:07.5865720Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5866466Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:152:39
2020-03-23T06:57:07.5867561Z    |
2020-03-23T06:57:07.5867561Z    |
2020-03-23T06:57:07.5867820Z LL |     assert!(yes!(core::f64::INFINITY, 1.0f64..));
2020-03-23T06:57:07.5868619Z 
2020-03-23T06:57:07.5869175Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.5869869Z   --> /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-semantics.rs:153:39
2020-03-23T06:57:07.5870227Z    |
2020-03-23T06:57:07.5870227Z    |
2020-03-23T06:57:07.5870528Z LL |     assert!(yes!(core::f64::INFINITY, core::f64::INFINITY..));
2020-03-23T06:57:07.5871170Z 
2020-03-23T06:57:07.5871399Z error: aborting due to 64 previous errors
2020-03-23T06:57:07.5871591Z 
2020-03-23T06:57:07.5871699Z 
---
2020-03-23T06:57:07.5875035Z error: /checkout/src/test/ui/imports/local-modularized-tricky-fail-3.rs:19: expected warning not found: this was previously accepted
2020-03-23T06:57:07.5875460Z 
2020-03-23T06:57:07.5875713Z error: 0 unexpected errors found, 2 expected errors not found
2020-03-23T06:57:07.5876013Z status: exit code: 1
2020-03-23T06:57:07.5878354Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/local-modularized-tricky-fail-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/local-modularized-tricky-fail-3" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/local-modularized-tricky-fail-3/auxiliary"
2020-03-23T06:57:07.5880139Z     Error {
2020-03-23T06:57:07.5880346Z         line_num: 13,
2020-03-23T06:57:07.5880571Z         kind: Some(
2020-03-23T06:57:07.5880804Z             Warning,
---
2020-03-23T06:57:07.5885185Z ---- [ui] ui/inference/inference-variable-behind-raw-pointer.rs stdout ----
2020-03-23T06:57:07.5885448Z 
2020-03-23T06:57:07.5885914Z error: test compilation failed although it shouldn't!
2020-03-23T06:57:07.5886203Z status: exit code: 1
2020-03-23T06:57:07.5888701Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/inference-variable-behind-raw-pointer.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/inference-variable-behind-raw-pointer" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/inference-variable-behind-raw-pointer/auxiliary"
2020-03-23T06:57:07.5890769Z ------------------------------------------
2020-03-23T06:57:07.5890966Z 
2020-03-23T06:57:07.5891374Z ------------------------------------------
2020-03-23T06:57:07.5891600Z stderr:
---
2020-03-23T06:57:07.5896465Z ---- [ui] ui/issues/issue-15881-model-lexer-dotdotdot.rs stdout ----
2020-03-23T06:57:07.5896710Z 
2020-03-23T06:57:07.5897144Z error: test compilation failed although it shouldn't!
2020-03-23T06:57:07.5897431Z status: exit code: 1
2020-03-23T06:57:07.5899655Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-15881-model-lexer-dotdotdot.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-15881-model-lexer-dotdotdot/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-15881-model-lexer-dotdotdot/auxiliary"
2020-03-23T06:57:07.5901481Z ------------------------------------------
2020-03-23T06:57:07.5901675Z 
2020-03-23T06:57:07.5902090Z ------------------------------------------
2020-03-23T06:57:07.5902315Z stderr:
---
2020-03-23T06:57:07.5921993Z error: /checkout/src/test/ui/issues/issue-39404.rs:3: expected warning not found: previously accepted
2020-03-23T06:57:07.5922351Z 
2020-03-23T06:57:07.5922607Z error: 0 unexpected errors found, 1 expected errors not found
2020-03-23T06:57:07.5922907Z status: exit code: 1
2020-03-23T06:57:07.5925219Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-39404.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39404" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39404/auxiliary"
2020-03-23T06:57:07.5926901Z     Error {
2020-03-23T06:57:07.5927104Z         line_num: 3,
2020-03-23T06:57:07.5927345Z         kind: Some(
2020-03-23T06:57:07.5927559Z             Warning,
---
2020-03-23T06:57:07.5952725Z error: /checkout/src/test/ui/issues/issue-41255.rs:69: expected warning not found: hard error
2020-03-23T06:57:07.5953043Z 
2020-03-23T06:57:07.5953315Z error: 0 unexpected errors found, 24 expected errors not found
2020-03-23T06:57:07.5953616Z status: exit code: 1
2020-03-23T06:57:07.5955774Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-41255.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41255" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41255/auxiliary"
2020-03-23T06:57:07.5957440Z     Error {
2020-03-23T06:57:07.5957647Z         line_num: 11,
2020-03-23T06:57:07.5957872Z         kind: Some(
2020-03-23T06:57:07.5958104Z             Warning,
---
2020-03-23T06:57:07.5998908Z error: /checkout/src/test/ui/issues/issue-6804.rs:19: expected warning not found: this was previously accepted by the compiler but is being phased out
2020-03-23T06:57:07.5999336Z 
2020-03-23T06:57:07.5999608Z error: 0 unexpected errors found, 4 expected errors not found
2020-03-23T06:57:07.5999907Z status: exit code: 1
2020-03-23T06:57:07.6002039Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-6804.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-6804" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-6804/auxiliary"
2020-03-23T06:57:07.6003698Z     Error {
2020-03-23T06:57:07.6003921Z         line_num: 11,
2020-03-23T06:57:07.6004147Z         kind: Some(
2020-03-23T06:57:07.6004360Z             Warning,
---
2020-03-23T06:57:07.6012018Z ---- [ui] ui/issues/issue-7222.rs stdout ----
2020-03-23T06:57:07.6012239Z 
2020-03-23T06:57:07.6012678Z error: test compilation failed although it shouldn't!
2020-03-23T06:57:07.6012966Z status: exit code: 1
2020-03-23T06:57:07.6017553Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-7222.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-7222/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-7222/auxiliary"
2020-03-23T06:57:07.6019286Z ------------------------------------------
2020-03-23T06:57:07.6019483Z 
2020-03-23T06:57:07.6020155Z ------------------------------------------
2020-03-23T06:57:07.6020393Z stderr:
---
2020-03-23T06:57:07.6052550Z error: /checkout/src/test/ui/malformed/malformed-regressions.rs:9: expected warning not found: this was previously accepted
2020-03-23T06:57:07.6052930Z 
2020-03-23T06:57:07.6053184Z error: 5 unexpected errors found, 10 expected errors not found
2020-03-23T06:57:07.6053505Z status: exit code: 1
2020-03-23T06:57:07.6055751Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/malformed/malformed-regressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/malformed/malformed-regressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/malformed/malformed-regressions/auxiliary"
2020-03-23T06:57:07.6057510Z     Error {
2020-03-23T06:57:07.6057733Z         line_num: 1,
2020-03-23T06:57:07.6057955Z         kind: Some(
2020-03-23T06:57:07.6058165Z             Error,
---
2020-03-23T06:57:07.6066077Z         line_num: 1,
2020-03-23T06:57:07.6066313Z         kind: Some(
2020-03-23T06:57:07.6066522Z             Error,
2020-03-23T06:57:07.6066703Z         ),
2020-03-23T06:57:07.6066945Z         msg: "attribute must be of the form",
2020-03-23T06:57:07.6067358Z     Error {
2020-03-23T06:57:07.6067555Z         line_num: 1,
2020-03-23T06:57:07.6067795Z         kind: Some(
2020-03-23T06:57:07.6068006Z             Warning,
---
2020-03-23T06:57:07.6069194Z         line_num: 3,
2020-03-23T06:57:07.6069442Z         kind: Some(
2020-03-23T06:57:07.6069650Z             Error,
2020-03-23T06:57:07.6069831Z         ),
2020-03-23T06:57:07.6070091Z         msg: "attribute must be of the form",
2020-03-23T06:57:07.6070484Z     Error {
2020-03-23T06:57:07.6070700Z         line_num: 3,
2020-03-23T06:57:07.6070922Z         kind: Some(
2020-03-23T06:57:07.6071133Z             Warning,
---
2020-03-23T06:57:07.6072186Z         line_num: 5,
2020-03-23T06:57:07.6072407Z         kind: Some(
2020-03-23T06:57:07.6072614Z             Error,
2020-03-23T06:57:07.6072813Z         ),
2020-03-23T06:57:07.6073054Z         msg: "attribute must be of the form",
2020-03-23T06:57:07.6073465Z     Error {
2020-03-23T06:57:07.6073673Z         line_num: 5,
2020-03-23T06:57:07.6073894Z         kind: Some(
2020-03-23T06:57:07.6074105Z             Warning,
---
2020-03-23T06:57:07.6075157Z         line_num: 7,
2020-03-23T06:57:07.6075378Z         kind: Some(
2020-03-23T06:57:07.6075602Z             Error,
2020-03-23T06:57:07.6075785Z         ),
2020-03-23T06:57:07.6076026Z         msg: "attribute must be of the form",
2020-03-23T06:57:07.6076437Z     Error {
2020-03-23T06:57:07.6076634Z         line_num: 7,
2020-03-23T06:57:07.6076854Z         kind: Some(
2020-03-23T06:57:07.6077080Z             Warning,
---
2020-03-23T06:57:07.6078118Z         line_num: 9,
2020-03-23T06:57:07.6078355Z         kind: Some(
2020-03-23T06:57:07.6078564Z             Error,
2020-03-23T06:57:07.6078745Z         ),
2020-03-23T06:57:07.6078986Z         msg: "attribute must be of the form",
2020-03-23T06:57:07.6079396Z     Error {
2020-03-23T06:57:07.6079592Z         line_num: 9,
2020-03-23T06:57:07.6079828Z         kind: Some(
2020-03-23T06:57:07.6080039Z             Warning,
---
2020-03-23T06:57:07.6083387Z error: /checkout/src/test/ui/no-patterns-in-args-2.rs:4: expected warning not found: was previously accepted
2020-03-23T06:57:07.6083747Z 
2020-03-23T06:57:07.6083998Z error: 0 unexpected errors found, 1 expected errors not found
2020-03-23T06:57:07.6084316Z status: exit code: 1
2020-03-23T06:57:07.6086657Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/no-patterns-in-args-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-patterns-in-args-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-patterns-in-args-2/auxiliary"
2020-03-23T06:57:07.6088340Z     Error {
2020-03-23T06:57:07.6088739Z         line_num: 4,
2020-03-23T06:57:07.6088969Z         kind: Some(
2020-03-23T06:57:07.6089181Z             Warning,
---
2020-03-23T06:57:07.6113988Z error: /checkout/src/test/ui/pattern/usefulness/match-range-fail-dominate.rs:42: expected warning not found: this was previously accepted by the compiler
2020-03-23T06:57:07.6114427Z 
2020-03-23T06:57:07.6114700Z error: 6 unexpected errors found, 12 expected errors not found
2020-03-23T06:57:07.6115002Z status: exit code: 1
2020-03-23T06:57:07.6117357Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/usefulness/match-range-fail-dominate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/match-range-fail-dominate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/match-range-fail-dominate/auxiliary"
2020-03-23T06:57:07.6119204Z     Error {
2020-03-23T06:57:07.6119409Z         line_num: 33,
2020-03-23T06:57:07.6119634Z         kind: Some(
2020-03-23T06:57:07.6119843Z             Error,
---
2020-03-23T06:57:07.6166392Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-float-range-match.rs:11: unexpected error: '11:13: 11:16: floating-point types cannot be used in patterns'
2020-03-23T06:57:07.6166906Z 
2020-03-23T06:57:07.6167164Z error: 8 unexpected errors found, 0 expected errors not found
2020-03-23T06:57:07.6167463Z status: exit code: 1
2020-03-23T06:57:07.6169905Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/usefulness/non-exhaustive-float-range-match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/non-exhaustive-float-range-match" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/non-exhaustive-float-range-match/auxiliary"
2020-03-23T06:57:07.6171776Z     Error {
2020-03-23T06:57:07.6171989Z         line_num: 6,
2020-03-23T06:57:07.6172232Z         kind: Some(
2020-03-23T06:57:07.6172441Z             Error,
---
2020-03-23T06:57:07.6208703Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:49: unexpected error: '49:10: 49:13: floating-point types cannot be used in patterns'
2020-03-23T06:57:07.6209174Z 
2020-03-23T06:57:07.6209429Z error: 12 unexpected errors found, 0 expected errors not found
2020-03-23T06:57:07.6210117Z status: exit code: 1
2020-03-23T06:57:07.6213496Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/non-exhaustive-match" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/non-exhaustive-match/auxiliary"
2020-03-23T06:57:07.6215321Z     Error {
2020-03-23T06:57:07.6215528Z         line_num: 47,
2020-03-23T06:57:07.6215752Z         kind: Some(
2020-03-23T06:57:07.6215980Z             Error,
---
2020-03-23T06:57:07.6240753Z thread '[ui] ui/pattern/usefulness/non-exhaustive-match.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1436:13
2020-03-23T06:57:07.6241158Z 
2020-03-23T06:57:07.6241600Z ---- [ui] ui/proc-macro/generate-mod.rs stdout ----
2020-03-23T06:57:07.6241813Z 
2020-03-23T06:57:07.6242539Z error: /checkout/src/test/ui/proc-macro/generate-mod.rs:16: unexpected error: '16:10: 16:35: cannot find type `FromOutside` in this scope'
2020-03-23T06:57:07.6243698Z error: /checkout/src/test/ui/proc-macro/generate-mod.rs:16: unexpected error: '16:10: 16:35: cannot find type `OuterDerive` in this scope'
2020-03-23T06:57:07.6244110Z 
2020-03-23T06:57:07.6244110Z 
2020-03-23T06:57:07.6245036Z error: /checkout/src/test/ui/proc-macro/generate-mod.rs:23: unexpected error: '23:14: 23:39: cannot find type `FromOutside` in this scope'
2020-03-23T06:57:07.6246211Z error: /checkout/src/test/ui/proc-macro/generate-mod.rs:23: unexpected error: '23:14: 23:39: cannot find type `OuterDerive` in this scope'
2020-03-23T06:57:07.6246629Z 
2020-03-23T06:57:07.6246629Z 
2020-03-23T06:57:07.6247374Z error: /checkout/src/test/ui/proc-macro/generate-mod.rs:30: unexpected error: '30:10: 30:39: cannot find type `FromOutside` in this scope'
2020-03-23T06:57:07.6247785Z 
2020-03-23T06:57:07.6248517Z error: /checkout/src/test/ui/proc-macro/generate-mod.rs:30: unexpected error: '30:10: 30:39: cannot find type `OuterDeriveLint` in this scope'
2020-03-23T06:57:07.6249652Z error: /checkout/src/test/ui/proc-macro/generate-mod.rs:16: expected warning not found: cannot find type `FromOutside` in this scope
2020-03-23T06:57:07.6250048Z 
2020-03-23T06:57:07.6250741Z error: /checkout/src/test/ui/proc-macro/generate-mod.rs:16: expected warning not found: cannot find type `OuterDerive` in this scope
2020-03-23T06:57:07.6251158Z 
---
2020-03-23T06:57:07.6260325Z error: /checkout/src/test/ui/proc-macro/generate-mod.rs:23: expected warning not found: this was previously accepted
2020-03-23T06:57:07.6260690Z 
2020-03-23T06:57:07.6260956Z error: 6 unexpected errors found, 8 expected errors not found
2020-03-23T06:57:07.6261282Z status: exit code: 1
2020-03-23T06:57:07.6263467Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/generate-mod.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/generate-mod" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/generate-mod/auxiliary"
2020-03-23T06:57:07.6265180Z     Error {
2020-03-23T06:57:07.6265921Z         line_num: 16,
2020-03-23T06:57:07.6266173Z         kind: Some(
2020-03-23T06:57:07.6266624Z             Error,
2020-03-23T06:57:07.6266624Z             Error,
2020-03-23T06:57:07.6266819Z         ),
2020-03-23T06:57:07.6269930Z         msg: "16:10: 16:35: cannot find type `FromOutside` in this scope",
2020-03-23T06:57:07.6270430Z     Error {
2020-03-23T06:57:07.6270655Z         line_num: 16,
2020-03-23T06:57:07.6270878Z         kind: Some(
2020-03-23T06:57:07.6271085Z             Error,
---
2020-03-23T06:57:07.6272277Z         line_num: 23,
2020-03-23T06:57:07.6272499Z         kind: Some(
2020-03-23T06:57:07.6272707Z             Error,
2020-03-23T06:57:07.6272908Z         ),
2020-03-23T06:57:07.6273218Z         msg: "23:14: 23:39: cannot find type `FromOutside` in this scope",
2020-03-23T06:57:07.6273695Z     Error {
2020-03-23T06:57:07.6273895Z         line_num: 23,
2020-03-23T06:57:07.6274118Z         kind: Some(
2020-03-23T06:57:07.6274352Z             Error,
---
2020-03-23T06:57:07.6275522Z         line_num: 30,
2020-03-23T06:57:07.6275744Z         kind: Some(
2020-03-23T06:57:07.6275971Z             Error,
2020-03-23T06:57:07.6276151Z         ),
2020-03-23T06:57:07.6276460Z         msg: "30:10: 30:39: cannot find type `FromOutside` in this scope",
2020-03-23T06:57:07.6276936Z     Error {
2020-03-23T06:57:07.6277136Z         line_num: 30,
2020-03-23T06:57:07.6277378Z         kind: Some(
2020-03-23T06:57:07.6277586Z             Error,
---
2020-03-23T06:57:07.6301478Z error: /checkout/src/test/ui/rfc1445/match-forbidden-without-eq.rs:21: expected warning not found: this was previously accepted by the compiler but is being phased out
2020-03-23T06:57:07.6301952Z 
2020-03-23T06:57:07.6302204Z error: 2 unexpected errors found, 4 expected errors not found
2020-03-23T06:57:07.6302524Z status: exit code: 1
2020-03-23T06:57:07.6304816Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1445/match-forbidden-without-eq.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/match-forbidden-without-eq" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1445/match-forbidden-without-eq/auxiliary"
2020-03-23T06:57:07.6306600Z     Error {
2020-03-23T06:57:07.6306805Z         line_num: 21,
2020-03-23T06:57:07.6307048Z         kind: Some(
2020-03-23T06:57:07.6307257Z             Error,
---
2020-03-23T06:57:07.6318546Z thread '[ui] ui/rfc1445/match-forbidden-without-eq.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1436:13
2020-03-23T06:57:07.6318942Z 
2020-03-23T06:57:07.6319480Z ---- [ui] ui/type-inference/unbounded-type-param-in-fn-with-assoc-type.rs stdout ----
2020-03-23T06:57:07.6319758Z 
2020-03-23T06:57:07.6320860Z error: /checkout/src/test/ui/type-inference/unbounded-type-param-in-fn-with-assoc-type.rs:3: unexpected error: '3:11: 3:12: defaults for type parameters are only allowed in `struct`, `enum`, `type`, or `trait` definitions.'
2020-03-23T06:57:07.6322297Z error: /checkout/src/test/ui/type-inference/unbounded-type-param-in-fn-with-assoc-type.rs:8: expected error not found: type annotations needed
2020-03-23T06:57:07.6322751Z 
2020-03-23T06:57:07.6323005Z error: 1 unexpected errors found, 1 expected errors not found
2020-03-23T06:57:07.6323305Z status: exit code: 1
2020-03-23T06:57:07.6323305Z status: exit code: 1
2020-03-23T06:57:07.6325968Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-inference/unbounded-type-param-in-fn-with-assoc-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-inference/unbounded-type-param-in-fn-with-assoc-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-inference/unbounded-type-param-in-fn-with-assoc-type/auxiliary"
2020-03-23T06:57:07.6327876Z     Error {
2020-03-23T06:57:07.6328078Z         line_num: 3,
2020-03-23T06:57:07.6328317Z         kind: Some(
2020-03-23T06:57:07.6328527Z             Error,
2020-03-23T06:57:07.6328527Z             Error,
2020-03-23T06:57:07.6328708Z         ),
2020-03-23T06:57:07.6329116Z         msg: "3:11: 3:12: defaults for type parameters are only allowed in `struct`, `enum`, `type`, or `trait` definitions.",
2020-03-23T06:57:07.6329674Z ]
2020-03-23T06:57:07.6329785Z 
2020-03-23T06:57:07.6330020Z not found errors (from test file): [
2020-03-23T06:57:07.6330258Z     Error {
---
2020-03-23T06:57:07.6336492Z ---- [ui] ui/union/union-pat-refutability.rs stdout ----
2020-03-23T06:57:07.6336767Z 
2020-03-23T06:57:07.6337334Z error: test compilation failed although it shouldn't!
2020-03-23T06:57:07.6337617Z status: exit code: 1
2020-03-23T06:57:07.6340001Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/union-pat-refutability.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-pat-refutability/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-pat-refutability/auxiliary"
2020-03-23T06:57:07.6341831Z ------------------------------------------
2020-03-23T06:57:07.6342031Z 
2020-03-23T06:57:07.6342441Z ------------------------------------------
2020-03-23T06:57:07.6342668Z stderr:
2020-03-23T06:57:07.6342668Z stderr:
2020-03-23T06:57:07.6343101Z ------------------------------------------
2020-03-23T06:57:07.6343623Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.6344223Z   --> /checkout/src/test/ui/union/union-pat-refutability.rs:24:44
2020-03-23T06:57:07.6344529Z    |
2020-03-23T06:57:07.6344828Z LL |             Value { tag: Tag::F, u: U { f: 0.0 } } => true,
2020-03-23T06:57:07.6345744Z 
2020-03-23T06:57:07.6346262Z error: floating-point types cannot be used in patterns
2020-03-23T06:57:07.6346868Z   --> /checkout/src/test/ui/union/union-pat-refutability.rs:24:44
2020-03-23T06:57:07.6347169Z    |
2020-03-23T06:57:07.6347169Z    |
2020-03-23T06:57:07.6347461Z LL |             Value { tag: Tag::F, u: U { f: 0.0 } } => true,
2020-03-23T06:57:07.6348049Z 
2020-03-23T06:57:07.6348277Z error: aborting due to 2 previous errors
2020-03-23T06:57:07.6348467Z 
2020-03-23T06:57:07.6348574Z 
---
2020-03-23T06:57:07.6368313Z 
2020-03-23T06:57:07.6368949Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-23T06:57:07.6369240Z 
2020-03-23T06:57:07.6369348Z 
2020-03-23T06:57:07.6373720Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-23T06:57:07.6376859Z 
2020-03-23T06:57:07.6376971Z 
2020-03-23T06:57:07.6377594Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-23T06:57:07.6378018Z Build completed unsuccessfully in 1:03:04
2020-03-23T06:57:07.6378018Z Build completed unsuccessfully in 1:03:04
2020-03-23T06:57:07.6378316Z == clock drift check ==
2020-03-23T06:57:07.6378592Z   local time: Mon Mar 23 06:57:07 UTC 2020
2020-03-23T06:57:07.7228822Z   network time: Mon, 23 Mar 2020 06:57:07 GMT
2020-03-23T06:57:07.7229442Z == end clock drift check ==
2020-03-23T06:57:08.2235454Z 
2020-03-23T06:57:08.2320656Z ##[error]Bash exited with code '1'.
2020-03-23T06:57:08.2341463Z ##[section]Finishing: Run build
2020-03-23T06:57:08.2386720Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70298/merge to s
2020-03-23T06:57:08.2391255Z Task         : Get sources
2020-03-23T06:57:08.2391556Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T06:57:08.2391846Z Version      : 1.0.0
2020-03-23T06:57:08.2392043Z Author       : Microsoft
2020-03-23T06:57:08.2392043Z Author       : Microsoft
2020-03-23T06:57:08.2392365Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-23T06:57:08.2392734Z ==============================================================================
2020-03-23T06:57:08.5624019Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-23T06:57:08.5669867Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70298/merge to s
2020-03-23T06:57:08.5770258Z Cleaning up task key
2020-03-23T06:57:08.5771644Z Start cleaning up orphan processes.
2020-03-23T06:57:08.6029753Z Terminate orphan process: pid (3400) (python)
2020-03-23T06:57:08.6215095Z ##[section]Finishing: Finalize Job
