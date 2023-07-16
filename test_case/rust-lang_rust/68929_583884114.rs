plain
2020-02-09T18:26:15.7443834Z ========================== Starting Command Output ===========================
2020-02-09T18:26:15.7446911Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/49e7910b-af5a-4661-9d8d-72ad336a5755.sh
2020-02-09T18:26:15.7446953Z 
2020-02-09T18:26:15.7452444Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-09T18:26:15.7459995Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68929/merge to s
2020-02-09T18:26:15.7462301Z Task         : Get sources
2020-02-09T18:26:15.7462376Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-09T18:26:15.7462409Z Version      : 1.0.0
2020-02-09T18:26:15.7462442Z Author       : Microsoft
---
2020-02-09T18:26:20.3860865Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-09T18:26:20.3870922Z ##[command]git config gc.auto 0
2020-02-09T18:26:20.3873406Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-09T18:26:20.3874971Z ##[command]git config --get-all http.proxy
2020-02-09T18:26:20.3882285Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68929/merge:refs/remotes/pull/68929/merge
---
2020-02-09T19:25:41.1824384Z .................................................................................................... 1700/9621
2020-02-09T19:25:46.0919100Z .................................................................................................... 1800/9621
2020-02-09T19:25:58.2431058Z ............................i....................................................................... 1900/9621
2020-02-09T19:26:05.3796247Z .................................................................................................... 2000/9621
2020-02-09T19:26:19.9921123Z ..................iiiii............................................................................. 2100/9621
2020-02-09T19:26:29.7813910Z .................................................................................................... 2300/9621
2020-02-09T19:26:32.1435335Z .................................................................................................... 2400/9621
2020-02-09T19:26:36.9506637Z .................................................................................................... 2500/9621
2020-02-09T19:26:58.0863847Z .................................................................................................... 2600/9621
---
2020-02-09T19:29:35.3789034Z .....................................................................i...............i.............. 4900/9621
2020-02-09T19:29:43.1652416Z .................................................................................................... 5000/9621
2020-02-09T19:29:51.4119243Z .................................................................................................... 5100/9621
2020-02-09T19:29:56.1392581Z ...........i........................................................................................ 5200/9621
2020-02-09T19:30:07.3337505Z .....................................................................................ii.ii........i. 5300/9621
2020-02-09T19:30:11.1948979Z ..i................................................................................................. 5400/9621
2020-02-09T19:30:23.4321375Z .................................................................................................... 5600/9621
2020-02-09T19:30:32.0086649Z .........................................................................i.......................... 5700/9621
2020-02-09T19:30:39.2516857Z .................................................................................................... 5800/9621
2020-02-09T19:30:45.8600681Z .................................................................................................... 5900/9621
2020-02-09T19:30:45.8600681Z .................................................................................................... 5900/9621
2020-02-09T19:30:56.1267867Z .................................................................ii...i..ii...........i............. 6000/9621
2020-02-09T19:31:18.3957311Z .................................................................................................... 6200/9621
2020-02-09T19:31:25.3416319Z .................................................................................................... 6300/9621
2020-02-09T19:31:25.3416319Z .................................................................................................... 6300/9621
2020-02-09T19:31:30.6148593Z .............................................................................................i..ii.. 6400/9621
2020-02-09T19:31:53.5343852Z .................................................................................................... 6600/9621
2020-02-09T19:32:03.7297135Z ................................................................................i................... 6700/9621
2020-02-09T19:32:05.8681879Z .................................................................................................... 6800/9621
2020-02-09T19:32:08.4085366Z .......................................................................................i............ 6900/9621
---
2020-02-09T19:33:49.1601819Z .................................................................................................... 7600/9621
2020-02-09T19:33:53.2302441Z .................................................................................................... 7700/9621
2020-02-09T19:33:58.8396347Z .................................................................................................... 7800/9621
2020-02-09T19:34:07.5067838Z .................................................................................................... 7900/9621
2020-02-09T19:34:17.0444958Z ...............................................................iiiiiii.i............................ 8000/9621
2020-02-09T19:34:32.2983918Z ...i......i......................................................................................... 8200/9621
2020-02-09T19:34:37.7579516Z .................................................................................................... 8300/9621
2020-02-09T19:34:52.5360846Z .................................................................................................... 8400/9621
2020-02-09T19:35:01.2707061Z .................................................................................................... 8500/9621
---
2020-02-09T19:37:01.5254084Z 
2020-02-09T19:37:01.5254752Z ---- [ui] ui/feature-gates/feature-gate-no_sanitize.rs stdout ----
2020-02-09T19:37:01.5257517Z diff of stderr:
2020-02-09T19:37:01.5257701Z 
2020-02-09T19:37:01.5257911Z 4 LL | #[no_sanitize(address)]
2020-02-09T19:37:01.5258198Z 6    |
2020-02-09T19:37:01.5258198Z 6    |
2020-02-09T19:37:01.5258844Z -    = note: for more information, see ***/issues/39699
2020-02-09T19:37:01.5259527Z +    = note: see issue #39699 <***/issues/39699> for more information
2020-02-09T19:37:01.5259731Z 8    = help: add `#![feature(no_sanitize)]` to the crate attributes to enable
2020-02-09T19:37:01.5259990Z 10 error: aborting due to previous error
2020-02-09T19:37:01.5260122Z 
2020-02-09T19:37:01.5260232Z 
2020-02-09T19:37:01.5260360Z The actual stderr differed from the expected stderr.
2020-02-09T19:37:01.5260360Z The actual stderr differed from the expected stderr.
2020-02-09T19:37:01.5260821Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-no_sanitize/feature-gate-no_sanitize.stderr
2020-02-09T19:37:01.5261212Z To update references, rerun the tests and pass the `--bless` flag
2020-02-09T19:37:01.5261661Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-no_sanitize.rs`
2020-02-09T19:37:01.5261958Z error: 1 errors occurred comparing output.
2020-02-09T19:37:01.5262126Z status: exit code: 1
2020-02-09T19:37:01.5262126Z status: exit code: 1
2020-02-09T19:37:01.5263042Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-no_sanitize.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-no_sanitize" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-no_sanitize/auxiliary" "-A" "unused"
2020-02-09T19:37:01.5264676Z ------------------------------------------
2020-02-09T19:37:01.5264984Z 
2020-02-09T19:37:01.5265393Z ------------------------------------------
2020-02-09T19:37:01.5265564Z stderr:
2020-02-09T19:37:01.5265564Z stderr:
2020-02-09T19:37:01.5266317Z ------------------------------------------
2020-02-09T19:37:01.5266565Z error[E0658]: the `#[no_sanitize]` attribute is an experimental feature
2020-02-09T19:37:01.5266990Z   --> /checkout/src/test/ui/feature-gates/feature-gate-no_sanitize.rs:1:1
2020-02-09T19:37:01.5267182Z    |
2020-02-09T19:37:01.5267324Z LL | #[no_sanitize(address)]
2020-02-09T19:37:01.5267629Z    |
2020-02-09T19:37:01.5267629Z    |
2020-02-09T19:37:01.5268129Z    = note: see issue #39699 <***/issues/39699> for more information
2020-02-09T19:37:01.5268335Z    = help: add `#![feature(no_sanitize)]` to the crate attributes to enable
2020-02-09T19:37:01.5268604Z error: aborting due to previous error
2020-02-09T19:37:01.5268747Z 
2020-02-09T19:37:01.5269506Z For more information about this error, try `rustc --explain E0658`.
2020-02-09T19:37:01.5269777Z 
---
2020-02-09T19:37:01.5271067Z 
2020-02-09T19:37:01.5271193Z 6    |               |
2020-02-09T19:37:01.5271516Z 7    |               by-move pattern here
2020-02-09T19:37:01.5271674Z 8    |
2020-02-09T19:37:01.5272080Z -    = note: for more information, see ***/issues/68354
2020-02-09T19:37:01.5272529Z +    = note: see issue #68354 <***/issues/68354> for more information
2020-02-09T19:37:01.5272857Z 11 
2020-02-09T19:37:01.5273234Z 12 error[E0658]: binding by-move and by-ref in the same pattern is unstable
2020-02-09T19:37:01.5273385Z 
2020-02-09T19:37:01.5273524Z 17    |          |
2020-02-09T19:37:01.5273524Z 17    |          |
2020-02-09T19:37:01.5273851Z 18    |          by-ref pattern here
2020-02-09T19:37:01.5274030Z 19    |
2020-02-09T19:37:01.5274429Z -    = note: for more information, see ***/issues/68354
2020-02-09T19:37:01.5274871Z +    = note: see issue #68354 <***/issues/68354> for more information
2020-02-09T19:37:01.5275199Z 22 
2020-02-09T19:37:01.5275572Z 23 error[E0658]: binding by-move and by-ref in the same pattern is unstable
2020-02-09T19:37:01.5276110Z 
2020-02-09T19:37:01.5276277Z 28    |          |
2020-02-09T19:37:01.5276277Z 28    |          |
2020-02-09T19:37:01.5276664Z 29    |          by-ref pattern here
2020-02-09T19:37:01.5276875Z 30    |
2020-02-09T19:37:01.5277312Z -    = note: for more information, see ***/issues/68354
2020-02-09T19:37:01.5277807Z +    = note: see issue #68354 <***/issues/68354> for more information
2020-02-09T19:37:01.5278207Z 33 
2020-02-09T19:37:01.5278969Z 34 error[E0658]: binding by-move and by-ref in the same pattern is unstable
2020-02-09T19:37:01.5279210Z 
2020-02-09T19:37:01.5279505Z 39    |          |
2020-02-09T19:37:01.5279505Z 39    |          |
2020-02-09T19:37:01.5279832Z 40    |          by-move pattern here
2020-02-09T19:37:01.5280008Z 41    |
2020-02-09T19:37:01.5280400Z -    = note: for more information, see ***/issues/68354
2020-02-09T19:37:01.5280866Z +    = note: see issue #68354 <***/issues/68354> for more information
2020-02-09T19:37:01.5282185Z 44 
2020-02-09T19:37:01.5282333Z 45 error[E0507]: cannot move out of a shared reference
2020-02-09T19:37:01.5282444Z 
2020-02-09T19:37:01.5282550Z 
2020-02-09T19:37:01.5282550Z 
2020-02-09T19:37:01.5282673Z The actual stderr differed from the expected stderr.
2020-02-09T19:37:01.5283351Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/move-ref-patterns/feature-gate-move_ref_pattern/feature-gate-move_ref_pattern.stderr
2020-02-09T19:37:01.5283778Z To update references, rerun the tests and pass the `--bless` flag
2020-02-09T19:37:01.5284231Z To only update this specific test, also pass `--test-args pattern/move-ref-patterns/feature-gate-move_ref_pattern.rs`
2020-02-09T19:37:01.5284701Z error: 1 errors occurred comparing output.
2020-02-09T19:37:01.5284826Z status: exit code: 1
2020-02-09T19:37:01.5284826Z status: exit code: 1
2020-02-09T19:37:01.5304569Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/move-ref-patterns/feature-gate-move_ref_pattern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/move-ref-patterns/feature-gate-move_ref_pattern" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/move-ref-patterns/feature-gate-move_ref_pattern/auxiliary" "-A" "unused"
2020-02-09T19:37:01.5305286Z ------------------------------------------
2020-02-09T19:37:01.5305321Z 
2020-02-09T19:37:01.5305534Z ------------------------------------------
2020-02-09T19:37:01.5305576Z stderr:
2020-02-09T19:37:01.5305576Z stderr:
2020-02-09T19:37:01.5306277Z ------------------------------------------
2020-02-09T19:37:01.5306577Z error[E0658]: binding by-move and by-ref in the same pattern is unstable
2020-02-09T19:37:01.5306862Z   --> /checkout/src/test/ui/pattern/move-ref-patterns/feature-gate-move_ref_pattern.rs:8:15
2020-02-09T19:37:01.5306937Z    |
2020-02-09T19:37:01.5306983Z LL |         Some((y, ref z)) => {}
2020-02-09T19:37:01.5307220Z    |               ^  ----- by-ref pattern here
2020-02-09T19:37:01.5307511Z    |               by-move pattern here
2020-02-09T19:37:01.5307570Z    |
2020-02-09T19:37:01.5307570Z    |
2020-02-09T19:37:01.5307944Z    = note: see issue #68354 <***/issues/68354> for more information
2020-02-09T19:37:01.5308039Z 
2020-02-09T19:37:01.5308320Z error[E0658]: binding by-move and by-ref in the same pattern is unstable
2020-02-09T19:37:01.5308603Z   --> /checkout/src/test/ui/pattern/move-ref-patterns/feature-gate-move_ref_pattern.rs:13:17
2020-02-09T19:37:01.5308653Z    |
2020-02-09T19:37:01.5308653Z    |
2020-02-09T19:37:01.5308713Z LL |     let (ref a, b) = tup.clone();
2020-02-09T19:37:01.5308941Z    |          -----  ^ by-move pattern here
2020-02-09T19:37:01.5309204Z    |          by-ref pattern here
2020-02-09T19:37:01.5309266Z    |
2020-02-09T19:37:01.5309266Z    |
2020-02-09T19:37:01.5309710Z    = note: see issue #68354 <***/issues/68354> for more information
2020-02-09T19:37:01.5309815Z 
2020-02-09T19:37:01.5310051Z error[E0658]: binding by-move and by-ref in the same pattern is unstable
2020-02-09T19:37:01.5310293Z   --> /checkout/src/test/ui/pattern/move-ref-patterns/feature-gate-move_ref_pattern.rs:16:13
2020-02-09T19:37:01.5310352Z    |
2020-02-09T19:37:01.5310352Z    |
2020-02-09T19:37:01.5310390Z LL |     let (a, mut b) = &tup;
2020-02-09T19:37:01.5310588Z    |          -  ^^^^^ by-move pattern here
2020-02-09T19:37:01.5310830Z    |          by-ref pattern here
2020-02-09T19:37:01.5310868Z    |
2020-02-09T19:37:01.5310868Z    |
2020-02-09T19:37:01.5311143Z    = note: see issue #68354 <***/issues/68354> for more information
2020-02-09T19:37:01.5311223Z 
2020-02-09T19:37:01.5311447Z error[E0658]: binding by-move and by-ref in the same pattern is unstable
2020-02-09T19:37:01.5311708Z   --> /checkout/src/test/ui/pattern/move-ref-patterns/feature-gate-move_ref_pattern.rs:20:10
2020-02-09T19:37:01.5311759Z    |
2020-02-09T19:37:01.5311759Z    |
2020-02-09T19:37:01.5311804Z LL |     let (mut a, b) = &mut tup;
2020-02-09T19:37:01.5312024Z    |          ^^^^^  - by-ref pattern here
2020-02-09T19:37:01.5312249Z    |          by-move pattern here
2020-02-09T19:37:01.5312302Z    |
2020-02-09T19:37:01.5312302Z    |
2020-02-09T19:37:01.5312563Z    = note: see issue #68354 <***/issues/68354> for more information
2020-02-09T19:37:01.5312657Z 
2020-02-09T19:37:01.5312695Z error[E0507]: cannot move out of a shared reference
2020-02-09T19:37:01.5312943Z   --> /checkout/src/test/ui/pattern/move-ref-patterns/feature-gate-move_ref_pattern.rs:16:22
2020-02-09T19:37:01.5313003Z    |
2020-02-09T19:37:01.5313003Z    |
2020-02-09T19:37:01.5313039Z LL |     let (a, mut b) = &tup;
2020-02-09T19:37:01.5313263Z    |             |
2020-02-09T19:37:01.5313317Z    |             data moved here
2020-02-09T19:37:01.5313317Z    |             data moved here
2020-02-09T19:37:01.5313476Z    |             move occurs because `b` has type `main::X`, which does not implement the `Copy` trait
2020-02-09T19:37:01.5313628Z error[E0507]: cannot move out of a mutable reference
2020-02-09T19:37:01.5313910Z   --> /checkout/src/test/ui/pattern/move-ref-patterns/feature-gate-move_ref_pattern.rs:20:22
2020-02-09T19:37:01.5313953Z    |
2020-02-09T19:37:01.5313953Z    |
2020-02-09T19:37:01.5314007Z LL |     let (mut a, b) = &mut tup;
2020-02-09T19:37:01.5314236Z    |          |
2020-02-09T19:37:01.5314290Z    |          data moved here
2020-02-09T19:37:01.5314290Z    |          data moved here
2020-02-09T19:37:01.5314337Z    |          move occurs because `a` has type `main::X`, which does not implement the `Copy` trait
2020-02-09T19:37:01.5314405Z error: aborting due to 6 previous errors
2020-02-09T19:37:01.5314448Z 
2020-02-09T19:37:01.5314488Z Some errors have detailed explanations: E0507, E0658.
2020-02-09T19:37:01.5314721Z For more information about an error, try `rustc --explain E0507`.
2020-02-09T19:37:01.5314721Z For more information about an error, try `rustc --explain E0507`.
2020-02-09T19:37:01.5314757Z 
2020-02-09T19:37:01.5314971Z ------------------------------------------
2020-02-09T19:37:01.5314999Z 
2020-02-09T19:37:01.5315021Z 
2020-02-09T19:37:01.5315225Z ---- [ui] ui/suggestions/missing-assoc-fn.rs stdout ----
2020-02-09T19:37:01.5315282Z diff of stderr:
2020-02-09T19:37:01.5315306Z 
2020-02-09T19:37:01.5315510Z 4 LL |     fn bat<T: TraitB<Item: Copy>>(_: T) -> Self;
2020-02-09T19:37:01.5315609Z 6    |
2020-02-09T19:37:01.5315609Z 6    |
2020-02-09T19:37:01.5316562Z -    = note: for more information, see ***/issues/52662
2020-02-09T19:37:01.5316905Z +    = note: see issue #52662 <***/issues/52662> for more information
2020-02-09T19:37:01.5317014Z 9 
2020-02-09T19:37:01.5317014Z 9 
2020-02-09T19:37:01.5317081Z 10 error[E0046]: not all trait items implemented, missing: `foo`, `bar`, `baz`, `bat`
2020-02-09T19:37:01.5317153Z 
2020-02-09T19:37:01.5317206Z The actual stderr differed from the expected stderr.
2020-02-09T19:37:01.5317564Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-assoc-fn/missing-assoc-fn.stderr
2020-02-09T19:37:01.5317564Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-assoc-fn/missing-assoc-fn.stderr
2020-02-09T19:37:01.5317824Z To update references, rerun the tests and pass the `--bless` flag
2020-02-09T19:37:01.5318097Z To only update this specific test, also pass `--test-args suggestions/missing-assoc-fn.rs`
2020-02-09T19:37:01.5318194Z error: 1 errors occurred comparing output.
2020-02-09T19:37:01.5318238Z status: exit code: 1
2020-02-09T19:37:01.5318238Z status: exit code: 1
2020-02-09T19:37:01.5319099Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/missing-assoc-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-assoc-fn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-assoc-fn/auxiliary" "-A" "unused"
2020-02-09T19:37:01.5319570Z ------------------------------------------
2020-02-09T19:37:01.5319599Z 
2020-02-09T19:37:01.5319790Z ------------------------------------------
2020-02-09T19:37:01.5319828Z stderr:
2020-02-09T19:37:01.5319828Z stderr:
2020-02-09T19:37:01.5320029Z ------------------------------------------
2020-02-09T19:37:01.5320070Z error[E0658]: associated type bounds are unstable
2020-02-09T19:37:01.5320277Z   --> /checkout/src/test/ui/suggestions/missing-assoc-fn.rs:9:22
2020-02-09T19:37:01.5320335Z    |
2020-02-09T19:37:01.5320574Z LL |     fn bat<T: TraitB<Item: Copy>>(_: T) -> Self; //~ ERROR associated type bounds are unstable
2020-02-09T19:37:01.5320675Z    |
2020-02-09T19:37:01.5320675Z    |
2020-02-09T19:37:01.5321163Z    = note: see issue #52662 <***/issues/52662> for more information
2020-02-09T19:37:01.5321445Z 
2020-02-09T19:37:01.5321445Z 
2020-02-09T19:37:01.5321486Z error[E0046]: not all trait items implemented, missing: `foo`, `bar`, `baz`, `bat`
2020-02-09T19:37:01.5321800Z    |
2020-02-09T19:37:01.5321800Z    |
2020-02-09T19:37:01.5322000Z LL |     fn foo<T: TraitB<Item = A>>(_: T) -> Self;
2020-02-09T19:37:01.5322218Z    |     ------------------------------------------ `foo` from trait
2020-02-09T19:37:01.5322629Z    |     ------------------------ `bar` from trait
2020-02-09T19:37:01.5322853Z LL |     fn baz<T>(_: T) -> Self where T: TraitB, <T as TraitB>::Item: Copy;
2020-02-09T19:37:01.5323120Z    |     ------------------------------------------------------------------- `baz` from trait
2020-02-09T19:37:01.5323120Z    |     ------------------------------------------------------------------- `baz` from trait
2020-02-09T19:37:01.5323381Z LL |     fn bat<T: TraitB<Item: Copy>>(_: T) -> Self; //~ ERROR associated type bounds are unstable
2020-02-09T19:37:01.5323604Z    |     -------------------------------------------- `bat` from trait
2020-02-09T19:37:01.5323702Z LL | impl TraitA<()> for S { //~ ERROR not all trait items implemented
2020-02-09T19:37:01.5323702Z LL | impl TraitA<()> for S { //~ ERROR not all trait items implemented
2020-02-09T19:37:01.5323748Z    | ^^^^^^^^^^^^^^^^^^^^^ missing `foo`, `bar`, `baz`, `bat` in implementation
2020-02-09T19:37:01.5323834Z error[E0046]: not all trait items implemented, missing: `from_iter`
2020-02-09T19:37:01.5324055Z   --> /checkout/src/test/ui/suggestions/missing-assoc-fn.rs:19:1
2020-02-09T19:37:01.5324096Z    |
2020-02-09T19:37:01.5324096Z    |
2020-02-09T19:37:01.5324361Z LL | impl FromIterator<()> for X { //~ ERROR not all trait items implemented
2020-02-09T19:37:01.5324635Z    |
2020-02-09T19:37:01.5324635Z    |
2020-02-09T19:37:01.5325134Z    = help: implement the missing item: `fn from_iter<T>(_: T) -> Self where T: std::iter::IntoIterator, std::iter::IntoIterator::Item = A { todo!() }`
2020-02-09T19:37:01.5325224Z error: aborting due to 3 previous errors
2020-02-09T19:37:01.5325266Z 
2020-02-09T19:37:01.5325306Z Some errors have detailed explanations: E0046, E0658.
2020-02-09T19:37:01.5325543Z For more information about an error, try `rustc --explain E0046`.
---
2020-02-09T19:37:01.5328260Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-09T19:37:01.5328335Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-09T19:37:01.5328367Z 
2020-02-09T19:37:01.5328394Z 
2020-02-09T19:37:01.5330318Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-09T19:37:01.5330568Z 
2020-02-09T19:37:01.5330595Z 
2020-02-09T19:37:01.5330653Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-09T19:37:01.5330697Z Build completed unsuccessfully in 1:04:08
2020-02-09T19:37:01.5330697Z Build completed unsuccessfully in 1:04:08
2020-02-09T19:37:01.5345247Z == clock drift check ==
2020-02-09T19:37:01.5373734Z   local time: Sun Feb  9 19:37:01 UTC 2020
2020-02-09T19:37:01.8329356Z   network time: Sun, 09 Feb 2020 19:37:01 GMT
2020-02-09T19:37:01.8333140Z == end clock drift check ==
2020-02-09T19:37:02.2688464Z 
2020-02-09T19:37:02.2794025Z ##[error]Bash exited with code '1'.
2020-02-09T19:37:02.2806884Z ##[section]Finishing: Run build
2020-02-09T19:37:02.2829407Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68929/merge to s
2020-02-09T19:37:02.2832148Z Task         : Get sources
2020-02-09T19:37:02.2832187Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-09T19:37:02.2832241Z Version      : 1.0.0
2020-02-09T19:37:02.2832274Z Author       : Microsoft
2020-02-09T19:37:02.2832274Z Author       : Microsoft
2020-02-09T19:37:02.2832311Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-09T19:37:02.2832368Z ==============================================================================
2020-02-09T19:37:02.7191295Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-09T19:37:02.7233233Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68929/merge to s
2020-02-09T19:37:02.7350141Z Cleaning up task key
2020-02-09T19:37:02.7351194Z Start cleaning up orphan processes.
2020-02-09T19:37:02.7457262Z Terminate orphan process: pid (3866) (python)
2020-02-09T19:37:02.7913772Z ##[section]Finishing: Finalize Job
