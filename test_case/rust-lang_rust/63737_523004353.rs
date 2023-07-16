plain
2019-08-20T11:48:11.7773550Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-20T11:48:11.7941494Z ##[command]git config gc.auto 0
2019-08-20T11:48:11.8328341Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-20T11:48:11.8382383Z ##[command]git config --get-all http.proxy
2019-08-20T11:48:11.8524252Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63737/merge:refs/remotes/pull/63737/merge
---
2019-08-20T11:48:48.4825253Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-20T11:48:48.4825286Z 
2019-08-20T11:48:48.4825520Z   git checkout -b <new-branch-name>
2019-08-20T11:48:48.4825945Z 
2019-08-20T11:48:48.4826003Z HEAD is now at cef8c642a Merge 70e3c0583c5798de50b9864338bd4c3803cadc80 into 14890954ce17c44d944eda988c5a64bb4c5ec9eb
2019-08-20T11:48:48.4990848Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-20T11:48:48.4993985Z ==============================================================================
2019-08-20T11:48:48.4994057Z Task         : Bash
2019-08-20T11:48:48.4994099Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-20T12:52:02.2599472Z .................................................................................................... 1500/8943
2019-08-20T12:52:07.9262523Z .................................................................................................... 1600/8943
2019-08-20T12:52:21.1206162Z .......................................i...............i............................................ 1700/8943
2019-08-20T12:52:29.2799858Z .................................................................................................... 1800/8943
2019-08-20T12:52:43.9686312Z ...............................iiiii................................................................ 1900/8943
2019-08-20T12:52:54.8946709Z .................................................................................................... 2100/8943
2019-08-20T12:52:57.5632106Z .................................................................................................... 2200/8943
2019-08-20T12:53:02.3612818Z .................................................................................................... 2300/8943
2019-08-20T12:53:09.5238444Z .................................................................................................... 2400/8943
---
2019-08-20T12:56:15.9884829Z ...................i...............i................................................................ 4700/8943
2019-08-20T12:56:27.6119494Z .................................................................................................... 4800/8943
2019-08-20T12:56:33.9697132Z .................................................................................................... 4900/8943
2019-08-20T12:56:45.3667762Z .................................................................................................... 5000/8943
2019-08-20T12:56:50.7601797Z ii.ii............................................................................................... 5100/8943
2019-08-20T12:57:06.0247121Z .................................................................................................... 5300/8943
2019-08-20T12:57:13.2013979Z ........................................................i........................................... 5400/8943
2019-08-20T12:57:20.4479827Z .................................................................................................... 5500/8943
2019-08-20T12:57:30.1863967Z .................................................................................................... 5600/8943
2019-08-20T12:57:30.1863967Z .................................................................................................... 5600/8943
2019-08-20T12:57:39.9206332Z .................................................ii...i..ii...........i............................. 5700/8943
2019-08-20T12:58:03.7271666Z .................................................................................................... 5900/8943
2019-08-20T12:58:08.9445946Z .................................................................................................... 6000/8943
2019-08-20T12:58:08.9445946Z .................................................................................................... 6000/8943
2019-08-20T12:58:20.3558297Z .................................................Fi..ii............................................. 6100/8943
2019-08-20T12:58:44.8915462Z ................................................................................................i... 6300/8943
2019-08-20T12:58:47.3471300Z .................................................................................................... 6400/8943
2019-08-20T12:58:49.7311275Z ....................................................................i............................... 6500/8943
2019-08-20T12:58:52.9743791Z .................................................................................................... 6600/8943
---
2019-08-20T13:03:00.9551277Z 
2019-08-20T13:03:00.9551866Z ---- [ui] ui/non-integer-atomic.rs stdout ----
2019-08-20T13:03:00.9551930Z diff of stderr:
2019-08-20T13:03:00.9551967Z 
2019-08-20T13:03:00.9552042Z 1 error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `bool`
2019-08-20T13:03:00.9552598Z +   --> $DIR/non-integer-atomic.rs:13:5
2019-08-20T13:03:00.9552666Z 3    |
2019-08-20T13:03:00.9552729Z 4 LL |     intrinsics::atomic_load(p);
2019-08-20T13:03:00.9552778Z 5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:03:00.9552778Z 5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:03:00.9552816Z 
2019-08-20T13:03:00.9552879Z 6 
2019-08-20T13:03:00.9552935Z 7 error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `bool`
2019-08-20T13:03:00.9553430Z +   --> $DIR/non-integer-atomic.rs:18:5
2019-08-20T13:03:00.9553480Z 9    |
2019-08-20T13:03:00.9553480Z 9    |
2019-08-20T13:03:00.9553611Z 10 LL |     intrinsics::atomic_store(p, v);
2019-08-20T13:03:00.9553718Z 
2019-08-20T13:03:00.9553761Z 12 
2019-08-20T13:03:00.9553761Z 12 
2019-08-20T13:03:00.9553988Z 13 error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `bool`
2019-08-20T13:03:00.9554515Z +   --> $DIR/non-integer-atomic.rs:23:5
2019-08-20T13:03:00.9554566Z 15    |
2019-08-20T13:03:00.9554566Z 15    |
2019-08-20T13:03:00.9554642Z 16 LL |     intrinsics::atomic_xchg(p, v);
2019-08-20T13:03:00.9554735Z 
2019-08-20T13:03:00.9554777Z 18 
2019-08-20T13:03:00.9554777Z 18 
2019-08-20T13:03:00.9554853Z 19 error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `bool`
2019-08-20T13:03:00.9555340Z +   --> $DIR/non-integer-atomic.rs:28:5
2019-08-20T13:03:00.9555409Z 21    |
2019-08-20T13:03:00.9555409Z 21    |
2019-08-20T13:03:00.9555457Z 22 LL |     intrinsics::atomic_cxchg(p, v, v);
2019-08-20T13:03:00.9555540Z 
2019-08-20T13:03:00.9555603Z 24 
2019-08-20T13:03:00.9555603Z 24 
2019-08-20T13:03:00.9555659Z 25 error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `Foo`
2019-08-20T13:03:00.9556161Z +   --> $DIR/non-integer-atomic.rs:33:5
2019-08-20T13:03:00.9556384Z 27    |
2019-08-20T13:03:00.9556443Z 28 LL |     intrinsics::atomic_load(p);
2019-08-20T13:03:00.9556522Z 29    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:03:00.9556522Z 29    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:03:00.9556554Z 
2019-08-20T13:03:00.9556597Z 30 
2019-08-20T13:03:00.9556653Z 31 error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `Foo`
2019-08-20T13:03:00.9557409Z +   --> $DIR/non-integer-atomic.rs:38:5
2019-08-20T13:03:00.9557457Z 33    |
2019-08-20T13:03:00.9557457Z 33    |
2019-08-20T13:03:00.9557520Z 34 LL |     intrinsics::atomic_store(p, v);
2019-08-20T13:03:00.9557600Z 
2019-08-20T13:03:00.9557640Z 36 
2019-08-20T13:03:00.9557640Z 36 
2019-08-20T13:03:00.9557714Z 37 error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `Foo`
2019-08-20T13:03:00.9558188Z +   --> $DIR/non-integer-atomic.rs:43:5
2019-08-20T13:03:00.9558255Z 39    |
2019-08-20T13:03:00.9558255Z 39    |
2019-08-20T13:03:00.9558301Z 40 LL |     intrinsics::atomic_xchg(p, v);
2019-08-20T13:03:00.9558474Z 
2019-08-20T13:03:00.9558535Z 42 
2019-08-20T13:03:00.9558535Z 42 
2019-08-20T13:03:00.9558590Z 43 error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `Foo`
2019-08-20T13:03:00.9559099Z +   --> $DIR/non-integer-atomic.rs:48:5
2019-08-20T13:03:00.9559149Z 45    |
2019-08-20T13:03:00.9559149Z 45    |
2019-08-20T13:03:00.9559195Z 46 LL |     intrinsics::atomic_cxchg(p, v, v);
2019-08-20T13:03:00.9559296Z 
2019-08-20T13:03:00.9559337Z 48 
2019-08-20T13:03:00.9559337Z 48 
2019-08-20T13:03:00.9559392Z 49 error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-08-20T13:03:00.9559889Z +   --> $DIR/non-integer-atomic.rs:53:5
2019-08-20T13:03:00.9559938Z 51    |
2019-08-20T13:03:00.9560010Z 52 LL |     intrinsics::atomic_load(p);
2019-08-20T13:03:00.9560058Z 53    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:03:00.9560058Z 53    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:03:00.9560087Z 
2019-08-20T13:03:00.9560129Z 54 
2019-08-20T13:03:00.9560204Z 55 error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-08-20T13:03:00.9560674Z +   --> $DIR/non-integer-atomic.rs:58:5
2019-08-20T13:03:00.9560742Z 57    |
2019-08-20T13:03:00.9560742Z 57    |
2019-08-20T13:03:00.9560789Z 58 LL |     intrinsics::atomic_store(p, v);
2019-08-20T13:03:00.9560866Z 
2019-08-20T13:03:00.9560928Z 60 
2019-08-20T13:03:00.9560928Z 60 
2019-08-20T13:03:00.9560983Z 61 error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-08-20T13:03:00.9561482Z +   --> $DIR/non-integer-atomic.rs:63:5
2019-08-20T13:03:00.9561539Z 63    |
2019-08-20T13:03:00.9561539Z 63    |
2019-08-20T13:03:00.9561585Z 64 LL |     intrinsics::atomic_xchg(p, v);
2019-08-20T13:03:00.9561683Z 
2019-08-20T13:03:00.9561725Z 66 
2019-08-20T13:03:00.9561725Z 66 
2019-08-20T13:03:00.9561781Z 67 error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-08-20T13:03:00.9562273Z +   --> $DIR/non-integer-atomic.rs:68:5
2019-08-20T13:03:00.9562322Z 69    |
2019-08-20T13:03:00.9562322Z 69    |
2019-08-20T13:03:00.9562387Z 70 LL |     intrinsics::atomic_cxchg(p, v, v);
2019-08-20T13:03:00.9562467Z 
2019-08-20T13:03:00.9562509Z 72 
2019-08-20T13:03:00.9562509Z 72 
2019-08-20T13:03:00.9562668Z 73 error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `[u8; 100]`
2019-08-20T13:03:00.9563184Z +   --> $DIR/non-integer-atomic.rs:73:5
2019-08-20T13:03:00.9563255Z 75    |
2019-08-20T13:03:00.9563300Z 76 LL |     intrinsics::atomic_load(p);
2019-08-20T13:03:00.9563347Z 77    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:03:00.9563347Z 77    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:03:00.9563378Z 
2019-08-20T13:03:00.9563441Z 78 
2019-08-20T13:03:00.9563496Z 79 error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `[u8; 100]`
2019-08-20T13:03:00.9564283Z +   --> $DIR/non-integer-atomic.rs:78:5
2019-08-20T13:03:00.9564335Z 81    |
2019-08-20T13:03:00.9564335Z 81    |
2019-08-20T13:03:00.9564383Z 82 LL |     intrinsics::atomic_store(p, v);
2019-08-20T13:03:00.9564491Z 
2019-08-20T13:03:00.9564535Z 84 
2019-08-20T13:03:00.9564535Z 84 
2019-08-20T13:03:00.9564601Z 85 error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `[u8; 100]`
2019-08-20T13:03:00.9565218Z +   --> $DIR/non-integer-atomic.rs:83:5
2019-08-20T13:03:00.9565269Z 87    |
2019-08-20T13:03:00.9565269Z 87    |
2019-08-20T13:03:00.9565335Z 88 LL |     intrinsics::atomic_xchg(p, v);
2019-08-20T13:03:00.9565416Z 
2019-08-20T13:03:00.9565462Z 90 
2019-08-20T13:03:00.9565462Z 90 
2019-08-20T13:03:00.9565538Z 91 error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `[u8; 100]`
2019-08-20T13:03:00.9566018Z +   --> $DIR/non-integer-atomic.rs:88:5
2019-08-20T13:03:00.9566087Z 93    |
2019-08-20T13:03:00.9566087Z 93    |
2019-08-20T13:03:00.9566136Z 94 LL |     intrinsics::atomic_cxchg(p, v, v);
2019-08-20T13:03:00.9566219Z 
2019-08-20T13:03:00.9566267Z 
2019-08-20T13:03:00.9566326Z The actual stderr differed from the expected stderr.
2019-08-20T13:03:00.9566653Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/non-integer-atomic/non-integer-atomic.stderr
2019-08-20T13:03:00.9566653Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/non-integer-atomic/non-integer-atomic.stderr
2019-08-20T13:03:00.9566947Z To update references, rerun the tests and pass the `--bless` flag
2019-08-20T13:03:00.9567224Z To only update this specific test, also pass `--test-args non-integer-atomic.rs`
2019-08-20T13:03:00.9567312Z error: 1 errors occurred comparing output.
2019-08-20T13:03:00.9567379Z status: exit code: 1
2019-08-20T13:03:00.9567379Z status: exit code: 1
2019-08-20T13:03:00.9568256Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/non-integer-atomic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/non-integer-atomic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/non-integer-atomic/auxiliary" "-A" "unused"
2019-08-20T13:03:00.9568760Z ------------------------------------------
2019-08-20T13:03:00.9568795Z 
2019-08-20T13:03:00.9569038Z ------------------------------------------
2019-08-20T13:03:00.9569085Z stderr:
2019-08-20T13:03:00.9569085Z stderr:
2019-08-20T13:03:00.9569297Z ------------------------------------------
2019-08-20T13:03:00.9569373Z error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `bool`
2019-08-20T13:03:00.9569668Z    |
2019-08-20T13:03:00.9569730Z LL |     intrinsics::atomic_load(p);
2019-08-20T13:03:00.9569777Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:03:00.9569807Z 
2019-08-20T13:03:00.9569807Z 
2019-08-20T13:03:00.9569859Z error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `bool`
2019-08-20T13:03:00.9570255Z    |
2019-08-20T13:03:00.9570255Z    |
2019-08-20T13:03:00.9570298Z LL |     intrinsics::atomic_store(p, v);
2019-08-20T13:03:00.9570399Z 
2019-08-20T13:03:00.9570399Z 
2019-08-20T13:03:00.9570450Z error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `bool`
2019-08-20T13:03:00.9570779Z    |
2019-08-20T13:03:00.9570779Z    |
2019-08-20T13:03:00.9570822Z LL |     intrinsics::atomic_xchg(p, v);
2019-08-20T13:03:00.9570915Z 
2019-08-20T13:03:00.9570915Z 
2019-08-20T13:03:00.9570967Z error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `bool`
2019-08-20T13:03:00.9571271Z    |
2019-08-20T13:03:00.9571271Z    |
2019-08-20T13:03:00.9571317Z LL |     intrinsics::atomic_cxchg(p, v, v);
2019-08-20T13:03:00.9571403Z 
2019-08-20T13:03:00.9571403Z 
2019-08-20T13:03:00.9571474Z error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `Foo`
2019-08-20T13:03:00.9571857Z    |
2019-08-20T13:03:00.9571917Z LL |     intrinsics::atomic_load(p);
2019-08-20T13:03:00.9571964Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:03:00.9571994Z 
2019-08-20T13:03:00.9571994Z 
2019-08-20T13:03:00.9572046Z error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `Foo`
2019-08-20T13:03:00.9572349Z    |
2019-08-20T13:03:00.9572349Z    |
2019-08-20T13:03:00.9572394Z LL |     intrinsics::atomic_store(p, v);
2019-08-20T13:03:00.9572487Z 
2019-08-20T13:03:00.9572487Z 
2019-08-20T13:03:00.9572539Z error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `Foo`
2019-08-20T13:03:00.9576804Z    |
2019-08-20T13:03:00.9576804Z    |
2019-08-20T13:03:00.9576851Z LL |     intrinsics::atomic_xchg(p, v);
2019-08-20T13:03:00.9576952Z 
2019-08-20T13:03:00.9576952Z 
2019-08-20T13:03:00.9577009Z error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `Foo`
2019-08-20T13:03:00.9577353Z    |
2019-08-20T13:03:00.9577353Z    |
2019-08-20T13:03:00.9577563Z LL |     intrinsics::atomic_cxchg(p, v, v);
2019-08-20T13:03:00.9577640Z 
2019-08-20T13:03:00.9577640Z 
2019-08-20T13:03:00.9577714Z error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-08-20T13:03:00.9578004Z    |
2019-08-20T13:03:00.9578074Z LL |     intrinsics::atomic_load(p);
2019-08-20T13:03:00.9578120Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:03:00.9578156Z 
2019-08-20T13:03:00.9578156Z 
2019-08-20T13:03:00.9578209Z error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-08-20T13:03:00.9578534Z    |
2019-08-20T13:03:00.9578534Z    |
2019-08-20T13:03:00.9578578Z LL |     intrinsics::atomic_store(p, v);
2019-08-20T13:03:00.9578670Z 
2019-08-20T13:03:00.9578670Z 
2019-08-20T13:03:00.9578723Z error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-08-20T13:03:00.9579417Z    |
2019-08-20T13:03:00.9579417Z    |
2019-08-20T13:03:00.9579460Z LL |     intrinsics::atomic_xchg(p, v);
2019-08-20T13:03:00.9579554Z 
2019-08-20T13:03:00.9579554Z 
2019-08-20T13:03:00.9579735Z error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `&dyn std::ops::Fn()`
2019-08-20T13:03:00.9580152Z    |
2019-08-20T13:03:00.9580152Z    |
2019-08-20T13:03:00.9580197Z LL |     intrinsics::atomic_cxchg(p, v, v);
2019-08-20T13:03:00.9580273Z 
2019-08-20T13:03:00.9580273Z 
2019-08-20T13:03:00.9580347Z error[E0511]: invalid monomorphization of `atomic_load` intrinsic: expected basic integer type, found `[u8; 100]`
2019-08-20T13:03:00.9580638Z    |
2019-08-20T13:03:00.9580699Z LL |     intrinsics::atomic_load(p);
2019-08-20T13:03:00.9580745Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-20T13:03:00.9580775Z 
2019-08-20T13:03:00.9580775Z 
2019-08-20T13:03:00.9580828Z error[E0511]: invalid monomorphization of `atomic_store` intrinsic: expected basic integer type, found `[u8; 100]`
2019-08-20T13:03:00.9581147Z    |
2019-08-20T13:03:00.9581147Z    |
2019-08-20T13:03:00.9581279Z LL |     intrinsics::atomic_store(p, v);
2019-08-20T13:03:00.9581373Z 
2019-08-20T13:03:00.9581373Z 
2019-08-20T13:03:00.9581425Z error[E0511]: invalid monomorphization of `atomic_xchg` intrinsic: expected basic integer type, found `[u8; 100]`
2019-08-20T13:03:00.9581758Z    |
2019-08-20T13:03:00.9581758Z    |
2019-08-20T13:03:00.9581801Z LL |     intrinsics::atomic_xchg(p, v);
2019-08-20T13:03:00.9581894Z 
2019-08-20T13:03:00.9581894Z 
2019-08-20T13:03:00.9581947Z error[E0511]: invalid monomorphization of `atomic_cxchg` intrinsic: expected basic integer type, found `[u8; 100]`
2019-08-20T13:03:00.9582254Z    |
2019-08-20T13:03:00.9582254Z    |
2019-08-20T13:03:00.9582299Z LL |     intrinsics::atomic_cxchg(p, v, v);
2019-08-20T13:03:00.9582384Z 
2019-08-20T13:03:00.9582458Z error: aborting due to 16 previous errors
2019-08-20T13:03:00.9582489Z 
2019-08-20T13:03:00.9582515Z 
---
2019-08-20T13:03:00.9583781Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-20T13:03:00.9583973Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-20T13:03:00.9594555Z 
2019-08-20T13:03:00.9594915Z 
2019-08-20T13:03:00.9597107Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-20T13:03:00.9597944Z 
2019-08-20T13:03:00.9598034Z 
2019-08-20T13:03:00.9602198Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-20T13:03:00.9602263Z Build completed unsuccessfully in 1:07:39
2019-08-20T13:03:00.9602263Z Build completed unsuccessfully in 1:07:39
2019-08-20T13:03:00.9662292Z == clock drift check ==
2019-08-20T13:03:00.9679127Z   local time: Tue Aug 20 13:03:00 UTC 2019
2019-08-20T13:03:01.1324740Z   network time: Tue, 20 Aug 2019 13:03:01 GMT
2019-08-20T13:03:01.1328634Z == end clock drift check ==
2019-08-20T13:03:02.2319302Z ##[error]Bash exited with code '1'.
2019-08-20T13:03:02.2371809Z ##[section]Starting: Checkout
2019-08-20T13:03:02.2373512Z ==============================================================================
2019-08-20T13:03:02.2373570Z Task         : Get sources
2019-08-20T13:03:02.2373878Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
