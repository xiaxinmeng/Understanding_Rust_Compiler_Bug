plain
2020-02-21T14:00:29.5351300Z ========================== Starting Command Output ===========================
2020-02-21T14:00:29.5372581Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fa305975-e6ff-4ea8-8c4e-8ac30c40bec3.sh
2020-02-21T14:00:29.8101627Z 
2020-02-21T14:00:29.8217825Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-21T14:00:29.8258279Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69348/merge to s
2020-02-21T14:00:29.8275992Z Task         : Get sources
2020-02-21T14:00:29.8276542Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-21T14:00:29.8277068Z Version      : 1.0.0
2020-02-21T14:00:29.8277488Z Author       : Microsoft
---
2020-02-21T14:00:32.4191090Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-21T14:00:32.4370152Z ##[command]git config gc.auto 0
2020-02-21T14:00:32.4412498Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-21T14:00:32.4449012Z ##[command]git config --get-all http.proxy
2020-02-21T14:00:32.4567718Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69348/merge:refs/remotes/pull/69348/merge
---
2020-02-21T15:09:24.6929331Z .................................................................................................... 1700/9694
2020-02-21T15:09:29.6518301Z .................................................................................................... 1800/9694
2020-02-21T15:09:41.8921259Z ..........................................i......................................................... 1900/9694
2020-02-21T15:09:50.4140583Z .................................................................................................... 2000/9694
2020-02-21T15:10:05.5098445Z ................................iiiii............................................................... 2100/9694
2020-02-21T15:10:16.0556363Z .................................................................................................... 2300/9694
2020-02-21T15:10:18.6549407Z .................................................................................................... 2400/9694
2020-02-21T15:10:23.3238713Z .................................................................................................... 2500/9694
2020-02-21T15:10:45.7913468Z .................................................................................................... 2600/9694
---
2020-02-21T15:13:37.5365392Z .......i............................................................................................ 5000/9694
2020-02-21T15:13:47.5424375Z .................................................................................................... 5100/9694
2020-02-21T15:13:52.5001073Z ..................................i................................................................. 5200/9694
2020-02-21T15:14:03.1072821Z .................................................................................................... 5300/9694
2020-02-21T15:14:09.1527739Z ..........ii.ii........i...i........................................................................ 5400/9694
2020-02-21T15:14:17.9412577Z .................................................................................................... 5600/9694
2020-02-21T15:14:29.4829095Z .................................................................................................... 5700/9694
2020-02-21T15:14:37.5623911Z .i.................................................................................................. 5800/9694
2020-02-21T15:14:43.3569180Z .................................................................................................... 5900/9694
2020-02-21T15:14:43.3569180Z .................................................................................................... 5900/9694
2020-02-21T15:14:54.0196266Z ............................................................................................ii...i.. 6000/9694
2020-02-21T15:15:06.9442622Z ii...........i...................................................................................... 6100/9694
2020-02-21T15:15:24.7907393Z .................................................................................................... 6300/9694
2020-02-21T15:15:31.5115498Z .................................................................................................... 6400/9694
2020-02-21T15:15:31.5115498Z .................................................................................................... 6400/9694
2020-02-21T15:15:49.0625166Z .......................i..ii........................................................................ 6500/9694
2020-02-21T15:16:11.1129284Z .................................................................................................... 6700/9694
2020-02-21T15:16:13.6889775Z ...............i.................................................................................... 6800/9694
2020-02-21T15:16:16.1253624Z .................................................................................................... 6900/9694
2020-02-21T15:16:18.9635406Z .....................................i.............................................................. 7000/9694
2020-02-21T15:16:18.9635406Z .....................................i.............................................................. 7000/9694
2020-02-21T15:16:22.5271827Z ....................................................................F......F.F.F..F................. 7100/9694
2020-02-21T15:17:06.1765416Z .................................................................................................... 7300/9694
2020-02-21T15:17:49.9168247Z ...................................i.......i........................................................ 7400/9694
2020-02-21T15:17:56.1274597Z .................................................................................................... 7500/9694
2020-02-21T15:18:01.5841590Z .................................................................................................... 7600/9694
2020-02-21T15:18:01.5841590Z .................................................................................................... 7600/9694
2020-02-21T15:18:07.4530859Z .................................................................................................... 7700/9694
2020-02-21T15:18:12.7644007Z .................................................................................................... 7800/9694
2020-02-21T15:18:19.3766247Z .................................................................................i.................. 7900/9694
2020-02-21T15:18:29.1779733Z .................................................................................................... 8000/9694
2020-02-21T15:18:35.9266852Z ..............................iiiiiii.i............................................................. 8100/9694
2020-02-21T15:18:51.1527355Z .................................................................................................... 8300/9694
2020-02-21T15:19:00.1954159Z .................................................................................................... 8400/9694
2020-02-21T15:19:14.1287784Z .................................................................................................... 8500/9694
2020-02-21T15:19:21.4283301Z .................................................................................................... 8600/9694
---
2020-02-21T15:21:23.4365248Z ---- [ui] ui/pattern/bindings-after-at/bind-by-move-neither-can-live-while-the-other-survives-1.rs stdout ----
2020-02-21T15:21:23.4365891Z diff of stderr:
2020-02-21T15:21:23.4366191Z 
2020-02-21T15:21:23.4366514Z 15    |              |    |
2020-02-21T15:21:23.4366963Z 16    |              |    value borrowed here after move
2020-02-21T15:21:23.4367439Z 17    |              value moved into `_z` here
2020-02-21T15:21:23.4368328Z -    |              move occurs because `_z` has type `X` which does implement the `Copy` trait
2020-02-21T15:21:23.4369039Z +    |              move occurs because `_z` has type `X` which does not implement the `Copy` trait
2020-02-21T15:21:23.4369909Z 20 error: cannot move out of value because it is borrowed
2020-02-21T15:21:23.4370779Z 21   --> $DIR/bind-by-move-neither-can-live-while-the-other-survives-1.rs:29:14
2020-02-21T15:21:23.4371243Z 
2020-02-21T15:21:23.4371581Z 34    |              |    |
2020-02-21T15:21:23.4371581Z 34    |              |    |
2020-02-21T15:21:23.4372010Z 35    |              |    value borrowed here after move
2020-02-21T15:21:23.4372473Z 36    |              value moved into `_z` here
2020-02-21T15:21:23.4373341Z -    |              move occurs because `_z` has type `X` which does implement the `Copy` trait
2020-02-21T15:21:23.4374028Z +    |              move occurs because `_z` has type `X` which does not implement the `Copy` trait
2020-02-21T15:21:23.4374873Z 39 error[E0382]: borrow of moved value
2020-02-21T15:21:23.4375701Z 40   --> $DIR/bind-by-move-neither-can-live-while-the-other-survives-1.rs:21:19
2020-02-21T15:21:23.4376178Z 
2020-02-21T15:21:23.4376422Z 
2020-02-21T15:21:23.4376422Z 
2020-02-21T15:21:23.4376914Z The actual stderr differed from the expected stderr.
2020-02-21T15:21:23.4381615Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/bind-by-move-neither-can-live-while-the-other-survives-1/bind-by-move-neither-can-live-while-the-other-survives-1.stderr
2020-02-21T15:21:23.4382567Z To update references, rerun the tests and pass the `--bless` flag
2020-02-21T15:21:23.4383618Z To only update this specific test, also pass `--test-args pattern/bindings-after-at/bind-by-move-neither-can-live-while-the-other-survives-1.rs`
2020-02-21T15:21:23.4384611Z error: 1 errors occurred comparing output.
2020-02-21T15:21:23.4385025Z status: exit code: 1
2020-02-21T15:21:23.4385025Z status: exit code: 1
2020-02-21T15:21:23.4387871Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/bindings-after-at/bind-by-move-neither-can-live-while-the-other-survives-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/bind-by-move-neither-can-live-while-the-other-survives-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/bind-by-move-neither-can-live-while-the-other-survives-1/auxiliary"
2020-02-21T15:21:23.4390045Z ------------------------------------------
2020-02-21T15:21:23.4390251Z 
2020-02-21T15:21:23.4390681Z ------------------------------------------
2020-02-21T15:21:23.4390909Z stderr:
2020-02-21T15:21:23.4390909Z stderr:
2020-02-21T15:21:23.4391330Z ------------------------------------------
2020-02-21T15:21:23.4392102Z error: cannot move out of value because it is borrowed
2020-02-21T15:21:23.4392984Z   --> /checkout/src/test/ui/pattern/bindings-after-at/bind-by-move-neither-can-live-while-the-other-survives-1.rs:15:14
2020-02-21T15:21:23.4393394Z    |
2020-02-21T15:21:23.4393743Z LL |         Some(ref _y @ _z) => {} //~ ERROR cannot move out of value because it is borrowed
2020-02-21T15:21:23.4394306Z    |              ------^^^--
2020-02-21T15:21:23.4394549Z    |              |        |
2020-02-21T15:21:23.4394854Z    |              |        value moved into `_z` here
2020-02-21T15:21:23.4395183Z    |              value borrowed, by `_y`, here
2020-02-21T15:21:23.4395584Z error: borrow of moved value
2020-02-21T15:21:23.4396347Z   --> /checkout/src/test/ui/pattern/bindings-after-at/bind-by-move-neither-can-live-while-the-other-survives-1.rs:21:14
2020-02-21T15:21:23.4396761Z    |
2020-02-21T15:21:23.4396761Z    |
2020-02-21T15:21:23.4396971Z LL |         Some(_z @ ref _y) => {}
2020-02-21T15:21:23.4397429Z    |              --^^^------
2020-02-21T15:21:23.4397950Z    |              |    value borrowed here after move
2020-02-21T15:21:23.4397950Z    |              |    value borrowed here after move
2020-02-21T15:21:23.4398289Z    |              value moved into `_z` here
2020-02-21T15:21:23.4398699Z    |              move occurs because `_z` has type `X` which does not implement the `Copy` trait
2020-02-21T15:21:23.4399239Z error: cannot move out of value because it is borrowed
2020-02-21T15:21:23.4400046Z   --> /checkout/src/test/ui/pattern/bindings-after-at/bind-by-move-neither-can-live-while-the-other-survives-1.rs:29:14
2020-02-21T15:21:23.4400451Z    |
2020-02-21T15:21:23.4400451Z    |
2020-02-21T15:21:23.4400791Z LL |         Some(ref mut _y @ _z) => {} //~ ERROR cannot move out of value because it is borrowed
2020-02-21T15:21:23.4401391Z    |              ----------^^^--
2020-02-21T15:21:23.4401651Z    |              |            |
2020-02-21T15:21:23.4401957Z    |              |            value moved into `_z` here
2020-02-21T15:21:23.4402307Z    |              value borrowed, by `_y`, here
2020-02-21T15:21:23.4402698Z error: borrow of moved value
2020-02-21T15:21:23.4403450Z   --> /checkout/src/test/ui/pattern/bindings-after-at/bind-by-move-neither-can-live-while-the-other-survives-1.rs:35:14
2020-02-21T15:21:23.4403855Z    |
2020-02-21T15:21:23.4403855Z    |
2020-02-21T15:21:23.4404074Z LL |         Some(_z @ ref mut _y) => {}
2020-02-21T15:21:23.4404548Z    |              --^^^----------
2020-02-21T15:21:23.4405075Z    |              |    value borrowed here after move
2020-02-21T15:21:23.4405075Z    |              |    value borrowed here after move
2020-02-21T15:21:23.4405412Z    |              value moved into `_z` here
2020-02-21T15:21:23.4405941Z    |              move occurs because `_z` has type `X` which does not implement the `Copy` trait
2020-02-21T15:21:23.4406514Z error[E0382]: borrow of moved value
2020-02-21T15:21:23.4407325Z   --> /checkout/src/test/ui/pattern/bindings-after-at/bind-by-move-neither-can-live-while-the-other-survives-1.rs:21:19
2020-02-21T15:21:23.4407732Z    |
2020-02-21T15:21:23.4407732Z    |
2020-02-21T15:21:23.4407940Z LL |         Some(_z @ ref _y) => {}
2020-02-21T15:21:23.4408633Z    |              |    |
2020-02-21T15:21:23.4408916Z    |              |    value borrowed here after move
2020-02-21T15:21:23.4409234Z    |              value moved here
2020-02-21T15:21:23.4409435Z    |
2020-02-21T15:21:23.4409435Z    |
2020-02-21T15:21:23.4409763Z    = note: move occurs because value has type `X`, which does not implement the `Copy` trait
2020-02-21T15:21:23.4410076Z 
2020-02-21T15:21:23.4410281Z error[E0382]: borrow of moved value
2020-02-21T15:21:23.4411045Z   --> /checkout/src/test/ui/pattern/bindings-after-at/bind-by-move-neither-can-live-while-the-other-survives-1.rs:35:19
2020-02-21T15:21:23.4411451Z    |
2020-02-21T15:21:23.4411687Z LL |         Some(_z @ ref mut _y) => {}
2020-02-21T15:21:23.4412394Z    |              |    |
2020-02-21T15:21:23.4412692Z    |              |    value borrowed here after move
2020-02-21T15:21:23.4412993Z    |              value moved here
2020-02-21T15:21:23.4413197Z    |
---
2020-02-21T15:21:23.4416303Z ---- [ui] ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse-promotion.rs stdout ----
2020-02-21T15:21:23.4416667Z diff of stderr:
2020-02-21T15:21:23.4416807Z 
2020-02-21T15:21:23.4416992Z 6    |         |   |
2020-02-21T15:21:23.4417259Z 7    |         |   value borrowed here after move
2020-02-21T15:21:23.4417565Z 8    |         value moved into `a` here
2020-02-21T15:21:23.4418263Z -    |         move occurs because `a` has type `main::U` which does implement the `Copy` trait
2020-02-21T15:21:23.4418840Z +    |         move occurs because `a` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4419435Z 11 error: aborting due to previous error
2020-02-21T15:21:23.4419653Z 12 
2020-02-21T15:21:23.4419768Z 
2020-02-21T15:21:23.4419878Z 
2020-02-21T15:21:23.4419878Z 
2020-02-21T15:21:23.4420126Z The actual stderr differed from the expected stderr.
2020-02-21T15:21:23.4421122Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse-promotion/borrowck-pat-by-move-and-ref-inverse-promotion.stderr
2020-02-21T15:21:23.4422003Z To update references, rerun the tests and pass the `--bless` flag
2020-02-21T15:21:23.4422840Z To only update this specific test, also pass `--test-args pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse-promotion.rs`
2020-02-21T15:21:23.4423429Z error: 1 errors occurred comparing output.
2020-02-21T15:21:23.4423713Z status: exit code: 1
2020-02-21T15:21:23.4423713Z status: exit code: 1
2020-02-21T15:21:23.4433877Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse-promotion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse-promotion" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse-promotion/auxiliary"
2020-02-21T15:21:23.4436221Z ------------------------------------------
2020-02-21T15:21:23.4436447Z 
2020-02-21T15:21:23.4436881Z ------------------------------------------
2020-02-21T15:21:23.4437110Z stderr:
2020-02-21T15:21:23.4437110Z stderr:
2020-02-21T15:21:23.4437547Z ------------------------------------------
2020-02-21T15:21:23.4437821Z error: borrow of moved value
2020-02-21T15:21:23.4438522Z   --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse-promotion.rs:9:9
2020-02-21T15:21:23.4438917Z    |
2020-02-21T15:21:23.4439181Z LL |     let a @ ref b = U; //~ ERROR borrow of moved value
2020-02-21T15:21:23.4439652Z    |         -^^^-----
2020-02-21T15:21:23.4440150Z    |         |   value borrowed here after move
2020-02-21T15:21:23.4440454Z    |         value moved into `a` here
2020-02-21T15:21:23.4440454Z    |         value moved into `a` here
2020-02-21T15:21:23.4440900Z    |         move occurs because `a` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4441443Z error: aborting due to previous error
2020-02-21T15:21:23.4441630Z 
2020-02-21T15:21:23.4441738Z 
2020-02-21T15:21:23.4442169Z ------------------------------------------
2020-02-21T15:21:23.4442169Z ------------------------------------------
2020-02-21T15:21:23.4442363Z 
2020-02-21T15:21:23.4442471Z 
2020-02-21T15:21:23.4443020Z ---- [ui] ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs stdout ----
2020-02-21T15:21:23.4443374Z diff of stderr:
2020-02-21T15:21:23.4443515Z 
2020-02-21T15:21:23.4443683Z 6    |         |   |
2020-02-21T15:21:23.4443968Z 7    |         |   value borrowed here after move
2020-02-21T15:21:23.4444277Z 8    |         value moved into `a` here
2020-02-21T15:21:23.4444963Z -    |         move occurs because `a` has type `main::U` which does implement the `Copy` trait
2020-02-21T15:21:23.4445557Z +    |         move occurs because `a` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4446127Z 11 error: borrow of moved value
2020-02-21T15:21:23.4446703Z 12   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:31:9
2020-02-21T15:21:23.4446955Z 
2020-02-21T15:21:23.4447230Z 17    |         |            |              value borrowed here after move
2020-02-21T15:21:23.4447230Z 17    |         |            |              value borrowed here after move
2020-02-21T15:21:23.4447622Z 18    |         |            value borrowed here after move
2020-02-21T15:21:23.4447966Z 19    |         value moved into `a` here
2020-02-21T15:21:23.4454626Z -    |         move occurs because `a` has type `(main::U, main::U)` which does implement the `Copy` trait
2020-02-21T15:21:23.4455280Z +    |         move occurs because `a` has type `(main::U, main::U)` which does not implement the `Copy` trait
2020-02-21T15:21:23.4455911Z 22 error: borrow of moved value
2020-02-21T15:21:23.4456549Z 23   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:31:14
2020-02-21T15:21:23.4456833Z 
2020-02-21T15:21:23.4457025Z 27    |              |       |
2020-02-21T15:21:23.4457025Z 27    |              |       |
2020-02-21T15:21:23.4457330Z 28    |              |       value borrowed here after move
2020-02-21T15:21:23.4457682Z 29    |              value moved into `b` here
2020-02-21T15:21:23.4458402Z -    |              move occurs because `b` has type `main::U` which does implement the `Copy` trait
2020-02-21T15:21:23.4459002Z +    |              move occurs because `b` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4459591Z 32 error: borrow of moved value
2020-02-21T15:21:23.4460151Z 33   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:31:33
2020-02-21T15:21:23.4460404Z 
2020-02-21T15:21:23.4460638Z 37    |                                 |   |
2020-02-21T15:21:23.4460638Z 37    |                                 |   |
2020-02-21T15:21:23.4461001Z 38    |                                 |   value borrowed here after move
2020-02-21T15:21:23.4461554Z 39    |                                 value moved into `d` here
2020-02-21T15:21:23.4462508Z -    |                                 move occurs because `d` has type `main::U` which does implement the `Copy` trait
2020-02-21T15:21:23.4463201Z +    |                                 move occurs because `d` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4463837Z 42 error: borrow of moved value
2020-02-21T15:21:23.4464966Z 43   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:38:9
2020-02-21T15:21:23.4465216Z 
2020-02-21T15:21:23.4465488Z 48    |         |    |          value borrowed here after move
2020-02-21T15:21:23.4465488Z 48    |         |    |          value borrowed here after move
2020-02-21T15:21:23.4465840Z 49    |         |    value borrowed here after move
2020-02-21T15:21:23.4466153Z 50    |         value moved into `a` here
2020-02-21T15:21:23.4466882Z -    |         move occurs because `a` has type `[main::U; 2]` which does implement the `Copy` trait
2020-02-21T15:21:23.4467488Z +    |         move occurs because `a` has type `[main::U; 2]` which does not implement the `Copy` trait
2020-02-21T15:21:23.4468080Z 53 error: borrow of moved value
2020-02-21T15:21:23.4468633Z 54   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:41:9
2020-02-21T15:21:23.4468883Z 
2020-02-21T15:21:23.4469054Z 58    |         |   |
2020-02-21T15:21:23.4469054Z 58    |         |   |
2020-02-21T15:21:23.4469342Z 59    |         |   value borrowed here after move
2020-02-21T15:21:23.4469651Z 60    |         value moved into `a` here
2020-02-21T15:21:23.4470329Z -    |         move occurs because `a` has type `main::U` which does implement the `Copy` trait
2020-02-21T15:21:23.4470916Z +    |         move occurs because `a` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4471477Z 63 error: borrow of moved value
2020-02-21T15:21:23.4472045Z 64   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:44:9
2020-02-21T15:21:23.4472294Z 
2020-02-21T15:21:23.4472579Z 69    |         |            |              value borrowed here after move
2020-02-21T15:21:23.4472579Z 69    |         |            |              value borrowed here after move
2020-02-21T15:21:23.4472986Z 70    |         |            value borrowed here after move
2020-02-21T15:21:23.4473318Z 71    |         value moved into `a` here
2020-02-21T15:21:23.4474039Z -    |         move occurs because `a` has type `(main::U, main::U)` which does implement the `Copy` trait
2020-02-21T15:21:23.4474684Z +    |         move occurs because `a` has type `(main::U, main::U)` which does not implement the `Copy` trait
2020-02-21T15:21:23.4475270Z 74 error: borrow of moved value
2020-02-21T15:21:23.4475890Z 75   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:44:14
2020-02-21T15:21:23.4476149Z 
2020-02-21T15:21:23.4476338Z 79    |              |       |
2020-02-21T15:21:23.4476338Z 79    |              |       |
2020-02-21T15:21:23.4476642Z 80    |              |       value borrowed here after move
2020-02-21T15:21:23.4476996Z 81    |              value moved into `b` here
2020-02-21T15:21:23.4477724Z -    |              move occurs because `b` has type `main::U` which does implement the `Copy` trait
2020-02-21T15:21:23.4478324Z +    |              move occurs because `b` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4478916Z 84 error: borrow of moved value
2020-02-21T15:21:23.4479476Z 85   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:44:33
2020-02-21T15:21:23.4479745Z 
2020-02-21T15:21:23.4479963Z 89    |                                 |   |
2020-02-21T15:21:23.4479963Z 89    |                                 |   |
2020-02-21T15:21:23.4480326Z 90    |                                 |   value borrowed here after move
2020-02-21T15:21:23.4480746Z 91    |                                 value moved into `d` here
2020-02-21T15:21:23.4481552Z -    |                                 move occurs because `d` has type `main::U` which does implement the `Copy` trait
2020-02-21T15:21:23.4482243Z +    |                                 move occurs because `d` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4482998Z 94 error: borrow of moved value
2020-02-21T15:21:23.4483598Z 95   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:51:9
2020-02-21T15:21:23.4483935Z 
2020-02-21T15:21:23.4484211Z 100    |         |    |          value borrowed here after move
2020-02-21T15:21:23.4484211Z 100    |         |    |          value borrowed here after move
2020-02-21T15:21:23.4484569Z 101    |         |    value borrowed here after move
2020-02-21T15:21:23.4484886Z 102    |         value moved into `a` here
2020-02-21T15:21:23.4485632Z -    |         move occurs because `a` has type `[main::U; 2]` which does implement the `Copy` trait
2020-02-21T15:21:23.4486228Z +    |         move occurs because `a` has type `[main::U; 2]` which does not implement the `Copy` trait
2020-02-21T15:21:23.4486823Z 105 error: borrow of moved value
2020-02-21T15:21:23.4487383Z 106   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:56:9
2020-02-21T15:21:23.4487635Z 
2020-02-21T15:21:23.4487836Z 110    |         |        |
2020-02-21T15:21:23.4487836Z 110    |         |        |
2020-02-21T15:21:23.4488132Z 111    |         |        value borrowed here after move
2020-02-21T15:21:23.4488460Z 112    |         value moved into `a` here
2020-02-21T15:21:23.4489240Z -    |         move occurs because `a` has type `std::option::Option<main::U>` which does implement the `Copy` trait
2020-02-21T15:21:23.4489926Z +    |         move occurs because `a` has type `std::option::Option<main::U>` which does not implement the `Copy` trait
2020-02-21T15:21:23.4490562Z 115 error: borrow of moved value
2020-02-21T15:21:23.4491122Z 116   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:61:9
2020-02-21T15:21:23.4491376Z 
2020-02-21T15:21:23.4492262Z 121    |         |                 |              value borrowed here after move
2020-02-21T15:21:23.4492262Z 121    |         |                 |              value borrowed here after move
2020-02-21T15:21:23.4492726Z 122    |         |                 value borrowed here after move
2020-02-21T15:21:23.4493066Z 123    |         value moved into `a` here
2020-02-21T15:21:23.4493954Z -    |         move occurs because `a` has type `std::option::Option<(main::U, main::U)>` which does implement the `Copy` trait
2020-02-21T15:21:23.4494722Z +    |         move occurs because `a` has type `std::option::Option<(main::U, main::U)>` which does not implement the `Copy` trait
2020-02-21T15:21:23.4495656Z 126 error: borrow of moved value
2020-02-21T15:21:23.4496273Z 127   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:61:19
2020-02-21T15:21:23.4496529Z 
2020-02-21T15:21:23.4496731Z 131    |                   |       |
2020-02-21T15:21:23.4496731Z 131    |                   |       |
2020-02-21T15:21:23.4497077Z 132    |                   |       value borrowed here after move
2020-02-21T15:21:23.4497437Z 133    |                   value moved into `b` here
2020-02-21T15:21:23.4498179Z -    |                   move occurs because `b` has type `main::U` which does implement the `Copy` trait
2020-02-21T15:21:23.4498820Z +    |                   move occurs because `b` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4499410Z 136 error: borrow of moved value
2020-02-21T15:21:23.4499994Z 137   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:61:38
2020-02-21T15:21:23.4500250Z 
2020-02-21T15:21:23.4500480Z 141    |                                      |   |
2020-02-21T15:21:23.4500480Z 141    |                                      |   |
2020-02-21T15:21:23.4500888Z 142    |                                      |   value borrowed here after move
2020-02-21T15:21:23.4501316Z 143    |                                      value moved into `d` here
2020-02-21T15:21:23.4502154Z -    |                                      move occurs because `d` has type `main::U` which does implement the `Copy` trait
2020-02-21T15:21:23.4502885Z +    |                                      move occurs because `d` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4503522Z 146 error: borrow of moved value
2020-02-21T15:21:23.4504270Z 147   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:71:9
2020-02-21T15:21:23.4504535Z 
2020-02-21T15:21:23.4504804Z 152    |         |             |      value borrowed here after move
2020-02-21T15:21:23.4504804Z 152    |         |             |      value borrowed here after move
2020-02-21T15:21:23.4505327Z 153    |         |             value borrowed here after move
2020-02-21T15:21:23.4505691Z 154    |         value moved into `a` here
2020-02-21T15:21:23.4506608Z -    |         move occurs because `a` has type `std::option::Option<[main::U; 2]>` which does implement the `Copy` trait
2020-02-21T15:21:23.4507329Z +    |         move occurs because `a` has type `std::option::Option<[main::U; 2]>` which does not implement the `Copy` trait
2020-02-21T15:21:23.4507958Z 157 error: borrow of moved value
2020-02-21T15:21:23.4508518Z 158   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:77:9
2020-02-21T15:21:23.4508785Z 
2020-02-21T15:21:23.4508968Z 162    |         |        |
2020-02-21T15:21:23.4508968Z 162    |         |        |
2020-02-21T15:21:23.4509263Z 163    |         |        value borrowed here after move
2020-02-21T15:21:23.4509603Z 164    |         value moved into `a` here
2020-02-21T15:21:23.4510358Z -    |         move occurs because `a` has type `std::option::Option<main::U>` which does implement the `Copy` trait
2020-02-21T15:21:23.4511044Z +    |         move occurs because `a` has type `std::option::Option<main::U>` which does not implement the `Copy` trait
2020-02-21T15:21:23.4511685Z 167 error: borrow of moved value
2020-02-21T15:21:23.4512247Z 168   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:83:9
2020-02-21T15:21:23.4512513Z 
2020-02-21T15:21:23.4512802Z 173    |         |                 |              value borrowed here after move
2020-02-21T15:21:23.4512802Z 173    |         |                 |              value borrowed here after move
2020-02-21T15:21:23.4513216Z 174    |         |                 value borrowed here after move
2020-02-21T15:21:23.4518081Z 175    |         value moved into `a` here
2020-02-21T15:21:23.4519068Z -    |         move occurs because `a` has type `std::option::Option<(main::U, main::U)>` which does implement the `Copy` trait
2020-02-21T15:21:23.4519811Z +    |         move occurs because `a` has type `std::option::Option<(main::U, main::U)>` which does not implement the `Copy` trait
2020-02-21T15:21:23.4520494Z 178 error: borrow of moved value
2020-02-21T15:21:23.4521080Z 179   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:83:19
2020-02-21T15:21:23.4521344Z 
2020-02-21T15:21:23.4521563Z 183    |                   |       |
2020-02-21T15:21:23.4521563Z 183    |                   |       |
2020-02-21T15:21:23.4521892Z 184    |                   |       value borrowed here after move
2020-02-21T15:21:23.4522253Z 185    |                   value moved into `b` here
2020-02-21T15:21:23.4523014Z -    |                   move occurs because `b` has type `main::U` which does implement the `Copy` trait
2020-02-21T15:21:23.4523638Z +    |                   move occurs because `b` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4524245Z 188 error: borrow of moved value
2020-02-21T15:21:23.4524807Z 189   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:83:38
2020-02-21T15:21:23.4525060Z 
2020-02-21T15:21:23.4525306Z 193    |                                      |   |
2020-02-21T15:21:23.4525306Z 193    |                                      |   |
2020-02-21T15:21:23.4525700Z 194    |                                      |   value borrowed here after move
2020-02-21T15:21:23.4526127Z 195    |                                      value moved into `d` here
2020-02-21T15:21:23.4526989Z -    |                                      move occurs because `d` has type `main::U` which does implement the `Copy` trait
2020-02-21T15:21:23.4527706Z +    |                                      move occurs because `d` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4528361Z 198 error: borrow of moved value
2020-02-21T15:21:23.4528917Z 199   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:93:9
2020-02-21T15:21:23.4529170Z 
2020-02-21T15:21:23.4529434Z 204    |         |             |      value borrowed here after move
2020-02-21T15:21:23.4529434Z 204    |         |             |      value borrowed here after move
2020-02-21T15:21:23.4529834Z 205    |         |             value borrowed here after move
2020-02-21T15:21:23.4530164Z 206    |         value moved into `a` here
2020-02-21T15:21:23.4531111Z -    |         move occurs because `a` has type `std::option::Option<[main::U; 2]>` which does implement the `Copy` trait
2020-02-21T15:21:23.4531850Z +    |         move occurs because `a` has type `std::option::Option<[main::U; 2]>` which does not implement the `Copy` trait
2020-02-21T15:21:23.4532559Z 209 error: borrow of moved value
2020-02-21T15:21:23.4542315Z 210   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:14:11
2020-02-21T15:21:23.4542602Z 
2020-02-21T15:21:23.4542792Z 214    |           |   |
2020-02-21T15:21:23.4542792Z 214    |           |   |
2020-02-21T15:21:23.4543110Z 215    |           |   value borrowed here after move
2020-02-21T15:21:23.4543434Z 216    |           value moved into `a` here
2020-02-21T15:21:23.4544628Z -    |           move occurs because `a` has type `main::U` which does implement the `Copy` trait
2020-02-21T15:21:23.4545935Z +    |           move occurs because `a` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4546713Z 219 error: borrow of moved value
2020-02-21T15:21:23.4547460Z 220   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:18:11
2020-02-21T15:21:23.4547720Z 
2020-02-21T15:21:23.4548004Z 225    |           |            |              value borrowed here after move
2020-02-21T15:21:23.4548004Z 225    |           |            |              value borrowed here after move
2020-02-21T15:21:23.4548414Z 226    |           |            value borrowed here after move
2020-02-21T15:21:23.4548771Z 227    |           value moved into `a` here
2020-02-21T15:21:23.4549520Z -    |           move occurs because `a` has type `(main::U, main::U)` which does implement the `Copy` trait
2020-02-21T15:21:23.4550176Z +    |           move occurs because `a` has type `(main::U, main::U)` which does not implement the `Copy` trait
2020-02-21T15:21:23.4550896Z 230 error: borrow of moved value
2020-02-21T15:21:23.4552154Z 231   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:18:20
2020-02-21T15:21:23.4552427Z 
2020-02-21T15:21:23.4552630Z 235    |                    |   |
2020-02-21T15:21:23.4552630Z 235    |                    |   |
2020-02-21T15:21:23.4552949Z 236    |                    |   value borrowed here after move
2020-02-21T15:21:23.4553334Z 237    |                    value moved into `b` here
2020-02-21T15:21:23.4554144Z -    |                    move occurs because `b` has type `main::U` which does implement the `Copy` trait
2020-02-21T15:21:23.4555344Z +    |                    move occurs because `b` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4555987Z 240 error: borrow of moved value
2020-02-21T15:21:23.4556665Z 241   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:18:31
2020-02-21T15:21:23.4556939Z 
2020-02-21T15:21:23.4557165Z 245    |                               |       |
2020-02-21T15:21:23.4557165Z 245    |                               |       |
2020-02-21T15:21:23.4557762Z 246    |                               |       value borrowed here after move
2020-02-21T15:21:23.4558192Z 247    |                               value moved into `d` here
2020-02-21T15:21:23.4559057Z -    |                               move occurs because `d` has type `main::U` which does implement the `Copy` trait
2020-02-21T15:21:23.4559870Z +    |                               move occurs because `d` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4560519Z 250 error: borrow of moved value
2020-02-21T15:21:23.4561125Z 251   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:25:11
2020-02-21T15:21:23.4561381Z 
2020-02-21T15:21:23.4561655Z 256    |           |    |          value borrowed here after move
2020-02-21T15:21:23.4561655Z 256    |           |    |          value borrowed here after move
2020-02-21T15:21:23.4562019Z 257    |           |    value borrowed here after move
2020-02-21T15:21:23.4562343Z 258    |           value moved into `a` here
2020-02-21T15:21:23.4563068Z -    |           move occurs because `a` has type `[main::U; 2]` which does implement the `Copy` trait
2020-02-21T15:21:23.4566967Z +    |           move occurs because `a` has type `[main::U; 2]` which does not implement the `Copy` trait
2020-02-21T15:21:23.4567826Z 261 error[E0382]: borrow of moved value
2020-02-21T15:21:23.4568667Z 262   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:31:22
2020-02-21T15:21:23.4568937Z 
2020-02-21T15:21:23.4569047Z 
2020-02-21T15:21:23.4569047Z 
2020-02-21T15:21:23.4569300Z The actual stderr differed from the expected stderr.
2020-02-21T15:21:23.4570354Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse/borrowck-pat-by-move-and-ref-inverse.stderr
2020-02-21T15:21:23.4571214Z To update references, rerun the tests and pass the `--bless` flag
2020-02-21T15:21:23.4572006Z To only update this specific test, also pass `--test-args pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs`
2020-02-21T15:21:23.4572591Z error: 1 errors occurred comparing output.
2020-02-21T15:21:23.4572860Z status: exit code: 1
2020-02-21T15:21:23.4572860Z status: exit code: 1
2020-02-21T15:21:23.4575406Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse/auxiliary"
2020-02-21T15:21:23.4577714Z ------------------------------------------
2020-02-21T15:21:23.4577932Z 
2020-02-21T15:21:23.4578351Z ------------------------------------------
2020-02-21T15:21:23.4578578Z stderr:
2020-02-21T15:21:23.4578578Z stderr:
2020-02-21T15:21:23.4579015Z ------------------------------------------
2020-02-21T15:21:23.4579288Z error: borrow of moved value
2020-02-21T15:21:23.4579951Z   --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs:29:9
2020-02-21T15:21:23.4580330Z    |
2020-02-21T15:21:23.4580524Z LL |     let a @ ref b = U;
2020-02-21T15:21:23.4580929Z    |         -^^^-----
2020-02-21T15:21:23.4581587Z    |         |   value borrowed here after move
2020-02-21T15:21:23.4581886Z    |         value moved into `a` here
2020-02-21T15:21:23.4581886Z    |         value moved into `a` here
2020-02-21T15:21:23.4582330Z    |         move occurs because `a` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4582848Z error: borrow of moved value
2020-02-21T15:21:23.4583568Z   --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs:31:9
2020-02-21T15:21:23.4583938Z    |
2020-02-21T15:21:23.4583938Z    |
2020-02-21T15:21:23.4584318Z LL |     let a @ (mut b @ ref mut c, d @ ref e) = (U, U);
2020-02-21T15:21:23.4585071Z    |         -^^^^^^^^^^^^---------^^^^^^-----^
2020-02-21T15:21:23.4585765Z    |         |            |              value borrowed here after move
2020-02-21T15:21:23.4586146Z    |         |            value borrowed here after move
2020-02-21T15:21:23.4586483Z    |         value moved into `a` here
2020-02-21T15:21:23.4586483Z    |         value moved into `a` here
2020-02-21T15:21:23.4586937Z    |         move occurs because `a` has type `(main::U, main::U)` which does not implement the `Copy` trait
2020-02-21T15:21:23.4587500Z error: borrow of moved value
2020-02-21T15:21:23.4588181Z   --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs:31:14
2020-02-21T15:21:23.4588538Z    |
2020-02-21T15:21:23.4588538Z    |
2020-02-21T15:21:23.4588949Z LL |     let a @ (mut b @ ref mut c, d @ ref e) = (U, U);
2020-02-21T15:21:23.4589510Z    |              -----^^^---------
2020-02-21T15:21:23.4590062Z    |              |       value borrowed here after move
2020-02-21T15:21:23.4590062Z    |              |       value borrowed here after move
2020-02-21T15:21:23.4590405Z    |              value moved into `b` here
2020-02-21T15:21:23.4590974Z    |              move occurs because `b` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4591536Z error: borrow of moved value
2020-02-21T15:21:23.4592527Z   --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs:31:33
2020-02-21T15:21:23.4592892Z    |
2020-02-21T15:21:23.4592892Z    |
2020-02-21T15:21:23.4593159Z LL |     let a @ (mut b @ ref mut c, d @ ref e) = (U, U);
2020-02-21T15:21:23.4593699Z    |                                 -^^^-----
2020-02-21T15:21:23.4594375Z    |                                 |   value borrowed here after move
2020-02-21T15:21:23.4594375Z    |                                 |   value borrowed here after move
2020-02-21T15:21:23.4594766Z    |                                 value moved into `d` here
2020-02-21T15:21:23.4595298Z    |                                 move occurs because `d` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4595891Z error: borrow of moved value
2020-02-21T15:21:23.4596562Z   --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs:38:9
2020-02-21T15:21:23.4596931Z    |
2020-02-21T15:21:23.4596931Z    |
2020-02-21T15:21:23.4597165Z LL |     let a @ [ref mut b, ref c] = [U, U];
2020-02-21T15:21:23.4597646Z    |         -^^^^---------^^-----^
2020-02-21T15:21:23.4598404Z    |         |    |          value borrowed here after move
2020-02-21T15:21:23.4598744Z    |         |    value borrowed here after move
2020-02-21T15:21:23.4599043Z    |         value moved into `a` here
2020-02-21T15:21:23.4599043Z    |         value moved into `a` here
2020-02-21T15:21:23.4599502Z    |         move occurs because `a` has type `[main::U; 2]` which does not implement the `Copy` trait
2020-02-21T15:21:23.4600031Z error: borrow of moved value
2020-02-21T15:21:23.4600764Z   --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs:41:9
2020-02-21T15:21:23.4601119Z    |
2020-02-21T15:21:23.4601119Z    |
2020-02-21T15:21:23.4601314Z LL |     let a @ ref b = u();
2020-02-21T15:21:23.4601932Z    |         -^^^-----
2020-02-21T15:21:23.4602421Z    |         |   value borrowed here after move
2020-02-21T15:21:23.4602741Z    |         value moved into `a` here
2020-02-21T15:21:23.4602741Z    |         value moved into `a` here
2020-02-21T15:21:23.4603167Z    |         move occurs because `a` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4603684Z error: borrow of moved value
2020-02-21T15:21:23.4604531Z   --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs:44:9
2020-02-21T15:21:23.4604895Z    |
2020-02-21T15:21:23.4604895Z    |
2020-02-21T15:21:23.4605150Z LL |     let a @ (mut b @ ref mut c, d @ ref e) = (u(), u());
2020-02-21T15:21:23.4605752Z    |         -^^^^^^^^^^^^---------^^^^^^-----^
2020-02-21T15:21:23.4606411Z    |         |            |              value borrowed here after move
2020-02-21T15:21:23.4606807Z    |         |            value borrowed here after move
2020-02-21T15:21:23.4607129Z    |         value moved into `a` here
2020-02-21T15:21:23.4607129Z    |         value moved into `a` here
2020-02-21T15:21:23.4607583Z    |         move occurs because `a` has type `(main::U, main::U)` which does not implement the `Copy` trait
2020-02-21T15:21:23.4608148Z error: borrow of moved value
2020-02-21T15:21:23.4608819Z   --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs:44:14
2020-02-21T15:21:23.4609190Z    |
2020-02-21T15:21:23.4609190Z    |
2020-02-21T15:21:23.4609447Z LL |     let a @ (mut b @ ref mut c, d @ ref e) = (u(), u());
2020-02-21T15:21:23.4609951Z    |              -----^^^---------
2020-02-21T15:21:23.4610517Z    |              |       value borrowed here after move
2020-02-21T15:21:23.4610517Z    |              |       value borrowed here after move
2020-02-21T15:21:23.4610997Z    |              value moved into `b` here
2020-02-21T15:21:23.4611450Z    |              move occurs because `b` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4611998Z error: borrow of moved value
2020-02-21T15:21:23.4612820Z   --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs:44:33
2020-02-21T15:21:23.4613254Z    |
2020-02-21T15:21:23.4613254Z    |
2020-02-21T15:21:23.4613510Z LL |     let a @ (mut b @ ref mut c, d @ ref e) = (u(), u());
2020-02-21T15:21:23.4614083Z    |                                 -^^^-----
2020-02-21T15:21:23.4614756Z    |                                 |   value borrowed here after move
2020-02-21T15:21:23.4614756Z    |                                 |   value borrowed here after move
2020-02-21T15:21:23.4615146Z    |                                 value moved into `d` here
2020-02-21T15:21:23.4615694Z    |                                 move occurs because `d` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4616269Z error: borrow of moved value
2020-02-21T15:21:23.4616930Z   --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs:51:9
2020-02-21T15:21:23.4617299Z    |
2020-02-21T15:21:23.4617299Z    |
2020-02-21T15:21:23.4617542Z LL |     let a @ [ref mut b, ref c] = [u(), u()];
2020-02-21T15:21:23.4618028Z    |         -^^^^---------^^-----^
2020-02-21T15:21:23.4618617Z    |         |    |          value borrowed here after move
2020-02-21T15:21:23.4618959Z    |         |    value borrowed here after move
2020-02-21T15:21:23.4619275Z    |         value moved into `a` here
2020-02-21T15:21:23.4619275Z    |         value moved into `a` here
2020-02-21T15:21:23.4619712Z    |         move occurs because `a` has type `[main::U; 2]` which does not implement the `Copy` trait
2020-02-21T15:21:23.4620254Z error: borrow of moved value
2020-02-21T15:21:23.4620919Z   --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs:56:9
2020-02-21T15:21:23.4621274Z    |
2020-02-21T15:21:23.4621274Z    |
2020-02-21T15:21:23.4621495Z LL |         a @ Some(ref b) => {}
2020-02-21T15:21:23.4621930Z    |         -^^^^^^^^-----^
2020-02-21T15:21:23.4622465Z    |         |        value borrowed here after move
2020-02-21T15:21:23.4622775Z    |         value moved into `a` here
2020-02-21T15:21:23.4622775Z    |         value moved into `a` here
2020-02-21T15:21:23.4623260Z    |         move occurs because `a` has type `std::option::Option<main::U>` which does not implement the `Copy` trait
2020-02-21T15:21:23.4623846Z error: borrow of moved value
2020-02-21T15:21:23.4625110Z   --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs:61:9
2020-02-21T15:21:23.4625470Z    |
2020-02-21T15:21:23.4625470Z    |
2020-02-21T15:21:23.4625738Z LL |         a @ Some((mut b @ ref mut c, d @ ref e)) => {}
2020-02-21T15:21:23.4626305Z    |         -^^^^^^^^^^^^^^^^^---------^^^^^^-----^^
2020-02-21T15:21:23.4627023Z    |         |                 |              value borrowed here after move
2020-02-21T15:21:23.4627423Z    |         |                 value borrowed here after move
2020-02-21T15:21:23.4627748Z    |         value moved into `a` here
2020-02-21T15:21:23.4627748Z    |         value moved into `a` here
2020-02-21T15:21:23.4628280Z    |         move occurs because `a` has type `std::option::Option<(main::U, main::U)>` which does not implement the `Copy` trait
2020-02-21T15:21:23.4628891Z error: borrow of moved value
2020-02-21T15:21:23.4629578Z   --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs:61:19
2020-02-21T15:21:23.4629935Z    |
2020-02-21T15:21:23.4629935Z    |
2020-02-21T15:21:23.4630188Z LL |         a @ Some((mut b @ ref mut c, d @ ref e)) => {}
2020-02-21T15:21:23.4630716Z    |                   -----^^^---------
2020-02-21T15:21:23.4631302Z    |                   |       value borrowed here after move
2020-02-21T15:21:23.4631302Z    |                   |       value borrowed here after move
2020-02-21T15:21:23.4631664Z    |                   value moved into `b` here
2020-02-21T15:21:23.4632134Z    |                   move occurs because `b` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4632677Z error: borrow of moved value
2020-02-21T15:21:23.4633725Z   --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs:61:38
2020-02-21T15:21:23.4634158Z    |
2020-02-21T15:21:23.4634158Z    |
2020-02-21T15:21:23.4634409Z LL |         a @ Some((mut b @ ref mut c, d @ ref e)) => {}
2020-02-21T15:21:23.4635020Z    |                                      -^^^-----
2020-02-21T15:21:23.4635718Z    |                                      |   value borrowed here after move
2020-02-21T15:21:23.4635718Z    |                                      |   value borrowed here after move
2020-02-21T15:21:23.4636146Z    |                                      value moved into `d` here
2020-02-21T15:21:23.4636700Z    |                                      move occurs because `d` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4637308Z error: borrow of moved value
2020-02-21T15:21:23.4637970Z   --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs:71:9
2020-02-21T15:21:23.4638322Z    |
2020-02-21T15:21:23.4638322Z    |
2020-02-21T15:21:23.4638583Z LL |         mut a @ Some([ref b, ref mut c]) => {}
2020-02-21T15:21:23.4639105Z    |         -----^^^^^^^^^-----^^---------^^
2020-02-21T15:21:23.4639746Z    |         |             |      value borrowed here after move
2020-02-21T15:21:23.4640115Z    |         |             value borrowed here after move
2020-02-21T15:21:23.4640433Z    |         value moved into `a` here
2020-02-21T15:21:23.4640433Z    |         value moved into `a` here
2020-02-21T15:21:23.4640939Z    |         move occurs because `a` has type `std::option::Option<[main::U; 2]>` which does not implement the `Copy` trait
2020-02-21T15:21:23.4641520Z error: borrow of moved value
2020-02-21T15:21:23.4642182Z   --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs:77:9
2020-02-21T15:21:23.4642551Z    |
2020-02-21T15:21:23.4642551Z    |
2020-02-21T15:21:23.4642755Z LL |         a @ Some(ref b) => {}
2020-02-21T15:21:23.4643190Z    |         -^^^^^^^^-----^
2020-02-21T15:21:23.4643725Z    |         |        value borrowed here after move
2020-02-21T15:21:23.4644033Z    |         value moved into `a` here
2020-02-21T15:21:23.4644033Z    |         value moved into `a` here
2020-02-21T15:21:23.4644533Z    |         move occurs because `a` has type `std::option::Option<main::U>` which does not implement the `Copy` trait
2020-02-21T15:21:23.4645102Z error: borrow of moved value
2020-02-21T15:21:23.4645783Z   --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs:83:9
2020-02-21T15:21:23.4646136Z    |
2020-02-21T15:21:23.4646136Z    |
2020-02-21T15:21:23.4646386Z LL |         a @ Some((mut b @ ref mut c, d @ ref e)) => {}
2020-02-21T15:21:23.4646955Z    |         -^^^^^^^^^^^^^^^^^---------^^^^^^-----^^
2020-02-21T15:21:23.4647656Z    |         |                 |              value borrowed here after move
2020-02-21T15:21:23.4653663Z    |         |                 value borrowed here after move
2020-02-21T15:21:23.4654239Z    |         value moved into `a` here
2020-02-21T15:21:23.4654239Z    |         value moved into `a` here
2020-02-21T15:21:23.4654758Z    |         move occurs because `a` has type `std::option::Option<(main::U, main::U)>` which does not implement the `Copy` trait
2020-02-21T15:21:23.4658260Z error: borrow of moved value
2020-02-21T15:21:23.4659091Z   --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs:83:19
2020-02-21T15:21:23.4659450Z    |
2020-02-21T15:21:23.4659450Z    |
2020-02-21T15:21:23.4659726Z LL |         a @ Some((mut b @ ref mut c, d @ ref e)) => {}
2020-02-21T15:21:23.4660254Z    |                   -----^^^---------
2020-02-21T15:21:23.4660855Z    |                   |       value borrowed here after move
2020-02-21T15:21:23.4660855Z    |                   |       value borrowed here after move
2020-02-21T15:21:23.4661199Z    |                   value moved into `b` here
2020-02-21T15:21:23.4661670Z    |                   move occurs because `b` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4662378Z error: borrow of moved value
2020-02-21T15:21:23.4663104Z   --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs:83:38
2020-02-21T15:21:23.4663566Z    |
2020-02-21T15:21:23.4663566Z    |
2020-02-21T15:21:23.4663817Z LL |         a @ Some((mut b @ ref mut c, d @ ref e)) => {}
2020-02-21T15:21:23.4664668Z    |                                      -^^^-----
2020-02-21T15:21:23.4665393Z    |                                      |   value borrowed here after move
2020-02-21T15:21:23.4665393Z    |                                      |   value borrowed here after move
2020-02-21T15:21:23.4665806Z    |                                      value moved into `d` here
2020-02-21T15:21:23.4666376Z    |                                      move occurs because `d` has type `main::U` which does not implement the `Copy` trait
2020-02-21T15:21:23.4666965Z error: borrow of moved value
2020-02-21T15:21:23.4667640Z   --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs:93:9
2020-02-21T15:21:23.4668021Z    |
2020-02-21T15:21:23.4668021Z    |
2020-02-21T15:21:23.4668261Z LL |         mut a @ Some([ref b, ref mut c]) => {}
2020-02-21T15:21:23.4668792Z    |         -----^^^^^^^^^-----^^---------^^
2020-02-21T15:21:23.4669426Z    |         |             |      value borrowed here after move
2020-02-21T15:21:23.4669795Z    |         |             value borrowed here after move
2020-02-21T15:21:23.4670128Z    |         value moved into `a` here
2020-02-21T15:21:23.4670128Z    |         value moved into `a` here
2020-02-21T15:21:23.4670667Z    |         move occurs because `a` has type `std::option::Option<[main::U; 2]>` which does not implement the `Copy` trait
2020-02-21T15:21:23.4671268Z error: borrow of moved value
2020-02-21T15:21:23.4671953Z   --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs:14:11
2020-02-21T15:21:23.4672310Z    |
2020-02-21T15:21:23.4672310Z    |
2020-02-21T15:21:23.4672540Z LL |     fn f1(a @ ref b: U) {}
2020-02-21T15:21:23.4672975Z    |           -^^^-----
2020-02-21T15:21:23.4673471Z    |           |   value borrowed here after move
---
2020-02-21T15:21:23.5078413Z ---- [ui] ui/pattern/bindings-after-at/default-binding-modes-both-sides-independent.rs stdout ----
2020-02-21T15:21:23.5078925Z diff of stderr:
2020-02-21T15:21:23.5079607Z 
2020-02-21T15:21:23.5080589Z 33    |                             |   |
2020-02-21T15:21:23.5081485Z 34    |                             |   value borrowed here after move
2020-02-21T15:21:23.5082326Z 35    |                             value moved into `b` here
2020-02-21T15:21:23.5083741Z -    |                             move occurs because `b` has type `main::NotCopy` which does implement the `Copy` trait
2020-02-21T15:21:23.5084462Z +    |                             move occurs because `b` has type `main::NotCopy` which does not implement the `Copy` trait
2020-02-21T15:21:23.5085737Z 38 error: cannot move out of value because it is borrowed
2020-02-21T15:21:23.5086931Z 39   --> $DIR/default-binding-modes-both-sides-independent.rs:44:9
2020-02-21T15:21:23.5087280Z 
2020-02-21T15:21:23.5088150Z 
2020-02-21T15:21:23.5088150Z 
2020-02-21T15:21:23.5089167Z The actual stderr differed from the expected stderr.
2020-02-21T15:21:23.5090750Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/default-binding-modes-both-sides-independent/default-binding-modes-both-sides-independent.stderr
2020-02-21T15:21:23.5091679Z To update references, rerun the tests and pass the `--bless` flag
2020-02-21T15:21:23.5092501Z To only update this specific test, also pass `--test-args pattern/bindings-after-at/default-binding-modes-both-sides-independent.rs`
2020-02-21T15:21:23.5093361Z error: 1 errors occurred comparing output.
2020-02-21T15:21:23.5094180Z status: exit code: 1
2020-02-21T15:21:23.5094180Z status: exit code: 1
2020-02-21T15:21:23.5097516Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/bindings-after-at/default-binding-modes-both-sides-independent.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/default-binding-modes-both-sides-independent" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/default-binding-modes-both-sides-independent/auxiliary"
2020-02-21T15:21:23.5099658Z ------------------------------------------
2020-02-21T15:21:23.5099856Z 
2020-02-21T15:21:23.5100282Z ------------------------------------------
2020-02-21T15:21:23.5100511Z stderr:
2020-02-21T15:21:23.5100511Z stderr:
2020-02-21T15:21:23.5100947Z ------------------------------------------
2020-02-21T15:21:23.5101288Z error: cannot move out of value because it is borrowed
2020-02-21T15:21:23.5102050Z   --> /checkout/src/test/ui/pattern/bindings-after-at/default-binding-modes-both-sides-independent.rs:28:9
2020-02-21T15:21:23.5102435Z    |
2020-02-21T15:21:23.5103402Z LL |     let ref a @ b = NotCopy; //~ ERROR cannot move out of value because it is borrowed
2020-02-21T15:21:23.5104496Z    |         -----^^^-
2020-02-21T15:21:23.5105237Z    |         |       |
2020-02-21T15:21:23.5106119Z    |         |       value moved into `b` here
2020-02-21T15:21:23.5106971Z    |         value borrowed, by `a`, here
2020-02-21T15:21:23.5108731Z error: cannot move out of value because it is borrowed
2020-02-21T15:21:23.5110068Z   --> /checkout/src/test/ui/pattern/bindings-after-at/default-binding-modes-both-sides-independent.rs:31:9
2020-02-21T15:21:23.5110483Z    |
2020-02-21T15:21:23.5110483Z    |
2020-02-21T15:21:23.5111403Z LL |     let ref mut a @ b = NotCopy; //~ ERROR cannot move out of value because it is borrowed
2020-02-21T15:21:23.5112364Z    |         ---------^^^-
2020-02-21T15:21:23.5113014Z    |         |           |
2020-02-21T15:21:23.5114051Z    |         |           value moved into `b` here
2020-02-21T15:21:23.5114804Z    |         value borrowed, by `a`, here
2020-02-21T15:21:23.5115597Z error: cannot move out of value because it is borrowed
2020-02-21T15:21:23.5116503Z   --> /checkout/src/test/ui/pattern/bindings-after-at/default-binding-modes-both-sides-independent.rs:36:12
2020-02-21T15:21:23.5122220Z    |
2020-02-21T15:21:23.5122220Z    |
2020-02-21T15:21:23.5122454Z LL |         Ok(ref a @ b) | Err(b @ ref a) => {
2020-02-21T15:21:23.5123296Z    |            -----^^^-
2020-02-21T15:21:23.5123536Z    |            |       |
2020-02-21T15:21:23.5123811Z    |            |       value moved into `b` here
2020-02-21T15:21:23.5124145Z    |            value borrowed, by `a`, here
2020-02-21T15:21:23.5127162Z error: borrow of moved value
2020-02-21T15:21:23.5127933Z   --> /checkout/src/test/ui/pattern/bindings-after-at/default-binding-modes-both-sides-independent.rs:36:29
2020-02-21T15:21:23.5128312Z    |
2020-02-21T15:21:23.5128312Z    |
2020-02-21T15:21:23.5128558Z LL |         Ok(ref a @ b) | Err(b @ ref a) => {
2020-02-21T15:21:23.5129085Z    |                             -^^^-----
2020-02-21T15:21:23.5129718Z    |                             |   value borrowed here after move
2020-02-21T15:21:23.5129718Z    |                             |   value borrowed here after move
2020-02-21T15:21:23.5130112Z    |                             value moved into `b` here
2020-02-21T15:21:23.5130639Z    |                             move occurs because `b` has type `main::NotCopy` which does not implement the `Copy` trait
2020-02-21T15:21:23.5131269Z error: cannot move out of value because it is borrowed
2020-02-21T15:21:23.5132034Z   --> /checkout/src/test/ui/pattern/bindings-after-at/default-binding-modes-both-sides-independent.rs:44:9
2020-02-21T15:21:23.5132407Z    |
2020-02-21T15:21:23.5132599Z LL |         ref a @ b => {
2020-02-21T15:21:23.5132599Z LL |         ref a @ b => {
2020-02-21T15:21:23.5133019Z    |         -----^^^-
2020-02-21T15:21:23.5133239Z    |         |       |
2020-02-21T15:21:23.5133510Z    |         |       value moved into `b` here
2020-02-21T15:21:23.5133827Z    |         value borrowed, by `a`, here
2020-02-21T15:21:23.5134279Z error[E0505]: cannot move out of `_` because it is borrowed
2020-02-21T15:21:23.5135038Z   --> /checkout/src/test/ui/pattern/bindings-after-at/default-binding-modes-both-sides-independent.rs:31:21
2020-02-21T15:21:23.5135429Z    |
2020-02-21T15:21:23.5135429Z    |
2020-02-21T15:21:23.5135757Z LL |     let ref mut a @ b = NotCopy; //~ ERROR cannot move out of value because it is borrowed
2020-02-21T15:21:23.5136552Z    |         |           |
2020-02-21T15:21:23.5136840Z    |         |           move out of value occurs here
2020-02-21T15:21:23.5137156Z    |         borrow of value occurs here
2020-02-21T15:21:23.5137156Z    |         borrow of value occurs here
2020-02-21T15:21:23.5137513Z LL |     //~^ ERROR cannot move out of `_` because it is borrowed
2020-02-21T15:21:23.5137841Z LL |     let _a: &NotCopy = a;
2020-02-21T15:21:23.5138356Z    |                        - borrow later used here
2020-02-21T15:21:23.5138806Z error: aborting due to 6 previous errors
2020-02-21T15:21:23.5139002Z 
2020-02-21T15:21:23.5139493Z For more information about this error, try `rustc --explain E0505`.
2020-02-21T15:21:23.5139751Z 
---
2020-02-21T15:21:23.5146325Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-21T15:21:23.5146793Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-21T15:21:23.5147057Z 
2020-02-21T15:21:23.5147166Z 
2020-02-21T15:21:23.5151360Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-21T15:21:23.5154377Z 
2020-02-21T15:21:23.5154508Z 
2020-02-21T15:21:23.5154775Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-21T15:21:23.5155142Z Build completed unsuccessfully in 1:12:48
2020-02-21T15:21:23.5155142Z Build completed unsuccessfully in 1:12:48
2020-02-21T15:21:23.5155442Z == clock drift check ==
2020-02-21T15:21:23.5155722Z   local time: Fri Feb 21 15:21:23 UTC 2020
2020-02-21T15:21:24.0075231Z   network time: Fri, 21 Feb 2020 15:21:24 GMT
2020-02-21T15:21:24.0075657Z == end clock drift check ==
2020-02-21T15:21:24.5477519Z 
2020-02-21T15:21:24.5552373Z ##[error]Bash exited with code '1'.
2020-02-21T15:21:24.5566304Z ##[section]Finishing: Run build
2020-02-21T15:21:24.5629094Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69348/merge to s
2020-02-21T15:21:24.5633866Z Task         : Get sources
2020-02-21T15:21:24.5634212Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-21T15:21:24.5634530Z Version      : 1.0.0
2020-02-21T15:21:24.5634767Z Author       : Microsoft
2020-02-21T15:21:24.5634767Z Author       : Microsoft
2020-02-21T15:21:24.5635118Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-21T15:21:24.5635529Z ==============================================================================
2020-02-21T15:21:24.9164271Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-21T15:21:24.9211213Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69348/merge to s
2020-02-21T15:21:24.9295413Z Cleaning up task key
2020-02-21T15:21:24.9296602Z Start cleaning up orphan processes.
2020-02-21T15:21:24.9465765Z Terminate orphan process: pid (4396) (python)
2020-02-21T15:21:24.9726122Z ##[section]Finishing: Finalize Job
