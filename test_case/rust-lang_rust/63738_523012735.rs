plain
2019-08-20T12:10:04.9940976Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-20T12:10:05.0125777Z ##[command]git config gc.auto 0
2019-08-20T12:10:05.5726494Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-20T12:10:05.5737628Z ##[command]git config --get-all http.proxy
2019-08-20T12:10:05.5744147Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63738/merge:refs/remotes/pull/63738/merge
---
2019-08-20T12:10:41.3554697Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-20T12:10:41.3554748Z 
2019-08-20T12:10:41.3554983Z   git checkout -b <new-branch-name>
2019-08-20T12:10:41.3555014Z 
2019-08-20T12:10:41.3555063Z HEAD is now at ffbc86457 Merge c139039a7b15b8ffdc083e5462aff42811d12300 into 14890954ce17c44d944eda988c5a64bb4c5ec9eb
2019-08-20T12:10:41.3681261Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-20T12:10:41.3684445Z ==============================================================================
2019-08-20T12:10:41.3684503Z Task         : Bash
2019-08-20T12:10:41.3684567Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-20T13:12:39.6915922Z .................................................................................................... 1500/8943
2019-08-20T13:12:45.3118451Z .................................................................................................... 1600/8943
2019-08-20T13:12:58.3059185Z .......................................i...............i............................................ 1700/8943
2019-08-20T13:13:06.4757735Z .................................................................................................... 1800/8943
2019-08-20T13:13:20.8797335Z ...............................iiiii................................................................ 1900/8943
2019-08-20T13:13:31.6677684Z .................................................................................................... 2100/8943
2019-08-20T13:13:34.3179838Z .................................................................................................... 2200/8943
2019-08-20T13:13:39.0037684Z .................................................................................................... 2300/8943
2019-08-20T13:13:46.0377948Z .................................................................................................... 2400/8943
---
2019-08-20T13:16:47.2571824Z ...................i...............i................................................................ 4700/8943
2019-08-20T13:16:58.7819291Z .................................................................................................... 4800/8943
2019-08-20T13:17:05.0660682Z .................................................................................................... 4900/8943
2019-08-20T13:17:16.2220915Z .................................................................................................... 5000/8943
2019-08-20T13:17:21.4825707Z ii.ii............................................................................................... 5100/8943
2019-08-20T13:17:36.1873707Z .................................................................................................... 5300/8943
2019-08-20T13:17:43.1156925Z ........................................................i........................................... 5400/8943
2019-08-20T13:17:49.9793765Z .................................................................................................... 5500/8943
2019-08-20T13:17:59.6540624Z .................................................................................................... 5600/8943
2019-08-20T13:17:59.6540624Z .................................................................................................... 5600/8943
2019-08-20T13:18:09.3112956Z .................................................ii...i..ii...........i............................. 5700/8943
2019-08-20T13:18:31.4927737Z .................................................................................................... 5900/8943
2019-08-20T13:18:36.3657393Z .................................................................................................... 6000/8943
2019-08-20T13:18:36.3657393Z .................................................................................................... 6000/8943
2019-08-20T13:18:47.2407396Z .................................................Fi..ii............................................. 6100/8943
2019-08-20T13:19:10.9413968Z ................................................................................................i... 6300/8943
2019-08-20T13:19:13.2227415Z .................................................................................................... 6400/8943
2019-08-20T13:19:15.5544413Z ....................................................................i............................... 6500/8943
2019-08-20T13:19:18.6084422Z .................................................................................................... 6600/8943
---
2019-08-20T13:23:18.0269319Z 
2019-08-20T13:23:18.0269845Z ---- [ui] ui/non-integer-atomic.rs stdout ----
2019-08-20T13:23:18.0269907Z diff of stderr:
2019-08-20T13:23:18.0269944Z 
2019-08-20T13:23:18.0270018Z 1 error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `bool`
2019-08-20T13:23:18.0270498Z +   --> $DIR/non-integer-atomic.rs:13:5
2019-08-20T13:23:18.0270563Z 3    |
2019-08-20T13:23:18.0270608Z 4 LL |     intrinsics::atomic_load(p);
2019-08-20T13:23:18.0271005Z 5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:23:18.0271005Z 5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:23:18.0271052Z 
2019-08-20T13:23:18.0271109Z 6 
2019-08-20T13:23:18.0271378Z 7 error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `bool`
2019-08-20T13:23:18.0273192Z +   --> $DIR/non-integer-atomic.rs:18:5
2019-08-20T13:23:18.0273473Z 9    |
2019-08-20T13:23:18.0273473Z 9    |
2019-08-20T13:23:18.0273699Z 10 LL |     intrinsics::atomic_store(p, v);
2019-08-20T13:23:18.0274287Z 
2019-08-20T13:23:18.0274489Z 12 
2019-08-20T13:23:18.0274489Z 12 
2019-08-20T13:23:18.0274724Z 13 error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `bool`
2019-08-20T13:23:18.0275650Z +   --> $DIR/non-integer-atomic.rs:23:5
2019-08-20T13:23:18.0275918Z 15    |
2019-08-20T13:23:18.0275918Z 15    |
2019-08-20T13:23:18.0276129Z 16 LL |     intrinsics::atomic_xchg(p, v);
2019-08-20T13:23:18.0277684Z 
2019-08-20T13:23:18.0277976Z 18 
2019-08-20T13:23:18.0277976Z 18 
2019-08-20T13:23:18.0278210Z 19 error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `bool`
2019-08-20T13:23:18.0290708Z +   --> $DIR/non-integer-atomic.rs:28:5
2019-08-20T13:23:18.0291029Z 21    |
2019-08-20T13:23:18.0291029Z 21    |
2019-08-20T13:23:18.0291261Z 22 LL |     intrinsics::atomic_cxchg(p, v, v);
2019-08-20T13:23:18.0291696Z 
2019-08-20T13:23:18.0291917Z 24 
2019-08-20T13:23:18.0291917Z 24 
2019-08-20T13:23:18.0292141Z 25 error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `Foo`
2019-08-20T13:23:18.0293707Z +   --> $DIR/non-integer-atomic.rs:33:5
2019-08-20T13:23:18.0293957Z 27    |
2019-08-20T13:23:18.0294113Z 28 LL |     intrinsics::atomic_load(p);
2019-08-20T13:23:18.0294279Z 29    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:23:18.0294279Z 29    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:23:18.0294432Z 
2019-08-20T13:23:18.0294563Z 30 
2019-08-20T13:23:18.0294722Z 31 error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `Foo`
2019-08-20T13:23:18.0295558Z +   --> $DIR/non-integer-atomic.rs:38:5
2019-08-20T13:23:18.0295752Z 33    |
2019-08-20T13:23:18.0295752Z 33    |
2019-08-20T13:23:18.0295894Z 34 LL |     intrinsics::atomic_store(p, v);
2019-08-20T13:23:18.0296160Z 
2019-08-20T13:23:18.0296313Z 36 
2019-08-20T13:23:18.0296313Z 36 
2019-08-20T13:23:18.0296461Z 37 error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `Foo`
2019-08-20T13:23:18.0298893Z +   --> $DIR/non-integer-atomic.rs:43:5
2019-08-20T13:23:18.0299142Z 39    |
2019-08-20T13:23:18.0299142Z 39    |
2019-08-20T13:23:18.0299329Z 40 LL |     intrinsics::atomic_xchg(p, v);
2019-08-20T13:23:18.0299606Z 
2019-08-20T13:23:18.0301687Z 42 
2019-08-20T13:23:18.0301687Z 42 
2019-08-20T13:23:18.0302106Z 43 error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `Foo`
2019-08-20T13:23:18.0305000Z +   --> $DIR/non-integer-atomic.rs:48:5
2019-08-20T13:23:18.0305198Z 45    |
2019-08-20T13:23:18.0305198Z 45    |
2019-08-20T13:23:18.0305333Z 46 LL |     intrinsics::atomic_cxchg(p, v, v);
2019-08-20T13:23:18.0305609Z 
2019-08-20T13:23:18.0305736Z 48 
2019-08-20T13:23:18.0305736Z 48 
2019-08-20T13:23:18.0305895Z 49 error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-08-20T13:23:18.0306599Z +   --> $DIR/non-integer-atomic.rs:53:5
2019-08-20T13:23:18.0306786Z 51    |
2019-08-20T13:23:18.0307598Z 52 LL |     intrinsics::atomic_load(p);
2019-08-20T13:23:18.0307813Z 53    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:23:18.0307813Z 53    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:23:18.0307971Z 
2019-08-20T13:23:18.0308122Z 54 
2019-08-20T13:23:18.0308276Z 55 error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-08-20T13:23:18.0309108Z +   --> $DIR/non-integer-atomic.rs:58:5
2019-08-20T13:23:18.0309281Z 57    |
2019-08-20T13:23:18.0309281Z 57    |
2019-08-20T13:23:18.0309452Z 58 LL |     intrinsics::atomic_store(p, v);
2019-08-20T13:23:18.0309721Z 
2019-08-20T13:23:18.0309877Z 60 
2019-08-20T13:23:18.0309877Z 60 
2019-08-20T13:23:18.0310029Z 61 error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-08-20T13:23:18.0310922Z +   --> $DIR/non-integer-atomic.rs:63:5
2019-08-20T13:23:18.0333196Z 63    |
2019-08-20T13:23:18.0333196Z 63    |
2019-08-20T13:23:18.0333483Z 64 LL |     intrinsics::atomic_xchg(p, v);
2019-08-20T13:23:18.0333992Z 
2019-08-20T13:23:18.0334149Z 66 
2019-08-20T13:23:18.0334149Z 66 
2019-08-20T13:23:18.0334306Z 67 error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-08-20T13:23:18.0335254Z +   --> $DIR/non-integer-atomic.rs:68:5
2019-08-20T13:23:18.0335431Z 69    |
2019-08-20T13:23:18.0335431Z 69    |
2019-08-20T13:23:18.0335572Z 70 LL |     intrinsics::atomic_cxchg(p, v, v);
2019-08-20T13:23:18.0335854Z 
2019-08-20T13:23:18.0335987Z 72 
2019-08-20T13:23:18.0335987Z 72 
2019-08-20T13:23:18.0336150Z 73 error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `[u8; 100]`
2019-08-20T13:23:18.0337445Z +   --> $DIR/non-integer-atomic.rs:73:5
2019-08-20T13:23:18.0338598Z 75    |
2019-08-20T13:23:18.0339072Z 76 LL |     intrinsics::atomic_load(p);
2019-08-20T13:23:18.0339248Z 77    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:23:18.0339248Z 77    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:23:18.0339397Z 
2019-08-20T13:23:18.0339537Z 78 
2019-08-20T13:23:18.0339710Z 79 error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `[u8; 100]`
2019-08-20T13:23:18.0341027Z +   --> $DIR/non-integer-atomic.rs:78:5
2019-08-20T13:23:18.0341200Z 81    |
2019-08-20T13:23:18.0341200Z 81    |
2019-08-20T13:23:18.0341355Z 82 LL |     intrinsics::atomic_store(p, v);
2019-08-20T13:23:18.0341608Z 
2019-08-20T13:23:18.0341753Z 84 
2019-08-20T13:23:18.0341753Z 84 
2019-08-20T13:23:18.0341895Z 85 error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `[u8; 100]`
2019-08-20T13:23:18.0342629Z +   --> $DIR/non-integer-atomic.rs:83:5
2019-08-20T13:23:18.0342797Z 87    |
2019-08-20T13:23:18.0342797Z 87    |
2019-08-20T13:23:18.0342947Z 88 LL |     intrinsics::atomic_xchg(p, v);
2019-08-20T13:23:18.0343216Z 
2019-08-20T13:23:18.0343342Z 90 
2019-08-20T13:23:18.0343342Z 90 
2019-08-20T13:23:18.0343504Z 91 error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `[u8; 100]`
2019-08-20T13:23:18.0344219Z +   --> $DIR/non-integer-atomic.rs:88:5
2019-08-20T13:23:18.0344381Z 93    |
2019-08-20T13:23:18.0344381Z 93    |
2019-08-20T13:23:18.0344533Z 94 LL |     intrinsics::atomic_cxchg(p, v, v);
2019-08-20T13:23:18.0344811Z 
2019-08-20T13:23:18.0344945Z 
2019-08-20T13:23:18.0345084Z The actual stderr differed from the expected stderr.
2019-08-20T13:23:18.0345646Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/non-integer-atomic/non-integer-atomic.stderr
2019-08-20T13:23:18.0345646Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/non-integer-atomic/non-integer-atomic.stderr
2019-08-20T13:23:18.0346132Z To update references, rerun the tests and pass the `--bless` flag
2019-08-20T13:23:18.0346571Z To only update this specific test, also pass `--test-args non-integer-atomic.rs`
2019-08-20T13:23:18.0346868Z error: 1 errors occurred comparing output.
2019-08-20T13:23:18.0347592Z status: exit code: 1
2019-08-20T13:23:18.0347592Z status: exit code: 1
2019-08-20T13:23:18.0348533Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/non-integer-atomic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/non-integer-atomic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/non-integer-atomic/auxiliary" "-A" "unused"
2019-08-20T13:23:18.0350839Z ------------------------------------------
2019-08-20T13:23:18.0351024Z 
2019-08-20T13:23:18.0351416Z ------------------------------------------
2019-08-20T13:23:18.0351804Z stderr:
2019-08-20T13:23:18.0351804Z stderr:
2019-08-20T13:23:18.0352181Z ------------------------------------------
2019-08-20T13:23:18.0352371Z error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `bool`
2019-08-20T13:23:18.0352995Z    |
2019-08-20T13:23:18.0353155Z LL |     intrinsics::atomic_load(p);
2019-08-20T13:23:18.0353295Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:23:18.0353412Z 
2019-08-20T13:23:18.0353412Z 
2019-08-20T13:23:18.0353564Z error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `bool`
2019-08-20T13:23:18.0354133Z    |
2019-08-20T13:23:18.0354133Z    |
2019-08-20T13:23:18.0354303Z LL |     intrinsics::atomic_store(p, v);
2019-08-20T13:23:18.0356597Z 
2019-08-20T13:23:18.0356597Z 
2019-08-20T13:23:18.0356751Z error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `bool`
2019-08-20T13:23:18.0358095Z    |
2019-08-20T13:23:18.0358095Z    |
2019-08-20T13:23:18.0358249Z LL |     intrinsics::atomic_xchg(p, v);
2019-08-20T13:23:18.0358516Z 
2019-08-20T13:23:18.0358516Z 
2019-08-20T13:23:18.0358687Z error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `bool`
2019-08-20T13:23:18.0359261Z    |
2019-08-20T13:23:18.0359261Z    |
2019-08-20T13:23:18.0359426Z LL |     intrinsics::atomic_cxchg(p, v, v);
2019-08-20T13:23:18.0359697Z 
2019-08-20T13:23:18.0359697Z 
2019-08-20T13:23:18.0359868Z error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `Foo`
2019-08-20T13:23:18.0360481Z    |
2019-08-20T13:23:18.0360791Z LL |     intrinsics::atomic_load(p);
2019-08-20T13:23:18.0360950Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:23:18.0361070Z 
2019-08-20T13:23:18.0361070Z 
2019-08-20T13:23:18.0361216Z error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `Foo`
2019-08-20T13:23:18.0361777Z    |
2019-08-20T13:23:18.0361777Z    |
2019-08-20T13:23:18.0361917Z LL |     intrinsics::atomic_store(p, v);
2019-08-20T13:23:18.0362197Z 
2019-08-20T13:23:18.0362197Z 
2019-08-20T13:23:18.0362364Z error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `Foo`
2019-08-20T13:23:18.0363044Z    |
2019-08-20T13:23:18.0363044Z    |
2019-08-20T13:23:18.0363234Z LL |     intrinsics::atomic_xchg(p, v);
2019-08-20T13:23:18.0363532Z 
2019-08-20T13:23:18.0363532Z 
2019-08-20T13:23:18.0363698Z error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `Foo`
2019-08-20T13:23:18.0364269Z    |
2019-08-20T13:23:18.0364269Z    |
2019-08-20T13:23:18.0364434Z LL |     intrinsics::atomic_cxchg(p, v, v);
2019-08-20T13:23:18.0364716Z 
2019-08-20T13:23:18.0364716Z 
2019-08-20T13:23:18.0364869Z error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-08-20T13:23:18.0365452Z    |
2019-08-20T13:23:18.0365591Z LL |     intrinsics::atomic_load(p);
2019-08-20T13:23:18.0365726Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:23:18.0365879Z 
2019-08-20T13:23:18.0365879Z 
2019-08-20T13:23:18.0366121Z error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-08-20T13:23:18.0366829Z    |
2019-08-20T13:23:18.0366829Z    |
2019-08-20T13:23:18.0367568Z LL |     intrinsics::atomic_store(p, v);
2019-08-20T13:23:18.0367866Z 
2019-08-20T13:23:18.0367866Z 
2019-08-20T13:23:18.0368019Z error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-08-20T13:23:18.0368692Z    |
2019-08-20T13:23:18.0368692Z    |
2019-08-20T13:23:18.0368835Z LL |     intrinsics::atomic_xchg(p, v);
2019-08-20T13:23:18.0369118Z 
2019-08-20T13:23:18.0369118Z 
2019-08-20T13:23:18.0369289Z error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-08-20T13:23:18.0369859Z    |
2019-08-20T13:23:18.0369859Z    |
2019-08-20T13:23:18.0370042Z LL |     intrinsics::atomic_cxchg(p, v, v);
2019-08-20T13:23:18.0370331Z 
2019-08-20T13:23:18.0370331Z 
2019-08-20T13:23:18.0370650Z error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `[u8; 100]`
2019-08-20T13:23:18.0371189Z    |
2019-08-20T13:23:18.0371346Z LL |     intrinsics::atomic_load(p);
2019-08-20T13:23:18.0371484Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:23:18.0371605Z 
2019-08-20T13:23:18.0371605Z 
2019-08-20T13:23:18.0371774Z error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `[u8; 100]`
2019-08-20T13:23:18.0372326Z    |
2019-08-20T13:23:18.0372326Z    |
2019-08-20T13:23:18.0372477Z LL |     intrinsics::atomic_store(p, v);
2019-08-20T13:23:18.0372771Z 
2019-08-20T13:23:18.0372771Z 
2019-08-20T13:23:18.0372920Z error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `[u8; 100]`
2019-08-20T13:23:18.0373479Z    |
2019-08-20T13:23:18.0373479Z    |
2019-08-20T13:23:18.0373636Z LL |     intrinsics::atomic_xchg(p, v);
2019-08-20T13:23:18.0374097Z 
2019-08-20T13:23:18.0374097Z 
2019-08-20T13:23:18.0374252Z error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `[u8; 100]`
2019-08-20T13:23:18.0374831Z    |
2019-08-20T13:23:18.0374831Z    |
2019-08-20T13:23:18.0374995Z LL |     intrinsics::atomic_cxchg(p, v, v);
2019-08-20T13:23:18.0375367Z 
2019-08-20T13:23:18.0375643Z error: aborting due to 16 previous errors
2019-08-20T13:23:18.0375842Z 
2019-08-20T13:23:18.0376040Z 
---
2019-08-20T13:23:18.0378802Z test result: FAILED. 8905 passed; 1 failed; 37 ignored; 0 measured; 0 filtered out
2019-08-20T13:23:18.0378840Z 
2019-08-20T13:23:18.0386528Z 
2019-08-20T13:23:18.0391372Z 
2019-08-20T13:23:18.0393260Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-20T13:23:18.0393808Z 
2019-08-20T13:23:18.0393843Z 
2019-08-20T13:23:18.0394131Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-20T13:23:18.0394260Z Build completed unsuccessfully in 1:05:48
2019-08-20T13:23:18.0394260Z Build completed unsuccessfully in 1:05:48
2019-08-20T13:23:18.0394709Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-20T13:23:18.0394790Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-20T13:23:18.0399202Z == clock drift check ==
2019-08-20T13:23:18.0414248Z   local time: Tue Aug 20 13:23:18 UTC 2019
2019-08-20T13:23:18.1910206Z   network time: Tue, 20 Aug 2019 13:23:18 GMT
2019-08-20T13:23:18.1916880Z == end clock drift check ==
2019-08-20T13:23:19.2902810Z ##[error]Bash exited with code '1'.
2019-08-20T13:23:19.2948840Z ##[section]Starting: Checkout
2019-08-20T13:23:19.2950977Z ==============================================================================
2019-08-20T13:23:19.2951035Z Task         : Get sources
2019-08-20T13:23:19.2951105Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
