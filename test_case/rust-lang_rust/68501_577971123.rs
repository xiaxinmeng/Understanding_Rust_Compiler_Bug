plain
2020-01-24T02:04:12.4117928Z ========================== Starting Command Output ===========================
2020-01-24T02:04:12.4119153Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9dcf6baa-08e0-4d8c-a4b5-b3978263baf1.sh
2020-01-24T02:04:12.4119184Z 
2020-01-24T02:04:12.4121467Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-24T02:04:12.4126109Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68501/merge to s
2020-01-24T02:04:12.4127683Z Task         : Get sources
2020-01-24T02:04:12.4127708Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-24T02:04:12.4127731Z Version      : 1.0.0
2020-01-24T02:04:12.4127794Z Author       : Microsoft
---
2020-01-24T02:04:13.4064289Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-24T02:04:13.4075273Z ##[command]git config gc.auto 0
2020-01-24T02:04:13.4078476Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-24T02:04:13.4081561Z ##[command]git config --get-all http.proxy
2020-01-24T02:04:13.4088441Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68501/merge:refs/remotes/pull/68501/merge
---
2020-01-24T02:46:15.4031428Z .................................................................................................... 1700/9546
2020-01-24T02:46:19.9695199Z .................................................................................................... 1800/9546
2020-01-24T02:46:28.9731440Z .....................i.............................................................................. 1900/9546
2020-01-24T02:46:34.4817426Z .................................................................................................... 2000/9546
2020-01-24T02:46:45.7422863Z ...........iiiii.................................................................................... 2100/9546
2020-01-24T02:46:52.8699134Z .................................................................................................... 2300/9546
2020-01-24T02:46:54.6697736Z .................................................................................................... 2400/9546
2020-01-24T02:46:58.6496916Z .................................................................................................... 2500/9546
2020-01-24T02:47:14.0301373Z .................................................................................................... 2600/9546
---
2020-01-24T02:49:14.9047366Z .......................................................i...............i............................ 4900/9546
2020-01-24T02:49:20.9802932Z .................................................................................................... 5000/9546
2020-01-24T02:49:27.1567411Z ..................................................................................................i. 5100/9546
2020-01-24T02:49:31.2692583Z ...................................................................................F................ 5200/9546
2020-01-24T02:49:39.7870135Z ......................................................................ii.ii...........i............. 5300/9546
2020-01-24T02:49:47.0817277Z .......i............................................................................................ 5500/9546
2020-01-24T02:49:55.1110875Z .................................................................................................... 5600/9546
2020-01-24T02:50:00.5696633Z ........................................................i........................................... 5700/9546
2020-01-24T02:50:06.1722451Z .................................................................................................... 5800/9546
2020-01-24T02:50:06.1722451Z .................................................................................................... 5800/9546
2020-01-24T02:50:13.9375595Z .................................................................................................... 5900/9546
2020-01-24T02:50:19.3058569Z ...............................................ii...i...ii...........i.............................. 6000/9546
2020-01-24T02:50:36.8146030Z .................................................................................................... 6200/9546
2020-01-24T02:50:43.0518476Z .................................................................................................... 6300/9546
2020-01-24T02:50:43.0518476Z .................................................................................................... 6300/9546
2020-01-24T02:50:50.4355335Z ...........................................................................i..ii.................... 6400/9546
2020-01-24T02:51:11.8380561Z .................................................................................................... 6600/9546
2020-01-24T02:51:15.4474501Z ...................................................i................................................ 6700/9546
2020-01-24T02:51:17.1399454Z .................................................................................................... 6800/9546
2020-01-24T02:51:18.8047423Z ..................................................i................................................. 6900/9546
---
2020-01-24T02:52:36.8165848Z .................................................................................................... 7600/9546
2020-01-24T02:52:41.1582764Z .................................................................................................... 7700/9546
2020-01-24T02:52:46.1638494Z .................................................................................................... 7800/9546
2020-01-24T02:52:54.3438986Z .................................................................................................... 7900/9546
2020-01-24T02:52:58.7602720Z ......iiiiiii....................................................................................... 8000/9546
2020-01-24T02:53:09.8319055Z .................................................................................................... 8200/9546
2020-01-24T02:53:18.5514591Z .................................................................................................... 8300/9546
2020-01-24T02:53:28.3795054Z .................................................................................................... 8400/9546
2020-01-24T02:53:33.1421563Z .................................................................................................... 8500/9546
---
2020-01-24T02:55:02.2556356Z failures:
2020-01-24T02:55:02.2584798Z 
2020-01-24T02:55:02.2585456Z ---- [ui] ui/array-slice-vec/vec-matching-autoslice.rs stdout ----
2020-01-24T02:55:02.2585797Z normalized stderr:
2020-01-24T02:55:02.2586140Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.2586957Z    |
2020-01-24T02:55:02.2586957Z    |
2020-01-24T02:55:02.2587090Z LL | #![allow(illegal_floating_point_literal_pattern)] // FIXME #41620
2020-01-24T02:55:02.2587338Z    |
2020-01-24T02:55:02.2587445Z    = note: `#[warn(unused_attributes)]` on by default
2020-01-24T02:55:02.2587445Z    = note: `#[warn(unused_attributes)]` on by default
2020-01-24T02:55:02.2587714Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.2587870Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.2587986Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.2588413Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2589052Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2589309Z 
2020-01-24T02:55:02.2589550Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.2590069Z    |
2020-01-24T02:55:02.2590069Z    |
2020-01-24T02:55:02.2590356Z LL | #![allow(illegal_floating_point_literal_pattern)] // FIXME #41620
2020-01-24T02:55:02.2590613Z    |
2020-01-24T02:55:02.2590613Z    |
2020-01-24T02:55:02.2590726Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.2590838Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.2591097Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.2591260Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2591690Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2592002Z 
2020-01-24T02:55:02.2592145Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.2592798Z    |
2020-01-24T02:55:02.2592798Z    |
2020-01-24T02:55:02.2592958Z LL | #![allow(illegal_floating_point_literal_pattern)] // FIXME #41620
2020-01-24T02:55:02.2593179Z    |
2020-01-24T02:55:02.2593179Z    |
2020-01-24T02:55:02.2593305Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.2593476Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.2593687Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.2593870Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2594335Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2594599Z 
2020-01-24T02:55:02.2594743Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.2595566Z    |
2020-01-24T02:55:02.2595566Z    |
2020-01-24T02:55:02.2595706Z LL | #![allow(illegal_floating_point_literal_pattern)] // FIXME #41620
2020-01-24T02:55:02.2595942Z    |
2020-01-24T02:55:02.2595942Z    |
2020-01-24T02:55:02.2596049Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.2596342Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.2596489Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.2596749Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2597306Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2597910Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2598325Z   --> $DIR/vec-matching-autoslice.rs:20:18
2020-01-24T02:55:02.2598583Z    |
2020-01-24T02:55:02.2598583Z    |
2020-01-24T02:55:02.2598733Z LL |         ([_, _], 0.5) => panic!(),
2020-01-24T02:55:02.2598949Z    |
2020-01-24T02:55:02.2599069Z note: lint level defined here
2020-01-24T02:55:02.2599524Z   --> $DIR/vec-matching-autoslice.rs:2:10
2020-01-24T02:55:02.2599691Z    |
2020-01-24T02:55:02.2599691Z    |
2020-01-24T02:55:02.2599826Z LL | #![allow(illegal_floating_point_literal_pattern)] // FIXME #41620
2020-01-24T02:55:02.2600282Z    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-24T02:55:02.2600417Z    = note: #[warn(illegal_floating_point_literal_pattern)] is the minimum lint level
2020-01-24T02:55:02.2600686Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2600686Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2601185Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2601784Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2602198Z   --> $DIR/vec-matching-autoslice.rs:20:18
2020-01-24T02:55:02.2602462Z    |
2020-01-24T02:55:02.2602462Z    |
2020-01-24T02:55:02.2602586Z LL |         ([_, _], 0.5) => panic!(),
2020-01-24T02:55:02.2602820Z    |
2020-01-24T02:55:02.2603077Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2603077Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2603629Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2603884Z 
2020-01-24T02:55:02.2604185Z 
2020-01-24T02:55:02.2604303Z 
2020-01-24T02:55:02.2604416Z The actual stderr differed from the expected stderr.
2020-01-24T02:55:02.2604416Z The actual stderr differed from the expected stderr.
2020-01-24T02:55:02.2604898Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/vec-matching-autoslice/vec-matching-autoslice.stderr
2020-01-24T02:55:02.2605381Z To update references, rerun the tests and pass the `--bless` flag
2020-01-24T02:55:02.2605807Z To only update this specific test, also pass `--test-args array-slice-vec/vec-matching-autoslice.rs`
2020-01-24T02:55:02.2606200Z error: 1 errors occurred comparing output.
2020-01-24T02:55:02.2606312Z status: exit code: 0
2020-01-24T02:55:02.2606312Z status: exit code: 0
2020-01-24T02:55:02.2607341Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/array-slice-vec/vec-matching-autoslice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/vec-matching-autoslice/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/vec-matching-autoslice/auxiliary"
2020-01-24T02:55:02.2609638Z ------------------------------------------
2020-01-24T02:55:02.2617512Z 
2020-01-24T02:55:02.2618257Z ------------------------------------------
2020-01-24T02:55:02.2618569Z stderr:
2020-01-24T02:55:02.2618569Z stderr:
2020-01-24T02:55:02.2618892Z ------------------------------------------
2020-01-24T02:55:02.2619092Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.2624484Z    |
2020-01-24T02:55:02.2624484Z    |
2020-01-24T02:55:02.2624534Z LL | #![allow(illegal_floating_point_literal_pattern)] // FIXME #41620
2020-01-24T02:55:02.2624943Z    |
2020-01-24T02:55:02.2624977Z    = note: `#[warn(unused_attributes)]` on by default
2020-01-24T02:55:02.2624977Z    = note: `#[warn(unused_attributes)]` on by default
2020-01-24T02:55:02.2625016Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.2625055Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.2625114Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.2625162Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2625558Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2625593Z 
2020-01-24T02:55:02.2625899Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.2626223Z    |
2020-01-24T02:55:02.2626223Z    |
2020-01-24T02:55:02.2626271Z LL | #![allow(illegal_floating_point_literal_pattern)] // FIXME #41620
2020-01-24T02:55:02.2626360Z    |
2020-01-24T02:55:02.2626360Z    |
2020-01-24T02:55:02.2626396Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.2626453Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.2626492Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.2626540Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2626806Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2626842Z 
2020-01-24T02:55:02.2626878Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.2627341Z    |
2020-01-24T02:55:02.2627341Z    |
2020-01-24T02:55:02.2627376Z LL | #![allow(illegal_floating_point_literal_pattern)] // FIXME #41620
2020-01-24T02:55:02.2627462Z    |
2020-01-24T02:55:02.2627462Z    |
2020-01-24T02:55:02.2627498Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.2627536Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.2627594Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.2627642Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2627949Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2627978Z 
2020-01-24T02:55:02.2628014Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.2628268Z    |
2020-01-24T02:55:02.2628268Z    |
2020-01-24T02:55:02.2628304Z LL | #![allow(illegal_floating_point_literal_pattern)] // FIXME #41620
2020-01-24T02:55:02.2628390Z    |
2020-01-24T02:55:02.2628390Z    |
2020-01-24T02:55:02.2628425Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.2628463Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.2628520Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.2628567Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2628822Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2629108Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2633422Z   --> /checkout/src/test/ui/array-slice-vec/vec-matching-autoslice.rs:20:18
2020-01-24T02:55:02.2633507Z    |
2020-01-24T02:55:02.2633507Z    |
2020-01-24T02:55:02.2633540Z LL |         ([_, _], 0.5) => panic!(),
2020-01-24T02:55:02.2633622Z    |
2020-01-24T02:55:02.2633654Z note: lint level defined here
2020-01-24T02:55:02.2633952Z   --> /checkout/src/test/ui/array-slice-vec/vec-matching-autoslice.rs:2:10
2020-01-24T02:55:02.2634008Z    |
2020-01-24T02:55:02.2634008Z    |
2020-01-24T02:55:02.2634044Z LL | #![allow(illegal_floating_point_literal_pattern)] // FIXME #41620
2020-01-24T02:55:02.2634081Z    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-24T02:55:02.2634137Z    = note: #[warn(illegal_floating_point_literal_pattern)] is the minimum lint level
2020-01-24T02:55:02.2634372Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2634372Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2634689Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2634898Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2635099Z   --> /checkout/src/test/ui/array-slice-vec/vec-matching-autoslice.rs:20:18
2020-01-24T02:55:02.2635135Z    |
2020-01-24T02:55:02.2635135Z    |
2020-01-24T02:55:02.2635167Z LL |         ([_, _], 0.5) => panic!(),
2020-01-24T02:55:02.2635249Z    |
2020-01-24T02:55:02.2635289Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2635289Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2635528Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2635583Z 
2020-01-24T02:55:02.2635745Z ------------------------------------------
2020-01-24T02:55:02.2635768Z 
2020-01-24T02:55:02.2635812Z 
2020-01-24T02:55:02.2635812Z 
2020-01-24T02:55:02.2635974Z ---- [ui] ui/binding/match-range.rs stdout ----
2020-01-24T02:55:02.2636011Z normalized stderr:
2020-01-24T02:55:02.2636063Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.2636252Z    |
2020-01-24T02:55:02.2636252Z    |
2020-01-24T02:55:02.2636287Z LL | #![allow(illegal_floating_point_literal_pattern)] // FIXME #41620
2020-01-24T02:55:02.2636373Z    |
2020-01-24T02:55:02.2636406Z    = note: `#[warn(unused_attributes)]` on by default
2020-01-24T02:55:02.2636406Z    = note: `#[warn(unused_attributes)]` on by default
2020-01-24T02:55:02.2636463Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.2636502Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.2636549Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.2636618Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2636853Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2636896Z 
2020-01-24T02:55:02.2636932Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.2637121Z    |
2020-01-24T02:55:02.2637121Z    |
2020-01-24T02:55:02.2637172Z LL | #![allow(illegal_floating_point_literal_pattern)] // FIXME #41620
2020-01-24T02:55:02.2637241Z    |
2020-01-24T02:55:02.2637241Z    |
2020-01-24T02:55:02.2637294Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.2637341Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.2637379Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.2637537Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2637807Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2637833Z 
2020-01-24T02:55:02.2637886Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.2638077Z    |
2020-01-24T02:55:02.2638077Z    |
2020-01-24T02:55:02.2638128Z LL | #![allow(illegal_floating_point_literal_pattern)] // FIXME #41620
2020-01-24T02:55:02.2638196Z    |
2020-01-24T02:55:02.2638196Z    |
2020-01-24T02:55:02.2638249Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.2638367Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.2638405Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.2638477Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2638735Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2638761Z 
2020-01-24T02:55:02.2638813Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.2639002Z    |
2020-01-24T02:55:02.2639002Z    |
2020-01-24T02:55:02.2639053Z LL | #![allow(illegal_floating_point_literal_pattern)] // FIXME #41620
2020-01-24T02:55:02.2639120Z    |
2020-01-24T02:55:02.2639120Z    |
2020-01-24T02:55:02.2639175Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.2639222Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.2639260Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.2639329Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2639562Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2639778Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2639929Z   --> $DIR/match-range.rs:36:7
2020-01-24T02:55:02.2639961Z    |
2020-01-24T02:55:02.2639992Z LL |       1.0..=5.0 => {}
2020-01-24T02:55:02.2639992Z LL |       1.0..=5.0 => {}
2020-01-24T02:55:02.2640041Z    |       ^^^
2020-01-24T02:55:02.2640071Z    |
2020-01-24T02:55:02.2640102Z note: lint level defined here
2020-01-24T02:55:02.2645361Z   --> $DIR/match-range.rs:2:10
2020-01-24T02:55:02.2645415Z    |
2020-01-24T02:55:02.2645467Z LL | #![allow(illegal_floating_point_literal_pattern)] // FIXME #41620
2020-01-24T02:55:02.2645527Z    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-24T02:55:02.2645575Z    = note: #[warn(illegal_floating_point_literal_pattern)] is the minimum lint level
2020-01-24T02:55:02.2645676Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2645676Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2645962Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2646189Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2646343Z   --> $DIR/match-range.rs:36:13
2020-01-24T02:55:02.2646376Z    |
2020-01-24T02:55:02.2646408Z LL |       1.0..=5.0 => {}
2020-01-24T02:55:02.2646408Z LL |       1.0..=5.0 => {}
2020-01-24T02:55:02.2646459Z    |             ^^^
2020-01-24T02:55:02.2646490Z    |
2020-01-24T02:55:02.2646531Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2646788Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2647465Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2647641Z   --> $DIR/match-range.rs:40:8
2020-01-24T02:55:02.2647676Z    |
2020-01-24T02:55:02.2647819Z LL |       -3.6..=3.6 => {}
2020-01-24T02:55:02.2647819Z LL |       -3.6..=3.6 => {}
2020-01-24T02:55:02.2647869Z    |        ^^^
2020-01-24T02:55:02.2647899Z    |
2020-01-24T02:55:02.2647940Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2648231Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2648429Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2648595Z   --> $DIR/match-range.rs:40:14
2020-01-24T02:55:02.2648721Z    |
2020-01-24T02:55:02.2648895Z LL |       -3.6..=3.6 => {}
2020-01-24T02:55:02.2648895Z LL |       -3.6..=3.6 => {}
2020-01-24T02:55:02.2648931Z    |              ^^^
2020-01-24T02:55:02.2648977Z    |
2020-01-24T02:55:02.2649025Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2649274Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2649472Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2649623Z   --> $DIR/match-range.rs:44:9
2020-01-24T02:55:02.2649671Z    |
2020-01-24T02:55:02.2649705Z LL |         0.0..3.5 => panic!("should not match the range end"),
2020-01-24T02:55:02.2649705Z LL |         0.0..3.5 => panic!("should not match the range end"),
2020-01-24T02:55:02.2649740Z    |         ^^^
2020-01-24T02:55:02.2649786Z    |
2020-01-24T02:55:02.2649827Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2650049Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2650270Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2650421Z   --> $DIR/match-range.rs:44:14
2020-01-24T02:55:02.2650476Z    |
2020-01-24T02:55:02.2650512Z LL |         0.0..3.5 => panic!("should not match the range end"),
2020-01-24T02:55:02.2650512Z LL |         0.0..3.5 => panic!("should not match the range end"),
2020-01-24T02:55:02.2650547Z    |              ^^^
2020-01-24T02:55:02.2650576Z    |
2020-01-24T02:55:02.2650635Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2650858Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2651067Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2651216Z   --> $DIR/match-range.rs:48:9
2020-01-24T02:55:02.2651248Z    |
2020-01-24T02:55:02.2651294Z LL |         0.0..3.5 => {},
2020-01-24T02:55:02.2651294Z LL |         0.0..3.5 => {},
2020-01-24T02:55:02.2651326Z    |         ^^^
2020-01-24T02:55:02.2651363Z    |
2020-01-24T02:55:02.2651421Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2651653Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2651868Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2652020Z   --> $DIR/match-range.rs:48:14
2020-01-24T02:55:02.2652053Z    |
2020-01-24T02:55:02.2652102Z LL |         0.0..3.5 => {},
2020-01-24T02:55:02.2652102Z LL |         0.0..3.5 => {},
2020-01-24T02:55:02.2652135Z    |              ^^^
2020-01-24T02:55:02.2652165Z    |
2020-01-24T02:55:02.2652205Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2652444Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2652636Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2652812Z   --> $DIR/match-range.rs:36:7
2020-01-24T02:55:02.2652844Z    |
2020-01-24T02:55:02.2652875Z LL |       1.0..=5.0 => {}
2020-01-24T02:55:02.2652875Z LL |       1.0..=5.0 => {}
2020-01-24T02:55:02.2652922Z    |       ^^^
2020-01-24T02:55:02.2652951Z    |
2020-01-24T02:55:02.2653239Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2653783Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2654041Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2654212Z   --> $DIR/match-range.rs:36:13
2020-01-24T02:55:02.2654245Z    |
2020-01-24T02:55:02.2654276Z LL |       1.0..=5.0 => {}
2020-01-24T02:55:02.2654276Z LL |       1.0..=5.0 => {}
2020-01-24T02:55:02.2654307Z    |             ^^^
2020-01-24T02:55:02.2654355Z    |
2020-01-24T02:55:02.2654395Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2654639Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2654991Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2655141Z   --> $DIR/match-range.rs:40:8
2020-01-24T02:55:02.2655203Z    |
2020-01-24T02:55:02.2655352Z LL |       -3.6..=3.6 => {}
2020-01-24T02:55:02.2655352Z LL |       -3.6..=3.6 => {}
2020-01-24T02:55:02.2655385Z    |        ^^^
2020-01-24T02:55:02.2655431Z    |
2020-01-24T02:55:02.2655471Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2655694Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2655905Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2656055Z   --> $DIR/match-range.rs:40:14
2020-01-24T02:55:02.2656103Z    |
2020-01-24T02:55:02.2656247Z LL |       -3.6..=3.6 => {}
2020-01-24T02:55:02.2656247Z LL |       -3.6..=3.6 => {}
2020-01-24T02:55:02.2656282Z    |              ^^^
2020-01-24T02:55:02.2656311Z    |
2020-01-24T02:55:02.2656376Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2656603Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2656815Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2656964Z   --> $DIR/match-range.rs:44:9
2020-01-24T02:55:02.2656997Z    |
2020-01-24T02:55:02.2657047Z LL |         0.0..3.5 => panic!("should not match the range end"),
2020-01-24T02:55:02.2657047Z LL |         0.0..3.5 => panic!("should not match the range end"),
2020-01-24T02:55:02.2657081Z    |         ^^^
2020-01-24T02:55:02.2657111Z    |
2020-01-24T02:55:02.2657167Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2657388Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2657596Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2657756Z   --> $DIR/match-range.rs:44:14
2020-01-24T02:55:02.2657788Z    |
2020-01-24T02:55:02.2657837Z LL |         0.0..3.5 => panic!("should not match the range end"),
2020-01-24T02:55:02.2657837Z LL |         0.0..3.5 => panic!("should not match the range end"),
2020-01-24T02:55:02.2657871Z    |              ^^^
2020-01-24T02:55:02.2657907Z    |
2020-01-24T02:55:02.2657947Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2658183Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2658378Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2658545Z   --> $DIR/match-range.rs:48:9
2020-01-24T02:55:02.2658578Z    |
2020-01-24T02:55:02.2658609Z LL |         0.0..3.5 => {},
2020-01-24T02:55:02.2658609Z LL |         0.0..3.5 => {},
2020-01-24T02:55:02.2658660Z    |         ^^^
2020-01-24T02:55:02.2658689Z    |
2020-01-24T02:55:02.2658728Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2658975Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2659168Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2659406Z   --> $DIR/match-range.rs:48:14
2020-01-24T02:55:02.2659448Z    |
2020-01-24T02:55:02.2659478Z LL |         0.0..3.5 => {},
2020-01-24T02:55:02.2659478Z LL |         0.0..3.5 => {},
2020-01-24T02:55:02.2659525Z    |              ^^^
2020-01-24T02:55:02.2659555Z    |
2020-01-24T02:55:02.2659595Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2659863Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2659910Z 
2020-01-24T02:55:02.2659929Z 
2020-01-24T02:55:02.2659948Z 
2020-01-24T02:55:02.2660000Z The actual stderr differed from the expected stderr.
2020-01-24T02:55:02.2660000Z The actual stderr differed from the expected stderr.
2020-01-24T02:55:02.2660218Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/match-range/match-range.stderr
2020-01-24T02:55:02.2660493Z To update references, rerun the tests and pass the `--bless` flag
2020-01-24T02:55:02.2660707Z To only update this specific test, also pass `--test-args binding/match-range.rs`
2020-01-24T02:55:02.2660768Z error: 1 errors occurred comparing output.
2020-01-24T02:55:02.2660801Z status: exit code: 0
2020-01-24T02:55:02.2660801Z status: exit code: 0
2020-01-24T02:55:02.2661372Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binding/match-range.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/match-range/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/match-range/auxiliary"
2020-01-24T02:55:02.2661620Z ------------------------------------------
2020-01-24T02:55:02.2661645Z 
2020-01-24T02:55:02.2661801Z ------------------------------------------
2020-01-24T02:55:02.2661849Z stderr:
2020-01-24T02:55:02.2661849Z stderr:
2020-01-24T02:55:02.2662008Z ------------------------------------------
2020-01-24T02:55:02.2662049Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.2662273Z    |
2020-01-24T02:55:02.2662273Z    |
2020-01-24T02:55:02.2662309Z LL | #![allow(illegal_floating_point_literal_pattern)] // FIXME #41620
2020-01-24T02:55:02.2662395Z    |
2020-01-24T02:55:02.2662428Z    = note: `#[warn(unused_attributes)]` on by default
2020-01-24T02:55:02.2662428Z    = note: `#[warn(unused_attributes)]` on by default
2020-01-24T02:55:02.2662483Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.2662522Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.2662568Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.2662638Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2662877Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2662905Z 
2020-01-24T02:55:02.2662956Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.2663166Z    |
2020-01-24T02:55:02.2663166Z    |
2020-01-24T02:55:02.2663216Z LL | #![allow(illegal_floating_point_literal_pattern)] // FIXME #41620
2020-01-24T02:55:02.2663285Z    |
2020-01-24T02:55:02.2663285Z    |
2020-01-24T02:55:02.2663337Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.2663375Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.2663420Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.2663551Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2663813Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2663839Z 
2020-01-24T02:55:02.2663890Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.2664100Z    |
2020-01-24T02:55:02.2664100Z    |
2020-01-24T02:55:02.2664150Z LL | #![allow(illegal_floating_point_literal_pattern)] // FIXME #41620
2020-01-24T02:55:02.2664219Z    |
2020-01-24T02:55:02.2664219Z    |
2020-01-24T02:55:02.2664272Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.2664379Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.2664417Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.2664485Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2664736Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2664762Z 
2020-01-24T02:55:02.2664813Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.2665022Z    |
2020-01-24T02:55:02.2665022Z    |
2020-01-24T02:55:02.2665072Z LL | #![allow(illegal_floating_point_literal_pattern)] // FIXME #41620
2020-01-24T02:55:02.2665140Z    |
2020-01-24T02:55:02.2665140Z    |
2020-01-24T02:55:02.2665175Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.2665238Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.2665282Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.2665343Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2665573Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2665792Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2665961Z   --> /checkout/src/test/ui/binding/match-range.rs:36:7
2020-01-24T02:55:02.2665995Z    |
2020-01-24T02:55:02.2666044Z LL |       1.0..=5.0 => {}
2020-01-24T02:55:02.2666044Z LL |       1.0..=5.0 => {}
2020-01-24T02:55:02.2666075Z    |       ^^^
2020-01-24T02:55:02.2666105Z    |
2020-01-24T02:55:02.2666154Z note: lint level defined here
2020-01-24T02:55:02.2666324Z   --> /checkout/src/test/ui/binding/match-range.rs:2:10
2020-01-24T02:55:02.2666366Z    |
2020-01-24T02:55:02.2666399Z LL | #![allow(illegal_floating_point_literal_pattern)] // FIXME #41620
2020-01-24T02:55:02.2666461Z    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-24T02:55:02.2666499Z    = note: #[warn(illegal_floating_point_literal_pattern)] is the minimum lint level
2020-01-24T02:55:02.2666600Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2666600Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2666825Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2667037Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2667206Z   --> /checkout/src/test/ui/binding/match-range.rs:36:13
2020-01-24T02:55:02.2667240Z    |
2020-01-24T02:55:02.2667288Z LL |       1.0..=5.0 => {}
2020-01-24T02:55:02.2667288Z LL |       1.0..=5.0 => {}
2020-01-24T02:55:02.2667327Z    |             ^^^
2020-01-24T02:55:02.2667357Z    |
2020-01-24T02:55:02.2667414Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2667705Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2667949Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2668119Z   --> /checkout/src/test/ui/binding/match-range.rs:40:8
2020-01-24T02:55:02.2668153Z    |
2020-01-24T02:55:02.2668315Z LL |       -3.6..=3.6 => {}
2020-01-24T02:55:02.2668315Z LL |       -3.6..=3.6 => {}
2020-01-24T02:55:02.2668348Z    |        ^^^
2020-01-24T02:55:02.2668377Z    |
2020-01-24T02:55:02.2668417Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2668663Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2668856Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2669142Z   --> /checkout/src/test/ui/binding/match-range.rs:40:14
2020-01-24T02:55:02.2669176Z    |
2020-01-24T02:55:02.2669319Z LL |       -3.6..=3.6 => {}
2020-01-24T02:55:02.2669319Z LL |       -3.6..=3.6 => {}
2020-01-24T02:55:02.2669380Z    |              ^^^
2020-01-24T02:55:02.2669411Z    |
2020-01-24T02:55:02.2669451Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2669695Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2669887Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2670076Z   --> /checkout/src/test/ui/binding/match-range.rs:44:9
2020-01-24T02:55:02.2670109Z    |
2020-01-24T02:55:02.2670144Z LL |         0.0..3.5 => panic!("should not match the range end"),
2020-01-24T02:55:02.2670144Z LL |         0.0..3.5 => panic!("should not match the range end"),
2020-01-24T02:55:02.2670196Z    |         ^^^
2020-01-24T02:55:02.2670226Z    |
2020-01-24T02:55:02.2670267Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2670516Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2670720Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2671422Z   --> /checkout/src/test/ui/binding/match-range.rs:44:14
2020-01-24T02:55:02.2671471Z    |
2020-01-24T02:55:02.2671505Z LL |         0.0..3.5 => panic!("should not match the range end"),
2020-01-24T02:55:02.2671505Z LL |         0.0..3.5 => panic!("should not match the range end"),
2020-01-24T02:55:02.2671540Z    |              ^^^
2020-01-24T02:55:02.2671589Z    |
2020-01-24T02:55:02.2671629Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
---
2020-01-24T02:55:02.2687182Z         line_num: 47,
2020-01-24T02:55:02.2687214Z         kind: Some(
2020-01-24T02:55:02.2687245Z             Warning,
2020-01-24T02:55:02.2687293Z         ),
2020-01-24T02:55:02.2687543Z         msg: "47:16: 47:39: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.2687640Z     Error {
2020-01-24T02:55:02.2687769Z         line_num: 47,
2020-01-24T02:55:02.2687805Z         kind: Some(
2020-01-24T02:55:02.2687835Z             Warning,
---
2020-01-24T02:55:02.2688048Z         line_num: 44,
2020-01-24T02:55:02.2688079Z         kind: Some(
2020-01-24T02:55:02.2688247Z             Warning,
2020-01-24T02:55:02.2688280Z         ),
2020-01-24T02:55:02.2688547Z         msg: "44:16: 44:39: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.2688635Z     Error {
2020-01-24T02:55:02.2688666Z         line_num: 44,
2020-01-24T02:55:02.2688697Z         kind: Some(
2020-01-24T02:55:02.2688754Z             Warning,
---
2020-01-24T02:55:02.2688947Z         line_num: 47,
2020-01-24T02:55:02.2688995Z         kind: Some(
2020-01-24T02:55:02.2689025Z             Warning,
2020-01-24T02:55:02.2689055Z         ),
2020-01-24T02:55:02.2689281Z         msg: "47:16: 47:39: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.2689367Z     Error {
2020-01-24T02:55:02.2689397Z         line_num: 47,
2020-01-24T02:55:02.2689446Z         kind: Some(
2020-01-24T02:55:02.2689477Z             Warning,
---
2020-01-24T02:55:02.2690084Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-24T02:55:02.2690108Z 
2020-01-24T02:55:02.2690349Z ---- [ui] ui/half-open-range-patterns/half-open-range-pats-exhaustive-fail.rs stdout ----
2020-01-24T02:55:02.2690376Z 
2020-01-24T02:55:02.2690668Z error: /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-exhaustive-fail.rs:16: unexpected warning: '16:14: 16:37: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]'
2020-01-24T02:55:02.2691039Z error: /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-exhaustive-fail.rs:16: unexpected warning: '16:14: 16:37: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2020-01-24T02:55:02.2691173Z 
2020-01-24T02:55:02.2691173Z 
2020-01-24T02:55:02.2691521Z error: /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-exhaustive-fail.rs:17: unexpected warning: '17:16: 17:35: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]'
2020-01-24T02:55:02.2691870Z error: /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-exhaustive-fail.rs:17: unexpected warning: '17:16: 17:35: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2020-01-24T02:55:02.2691924Z 
2020-01-24T02:55:02.2691924Z 
2020-01-24T02:55:02.2692215Z error: /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-exhaustive-fail.rs:16: unexpected warning: '16:14: 16:37: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]'
2020-01-24T02:55:02.2692576Z error: /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-exhaustive-fail.rs:16: unexpected warning: '16:14: 16:37: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2020-01-24T02:55:02.2692630Z 
2020-01-24T02:55:02.2692630Z 
2020-01-24T02:55:02.2692922Z error: /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-exhaustive-fail.rs:17: unexpected warning: '17:16: 17:35: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]'
2020-01-24T02:55:02.2693286Z error: /checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-exhaustive-fail.rs:17: unexpected warning: '17:16: 17:35: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2020-01-24T02:55:02.2693322Z 
2020-01-24T02:55:02.2693357Z error: 8 unexpected errors found, 0 expected errors not found
2020-01-24T02:55:02.2693416Z status: exit code: 1
2020-01-24T02:55:02.2693416Z status: exit code: 1
2020-01-24T02:55:02.2694091Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/half-open-range-patterns/half-open-range-pats-exhaustive-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/half-open-range-patterns/half-open-range-pats-exhaustive-fail" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/half-open-range-patterns/half-open-range-pats-exhaustive-fail/auxiliary" "-A" "unused"
2020-01-24T02:55:02.2694225Z     Error {
2020-01-24T02:55:02.2694276Z         line_num: 16,
2020-01-24T02:55:02.2694313Z         kind: Some(
2020-01-24T02:55:02.2694343Z             Warning,
2020-01-24T02:55:02.2694343Z             Warning,
2020-01-24T02:55:02.2694373Z         ),
2020-01-24T02:55:02.2694678Z         msg: "16:14: 16:37: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.2694758Z     Error {
2020-01-24T02:55:02.2724020Z         line_num: 16,
2020-01-24T02:55:02.2724087Z         kind: Some(
2020-01-24T02:55:02.2724121Z             Warning,
---
2020-01-24T02:55:02.2724324Z         line_num: 17,
2020-01-24T02:55:02.2724356Z         kind: Some(
2020-01-24T02:55:02.2724388Z             Warning,
2020-01-24T02:55:02.2724423Z         ),
2020-01-24T02:55:02.2724969Z         msg: "17:16: 17:35: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.2725051Z     Error {
2020-01-24T02:55:02.2725094Z         line_num: 17,
2020-01-24T02:55:02.2725126Z         kind: Some(
2020-01-24T02:55:02.2725162Z             Warning,
---
2020-01-24T02:55:02.2730627Z         line_num: 16,
2020-01-24T02:55:02.2730659Z         kind: Some(
2020-01-24T02:55:02.2730701Z             Warning,
2020-01-24T02:55:02.2730732Z         ),
2020-01-24T02:55:02.2731302Z         msg: "16:14: 16:37: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.2731411Z     Error {
2020-01-24T02:55:02.2731444Z         line_num: 16,
2020-01-24T02:55:02.2731488Z         kind: Some(
2020-01-24T02:55:02.2731522Z             Warning,
---
2020-01-24T02:55:02.2731720Z         line_num: 17,
2020-01-24T02:55:02.2731758Z         kind: Some(
2020-01-24T02:55:02.2731791Z             Warning,
2020-01-24T02:55:02.2731823Z         ),
2020-01-24T02:55:02.2732082Z         msg: "17:16: 17:35: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.2732156Z     Error {
2020-01-24T02:55:02.2732189Z         line_num: 17,
2020-01-24T02:55:02.2732228Z         kind: Some(
2020-01-24T02:55:02.2732261Z             Warning,
---
2020-01-24T02:55:02.2732739Z thread '[ui] ui/half-open-range-patterns/half-open-range-pats-exhaustive-fail.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1436:13
2020-01-24T02:55:02.2732773Z 
2020-01-24T02:55:02.2732976Z ---- [ui] ui/half-open-range-patterns/half-open-range-pats-semantics.rs stdout ----
2020-01-24T02:55:02.2733031Z normalized stderr:
2020-01-24T02:55:02.2733070Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.2733255Z   --> $DIR/half-open-range-pats-semantics.rs:8:10
2020-01-24T02:55:02.2733338Z LL | #![allow(illegal_floating_point_literal_pattern)]
2020-01-24T02:55:02.2733384Z    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-24T02:55:02.2733468Z    |
2020-01-24T02:55:02.2733505Z    = note: `#[warn(unused_attributes)]` on by default
2020-01-24T02:55:02.2733505Z    = note: `#[warn(unused_attributes)]` on by default
2020-01-24T02:55:02.2734036Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.2734234Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.2734278Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.2734333Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2734688Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2734721Z 
2020-01-24T02:55:02.2734762Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.2734972Z   --> $DIR/half-open-range-pats-semantics.rs:8:10
2020-01-24T02:55:02.2735145Z LL | #![allow(illegal_floating_point_literal_pattern)]
2020-01-24T02:55:02.2735186Z    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-24T02:55:02.2735228Z    |
2020-01-24T02:55:02.2735228Z    |
2020-01-24T02:55:02.2735276Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.2735479Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.2735527Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.2735580Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2735873Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2735903Z 
2020-01-24T02:55:02.2735942Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.2736134Z   --> $DIR/half-open-range-pats-semantics.rs:8:10
2020-01-24T02:55:02.2736223Z LL | #![allow(illegal_floating_point_literal_pattern)]
2020-01-24T02:55:02.2736262Z    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-24T02:55:02.2736307Z    |
2020-01-24T02:55:02.2736307Z    |
2020-01-24T02:55:02.2736347Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.2736388Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.2736436Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.2736488Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2736763Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2736969Z 
2020-01-24T02:55:02.2737006Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.2737349Z   --> $DIR/half-open-range-pats-semantics.rs:8:10
2020-01-24T02:55:02.2737439Z LL | #![allow(illegal_floating_point_literal_pattern)]
2020-01-24T02:55:02.2737474Z    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-24T02:55:02.2737528Z    |
2020-01-24T02:55:02.2737528Z    |
2020-01-24T02:55:02.2737565Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.2737602Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.2737656Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.2737704Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2737955Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2738157Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2738325Z   --> $DIR/half-open-range-pats-semantics.rs:46:46
2020-01-24T02:55:02.2738380Z    |
2020-01-24T02:55:02.2738380Z    |
2020-01-24T02:55:02.2738416Z LL |     assert!(yes!(core::f32::NEG_INFINITY, ..=core::f32::NEG_INFINITY));
2020-01-24T02:55:02.2738775Z    |
2020-01-24T02:55:02.2738808Z note: lint level defined here
2020-01-24T02:55:02.2739016Z   --> $DIR/half-open-range-pats-semantics.rs:8:10
2020-01-24T02:55:02.2739052Z    |
2020-01-24T02:55:02.2739052Z    |
2020-01-24T02:55:02.2739103Z LL | #![allow(illegal_floating_point_literal_pattern)]
2020-01-24T02:55:02.2739140Z    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-24T02:55:02.2739181Z    = note: #[warn(illegal_floating_point_literal_pattern)] is the minimum lint level
2020-01-24T02:55:02.2739283Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2739283Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2739643Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2739858Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2740047Z   --> $DIR/half-open-range-pats-semantics.rs:47:46
2020-01-24T02:55:02.2740100Z    |
2020-01-24T02:55:02.2740100Z    |
2020-01-24T02:55:02.2740136Z LL |     assert!(yes!(core::f32::NEG_INFINITY, ..=1.0f32));
2020-01-24T02:55:02.2740224Z    |
2020-01-24T02:55:02.2740267Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2740267Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2740523Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2740733Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2740910Z   --> $DIR/half-open-range-pats-semantics.rs:48:29
2020-01-24T02:55:02.2740959Z    |
2020-01-24T02:55:02.2740959Z    |
2020-01-24T02:55:02.2741002Z LL |     assert!(yes!(1.5f32, ..=1.5f32));
2020-01-24T02:55:02.2742365Z    |
2020-01-24T02:55:02.2742651Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2742651Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2743033Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2744273Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2744747Z   --> $DIR/half-open-range-pats-semantics.rs:49:31
2020-01-24T02:55:02.2744813Z    |
2020-01-24T02:55:02.2744813Z    |
2020-01-24T02:55:02.2744985Z LL |     assert!(!yes!(1.6f32, ..=-1.5f32));
2020-01-24T02:55:02.2745053Z    |
2020-01-24T02:55:02.2745108Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2745108Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2745376Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2745599Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2745775Z   --> $DIR/half-open-range-pats-semantics.rs:52:46
2020-01-24T02:55:02.2745810Z    |
2020-01-24T02:55:02.2745810Z    |
2020-01-24T02:55:02.2745860Z LL |     assert!(yes!(core::f64::NEG_INFINITY, ..=core::f64::NEG_INFINITY));
2020-01-24T02:55:02.2745933Z    |
2020-01-24T02:55:02.2745988Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2745988Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2746218Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2746433Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2746601Z   --> $DIR/half-open-range-pats-semantics.rs:53:46
2020-01-24T02:55:02.2746642Z    |
2020-01-24T02:55:02.2746642Z    |
2020-01-24T02:55:02.2751837Z LL |     assert!(yes!(core::f64::NEG_INFINITY, ..=1.0f64));
2020-01-24T02:55:02.2751936Z    |
2020-01-24T02:55:02.2752139Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2752139Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2752580Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2752836Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2753203Z   --> $DIR/half-open-range-pats-semantics.rs:54:29
2020-01-24T02:55:02.2753243Z    |
2020-01-24T02:55:02.2753243Z    |
2020-01-24T02:55:02.2753280Z LL |     assert!(yes!(1.5f64, ..=1.5f64));
2020-01-24T02:55:02.2753370Z    |
2020-01-24T02:55:02.2753415Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2753415Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2759646Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2760008Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2760397Z   --> $DIR/half-open-range-pats-semantics.rs:55:31
2020-01-24T02:55:02.2760436Z    |
2020-01-24T02:55:02.2760436Z    |
2020-01-24T02:55:02.2760789Z LL |     assert!(!yes!(1.6f64, ..=-1.5f64));
2020-01-24T02:55:02.2761075Z    |
2020-01-24T02:55:02.2761121Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2761121Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2761401Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2761628Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2761836Z   --> $DIR/half-open-range-pats-semantics.rs:90:45
2020-01-24T02:55:02.2761884Z    |
2020-01-24T02:55:02.2761884Z    |
2020-01-24T02:55:02.2761922Z LL |     assert!(yes!(core::f32::NEG_INFINITY, ..1.0f32));
2020-01-24T02:55:02.2762015Z    |
2020-01-24T02:55:02.2762067Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2762067Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2762347Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2762570Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2762759Z   --> $DIR/half-open-range-pats-semantics.rs:91:29
2020-01-24T02:55:02.2762812Z    |
2020-01-24T02:55:02.2762812Z    |
2020-01-24T02:55:02.2762849Z LL |     assert!(!yes!(1.5f32, ..1.5f32));
2020-01-24T02:55:02.2762939Z    |
2020-01-24T02:55:02.2762984Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2762984Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2763266Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2763487Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2763852Z   --> $DIR/half-open-range-pats-semantics.rs:93:28
2020-01-24T02:55:02.2763905Z    |
2020-01-24T02:55:02.2763905Z    |
2020-01-24T02:55:02.2764112Z LL |     assert!(yes!(1.5f32, ..E32));
2020-01-24T02:55:02.2764360Z    |
2020-01-24T02:55:02.2764400Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2764400Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2764630Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2764847Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2765014Z   --> $DIR/half-open-range-pats-semantics.rs:94:29
2020-01-24T02:55:02.2765067Z    |
2020-01-24T02:55:02.2765067Z    |
2020-01-24T02:55:02.2765106Z LL |     assert!(!yes!(1.6f32, ..1.5f32));
2020-01-24T02:55:02.2765172Z    |
2020-01-24T02:55:02.2765357Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2765357Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2765639Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2765859Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2766027Z   --> $DIR/half-open-range-pats-semantics.rs:97:45
2020-01-24T02:55:02.2766060Z    |
2020-01-24T02:55:02.2766060Z    |
2020-01-24T02:55:02.2766110Z LL |     assert!(yes!(core::f64::NEG_INFINITY, ..1.0f64));
2020-01-24T02:55:02.2766179Z    |
2020-01-24T02:55:02.2766234Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2766234Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2767117Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2767614Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2767814Z   --> $DIR/half-open-range-pats-semantics.rs:98:29
2020-01-24T02:55:02.2767853Z    |
2020-01-24T02:55:02.2767853Z    |
2020-01-24T02:55:02.2767903Z LL |     assert!(!yes!(1.5f64, ..1.5f64));
2020-01-24T02:55:02.2767976Z    |
2020-01-24T02:55:02.2768020Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2768020Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2768294Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2768533Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2768725Z   --> $DIR/half-open-range-pats-semantics.rs:100:28
2020-01-24T02:55:02.2768763Z    |
2020-01-24T02:55:02.2768763Z    |
2020-01-24T02:55:02.2768798Z LL |     assert!(yes!(1.5f64, ..E64));
2020-01-24T02:55:02.2769069Z    |
2020-01-24T02:55:02.2769121Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2769121Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2769661Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2770049Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2770257Z   --> $DIR/half-open-range-pats-semantics.rs:101:29
2020-01-24T02:55:02.2770296Z    |
2020-01-24T02:55:02.2770296Z    |
2020-01-24T02:55:02.2770331Z LL |     assert!(!yes!(1.6f64, ..1.5f64));
2020-01-24T02:55:02.2770418Z    |
2020-01-24T02:55:02.2770463Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2770463Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2770728Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2770962Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2771167Z   --> $DIR/half-open-range-pats-semantics.rs:136:43
2020-01-24T02:55:02.2771212Z    |
2020-01-24T02:55:02.2771212Z    |
2020-01-24T02:55:02.2771252Z LL |     assert!(yes!(core::f32::NEG_INFINITY, core::f32::NEG_INFINITY..));
2020-01-24T02:55:02.2771349Z    |
2020-01-24T02:55:02.2771394Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2771394Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2771663Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2771883Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2772072Z   --> $DIR/half-open-range-pats-semantics.rs:137:39
2020-01-24T02:55:02.2772284Z    |
2020-01-24T02:55:02.2772284Z    |
2020-01-24T02:55:02.2772322Z LL |     assert!(yes!(core::f32::INFINITY, core::f32::NEG_INFINITY..));
2020-01-24T02:55:02.2772423Z    |
2020-01-24T02:55:02.2772563Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2772563Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2773203Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2773412Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2773586Z   --> $DIR/half-open-range-pats-semantics.rs:138:44
2020-01-24T02:55:02.2773636Z    |
2020-01-24T02:55:02.2773636Z    |
2020-01-24T02:55:02.2773670Z LL |     assert!(!yes!(core::f32::NEG_INFINITY, 1.0f32..));
2020-01-24T02:55:02.2773760Z    |
2020-01-24T02:55:02.2773801Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2773801Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2774148Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2774382Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2774748Z   --> $DIR/half-open-range-pats-semantics.rs:139:39
2020-01-24T02:55:02.2774804Z    |
2020-01-24T02:55:02.2774804Z    |
2020-01-24T02:55:02.2774837Z LL |     assert!(yes!(core::f32::INFINITY, 1.0f32..));
2020-01-24T02:55:02.2774924Z    |
2020-01-24T02:55:02.2774965Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2774965Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2775206Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2775431Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2775606Z   --> $DIR/half-open-range-pats-semantics.rs:140:48
2020-01-24T02:55:02.2775650Z    |
2020-01-24T02:55:02.2775650Z    |
2020-01-24T02:55:02.2776030Z LL |     assert!(!yes!(1.0f32 - core::f32::EPSILON, 1.0f32..));
2020-01-24T02:55:02.2776108Z    |
2020-01-24T02:55:02.2776173Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2776173Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2776420Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2776650Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2777018Z   --> $DIR/half-open-range-pats-semantics.rs:141:26
2020-01-24T02:55:02.2777057Z    |
2020-01-24T02:55:02.2777057Z    |
2020-01-24T02:55:02.2777108Z LL |     assert!(yes!(1.0f32, 1.0f32..));
2020-01-24T02:55:02.2777179Z    |
2020-01-24T02:55:02.2777239Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2777239Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2777530Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2777771Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2777959Z   --> $DIR/half-open-range-pats-semantics.rs:142:39
2020-01-24T02:55:02.2777997Z    |
2020-01-24T02:55:02.2777997Z    |
2020-01-24T02:55:02.2778048Z LL |     assert!(yes!(core::f32::INFINITY, 1.0f32..));
2020-01-24T02:55:02.2778121Z    |
2020-01-24T02:55:02.2778165Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2778165Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2778422Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2778638Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2779011Z   --> $DIR/half-open-range-pats-semantics.rs:143:39
2020-01-24T02:55:02.2779219Z    |
2020-01-24T02:55:02.2779219Z    |
2020-01-24T02:55:02.2779254Z LL |     assert!(yes!(core::f32::INFINITY, core::f32::INFINITY..));
2020-01-24T02:55:02.2779602Z    |
2020-01-24T02:55:02.2779830Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2779830Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2780127Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2780341Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2780538Z   --> $DIR/half-open-range-pats-semantics.rs:146:43
2020-01-24T02:55:02.2780575Z    |
2020-01-24T02:55:02.2780575Z    |
2020-01-24T02:55:02.2780613Z LL |     assert!(yes!(core::f64::NEG_INFINITY, core::f64::NEG_INFINITY..));
2020-01-24T02:55:02.2780707Z    |
2020-01-24T02:55:02.2780750Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2780750Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2781124Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2781348Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2781545Z   --> $DIR/half-open-range-pats-semantics.rs:147:39
2020-01-24T02:55:02.2781581Z    |
2020-01-24T02:55:02.2781581Z    |
2020-01-24T02:55:02.2781619Z LL |     assert!(yes!(core::f64::INFINITY, core::f64::NEG_INFINITY..));
2020-01-24T02:55:02.2781713Z    |
2020-01-24T02:55:02.2781756Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2781756Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2782011Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2782398Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2782769Z   --> $DIR/half-open-range-pats-semantics.rs:148:44
2020-01-24T02:55:02.2782807Z    |
2020-01-24T02:55:02.2782807Z    |
2020-01-24T02:55:02.2782843Z LL |     assert!(!yes!(core::f64::NEG_INFINITY, 1.0f64..));
2020-01-24T02:55:02.2783143Z    |
2020-01-24T02:55:02.2783185Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2783185Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2783436Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2783644Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2783820Z   --> $DIR/half-open-range-pats-semantics.rs:149:39
2020-01-24T02:55:02.2783871Z    |
2020-01-24T02:55:02.2783871Z    |
2020-01-24T02:55:02.2783906Z LL |     assert!(yes!(core::f64::INFINITY, 1.0f64..));
2020-01-24T02:55:02.2784000Z    |
2020-01-24T02:55:02.2784042Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2784042Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2784289Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2784514Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2784690Z   --> $DIR/half-open-range-pats-semantics.rs:150:48
2020-01-24T02:55:02.2784744Z    |
2020-01-24T02:55:02.2784744Z    |
2020-01-24T02:55:02.2784927Z LL |     assert!(!yes!(1.0f64 - core::f64::EPSILON, 1.0f64..));
2020-01-24T02:55:02.2785017Z    |
2020-01-24T02:55:02.2785059Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2785059Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2785297Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2785531Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2785865Z   --> $DIR/half-open-range-pats-semantics.rs:151:26
2020-01-24T02:55:02.2785913Z    |
2020-01-24T02:55:02.2785913Z    |
2020-01-24T02:55:02.2786014Z LL |     assert!(yes!(1.0f64, 1.0f64..));
2020-01-24T02:55:02.2786267Z    |
2020-01-24T02:55:02.2786325Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2786325Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2786591Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2786817Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2786993Z   --> $DIR/half-open-range-pats-semantics.rs:152:39
2020-01-24T02:55:02.2787028Z    |
2020-01-24T02:55:02.2787028Z    |
2020-01-24T02:55:02.2787080Z LL |     assert!(yes!(core::f64::INFINITY, 1.0f64..));
2020-01-24T02:55:02.2787225Z    |
2020-01-24T02:55:02.2787281Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2787281Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2787554Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2787781Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2787958Z   --> $DIR/half-open-range-pats-semantics.rs:153:39
2020-01-24T02:55:02.2787993Z    |
2020-01-24T02:55:02.2787993Z    |
2020-01-24T02:55:02.2788044Z LL |     assert!(yes!(core::f64::INFINITY, core::f64::INFINITY..));
2020-01-24T02:55:02.2788118Z    |
2020-01-24T02:55:02.2788174Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2788174Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2788410Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2788643Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2788995Z   --> $DIR/half-open-range-pats-semantics.rs:46:46
2020-01-24T02:55:02.2789031Z    |
2020-01-24T02:55:02.2789031Z    |
2020-01-24T02:55:02.2789074Z LL |     assert!(yes!(core::f32::NEG_INFINITY, ..=core::f32::NEG_INFINITY));
2020-01-24T02:55:02.2789343Z    |
2020-01-24T02:55:02.2789384Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2789384Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2789640Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2789846Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2790036Z   --> $DIR/half-open-range-pats-semantics.rs:47:46
2020-01-24T02:55:02.2790072Z    |
2020-01-24T02:55:02.2790072Z    |
2020-01-24T02:55:02.2790108Z LL |     assert!(yes!(core::f32::NEG_INFINITY, ..=1.0f32));
2020-01-24T02:55:02.2790203Z    |
2020-01-24T02:55:02.2790250Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2790250Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2790504Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2790710Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2790904Z   --> $DIR/half-open-range-pats-semantics.rs:48:29
2020-01-24T02:55:02.2790940Z    |
2020-01-24T02:55:02.2790940Z    |
2020-01-24T02:55:02.2790973Z LL |     assert!(yes!(1.5f32, ..=1.5f32));
2020-01-24T02:55:02.2791057Z    |
2020-01-24T02:55:02.2791099Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2791099Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2791348Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2791566Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2791755Z   --> $DIR/half-open-range-pats-semantics.rs:49:31
2020-01-24T02:55:02.2791860Z    |
2020-01-24T02:55:02.2791860Z    |
2020-01-24T02:55:02.2792413Z LL |     assert!(!yes!(1.6f32, ..=-1.5f32));
2020-01-24T02:55:02.2792501Z    |
2020-01-24T02:55:02.2792724Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2792724Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2792987Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2793201Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2793379Z   --> $DIR/half-open-range-pats-semantics.rs:52:46
2020-01-24T02:55:02.2793431Z    |
2020-01-24T02:55:02.2793431Z    |
2020-01-24T02:55:02.2793469Z LL |     assert!(yes!(core::f64::NEG_INFINITY, ..=core::f64::NEG_INFINITY));
2020-01-24T02:55:02.2793639Z    |
2020-01-24T02:55:02.2793689Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2793689Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2793963Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2794193Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2794372Z   --> $DIR/half-open-range-pats-semantics.rs:53:46
2020-01-24T02:55:02.2794423Z    |
2020-01-24T02:55:02.2794423Z    |
2020-01-24T02:55:02.2794460Z LL |     assert!(yes!(core::f64::NEG_INFINITY, ..=1.0f64));
2020-01-24T02:55:02.2794549Z    |
2020-01-24T02:55:02.2794593Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2794593Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2794844Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2795251Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2795599Z   --> $DIR/half-open-range-pats-semantics.rs:54:29
2020-01-24T02:55:02.2795653Z    |
2020-01-24T02:55:02.2795653Z    |
2020-01-24T02:55:02.2795688Z LL |     assert!(yes!(1.5f64, ..=1.5f64));
2020-01-24T02:55:02.2795931Z    |
2020-01-24T02:55:02.2795990Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2795990Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2796230Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2796452Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2796626Z   --> $DIR/half-open-range-pats-semantics.rs:55:31
2020-01-24T02:55:02.2796661Z    |
2020-01-24T02:55:02.2796661Z    |
2020-01-24T02:55:02.2796845Z LL |     assert!(!yes!(1.6f64, ..=-1.5f64));
2020-01-24T02:55:02.2796924Z    |
2020-01-24T02:55:02.2796987Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2796987Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2797228Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2797452Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2797628Z   --> $DIR/half-open-range-pats-semantics.rs:90:45
2020-01-24T02:55:02.2797663Z    |
2020-01-24T02:55:02.2797663Z    |
2020-01-24T02:55:02.2797715Z LL |     assert!(yes!(core::f32::NEG_INFINITY, ..1.0f32));
2020-01-24T02:55:02.2797962Z    |
2020-01-24T02:55:02.2798004Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2798004Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2798432Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2798670Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2798915Z   --> $DIR/half-open-range-pats-semantics.rs:91:29
2020-01-24T02:55:02.2798959Z    |
2020-01-24T02:55:02.2798959Z    |
2020-01-24T02:55:02.2798993Z LL |     assert!(!yes!(1.5f32, ..1.5f32));
2020-01-24T02:55:02.2799081Z    |
2020-01-24T02:55:02.2799122Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2799122Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2799404Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2799609Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2799801Z   --> $DIR/half-open-range-pats-semantics.rs:93:28
2020-01-24T02:55:02.2799837Z    |
2020-01-24T02:55:02.2799837Z    |
2020-01-24T02:55:02.2799870Z LL |     assert!(yes!(1.5f32, ..E32));
2020-01-24T02:55:02.2800031Z    |
2020-01-24T02:55:02.2800081Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2800081Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2800364Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2800571Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2800763Z   --> $DIR/half-open-range-pats-semantics.rs:94:29
2020-01-24T02:55:02.2800798Z    |
2020-01-24T02:55:02.2800798Z    |
2020-01-24T02:55:02.2800831Z LL |     assert!(!yes!(1.6f32, ..1.5f32));
2020-01-24T02:55:02.2800915Z    |
2020-01-24T02:55:02.2800957Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2800957Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2801357Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2801563Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2801728Z   --> $DIR/half-open-range-pats-semantics.rs:97:45
2020-01-24T02:55:02.2801776Z    |
2020-01-24T02:55:02.2801776Z    |
2020-01-24T02:55:02.2801815Z LL |     assert!(yes!(core::f64::NEG_INFINITY, ..1.0f64));
2020-01-24T02:55:02.2801899Z    |
2020-01-24T02:55:02.2801939Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2801939Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2802163Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2802374Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2802539Z   --> $DIR/half-open-range-pats-semantics.rs:98:29
2020-01-24T02:55:02.2802588Z    |
2020-01-24T02:55:02.2802588Z    |
2020-01-24T02:55:02.2802619Z LL |     assert!(!yes!(1.5f64, ..1.5f64));
2020-01-24T02:55:02.2802705Z    |
2020-01-24T02:55:02.2802751Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2802751Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2802980Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2803193Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2803357Z   --> $DIR/half-open-range-pats-semantics.rs:100:28
2020-01-24T02:55:02.2803391Z    |
2020-01-24T02:55:02.2803391Z    |
2020-01-24T02:55:02.2803438Z LL |     assert!(yes!(1.5f64, ..E64));
2020-01-24T02:55:02.2803678Z    |
2020-01-24T02:55:02.2803736Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2803736Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2803972Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2804207Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2804382Z   --> $DIR/half-open-range-pats-semantics.rs:101:29
2020-01-24T02:55:02.2804418Z    |
2020-01-24T02:55:02.2804418Z    |
2020-01-24T02:55:02.2804535Z LL |     assert!(!yes!(1.6f64, ..1.5f64));
2020-01-24T02:55:02.2804610Z    |
2020-01-24T02:55:02.2804667Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2804667Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2804927Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.2805150Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.2805327Z   --> $DIR/half-open-range-pats-semantics.rs:136:43
2020-01-24T02:55:02.2805363Z    |
2020-01-24T02:55:02.2805363Z    |
2020-01-24T02:55:02.2805415Z LL |     assert!(yes!(core::f32::NEG_INFINITY, core::f32::NEG_INFINITY..));
2020-01-24T02:55:02.2805564Z    |
2020-01-24T02:55:02.2805612Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2805612Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.2805893Z    = note: for more information, see issue #41620 <***/issues/41620>
---
2020-01-24T02:55:02.3064418Z         line_num: 6,
2020-01-24T02:55:02.3064524Z         kind: Some(
2020-01-24T02:55:02.3064645Z             Warning,
2020-01-24T02:55:02.3064751Z         ),
2020-01-24T02:55:02.3065129Z         msg: "6:13: 6:16: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.3065518Z     Error {
2020-01-24T02:55:02.3065622Z         line_num: 6,
2020-01-24T02:55:02.3065725Z         kind: Some(
2020-01-24T02:55:02.3065857Z             Warning,
---
2020-01-24T02:55:02.3066469Z         line_num: 11,
2020-01-24T02:55:02.3066612Z         kind: Some(
2020-01-24T02:55:02.3066723Z             Warning,
2020-01-24T02:55:02.3066880Z         ),
2020-01-24T02:55:02.3067240Z         msg: "11:7: 11:10: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.3067626Z     Error {
2020-01-24T02:55:02.3067788Z         line_num: 11,
2020-01-24T02:55:02.3067986Z         kind: Some(
2020-01-24T02:55:02.3068103Z             Warning,
---
2020-01-24T02:55:02.3068822Z         line_num: 11,
2020-01-24T02:55:02.3068962Z         kind: Some(
2020-01-24T02:55:02.3069091Z             Warning,
2020-01-24T02:55:02.3069228Z         ),
2020-01-24T02:55:02.3069597Z         msg: "11:13: 11:16: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.3069967Z     Error {
2020-01-24T02:55:02.3070153Z         line_num: 11,
2020-01-24T02:55:02.3070336Z         kind: Some(
2020-01-24T02:55:02.3070449Z             Warning,
---
2020-01-24T02:55:02.3071169Z         line_num: 6,
2020-01-24T02:55:02.3071298Z         kind: Some(
2020-01-24T02:55:02.3071427Z             Warning,
2020-01-24T02:55:02.3071567Z         ),
2020-01-24T02:55:02.3071944Z         msg: "6:7: 6:10: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.3072295Z     Error {
2020-01-24T02:55:02.3072455Z         line_num: 6,
2020-01-24T02:55:02.3072635Z         kind: Some(
2020-01-24T02:55:02.3072774Z             Warning,
---
2020-01-24T02:55:02.3073475Z         line_num: 6,
2020-01-24T02:55:02.3073623Z         kind: Some(
2020-01-24T02:55:02.3073729Z             Warning,
2020-01-24T02:55:02.3073957Z         ),
2020-01-24T02:55:02.3074414Z         msg: "6:13: 6:16: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.3074857Z     Error {
2020-01-24T02:55:02.3075035Z         line_num: 6,
2020-01-24T02:55:02.3075150Z         kind: Some(
2020-01-24T02:55:02.3075290Z             Warning,
---
2020-01-24T02:55:02.3075968Z         line_num: 11,
2020-01-24T02:55:02.3076103Z         kind: Some(
2020-01-24T02:55:02.3076322Z             Warning,
2020-01-24T02:55:02.3076464Z         ),
2020-01-24T02:55:02.3076893Z         msg: "11:7: 11:10: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.3077258Z     Error {
2020-01-24T02:55:02.3077425Z         line_num: 11,
2020-01-24T02:55:02.3077630Z         kind: Some(
2020-01-24T02:55:02.3077772Z             Warning,
---
2020-01-24T02:55:02.3078439Z         line_num: 11,
2020-01-24T02:55:02.3078579Z         kind: Some(
2020-01-24T02:55:02.3078685Z             Warning,
2020-01-24T02:55:02.3078822Z         ),
2020-01-24T02:55:02.3079190Z         msg: "11:13: 11:16: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.3079607Z     Error {
2020-01-24T02:55:02.3079761Z         line_num: 11,
2020-01-24T02:55:02.3079965Z         kind: Some(
2020-01-24T02:55:02.3080079Z             Warning,
---
2020-01-24T02:55:02.3081159Z thread '[ui] ui/pattern/usefulness/non-exhaustive-float-range-match.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1436:13
2020-01-24T02:55:02.3081303Z 
2020-01-24T02:55:02.3081653Z ---- [ui] ui/pattern/usefulness/non-exhaustive-match.rs stdout ----
2020-01-24T02:55:02.3082083Z 
2020-01-24T02:55:02.3082612Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:47: unexpected warning: '47:10: 47:13: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]'
2020-01-24T02:55:02.3083338Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:47: unexpected warning: '47:10: 47:13: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2020-01-24T02:55:02.3083381Z 
2020-01-24T02:55:02.3083381Z 
2020-01-24T02:55:02.3083662Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:47: unexpected warning: '47:15: 47:18: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]'
2020-01-24T02:55:02.3084015Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:47: unexpected warning: '47:15: 47:18: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2020-01-24T02:55:02.3084051Z 
2020-01-24T02:55:02.3084051Z 
2020-01-24T02:55:02.3084355Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:47: unexpected warning: '47:20: 47:23: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]'
2020-01-24T02:55:02.3084814Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:47: unexpected warning: '47:20: 47:23: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2020-01-24T02:55:02.3084850Z 
2020-01-24T02:55:02.3084850Z 
2020-01-24T02:55:02.3085145Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:48: unexpected warning: '48:10: 48:13: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]'
2020-01-24T02:55:02.3085474Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:48: unexpected warning: '48:10: 48:13: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2020-01-24T02:55:02.3085595Z 
2020-01-24T02:55:02.3085595Z 
2020-01-24T02:55:02.3086139Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:48: unexpected warning: '48:15: 48:18: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]'
2020-01-24T02:55:02.3086795Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:48: unexpected warning: '48:15: 48:18: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2020-01-24T02:55:02.3087022Z 
2020-01-24T02:55:02.3087022Z 
2020-01-24T02:55:02.3087499Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:49: unexpected warning: '49:10: 49:13: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]'
2020-01-24T02:55:02.3088162Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:49: unexpected warning: '49:10: 49:13: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2020-01-24T02:55:02.3088213Z 
2020-01-24T02:55:02.3088213Z 
2020-01-24T02:55:02.3088694Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:47: unexpected warning: '47:10: 47:13: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]'
2020-01-24T02:55:02.3089325Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:47: unexpected warning: '47:10: 47:13: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2020-01-24T02:55:02.3089503Z 
2020-01-24T02:55:02.3089503Z 
2020-01-24T02:55:02.3089954Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:47: unexpected warning: '47:15: 47:18: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]'
2020-01-24T02:55:02.3090479Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:47: unexpected warning: '47:15: 47:18: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2020-01-24T02:55:02.3090640Z 
2020-01-24T02:55:02.3090640Z 
2020-01-24T02:55:02.3091097Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:47: unexpected warning: '47:20: 47:23: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]'
2020-01-24T02:55:02.3091738Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:47: unexpected warning: '47:20: 47:23: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2020-01-24T02:55:02.3091778Z 
2020-01-24T02:55:02.3091778Z 
2020-01-24T02:55:02.3092249Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:48: unexpected warning: '48:10: 48:13: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]'
2020-01-24T02:55:02.3092978Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:48: unexpected warning: '48:10: 48:13: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2020-01-24T02:55:02.3093169Z 
2020-01-24T02:55:02.3093169Z 
2020-01-24T02:55:02.3093630Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:48: unexpected warning: '48:15: 48:18: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]'
2020-01-24T02:55:02.3094158Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:48: unexpected warning: '48:15: 48:18: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2020-01-24T02:55:02.3094317Z 
2020-01-24T02:55:02.3094317Z 
2020-01-24T02:55:02.3113363Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:49: unexpected warning: '49:10: 49:13: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]'
2020-01-24T02:55:02.3113945Z error: /checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs:49: unexpected warning: '49:10: 49:13: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2020-01-24T02:55:02.3113997Z 
2020-01-24T02:55:02.3114039Z error: 24 unexpected errors found, 0 expected errors not found
2020-01-24T02:55:02.3114079Z status: exit code: 1
2020-01-24T02:55:02.3114079Z status: exit code: 1
2020-01-24T02:55:02.3114769Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/usefulness/non-exhaustive-match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/non-exhaustive-match" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/non-exhaustive-match/auxiliary" "-A" "unused"
2020-01-24T02:55:02.3114914Z     Error {
2020-01-24T02:55:02.3114953Z         line_num: 47,
2020-01-24T02:55:02.3114988Z         kind: Some(
2020-01-24T02:55:02.3115029Z             Warning,
2020-01-24T02:55:02.3115029Z             Warning,
2020-01-24T02:55:02.3115062Z         ),
2020-01-24T02:55:02.3115309Z         msg: "47:10: 47:13: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.3115394Z     Error {
2020-01-24T02:55:02.3115429Z         line_num: 47,
2020-01-24T02:55:02.3115468Z         kind: Some(
2020-01-24T02:55:02.3115503Z             Warning,
---
2020-01-24T02:55:02.3116193Z         line_num: 47,
2020-01-24T02:55:02.3116412Z         kind: Some(
2020-01-24T02:55:02.3116449Z             Warning,
2020-01-24T02:55:02.3116673Z         ),
2020-01-24T02:55:02.3117084Z         msg: "47:15: 47:18: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.3117409Z     Error {
2020-01-24T02:55:02.3117444Z         line_num: 47,
2020-01-24T02:55:02.3117503Z         kind: Some(
2020-01-24T02:55:02.3117558Z             Warning,
---
2020-01-24T02:55:02.3117880Z         line_num: 47,
2020-01-24T02:55:02.3117943Z         kind: Some(
2020-01-24T02:55:02.3117977Z             Warning,
2020-01-24T02:55:02.3118035Z         ),
2020-01-24T02:55:02.3118454Z         msg: "47:20: 47:23: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.3118549Z     Error {
2020-01-24T02:55:02.3118582Z         line_num: 47,
2020-01-24T02:55:02.3118783Z         kind: Some(
2020-01-24T02:55:02.3118826Z             Warning,
---
2020-01-24T02:55:02.3119117Z         line_num: 48,
2020-01-24T02:55:02.3119174Z         kind: Some(
2020-01-24T02:55:02.3119207Z             Warning,
2020-01-24T02:55:02.3119260Z         ),
2020-01-24T02:55:02.3119789Z         msg: "48:10: 48:13: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.3120099Z     Error {
2020-01-24T02:55:02.3120153Z         line_num: 48,
2020-01-24T02:55:02.3120214Z         kind: Some(
2020-01-24T02:55:02.3120265Z             Warning,
---
2020-01-24T02:55:02.3120526Z         line_num: 48,
2020-01-24T02:55:02.3120585Z         kind: Some(
2020-01-24T02:55:02.3120617Z             Warning,
2020-01-24T02:55:02.3120649Z         ),
2020-01-24T02:55:02.3121132Z         msg: "48:15: 48:18: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.3121478Z     Error {
2020-01-24T02:55:02.3121536Z         line_num: 48,
2020-01-24T02:55:02.3121571Z         kind: Some(
2020-01-24T02:55:02.3121626Z             Warning,
---
2020-01-24T02:55:02.3122093Z         line_num: 49,
2020-01-24T02:55:02.3122154Z         kind: Some(
2020-01-24T02:55:02.3122209Z             Warning,
2020-01-24T02:55:02.3122243Z         ),
2020-01-24T02:55:02.3122579Z         msg: "49:10: 49:13: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.3122878Z     Error {
2020-01-24T02:55:02.3122942Z         line_num: 49,
2020-01-24T02:55:02.3122977Z         kind: Some(
2020-01-24T02:55:02.3123043Z             Warning,
---
2020-01-24T02:55:02.3123326Z         line_num: 47,
2020-01-24T02:55:02.3123360Z         kind: Some(
2020-01-24T02:55:02.3123397Z             Warning,
2020-01-24T02:55:02.3123430Z         ),
2020-01-24T02:55:02.3123735Z         msg: "47:10: 47:13: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.3123991Z     Error {
2020-01-24T02:55:02.3124329Z         line_num: 47,
2020-01-24T02:55:02.3124452Z         kind: Some(
2020-01-24T02:55:02.3124509Z             Warning,
---
2020-01-24T02:55:02.3124931Z         line_num: 47,
2020-01-24T02:55:02.3125076Z         kind: Some(
2020-01-24T02:55:02.3125233Z             Warning,
2020-01-24T02:55:02.3125362Z         ),
2020-01-24T02:55:02.3125698Z         msg: "47:15: 47:18: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.3125778Z     Error {
2020-01-24T02:55:02.3125809Z         line_num: 47,
2020-01-24T02:55:02.3125844Z         kind: Some(
2020-01-24T02:55:02.3125876Z             Warning,
---
2020-01-24T02:55:02.3126506Z         line_num: 47,
2020-01-24T02:55:02.3126637Z         kind: Some(
2020-01-24T02:55:02.3126688Z             Warning,
2020-01-24T02:55:02.3126735Z         ),
2020-01-24T02:55:02.3127202Z         msg: "47:20: 47:23: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.3127530Z     Error {
2020-01-24T02:55:02.3127579Z         line_num: 47,
2020-01-24T02:55:02.3127628Z         kind: Some(
2020-01-24T02:55:02.3127680Z             Warning,
---
2020-01-24T02:55:02.3127908Z         line_num: 48,
2020-01-24T02:55:02.3128091Z         kind: Some(
2020-01-24T02:55:02.3128226Z             Warning,
2020-01-24T02:55:02.3128267Z         ),
2020-01-24T02:55:02.3128587Z         msg: "48:10: 48:13: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.3128887Z     Error {
2020-01-24T02:55:02.3128921Z         line_num: 48,
2020-01-24T02:55:02.3128982Z         kind: Some(
2020-01-24T02:55:02.3129013Z             Warning,
---
2020-01-24T02:55:02.3129233Z         line_num: 48,
2020-01-24T02:55:02.3129264Z         kind: Some(
2020-01-24T02:55:02.3129294Z             Warning,
2020-01-24T02:55:02.3129324Z         ),
2020-01-24T02:55:02.3129612Z         msg: "48:15: 48:18: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.3129689Z     Error {
2020-01-24T02:55:02.3129875Z         line_num: 48,
2020-01-24T02:55:02.3130000Z         kind: Some(
2020-01-24T02:55:02.3130039Z             Warning,
---
2020-01-24T02:55:02.3130299Z         line_num: 49,
2020-01-24T02:55:02.3130355Z         kind: Some(
2020-01-24T02:55:02.3130386Z             Warning,
2020-01-24T02:55:02.3130435Z         ),
2020-01-24T02:55:02.3130738Z         msg: "49:10: 49:13: floating-point types cannot be used in patterns [illegal_floating_point_literal_pattern]",
2020-01-24T02:55:02.3131043Z     Error {
2020-01-24T02:55:02.3131076Z         line_num: 49,
2020-01-24T02:55:02.3131152Z         kind: Some(
2020-01-24T02:55:02.3131183Z             Warning,
---
2020-01-24T02:55:02.3132086Z thread '[ui] ui/pattern/usefulness/non-exhaustive-match.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1436:13
2020-01-24T02:55:02.3132126Z 
2020-01-24T02:55:02.3132484Z ---- [ui] ui/union/union-pat-refutability.rs stdout ----
2020-01-24T02:55:02.3132644Z normalized stderr:
2020-01-24T02:55:02.3132787Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.3133185Z    |
2020-01-24T02:55:02.3133317Z LL | #![allow(illegal_floating_point_literal_pattern)]
2020-01-24T02:55:02.3133456Z    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-24T02:55:02.3133626Z    |
2020-01-24T02:55:02.3133626Z    |
2020-01-24T02:55:02.3133760Z    = note: `#[warn(unused_attributes)]` on by default
2020-01-24T02:55:02.3133810Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.3133959Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.3134097Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.3134166Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.3134531Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.3134562Z 
2020-01-24T02:55:02.3134735Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.3135184Z    |
2020-01-24T02:55:02.3135325Z LL | #![allow(illegal_floating_point_literal_pattern)]
2020-01-24T02:55:02.3135369Z    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-24T02:55:02.3135429Z    |
2020-01-24T02:55:02.3135429Z    |
2020-01-24T02:55:02.3135489Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.3135553Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.3135610Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.3135674Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.3136004Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.3136157Z 
2020-01-24T02:55:02.3136310Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.3136725Z    |
2020-01-24T02:55:02.3136853Z LL | #![allow(illegal_floating_point_literal_pattern)]
2020-01-24T02:55:02.3136910Z    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-24T02:55:02.3136943Z    |
2020-01-24T02:55:02.3136943Z    |
2020-01-24T02:55:02.3137016Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.3137055Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.3137094Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.3137148Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.3137556Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.3137713Z 
2020-01-24T02:55:02.3137858Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.3138165Z    |
2020-01-24T02:55:02.3138199Z LL | #![allow(illegal_floating_point_literal_pattern)]
2020-01-24T02:55:02.3138234Z    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-24T02:55:02.3138362Z    |
2020-01-24T02:55:02.3138362Z    |
2020-01-24T02:55:02.3138410Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.3138607Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.3138749Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.3138808Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.3139238Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.3139737Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.3139905Z   --> $DIR/union-pat-refutability.rs:24:44
2020-01-24T02:55:02.3140197Z    |
2020-01-24T02:55:02.3140197Z    |
2020-01-24T02:55:02.3140334Z LL |             Value { tag: Tag::F, u: U { f: 0.0 } } => true,
2020-01-24T02:55:02.3140527Z    |
2020-01-24T02:55:02.3140659Z note: lint level defined here
2020-01-24T02:55:02.3140920Z   --> $DIR/union-pat-refutability.rs:3:10
2020-01-24T02:55:02.3141090Z    |
2020-01-24T02:55:02.3141090Z    |
2020-01-24T02:55:02.3141293Z LL | #![allow(illegal_floating_point_literal_pattern)]
2020-01-24T02:55:02.3141353Z    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-24T02:55:02.3141393Z    = note: #[warn(illegal_floating_point_literal_pattern)] is the minimum lint level
2020-01-24T02:55:02.3141580Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.3141580Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.3141979Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.3142383Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.3142709Z   --> $DIR/union-pat-refutability.rs:24:44
2020-01-24T02:55:02.3142876Z    |
2020-01-24T02:55:02.3142876Z    |
2020-01-24T02:55:02.3143005Z LL |             Value { tag: Tag::F, u: U { f: 0.0 } } => true,
2020-01-24T02:55:02.3143199Z    |
2020-01-24T02:55:02.3143355Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.3143355Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.3143654Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.3143938Z 
2020-01-24T02:55:02.3143959Z 
2020-01-24T02:55:02.3143998Z 
2020-01-24T02:55:02.3144042Z The actual stderr differed from the expected stderr.
2020-01-24T02:55:02.3144042Z The actual stderr differed from the expected stderr.
2020-01-24T02:55:02.3144358Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-pat-refutability/union-pat-refutability.stderr
2020-01-24T02:55:02.3144555Z To update references, rerun the tests and pass the `--bless` flag
2020-01-24T02:55:02.3144941Z To only update this specific test, also pass `--test-args union/union-pat-refutability.rs`
2020-01-24T02:55:02.3145234Z error: 1 errors occurred comparing output.
2020-01-24T02:55:02.3145277Z status: exit code: 0
2020-01-24T02:55:02.3145277Z status: exit code: 0
2020-01-24T02:55:02.3145942Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/union-pat-refutability.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-pat-refutability/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-pat-refutability/auxiliary"
2020-01-24T02:55:02.3146201Z ------------------------------------------
2020-01-24T02:55:02.3146228Z 
2020-01-24T02:55:02.3146569Z ------------------------------------------
2020-01-24T02:55:02.3146809Z stderr:
2020-01-24T02:55:02.3146809Z stderr:
2020-01-24T02:55:02.3147087Z ------------------------------------------
2020-01-24T02:55:02.3147252Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.3147769Z    |
2020-01-24T02:55:02.3147898Z LL | #![allow(illegal_floating_point_literal_pattern)]
2020-01-24T02:55:02.3147953Z    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-24T02:55:02.3148010Z    |
2020-01-24T02:55:02.3148010Z    |
2020-01-24T02:55:02.3148061Z    = note: `#[warn(unused_attributes)]` on by default
2020-01-24T02:55:02.3148107Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.3148165Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.3148415Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.3148561Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.3148899Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.3149059Z 
2020-01-24T02:55:02.3149209Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.3149621Z    |
2020-01-24T02:55:02.3149750Z LL | #![allow(illegal_floating_point_literal_pattern)]
2020-01-24T02:55:02.3149807Z    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-24T02:55:02.3149840Z    |
2020-01-24T02:55:02.3149840Z    |
2020-01-24T02:55:02.3149903Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.3149951Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.3150098Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.3150246Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.3150563Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.3150598Z 
2020-01-24T02:55:02.3150769Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.3151164Z    |
2020-01-24T02:55:02.3151298Z LL | #![allow(illegal_floating_point_literal_pattern)]
2020-01-24T02:55:02.3151354Z    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-24T02:55:02.3151411Z    |
2020-01-24T02:55:02.3151411Z    |
2020-01-24T02:55:02.3151464Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.3151512Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.3151578Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.3151650Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.3151963Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.3151990Z 
2020-01-24T02:55:02.3152157Z warning: #[allow(illegal_floating_point_literal_pattern)] has no effect
2020-01-24T02:55:02.3152622Z    |
2020-01-24T02:55:02.3152752Z LL | #![allow(illegal_floating_point_literal_pattern)]
2020-01-24T02:55:02.3152796Z    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-24T02:55:02.3152855Z    |
2020-01-24T02:55:02.3152855Z    |
2020-01-24T02:55:02.3152892Z    = note: the minimum lint level for `illegal_floating_point_literal_pattern` is `warn`
2020-01-24T02:55:02.3153054Z    = note: the lint level cannot be reduced to `allow`
2020-01-24T02:55:02.3153200Z    = help: remove the #[allow(illegal_floating_point_literal_pattern)] directive
2020-01-24T02:55:02.3153375Z    = warning: `illegal_floating_point_literal_pattern` was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.3153785Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.3154285Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.3154634Z   --> /checkout/src/test/ui/union/union-pat-refutability.rs:24:44
2020-01-24T02:55:02.3154799Z    |
2020-01-24T02:55:02.3154799Z    |
2020-01-24T02:55:02.3154937Z LL |             Value { tag: Tag::F, u: U { f: 0.0 } } => true,
2020-01-24T02:55:02.3155118Z    |
2020-01-24T02:55:02.3155285Z note: lint level defined here
2020-01-24T02:55:02.3155540Z   --> /checkout/src/test/ui/union/union-pat-refutability.rs:3:10
2020-01-24T02:55:02.3155577Z    |
2020-01-24T02:55:02.3155577Z    |
2020-01-24T02:55:02.3155758Z LL | #![allow(illegal_floating_point_literal_pattern)]
2020-01-24T02:55:02.3155902Z    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-24T02:55:02.3155943Z    = note: #[warn(illegal_floating_point_literal_pattern)] is the minimum lint level
2020-01-24T02:55:02.3156239Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.3156239Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.3156563Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.3157050Z warning: floating-point types cannot be used in patterns
2020-01-24T02:55:02.3157231Z   --> /checkout/src/test/ui/union/union-pat-refutability.rs:24:44
2020-01-24T02:55:02.3157404Z    |
2020-01-24T02:55:02.3157404Z    |
2020-01-24T02:55:02.3157539Z LL |             Value { tag: Tag::F, u: U { f: 0.0 } } => true,
2020-01-24T02:55:02.3157646Z    |
2020-01-24T02:55:02.3157700Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.3157700Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-01-24T02:55:02.3158109Z    = note: for more information, see issue #41620 <***/issues/41620>
2020-01-24T02:55:02.3158392Z 
2020-01-24T02:55:02.3158643Z ------------------------------------------
2020-01-24T02:55:02.3158668Z 
2020-01-24T02:55:02.3158687Z 
---
2020-01-24T02:55:02.3162348Z test result: FAILED. 9486 passed; 10 failed; 50 ignored; 0 measured; 0 filtered out
2020-01-24T02:55:02.3162378Z 
2020-01-24T02:55:02.3162694Z 
2020-01-24T02:55:02.3162824Z 
2020-01-24T02:55:02.3165119Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-24T02:55:02.3165828Z 
2020-01-24T02:55:02.3165859Z 
2020-01-24T02:55:02.3165952Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-24T02:55:02.3166024Z Build completed unsuccessfully in 0:46:12
2020-01-24T02:55:02.3166024Z Build completed unsuccessfully in 0:46:12
2020-01-24T02:55:02.3166098Z == clock drift check ==
2020-01-24T02:55:02.3166167Z   local time: Fri Jan 24 02:55:02 UTC 2020
2020-01-24T02:55:02.7995808Z   network time: Fri, 24 Jan 2020 02:55:02 GMT
2020-01-24T02:55:02.7996553Z == end clock drift check ==
2020-01-24T02:55:03.3054576Z 
2020-01-24T02:55:03.3102571Z ##[error]Bash exited with code '1'.
2020-01-24T02:55:03.3112031Z ##[section]Finishing: Run build
2020-01-24T02:55:03.3128987Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68501/merge to s
2020-01-24T02:55:03.3130523Z Task         : Get sources
2020-01-24T02:55:03.3130581Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-24T02:55:03.3130616Z Version      : 1.0.0
2020-01-24T02:55:03.3130648Z Author       : Microsoft
2020-01-24T02:55:03.3130648Z Author       : Microsoft
2020-01-24T02:55:03.3130699Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-24T02:55:03.3130735Z ==============================================================================
2020-01-24T02:55:03.6481901Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-24T02:55:03.6512270Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68501/merge to s
2020-01-24T02:55:03.6598725Z Cleaning up task key
2020-01-24T02:55:03.6599341Z Start cleaning up orphan processes.
2020-01-24T02:55:03.6683596Z Terminate orphan process: pid (3391) (python)
2020-01-24T02:55:03.6870291Z ##[section]Finishing: Finalize Job
