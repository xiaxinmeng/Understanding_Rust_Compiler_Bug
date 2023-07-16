plain
2019-08-20T12:18:18.9875183Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-20T12:18:19.0059047Z ##[command]git config gc.auto 0
2019-08-20T12:18:19.0117115Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-20T12:18:19.0162145Z ##[command]git config --get-all http.proxy
2019-08-20T12:18:19.0292537Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63739/merge:refs/remotes/pull/63739/merge
---
2019-08-20T12:18:54.6900808Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-20T12:18:54.6900837Z 
2019-08-20T12:18:54.6901028Z   git checkout -b <new-branch-name>
2019-08-20T12:18:54.6901055Z 
2019-08-20T12:18:54.6901118Z HEAD is now at 9780e33fc Merge 3c4315888c9b24012bc5b600d803f0cbec29a270 into 14890954ce17c44d944eda988c5a64bb4c5ec9eb
2019-08-20T12:18:54.7055562Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-20T12:18:54.7058007Z ==============================================================================
2019-08-20T12:18:54.7058054Z Task         : Bash
2019-08-20T12:18:54.7058091Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-20T13:17:10.5588817Z .................................................................................................... 1500/8943
2019-08-20T13:17:16.2130432Z .................................................................................................... 1600/8943
2019-08-20T13:17:29.3871785Z .......................................i...............i............................................ 1700/8943
2019-08-20T13:17:37.6321458Z .................................................................................................... 1800/8943
2019-08-20T13:17:52.0400289Z ...............................iiiii................................................................ 1900/8943
2019-08-20T13:18:03.7083699Z .................................................................................................... 2100/8943
2019-08-20T13:18:05.7426503Z .................................................................................................... 2200/8943
2019-08-20T13:18:10.5457797Z .................................................................................................... 2300/8943
2019-08-20T13:18:17.6919734Z .................................................................................................... 2400/8943
---
2019-08-20T13:21:05.7596104Z ....................i..............i................................................................ 4700/8943
2019-08-20T13:21:16.3780699Z .................................................................................................... 4800/8943
2019-08-20T13:21:22.0900369Z .................................................................................................... 4900/8943
2019-08-20T13:21:32.3695602Z .................................................................................................... 5000/8943
2019-08-20T13:21:37.1436237Z ii.ii............................................................................................... 5100/8943
2019-08-20T13:21:50.6927788Z .................................................................................................... 5300/8943
2019-08-20T13:21:57.6499603Z ........................................................i........................................... 5400/8943
2019-08-20T13:22:04.4034597Z .................................................................................................... 5500/8943
2019-08-20T13:22:13.9470058Z .................................................................................................... 5600/8943
2019-08-20T13:22:13.9470058Z .................................................................................................... 5600/8943
2019-08-20T13:22:22.9974552Z .................................................ii...i..ii...........i............................. 5700/8943
2019-08-20T13:22:44.0117814Z .................................................................................................... 5900/8943
2019-08-20T13:22:48.8642782Z .................................................................................................... 6000/8943
2019-08-20T13:22:48.8642782Z .................................................................................................... 6000/8943
2019-08-20T13:22:59.1522842Z .................................................Fi..ii............................................. 6100/8943
2019-08-20T13:23:21.0233490Z ................................................................................................i... 6300/8943
2019-08-20T13:23:23.1453835Z .................................................................................................... 6400/8943
2019-08-20T13:23:25.3003463Z ....................................................................i............................... 6500/8943
2019-08-20T13:23:28.1413631Z .................................................................................................... 6600/8943
---
2019-08-20T13:27:09.0838893Z 
2019-08-20T13:27:09.0839601Z ---- [ui] ui/non-integer-atomic.rs stdout ----
2019-08-20T13:27:09.0839810Z diff of stderr:
2019-08-20T13:27:09.0840112Z 
2019-08-20T13:27:09.0840324Z 1 error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `bool`
2019-08-20T13:27:09.0841030Z +   --> $DIR/non-integer-atomic.rs:13:5
2019-08-20T13:27:09.0841178Z 3    |
2019-08-20T13:27:09.0841299Z 4 LL |     intrinsics::atomic_load(p);
2019-08-20T13:27:09.0841439Z 5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:27:09.0841439Z 5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:27:09.0841541Z 
2019-08-20T13:27:09.0841657Z 6 
2019-08-20T13:27:09.0841814Z 7 error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `bool`
2019-08-20T13:27:09.0842408Z +   --> $DIR/non-integer-atomic.rs:18:5
2019-08-20T13:27:09.0842570Z 9    |
2019-08-20T13:27:09.0842570Z 9    |
2019-08-20T13:27:09.0842689Z 10 LL |     intrinsics::atomic_store(p, v);
2019-08-20T13:27:09.0842947Z 
2019-08-20T13:27:09.0843065Z 12 
2019-08-20T13:27:09.0843065Z 12 
2019-08-20T13:27:09.0843363Z 13 error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `bool`
2019-08-20T13:27:09.0844848Z +   --> $DIR/non-integer-atomic.rs:23:5
2019-08-20T13:27:09.0845037Z 15    |
2019-08-20T13:27:09.0845037Z 15    |
2019-08-20T13:27:09.0845210Z 16 LL |     intrinsics::atomic_xchg(p, v);
2019-08-20T13:27:09.0845492Z 
2019-08-20T13:27:09.0845654Z 18 
2019-08-20T13:27:09.0845654Z 18 
2019-08-20T13:27:09.0846016Z 19 error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `bool`
2019-08-20T13:27:09.0846817Z +   --> $DIR/non-integer-atomic.rs:28:5
2019-08-20T13:27:09.0846992Z 21    |
2019-08-20T13:27:09.0846992Z 21    |
2019-08-20T13:27:09.0847162Z 22 LL |     intrinsics::atomic_cxchg(p, v, v);
2019-08-20T13:27:09.0847606Z 
2019-08-20T13:27:09.0847718Z 24 
2019-08-20T13:27:09.0847718Z 24 
2019-08-20T13:27:09.0847857Z 25 error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `Foo`
2019-08-20T13:27:09.0848457Z +   --> $DIR/non-integer-atomic.rs:33:5
2019-08-20T13:27:09.0848597Z 27    |
2019-08-20T13:27:09.0848711Z 28 LL |     intrinsics::atomic_load(p);
2019-08-20T13:27:09.0848862Z 29    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:27:09.0848862Z 29    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:27:09.0848964Z 
2019-08-20T13:27:09.0849084Z 30 
2019-08-20T13:27:09.0849221Z 31 error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `Foo`
2019-08-20T13:27:09.0849798Z +   --> $DIR/non-integer-atomic.rs:38:5
2019-08-20T13:27:09.0849974Z 33    |
2019-08-20T13:27:09.0849974Z 33    |
2019-08-20T13:27:09.0850089Z 34 LL |     intrinsics::atomic_store(p, v);
2019-08-20T13:27:09.0850334Z 
2019-08-20T13:27:09.0850446Z 36 
2019-08-20T13:27:09.0850446Z 36 
2019-08-20T13:27:09.0850566Z 37 error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `Foo`
2019-08-20T13:27:09.0851169Z +   --> $DIR/non-integer-atomic.rs:43:5
2019-08-20T13:27:09.0851309Z 39    |
2019-08-20T13:27:09.0851309Z 39    |
2019-08-20T13:27:09.0851444Z 40 LL |     intrinsics::atomic_xchg(p, v);
2019-08-20T13:27:09.0851672Z 
2019-08-20T13:27:09.0851801Z 42 
2019-08-20T13:27:09.0851801Z 42 
2019-08-20T13:27:09.0851922Z 43 error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `Foo`
2019-08-20T13:27:09.0852526Z +   --> $DIR/non-integer-atomic.rs:48:5
2019-08-20T13:27:09.0852666Z 45    |
2019-08-20T13:27:09.0852666Z 45    |
2019-08-20T13:27:09.0852781Z 46 LL |     intrinsics::atomic_cxchg(p, v, v);
2019-08-20T13:27:09.0853149Z 
2019-08-20T13:27:09.0853264Z 48 
2019-08-20T13:27:09.0853264Z 48 
2019-08-20T13:27:09.0854276Z 49 error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-08-20T13:27:09.0855214Z +   --> $DIR/non-integer-atomic.rs:53:5
2019-08-20T13:27:09.0855394Z 51    |
2019-08-20T13:27:09.0855540Z 52 LL |     intrinsics::atomic_load(p);
2019-08-20T13:27:09.0855709Z 53    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:27:09.0855709Z 53    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:27:09.0855859Z 
2019-08-20T13:27:09.0856001Z 54 
2019-08-20T13:27:09.0856176Z 55 error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-08-20T13:27:09.0857468Z +   --> $DIR/non-integer-atomic.rs:58:5
2019-08-20T13:27:09.0858802Z 57    |
2019-08-20T13:27:09.0858802Z 57    |
2019-08-20T13:27:09.0858856Z 58 LL |     intrinsics::atomic_store(p, v);
2019-08-20T13:27:09.0858935Z 
2019-08-20T13:27:09.0858965Z 60 
2019-08-20T13:27:09.0858965Z 60 
2019-08-20T13:27:09.0859023Z 61 error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-08-20T13:27:09.0859442Z +   --> $DIR/non-integer-atomic.rs:63:5
2019-08-20T13:27:09.0859476Z 63    |
2019-08-20T13:27:09.0859476Z 63    |
2019-08-20T13:27:09.0859525Z 64 LL |     intrinsics::atomic_xchg(p, v);
2019-08-20T13:27:09.0859708Z 
2019-08-20T13:27:09.0859755Z 66 
2019-08-20T13:27:09.0859755Z 66 
2019-08-20T13:27:09.0859797Z 67 error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-08-20T13:27:09.0860180Z +   --> $DIR/non-integer-atomic.rs:68:5
2019-08-20T13:27:09.0860221Z 69    |
2019-08-20T13:27:09.0860221Z 69    |
2019-08-20T13:27:09.0860256Z 70 LL |     intrinsics::atomic_cxchg(p, v, v);
2019-08-20T13:27:09.0860332Z 
2019-08-20T13:27:09.0860361Z 72 
2019-08-20T13:27:09.0860361Z 72 
2019-08-20T13:27:09.0860402Z 73 error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `[u8; 100]`
2019-08-20T13:27:09.0860757Z +   --> $DIR/non-integer-atomic.rs:73:5
2019-08-20T13:27:09.0860791Z 75    |
2019-08-20T13:27:09.0860829Z 76 LL |     intrinsics::atomic_load(p);
2019-08-20T13:27:09.0860880Z 77    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:27:09.0860880Z 77    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:27:09.0860902Z 
2019-08-20T13:27:09.0860931Z 78 
2019-08-20T13:27:09.0860989Z 79 error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `[u8; 100]`
2019-08-20T13:27:09.0861329Z +   --> $DIR/non-integer-atomic.rs:78:5
2019-08-20T13:27:09.0861379Z 81    |
2019-08-20T13:27:09.0861379Z 81    |
2019-08-20T13:27:09.0861413Z 82 LL |     intrinsics::atomic_store(p, v);
2019-08-20T13:27:09.0861471Z 
2019-08-20T13:27:09.0861517Z 84 
2019-08-20T13:27:09.0861517Z 84 
2019-08-20T13:27:09.0861557Z 85 error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `[u8; 100]`
2019-08-20T13:27:09.0861903Z +   --> $DIR/non-integer-atomic.rs:83:5
2019-08-20T13:27:09.0861943Z 87    |
2019-08-20T13:27:09.0861943Z 87    |
2019-08-20T13:27:09.0861977Z 88 LL |     intrinsics::atomic_xchg(p, v);
2019-08-20T13:27:09.0862052Z 
2019-08-20T13:27:09.0862081Z 90 
2019-08-20T13:27:09.0862081Z 90 
2019-08-20T13:27:09.0862121Z 91 error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `[u8; 100]`
2019-08-20T13:27:09.0862544Z +   --> $DIR/non-integer-atomic.rs:88:5
2019-08-20T13:27:09.0862585Z 93    |
2019-08-20T13:27:09.0862585Z 93    |
2019-08-20T13:27:09.0862635Z 94 LL |     intrinsics::atomic_cxchg(p, v, v);
2019-08-20T13:27:09.0862696Z 
2019-08-20T13:27:09.0862716Z 
2019-08-20T13:27:09.0862750Z The actual stderr differed from the expected stderr.
2019-08-20T13:27:09.0863021Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/non-integer-atomic/non-integer-atomic.stderr
2019-08-20T13:27:09.0863021Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/non-integer-atomic/non-integer-atomic.stderr
2019-08-20T13:27:09.0863208Z To update references, rerun the tests and pass the `--bless` flag
2019-08-20T13:27:09.0863425Z To only update this specific test, also pass `--test-args non-integer-atomic.rs`
2019-08-20T13:27:09.0863486Z error: 1 errors occurred comparing output.
2019-08-20T13:27:09.0863520Z status: exit code: 1
2019-08-20T13:27:09.0863520Z status: exit code: 1
2019-08-20T13:27:09.0864655Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/non-integer-atomic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/non-integer-atomic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/non-integer-atomic/auxiliary" "-A" "unused"
2019-08-20T13:27:09.0864986Z ------------------------------------------
2019-08-20T13:27:09.0865132Z 
2019-08-20T13:27:09.0868216Z ------------------------------------------
2019-08-20T13:27:09.0868291Z stderr:
2019-08-20T13:27:09.0868291Z stderr:
2019-08-20T13:27:09.0868462Z ------------------------------------------
2019-08-20T13:27:09.0868506Z error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `bool`
2019-08-20T13:27:09.0868750Z    |
2019-08-20T13:27:09.0868797Z LL |     intrinsics::atomic_load(p);
2019-08-20T13:27:09.0868834Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:27:09.0868875Z 
2019-08-20T13:27:09.0868875Z 
2019-08-20T13:27:09.0868914Z error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `bool`
2019-08-20T13:27:09.0869150Z    |
2019-08-20T13:27:09.0869150Z    |
2019-08-20T13:27:09.0869184Z LL |     intrinsics::atomic_store(p, v);
2019-08-20T13:27:09.0869247Z 
2019-08-20T13:27:09.0869247Z 
2019-08-20T13:27:09.0869338Z error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `bool`
2019-08-20T13:27:09.0869569Z    |
2019-08-20T13:27:09.0869569Z    |
2019-08-20T13:27:09.0869618Z LL |     intrinsics::atomic_xchg(p, v);
2019-08-20T13:27:09.0869676Z 
2019-08-20T13:27:09.0869676Z 
2019-08-20T13:27:09.0869721Z error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `bool`
2019-08-20T13:27:09.0869957Z    |
2019-08-20T13:27:09.0869957Z    |
2019-08-20T13:27:09.0869992Z LL |     intrinsics::atomic_cxchg(p, v, v);
2019-08-20T13:27:09.0870067Z 
2019-08-20T13:27:09.0870067Z 
2019-08-20T13:27:09.0870106Z error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `Foo`
2019-08-20T13:27:09.0870346Z    |
2019-08-20T13:27:09.0870379Z LL |     intrinsics::atomic_load(p);
2019-08-20T13:27:09.0870430Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:27:09.0870453Z 
2019-08-20T13:27:09.0870453Z 
2019-08-20T13:27:09.0870492Z error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `Foo`
2019-08-20T13:27:09.0870862Z    |
2019-08-20T13:27:09.0870862Z    |
2019-08-20T13:27:09.0870895Z LL |     intrinsics::atomic_store(p, v);
2019-08-20T13:27:09.0870953Z 
2019-08-20T13:27:09.0870953Z 
2019-08-20T13:27:09.0871009Z error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `Foo`
2019-08-20T13:27:09.0871251Z    |
2019-08-20T13:27:09.0871251Z    |
2019-08-20T13:27:09.0871301Z LL |     intrinsics::atomic_xchg(p, v);
2019-08-20T13:27:09.0871365Z 
2019-08-20T13:27:09.0871365Z 
2019-08-20T13:27:09.0871405Z error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `Foo`
2019-08-20T13:27:09.0871639Z    |
2019-08-20T13:27:09.0871639Z    |
2019-08-20T13:27:09.0871673Z LL |     intrinsics::atomic_cxchg(p, v, v);
2019-08-20T13:27:09.0871753Z 
2019-08-20T13:27:09.0871753Z 
2019-08-20T13:27:09.0871794Z error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-08-20T13:27:09.0872026Z    |
2019-08-20T13:27:09.0872058Z LL |     intrinsics::atomic_load(p);
2019-08-20T13:27:09.0872109Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:27:09.0872132Z 
2019-08-20T13:27:09.0872132Z 
2019-08-20T13:27:09.0872172Z error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-08-20T13:27:09.0872509Z    |
2019-08-20T13:27:09.0872509Z    |
2019-08-20T13:27:09.0872542Z LL |     intrinsics::atomic_store(p, v);
2019-08-20T13:27:09.0872600Z 
2019-08-20T13:27:09.0872600Z 
2019-08-20T13:27:09.0872666Z error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-08-20T13:27:09.0872883Z    |
2019-08-20T13:27:09.0872883Z    |
2019-08-20T13:27:09.0872931Z LL |     intrinsics::atomic_xchg(p, v);
2019-08-20T13:27:09.0872988Z 
2019-08-20T13:27:09.0872988Z 
2019-08-20T13:27:09.0873049Z error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-08-20T13:27:09.0873270Z    |
2019-08-20T13:27:09.0873270Z    |
2019-08-20T13:27:09.0873303Z LL |     intrinsics::atomic_cxchg(p, v, v);
2019-08-20T13:27:09.0873376Z 
2019-08-20T13:27:09.0873376Z 
2019-08-20T13:27:09.0873416Z error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `[u8; 100]`
2019-08-20T13:27:09.0874112Z    |
2019-08-20T13:27:09.0874154Z LL |     intrinsics::atomic_load(p);
2019-08-20T13:27:09.0874218Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:27:09.0874248Z 
2019-08-20T13:27:09.0874248Z 
2019-08-20T13:27:09.0874299Z error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `[u8; 100]`
2019-08-20T13:27:09.0874599Z    |
2019-08-20T13:27:09.0874599Z    |
2019-08-20T13:27:09.0874642Z LL |     intrinsics::atomic_store(p, v);
2019-08-20T13:27:09.0874743Z 
2019-08-20T13:27:09.0874743Z 
2019-08-20T13:27:09.0874795Z error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `[u8; 100]`
2019-08-20T13:27:09.0875074Z    |
2019-08-20T13:27:09.0875074Z    |
2019-08-20T13:27:09.0875133Z LL |     intrinsics::atomic_xchg(p, v);
2019-08-20T13:27:09.0875309Z 
2019-08-20T13:27:09.0875309Z 
2019-08-20T13:27:09.0875387Z error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `[u8; 100]`
2019-08-20T13:27:09.0875692Z    |
2019-08-20T13:27:09.0875692Z    |
2019-08-20T13:27:09.0875751Z LL |     intrinsics::atomic_cxchg(p, v, v);
2019-08-20T13:27:09.0875829Z 
2019-08-20T13:27:09.0875871Z error: aborting due to 16 previous errors
2019-08-20T13:27:09.0875927Z 
2019-08-20T13:27:09.0875953Z 
---
2019-08-20T13:27:09.0878197Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-20T13:27:09.0878313Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-20T13:27:09.0882829Z 
2019-08-20T13:27:09.0882890Z 
2019-08-20T13:27:09.0884814Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-20T13:27:09.0885240Z 
2019-08-20T13:27:09.0885273Z 
2019-08-20T13:27:09.0890927Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-20T13:27:09.0890985Z Build completed unsuccessfully in 1:01:49
2019-08-20T13:27:09.0890985Z Build completed unsuccessfully in 1:01:49
2019-08-20T13:27:09.0940017Z == clock drift check ==
2019-08-20T13:27:09.0951444Z   local time: Tue Aug 20 13:27:09 UTC 2019
2019-08-20T13:27:09.1291352Z   network time: Tue, 20 Aug 2019 13:27:09 GMT
2019-08-20T13:27:09.1294277Z == end clock drift check ==
2019-08-20T13:27:10.1690836Z ##[error]Bash exited with code '1'.
2019-08-20T13:27:10.1741246Z ##[section]Starting: Checkout
2019-08-20T13:27:10.1744263Z ==============================================================================
2019-08-20T13:27:10.1744323Z Task         : Get sources
2019-08-20T13:27:10.1744372Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
