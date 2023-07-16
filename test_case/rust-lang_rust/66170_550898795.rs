plain
2019-11-07T05:25:10.6503990Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-07T05:25:10.6689532Z ##[command]git config gc.auto 0
2019-11-07T05:25:10.6791545Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-07T05:25:10.6902883Z ##[command]git config --get-all http.proxy
2019-11-07T05:25:10.7094680Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66170/merge:refs/remotes/pull/66170/merge
---
2019-11-07T06:27:53.9298980Z .................................................................................................... 1600/9282
2019-11-07T06:28:00.1526402Z .................................................................................................... 1700/9282
2019-11-07T06:28:13.6740006Z ................................................................i................................... 1800/9282
2019-11-07T06:28:22.4783876Z .................................................................................................... 1900/9282
2019-11-07T06:28:38.2643242Z ................................................iiiii............................................... 2000/9282
2019-11-07T06:28:49.3786054Z .................................................................................................... 2200/9282
2019-11-07T06:28:52.0228755Z .................................................................................................... 2300/9282
2019-11-07T06:28:55.8721972Z .................................................................................................... 2400/9282
2019-11-07T06:29:19.8836954Z .................................................................................................... 2500/9282
---
2019-11-07T06:32:14.0379462Z ..............................................i...............i..................................... 4800/9282
2019-11-07T06:32:23.1189454Z ....FF..................................F........................................................... 4900/9282
2019-11-07T06:32:31.7907069Z .................................................................................................... 5000/9282
2019-11-07T06:32:38.5269047Z .................................................................................................... 5100/9282
2019-11-07T06:32:48.7327983Z ...............................................ii.ii...........i.................................... 5200/9282
2019-11-07T06:32:58.8078443Z .................................................................................................... 5400/9282
2019-11-07T06:33:09.4126970Z .................................................................................................... 5500/9282
2019-11-07T06:33:16.8767680Z .............................i...................................................................... 5600/9282
2019-11-07T06:33:23.7036170Z .................................................................................................... 5700/9282
2019-11-07T06:33:23.7036170Z .................................................................................................... 5700/9282
2019-11-07T06:33:36.0336777Z .................................................................................................... 5800/9282
2019-11-07T06:33:47.8360551Z ..............i.i..i..ii...........i................................................................ 5900/9282
2019-11-07T06:34:08.6162755Z .................................................................................................... 6100/9282
2019-11-07T06:34:15.4686214Z .................................................................................................... 6200/9282
2019-11-07T06:34:15.4686214Z .................................................................................................... 6200/9282
2019-11-07T06:34:29.5743282Z .................................i..ii.............................................................. 6300/9282
2019-11-07T06:34:51.0951885Z .................................................................................................... 6500/9282
2019-11-07T06:34:53.3604588Z i................................................................................................... 6600/9282
2019-11-07T06:34:55.6680334Z ...............................................................................i.................... 6700/9282
2019-11-07T06:34:58.4165862Z .................................................................................................... 6800/9282
---
2019-11-07T06:36:49.5009214Z ........................F........................................................................... 7500/9282
2019-11-07T06:36:57.8657681Z .........................F.......................................................................... 7600/9282
2019-11-07T06:37:09.2759308Z .................................................................................................... 7700/9282
2019-11-07T06:37:18.5814081Z .................................................................................................... 7800/9282
2019-11-07T06:37:25.3530357Z ..ii......i......................................................................................... 7900/9282
2019-11-07T06:37:46.9209291Z .................................................................................................... 8100/9282
2019-11-07T06:37:55.8944179Z .................................................................................................... 8200/9282
2019-11-07T06:38:04.5474939Z .................................................................................................... 8300/9282
2019-11-07T06:38:46.1332712Z .................................................................................................... 8400/9282
---
2019-11-07T06:40:00.0099578Z 
2019-11-07T06:40:00.0100709Z ---- [ui] ui/closures/issue-52437.rs stdout ----
2019-11-07T06:40:00.0100990Z diff of stderr:
2019-11-07T06:40:00.0101157Z 
2019-11-07T06:40:00.0101669Z 4 LL |     [(); &(&'static: loop { |x| {}; }) as *const _ as usize]
2019-11-07T06:40:00.0102072Z 6 
2019-11-07T06:40:00.0102232Z + error[E0744]: `loop` is not allowed in a `const`
2019-11-07T06:40:00.0102640Z +   --> $DIR/issue-52437.rs:2:13
2019-11-07T06:40:00.0103364Z +    |
2019-11-07T06:40:00.0103364Z +    |
2019-11-07T06:40:00.0104001Z + LL |     [(); &(&'static: loop { |x| {}; }) as *const _ as usize]
2019-11-07T06:40:00.0104399Z + 
2019-11-07T06:40:00.0104554Z 7 error[E0282]: type annotations needed
2019-11-07T06:40:00.0105843Z 8   --> $DIR/issue-52437.rs:2:30
2019-11-07T06:40:00.0106171Z 9    |
2019-11-07T06:40:00.0106171Z 9    |
2019-11-07T06:40:00.0106312Z 
2019-11-07T06:40:00.0106788Z 10 LL |     [(); &(&'static: loop { |x| {}; }) as *const _ as usize]
2019-11-07T06:40:00.0107236Z 11    |                              ^ consider giving this closure parameter a type
2019-11-07T06:40:00.0107893Z - error: aborting due to 2 previous errors
2019-11-07T06:40:00.0108133Z + error: aborting due to 3 previous errors
2019-11-07T06:40:00.0108287Z 14 
2019-11-07T06:40:00.0108723Z - For more information about this error, try `rustc --explain E0282`.
2019-11-07T06:40:00.0108723Z - For more information about this error, try `rustc --explain E0282`.
2019-11-07T06:40:00.0108967Z + Some errors have detailed explanations: E0282, E0744.
2019-11-07T06:40:00.0109402Z + For more information about an error, try `rustc --explain E0282`.
2019-11-07T06:40:00.0109625Z 16 
2019-11-07T06:40:00.0110573Z 
2019-11-07T06:40:00.0110754Z 
2019-11-07T06:40:00.0110916Z The actual stderr differed from the expected stderr.
2019-11-07T06:40:00.0111881Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/issue-52437/issue-52437.stderr
2019-11-07T06:40:00.0112460Z To update references, rerun the tests and pass the `--bless` flag
2019-11-07T06:40:00.0113415Z To only update this specific test, also pass `--test-args closures/issue-52437.rs`
2019-11-07T06:40:00.0113837Z error: 1 errors occurred comparing output.
2019-11-07T06:40:00.0114008Z status: exit code: 1
2019-11-07T06:40:00.0114008Z status: exit code: 1
2019-11-07T06:40:00.0115002Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/issue-52437.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/issue-52437" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/issue-52437/auxiliary" "-A" "unused"
2019-11-07T06:40:00.0118061Z ------------------------------------------
2019-11-07T06:40:00.0121492Z 
2019-11-07T06:40:00.0121894Z ------------------------------------------
2019-11-07T06:40:00.0121949Z stderr:
2019-11-07T06:40:00.0121949Z stderr:
2019-11-07T06:40:00.0122209Z ------------------------------------------
2019-11-07T06:40:00.0122470Z error: invalid label name `'static`
2019-11-07T06:40:00.0123058Z   --> /checkout/src/test/ui/closures/issue-52437.rs:2:13
2019-11-07T06:40:00.0123127Z    |
2019-11-07T06:40:00.0123480Z LL |     [(); &(&'static: loop { |x| {}; }) as *const _ as usize]
2019-11-07T06:40:00.0123570Z 
2019-11-07T06:40:00.0123621Z error[E0744]: `loop` is not allowed in a `const`
2019-11-07T06:40:00.0123910Z   --> /checkout/src/test/ui/closures/issue-52437.rs:2:13
2019-11-07T06:40:00.0123963Z    |
2019-11-07T06:40:00.0123963Z    |
2019-11-07T06:40:00.0124228Z LL |     [(); &(&'static: loop { |x| {}; }) as *const _ as usize]
2019-11-07T06:40:00.0124337Z 
2019-11-07T06:40:00.0124385Z error[E0282]: type annotations needed
2019-11-07T06:40:00.0124922Z   --> /checkout/src/test/ui/closures/issue-52437.rs:2:30
2019-11-07T06:40:00.0124997Z    |
2019-11-07T06:40:00.0124997Z    |
2019-11-07T06:40:00.0125279Z LL |     [(); &(&'static: loop { |x| {}; }) as *const _ as usize]
2019-11-07T06:40:00.0125343Z    |                              ^ consider giving this closure parameter a type
2019-11-07T06:40:00.0125454Z error: aborting due to 3 previous errors
2019-11-07T06:40:00.0125486Z 
2019-11-07T06:40:00.0125536Z Some errors have detailed explanations: E0282, E0744.
2019-11-07T06:40:00.0125835Z For more information about an error, try `rustc --explain E0282`.
---
2019-11-07T06:40:00.0126529Z 
2019-11-07T06:40:00.0126594Z + error[E0744]: `for` is not allowed in a `const`
2019-11-07T06:40:00.0126848Z +   --> $DIR/issue-50582.rs:2:20
2019-11-07T06:40:00.0126899Z +    |
2019-11-07T06:40:00.0127071Z + LL |     Vec::<[(); 1 + for x in 0..1 {}]>::new();
2019-11-07T06:40:00.0127179Z + 
2019-11-07T06:40:00.0127245Z 1 error[E0277]: cannot add `()` to `{integer}`
2019-11-07T06:40:00.0127525Z 2   --> $DIR/issue-50582.rs:2:18
2019-11-07T06:40:00.0127576Z 3    |
2019-11-07T06:40:00.0127576Z 3    |
2019-11-07T06:40:00.0127606Z 
2019-11-07T06:40:00.0127668Z 6    |
2019-11-07T06:40:00.0127723Z 7    = help: the trait `std::ops::Add<()>` is not implemented for `{integer}`
2019-11-07T06:40:00.0128018Z - error: aborting due to previous error
2019-11-07T06:40:00.0128091Z + error: aborting due to 2 previous errors
2019-11-07T06:40:00.0128138Z 10 
2019-11-07T06:40:00.0128415Z - For more information about this error, try `rustc --explain E0277`.
2019-11-07T06:40:00.0128415Z - For more information about this error, try `rustc --explain E0277`.
2019-11-07T06:40:00.0128493Z + Some errors have detailed explanations: E0277, E0744.
2019-11-07T06:40:00.0159948Z + For more information about an error, try `rustc --explain E0277`.
2019-11-07T06:40:00.0160049Z 12 
2019-11-07T06:40:00.0160135Z 
2019-11-07T06:40:00.0160166Z 
2019-11-07T06:40:00.0160219Z The actual stderr differed from the expected stderr.
2019-11-07T06:40:00.0160649Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50582/issue-50582.stderr
2019-11-07T06:40:00.0160967Z To update references, rerun the tests and pass the `--bless` flag
2019-11-07T06:40:00.0161306Z To only update this specific test, also pass `--test-args issues/issue-50582.rs`
2019-11-07T06:40:00.0161423Z error: 1 errors occurred comparing output.
2019-11-07T06:40:00.0161476Z status: exit code: 1
2019-11-07T06:40:00.0161476Z status: exit code: 1
2019-11-07T06:40:00.0162343Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50582.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50582" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50582/auxiliary" "-A" "unused"
2019-11-07T06:40:00.0163222Z ------------------------------------------
2019-11-07T06:40:00.0163295Z 
2019-11-07T06:40:00.0163555Z ------------------------------------------
2019-11-07T06:40:00.0163608Z stderr:
2019-11-07T06:40:00.0163608Z stderr:
2019-11-07T06:40:00.0163849Z ------------------------------------------
2019-11-07T06:40:00.0163924Z error[E0744]: `for` is not allowed in a `const`
2019-11-07T06:40:00.0164194Z   --> /checkout/src/test/ui/issues/issue-50582.rs:2:20
2019-11-07T06:40:00.0164249Z    |
2019-11-07T06:40:00.0164318Z LL |     Vec::<[(); 1 + for x in 0..1 {}]>::new();
2019-11-07T06:40:00.0164585Z 
2019-11-07T06:40:00.0164637Z error[E0277]: cannot add `()` to `{integer}`
2019-11-07T06:40:00.0164989Z   --> /checkout/src/test/ui/issues/issue-50582.rs:2:18
2019-11-07T06:40:00.0165043Z    |
2019-11-07T06:40:00.0165043Z    |
2019-11-07T06:40:00.0165094Z LL |     Vec::<[(); 1 + for x in 0..1 {}]>::new();
2019-11-07T06:40:00.0165169Z    |                  ^ no implementation for `{integer} + ()`
2019-11-07T06:40:00.0165220Z    |
2019-11-07T06:40:00.0165277Z    = help: the trait `std::ops::Add<()>` is not implemented for `{integer}`
2019-11-07T06:40:00.0165381Z error: aborting due to 2 previous errors
2019-11-07T06:40:00.0165412Z 
2019-11-07T06:40:00.0165462Z Some errors have detailed explanations: E0277, E0744.
2019-11-07T06:40:00.0165768Z For more information about an error, try `rustc --explain E0277`.
---
2019-11-07T06:40:00.0166478Z 
2019-11-07T06:40:00.0166667Z + error[E0744]: `for` is not allowed in a `const`
2019-11-07T06:40:00.0166956Z +   --> $DIR/issue-50585.rs:2:18
2019-11-07T06:40:00.0167008Z +    |
2019-11-07T06:40:00.0167057Z + LL |     |y: Vec<[(); for x in 0..2 {}]>| {};
2019-11-07T06:40:00.0167175Z + 
2019-11-07T06:40:00.0167223Z 1 error[E0308]: mismatched types
2019-11-07T06:40:00.0167487Z 2   --> $DIR/issue-50585.rs:2:18
2019-11-07T06:40:00.0167536Z 3    |
---
2019-11-07T06:40:00.0168831Z 
2019-11-07T06:40:00.0168877Z 
2019-11-07T06:40:00.0168926Z The actual stderr differed from the expected stderr.
2019-11-07T06:40:00.0169265Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50585/issue-50585.stderr
2019-11-07T06:40:00.0169563Z To update references, rerun the tests and pass the `--bless` flag
2019-11-07T06:40:00.0169890Z To only update this specific test, also pass `--test-args issues/issue-50585.rs`
2019-11-07T06:40:00.0169984Z error: 1 errors occurred comparing output.
2019-11-07T06:40:00.0170053Z status: exit code: 1
2019-11-07T06:40:00.0170053Z status: exit code: 1
2019-11-07T06:40:00.0170902Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50585.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50585" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50585/auxiliary" "-A" "unused"
2019-11-07T06:40:00.0171299Z ------------------------------------------
2019-11-07T06:40:00.0171338Z 
2019-11-07T06:40:00.0171625Z ------------------------------------------
2019-11-07T06:40:00.0171678Z stderr:
2019-11-07T06:40:00.0171678Z stderr:
2019-11-07T06:40:00.0171938Z ------------------------------------------
2019-11-07T06:40:00.0172015Z error[E0744]: `for` is not allowed in a `const`
2019-11-07T06:40:00.0172306Z   --> /checkout/src/test/ui/issues/issue-50585.rs:2:18
2019-11-07T06:40:00.0172363Z    |
2019-11-07T06:40:00.0172541Z LL |     |y: Vec<[(); for x in 0..2 {}]>| {};
2019-11-07T06:40:00.0172632Z 
2019-11-07T06:40:00.0172927Z error[E0308]: mismatched types
2019-11-07T06:40:00.0173334Z   --> /checkout/src/test/ui/issues/issue-50585.rs:2:18
2019-11-07T06:40:00.0173390Z    |
2019-11-07T06:40:00.0173390Z    |
2019-11-07T06:40:00.0173441Z LL |     |y: Vec<[(); for x in 0..2 {}]>| {};
2019-11-07T06:40:00.0173515Z    |                  ^^^^^^^^^^^^^^^^ expected usize, found ()
2019-11-07T06:40:00.0173620Z    = note: expected type `usize`
2019-11-07T06:40:00.0173669Z               found type `()`
2019-11-07T06:40:00.0173719Z 
2019-11-07T06:40:00.0173768Z error: aborting due to 2 previous errors
---
2019-11-07T06:40:00.0174993Z 
2019-11-07T06:40:00.0175066Z + error[E0744]: `while let` is not allowed in a `const`
2019-11-07T06:40:00.0175348Z +   --> $DIR/issue-51714.rs:11:17
2019-11-07T06:40:00.0175399Z +    |
2019-11-07T06:40:00.0175467Z + LL |     [(); return while let Some(n) = Some(0) {}];
2019-11-07T06:40:00.0175567Z + 
2019-11-07T06:40:00.0175636Z 1 error[E0572]: return statement outside of function body
2019-11-07T06:40:00.0175883Z 2   --> $DIR/issue-51714.rs:2:14
2019-11-07T06:40:00.0175932Z 3    |
2019-11-07T06:40:00.0175932Z 3    |
2019-11-07T06:40:00.0175962Z 
2019-11-07T06:40:00.0176028Z 22 LL |     [(); return while let Some(n) = Some(0) {}];
2019-11-07T06:40:00.0176140Z 24 
2019-11-07T06:40:00.0176410Z - error: aborting due to 4 previous errors
2019-11-07T06:40:00.0176465Z + error: aborting due to 5 previous errors
2019-11-07T06:40:00.0176520Z 26 
---
2019-11-07T06:40:00.0177255Z 
2019-11-07T06:40:00.0177283Z 
2019-11-07T06:40:00.0177332Z The actual stderr differed from the expected stderr.
2019-11-07T06:40:00.0177668Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51714/issue-51714.stderr
2019-11-07T06:40:00.0177965Z To update references, rerun the tests and pass the `--bless` flag
2019-11-07T06:40:00.0178284Z To only update this specific test, also pass `--test-args issues/issue-51714.rs`
2019-11-07T06:40:00.0178430Z error: 1 errors occurred comparing output.
2019-11-07T06:40:00.0178482Z status: exit code: 1
2019-11-07T06:40:00.0178482Z status: exit code: 1
2019-11-07T06:40:00.0179329Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-51714.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51714" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51714/auxiliary" "-A" "unused"
2019-11-07T06:40:00.0179721Z ------------------------------------------
2019-11-07T06:40:00.0179781Z 
2019-11-07T06:40:00.0180048Z ------------------------------------------
2019-11-07T06:40:00.0180101Z stderr:
2019-11-07T06:40:00.0180101Z stderr:
2019-11-07T06:40:00.0180360Z ------------------------------------------
2019-11-07T06:40:00.0180546Z error[E0744]: `while let` is not allowed in a `const`
2019-11-07T06:40:00.0180884Z   --> /checkout/src/test/ui/issues/issue-51714.rs:11:17
2019-11-07T06:40:00.0180942Z    |
2019-11-07T06:40:00.0181021Z LL |     [(); return while let Some(n) = Some(0) {}];
2019-11-07T06:40:00.0181112Z 
2019-11-07T06:40:00.0181182Z error[E0572]: return statement outside of function body
2019-11-07T06:40:00.0181486Z   --> /checkout/src/test/ui/issues/issue-51714.rs:2:14
2019-11-07T06:40:00.0181542Z    |
2019-11-07T06:40:00.0181542Z    |
2019-11-07T06:40:00.0181593Z LL |     |_:  [_; return || {}] | {};
2019-11-07T06:40:00.0181698Z 
2019-11-07T06:40:00.0181749Z error[E0572]: return statement outside of function body
2019-11-07T06:40:00.0182061Z   --> /checkout/src/test/ui/issues/issue-51714.rs:5:10
2019-11-07T06:40:00.0182115Z    |
2019-11-07T06:40:00.0182115Z    |
2019-11-07T06:40:00.0182166Z LL |     [(); return || {}];
2019-11-07T06:40:00.0182278Z 
2019-11-07T06:40:00.0182422Z error[E0572]: return statement outside of function body
2019-11-07T06:40:00.0183052Z   --> /checkout/src/test/ui/issues/issue-51714.rs:8:10
2019-11-07T06:40:00.0183136Z    |
2019-11-07T06:40:00.0183136Z    |
2019-11-07T06:40:00.0183185Z LL |     [(); return |ice| {}];
2019-11-07T06:40:00.0183267Z 
2019-11-07T06:40:00.0183390Z error[E0572]: return statement outside of function body
2019-11-07T06:40:00.0183720Z   --> /checkout/src/test/ui/issues/issue-51714.rs:11:10
2019-11-07T06:40:00.0183772Z    |
2019-11-07T06:40:00.0183772Z    |
2019-11-07T06:40:00.0183840Z LL |     [(); return while let Some(n) = Some(0) {}];
2019-11-07T06:40:00.0183929Z 
2019-11-07T06:40:00.0183977Z error: aborting due to 5 previous errors
2019-11-07T06:40:00.0184026Z 
2019-11-07T06:40:00.0184076Z Some errors have detailed explanations: E0572, E0744.
---
2019-11-07T06:40:00.0185109Z 
2019-11-07T06:40:00.0185158Z + error[E0744]: `match` is not allowed in a `const`
2019-11-07T06:40:00.0185431Z +   --> $DIR/return-match-array-const.rs:2:17
2019-11-07T06:40:00.0185484Z +    |
2019-11-07T06:40:00.0185532Z + LL |     [(); return match 0 { n => n }];
2019-11-07T06:40:00.0185647Z + 
2019-11-07T06:40:00.0185697Z + error[E0744]: `match` is not allowed in a `const`
2019-11-07T06:40:00.0185952Z +   --> $DIR/return-match-array-const.rs:4:17
2019-11-07T06:40:00.0186020Z +    |
2019-11-07T06:40:00.0186020Z +    |
2019-11-07T06:40:00.0186078Z + LL |     [(); return match 0 { 0 => 0 }];
2019-11-07T06:40:00.0186192Z + 
2019-11-07T06:40:00.0186248Z + error[E0744]: `match` is not allowed in a `const`
2019-11-07T06:40:00.0186505Z +   --> $DIR/return-match-array-const.rs:6:17
2019-11-07T06:40:00.0186556Z +    |
2019-11-07T06:40:00.0186556Z +    |
2019-11-07T06:40:00.0186835Z + LL |     [(); return match () { 'a' => 0, _ => 0 }];
2019-11-07T06:40:00.0186939Z + 
2019-11-07T06:40:00.0187007Z 1 error[E0572]: return statement outside of function body
2019-11-07T06:40:00.0187264Z 2   --> $DIR/return-match-array-const.rs:2:10
2019-11-07T06:40:00.0187314Z 3    |
2019-11-07T06:40:00.0187314Z 3    |
2019-11-07T06:40:00.0187343Z 
2019-11-07T06:40:00.0187619Z 16 LL |     [(); return match () { 'a' => 0, _ => 0 }];
2019-11-07T06:40:00.0187723Z 18 
2019-11-07T06:40:00.0187986Z - error: aborting due to 3 previous errors
2019-11-07T06:40:00.0188174Z + error: aborting due to 6 previous errors
2019-11-07T06:40:00.0188219Z 20 
2019-11-07T06:40:00.0188219Z 20 
2019-11-07T06:40:00.0188565Z - For more information about this error, try `rustc --explain E0572`.
2019-11-07T06:40:00.0188627Z + Some errors have detailed explanations: E0572, E0744.
2019-11-07T06:40:00.0188908Z + For more information about an error, try `rustc --explain E0572`.
2019-11-07T06:40:00.0188979Z 22 
2019-11-07T06:40:00.0189008Z 
2019-11-07T06:40:00.0189036Z 
2019-11-07T06:40:00.0189086Z The actual stderr differed from the expected stderr.
2019-11-07T06:40:00.0189466Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/return/return-match-array-const/return-match-array-const.stderr
2019-11-07T06:40:00.0189752Z To update references, rerun the tests and pass the `--bless` flag
2019-11-07T06:40:00.0190059Z To only update this specific test, also pass `--test-args return/return-match-array-const.rs`
2019-11-07T06:40:00.0190165Z error: 1 errors occurred comparing output.
2019-11-07T06:40:00.0190224Z status: exit code: 1
2019-11-07T06:40:00.0190224Z status: exit code: 1
2019-11-07T06:40:00.0191188Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/return/return-match-array-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/return/return-match-array-const" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/return/return-match-array-const/auxiliary" "-A" "unused"
2019-11-07T06:40:00.0191629Z ------------------------------------------
2019-11-07T06:40:00.0191669Z 
2019-11-07T06:40:00.0191940Z ------------------------------------------
2019-11-07T06:40:00.0191993Z stderr:
2019-11-07T06:40:00.0191993Z stderr:
2019-11-07T06:40:00.0192276Z ------------------------------------------
2019-11-07T06:40:00.0192345Z error[E0744]: `match` is not allowed in a `const`
2019-11-07T06:40:00.0192648Z   --> /checkout/src/test/ui/return/return-match-array-const.rs:2:17
2019-11-07T06:40:00.0192733Z    |
2019-11-07T06:40:00.0192795Z LL |     [(); return match 0 { n => n }]; //~ ERROR: return statement outside of function body
2019-11-07T06:40:00.0193141Z 
2019-11-07T06:40:00.0193191Z error[E0744]: `match` is not allowed in a `const`
2019-11-07T06:40:00.0193537Z   --> /checkout/src/test/ui/return/return-match-array-const.rs:4:17
2019-11-07T06:40:00.0193610Z    |
2019-11-07T06:40:00.0193610Z    |
2019-11-07T06:40:00.0193667Z LL |     [(); return match 0 { 0 => 0 }]; //~ ERROR: return statement outside of function body
2019-11-07T06:40:00.0193756Z 
2019-11-07T06:40:00.0193824Z error[E0744]: `match` is not allowed in a `const`
2019-11-07T06:40:00.0194109Z   --> /checkout/src/test/ui/return/return-match-array-const.rs:6:17
2019-11-07T06:40:00.0194174Z    |
2019-11-07T06:40:00.0194174Z    |
2019-11-07T06:40:00.0194518Z LL |     [(); return match () { 'a' => 0, _ => 0 }]; //~ ERROR: return statement outside of function body
2019-11-07T06:40:00.0194615Z 
2019-11-07T06:40:00.0194665Z error[E0572]: return statement outside of function body
2019-11-07T06:40:00.0194965Z   --> /checkout/src/test/ui/return/return-match-array-const.rs:2:10
2019-11-07T06:40:00.0195018Z    |
2019-11-07T06:40:00.0195018Z    |
2019-11-07T06:40:00.0195074Z LL |     [(); return match 0 { n => n }]; //~ ERROR: return statement outside of function body
2019-11-07T06:40:00.0195182Z 
2019-11-07T06:40:00.0195231Z error[E0572]: return statement outside of function body
2019-11-07T06:40:00.0196309Z   --> /checkout/src/test/ui/return/return-match-array-const.rs:4:10
2019-11-07T06:40:00.0196380Z    |
2019-11-07T06:40:00.0196380Z    |
2019-11-07T06:40:00.0196435Z LL |     [(); return match 0 { 0 => 0 }]; //~ ERROR: return statement outside of function body
2019-11-07T06:40:00.0196827Z 
2019-11-07T06:40:00.0196884Z error[E0572]: return statement outside of function body
2019-11-07T06:40:00.0197216Z   --> /checkout/src/test/ui/return/return-match-array-const.rs:6:10
2019-11-07T06:40:00.0197290Z    |
2019-11-07T06:40:00.0197290Z    |
2019-11-07T06:40:00.0197607Z LL |     [(); return match () { 'a' => 0, _ => 0 }]; //~ ERROR: return statement outside of function body
2019-11-07T06:40:00.0197704Z 
2019-11-07T06:40:00.0197768Z error: aborting due to 6 previous errors
2019-11-07T06:40:00.0197800Z 
2019-11-07T06:40:00.0197850Z Some errors have detailed explanations: E0572, E0744.
---
2019-11-07T06:40:00.0198507Z 
2019-11-07T06:40:00.0198804Z ---- [ui] ui/rfc-2497-if-let-chains/disallowed-positions.rs stdout ----
2019-11-07T06:40:00.0198858Z diff of stderr:
2019-11-07T06:40:00.0198989Z 
2019-11-07T06:40:00.0199054Z 513 LL | #![feature(let_chains)] // Avoid inflating `.stderr` with overzealous gates in this test.
2019-11-07T06:40:00.0199177Z 515 
2019-11-07T06:40:00.0199227Z + error[E0744]: `match` is not allowed in a `const`
2019-11-07T06:40:00.0199537Z +   --> $DIR/disallowed-positions.rs:218:17
2019-11-07T06:40:00.0199588Z +    |
2019-11-07T06:40:00.0199588Z +    |
2019-11-07T06:40:00.0199635Z + LL |         true && let 1 = 1
2019-11-07T06:40:00.0199747Z + 
2019-11-07T06:40:00.0199796Z + error[E0744]: `match` is not allowed in a `const`
2019-11-07T06:40:00.0200069Z +   --> $DIR/disallowed-positions.rs:224:17
2019-11-07T06:40:00.0200121Z +    |
2019-11-07T06:40:00.0200121Z +    |
2019-11-07T06:40:00.0200166Z + LL |         true && let 1 = 1
2019-11-07T06:40:00.0200289Z + 
2019-11-07T06:40:00.0200337Z + error[E0744]: `match` is not allowed in a `const`
2019-11-07T06:40:00.0200603Z +   --> $DIR/disallowed-positions.rs:230:17
2019-11-07T06:40:00.0200672Z +    |
2019-11-07T06:40:00.0200672Z +    |
2019-11-07T06:40:00.0200719Z + LL |         true && let 1 = 1
2019-11-07T06:40:00.0200833Z + 
2019-11-07T06:40:00.0200880Z 516 error[E0308]: mismatched types
2019-11-07T06:40:00.0201135Z 517   --> $DIR/disallowed-positions.rs:32:8
2019-11-07T06:40:00.0201187Z 518    |
2019-11-07T06:40:00.0201187Z 518    |
2019-11-07T06:40:00.0201236Z 
2019-11-07T06:40:00.0201283Z 989 LL |         true && let 1 = 1
2019-11-07T06:40:00.0201378Z 991 
2019-11-07T06:40:00.0201656Z - error: aborting due to 109 previous errors
2019-11-07T06:40:00.0201711Z + error: aborting due to 112 previous errors
2019-11-07T06:40:00.0201757Z 993 
---
2019-11-07T06:40:00.0202920Z 
2019-11-07T06:40:00.0202948Z 
2019-11-07T06:40:00.0202998Z The actual stderr differed from the expected stderr.
2019-11-07T06:40:00.0203432Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions/disallowed-positions.stderr
2019-11-07T06:40:00.0203720Z To update references, rerun the tests and pass the `--bless` flag
2019-11-07T06:40:00.0204047Z To only update this specific test, also pass `--test-args rfc-2497-if-let-chains/disallowed-positions.rs`
2019-11-07T06:40:00.0204158Z error: 1 errors occurred comparing output.
2019-11-07T06:40:00.0204207Z status: exit code: 1
2019-11-07T06:40:00.0204207Z status: exit code: 1
2019-11-07T06:40:00.0205334Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions/auxiliary" "-A" "unused"
2019-11-07T06:40:00.0205712Z ------------------------------------------
2019-11-07T06:40:00.0205749Z 
2019-11-07T06:40:00.0205992Z ------------------------------------------
2019-11-07T06:40:00.0206061Z stderr:
2019-11-07T06:40:00.0206061Z stderr:
2019-11-07T06:40:00.0206302Z ------------------------------------------
2019-11-07T06:40:00.0206358Z error: expected one of `,` or `>`, found `&&`
2019-11-07T06:40:00.0208171Z    |
2019-11-07T06:40:00.0208171Z    |
2019-11-07T06:40:00.0208373Z LL |         true && let 1 = 1 //~ ERROR expected one of `,` or `>`, found `&&`
2019-11-07T06:40:00.0208460Z    |              ^^ expected one of `,` or `>` here
2019-11-07T06:40:00.0208548Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0208989Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:32:9
2019-11-07T06:40:00.0209070Z    |
2019-11-07T06:40:00.0209070Z    |
2019-11-07T06:40:00.0209128Z LL |     if &let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0209249Z    |
2019-11-07T06:40:00.0209249Z    |
2019-11-07T06:40:00.0209563Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0209630Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0209749Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0210079Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:35:9
2019-11-07T06:40:00.0210137Z    |
2019-11-07T06:40:00.0210137Z    |
2019-11-07T06:40:00.0210213Z LL |     if !let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0210315Z    |
2019-11-07T06:40:00.0210315Z    |
2019-11-07T06:40:00.0210647Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0210715Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0210805Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0211139Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:36:9
2019-11-07T06:40:00.0211195Z    |
2019-11-07T06:40:00.0211195Z    |
2019-11-07T06:40:00.0211251Z LL |     if *let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0211383Z    |
2019-11-07T06:40:00.0211383Z    |
2019-11-07T06:40:00.0211699Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0211784Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0211873Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0212192Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:38:9
2019-11-07T06:40:00.0212266Z    |
2019-11-07T06:40:00.0212266Z    |
2019-11-07T06:40:00.0212980Z LL |     if -let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0213113Z    |
2019-11-07T06:40:00.0213113Z    |
2019-11-07T06:40:00.0213407Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0213472Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0213712Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0214050Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:46:9
2019-11-07T06:40:00.0214118Z    |
2019-11-07T06:40:00.0214118Z    |
2019-11-07T06:40:00.0214195Z LL |     if (let 0 = 0)? {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0214299Z    |
2019-11-07T06:40:00.0214299Z    |
2019-11-07T06:40:00.0214629Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0214695Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0214804Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0215126Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:50:16
2019-11-07T06:40:00.0215184Z    |
2019-11-07T06:40:00.0215184Z    |
2019-11-07T06:40:00.0215240Z LL |     if true || let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0215375Z    |
2019-11-07T06:40:00.0215375Z    |
2019-11-07T06:40:00.0215688Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0215863Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0215958Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0216328Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:51:17
2019-11-07T06:40:00.0216385Z    |
2019-11-07T06:40:00.0216385Z    |
2019-11-07T06:40:00.0216444Z LL |     if (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0216566Z    |
2019-11-07T06:40:00.0216566Z    |
2019-11-07T06:40:00.0216876Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0216943Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0217049Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0217381Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:52:25
2019-11-07T06:40:00.0217457Z    |
2019-11-07T06:40:00.0217457Z    |
2019-11-07T06:40:00.0217526Z LL |     if true && (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0217651Z    |
2019-11-07T06:40:00.0217651Z    |
2019-11-07T06:40:00.0217963Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0218029Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0218137Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0218455Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:53:25
2019-11-07T06:40:00.0218513Z    |
2019-11-07T06:40:00.0218513Z    |
2019-11-07T06:40:00.0218591Z LL |     if true || (true && let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0218709Z    |
2019-11-07T06:40:00.0218709Z    |
2019-11-07T06:40:00.0219037Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0219160Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0219252Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0219600Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:56:12
2019-11-07T06:40:00.0219657Z    |
2019-11-07T06:40:00.0219657Z    |
2019-11-07T06:40:00.0219714Z LL |     if x = let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0219834Z    |
2019-11-07T06:40:00.0219834Z    |
2019-11-07T06:40:00.0220144Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0220227Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0220427Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0220781Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:59:15
2019-11-07T06:40:00.0220869Z    |
2019-11-07T06:40:00.0220869Z    |
2019-11-07T06:40:00.0220927Z LL |     if true..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0221047Z    |
2019-11-07T06:40:00.0221047Z    |
2019-11-07T06:40:00.0221363Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0221429Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0221537Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0221847Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:61:11
2019-11-07T06:40:00.0221905Z    |
2019-11-07T06:40:00.0221905Z    |
2019-11-07T06:40:00.0221977Z LL |     if ..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0222089Z    |
2019-11-07T06:40:00.0222089Z    |
2019-11-07T06:40:00.0222646Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0222848Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0222965Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0223355Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:63:9
2019-11-07T06:40:00.0223412Z    |
2019-11-07T06:40:00.0223412Z    |
2019-11-07T06:40:00.0223485Z LL |     if (let 0 = 0).. {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0223583Z    |
2019-11-07T06:40:00.0223583Z    |
2019-11-07T06:40:00.0223888Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0223953Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0224039Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0224361Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:67:8
2019-11-07T06:40:00.0224418Z    |
2019-11-07T06:40:00.0224418Z    |
2019-11-07T06:40:00.0224484Z LL |     if let Range { start: _, end: _ } = true..true && false {}
2019-11-07T06:40:00.0224607Z    |
2019-11-07T06:40:00.0224607Z    |
2019-11-07T06:40:00.0224919Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0225006Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0225095Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0225416Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:71:8
2019-11-07T06:40:00.0225489Z    |
2019-11-07T06:40:00.0225489Z    |
2019-11-07T06:40:00.0225542Z LL |     if let Range { start: _, end: _ } = true..true || false {}
2019-11-07T06:40:00.0225679Z    |
2019-11-07T06:40:00.0225679Z    |
2019-11-07T06:40:00.0225987Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0226062Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0226167Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0228048Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:78:8
2019-11-07T06:40:00.0228127Z    |
2019-11-07T06:40:00.0228127Z    |
2019-11-07T06:40:00.0228426Z LL |     if let Range { start: F, end } = F..|| true {}
2019-11-07T06:40:00.0228535Z    |
2019-11-07T06:40:00.0228535Z    |
2019-11-07T06:40:00.0229867Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0230232Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0230330Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0230960Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:86:8
2019-11-07T06:40:00.0231019Z    |
2019-11-07T06:40:00.0231019Z    |
2019-11-07T06:40:00.0231084Z LL |     if let Range { start: true, end } = t..&&false {}
2019-11-07T06:40:00.0231211Z    |
2019-11-07T06:40:00.0231211Z    |
2019-11-07T06:40:00.0231526Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0231609Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0231698Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0232033Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:92:19
2019-11-07T06:40:00.0232091Z    |
2019-11-07T06:40:00.0232149Z LL |     if let true = let true = true {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0232149Z LL |     if let true = let true = true {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0232207Z    |                   ^^^^^^^^^^^^^^^
2019-11-07T06:40:00.0232283Z    |
2019-11-07T06:40:00.0232981Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0233181Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0233298Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0233641Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:96:12
2019-11-07T06:40:00.0233717Z    |
2019-11-07T06:40:00.0233717Z    |
2019-11-07T06:40:00.0233773Z LL |     while &let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0233872Z    |
2019-11-07T06:40:00.0233872Z    |
2019-11-07T06:40:00.0234179Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0234243Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0234348Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0241365Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:99:12
2019-11-07T06:40:00.0241449Z    |
2019-11-07T06:40:00.0241449Z    |
2019-11-07T06:40:00.0241546Z LL |     while !let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0241646Z    |
2019-11-07T06:40:00.0241646Z    |
2019-11-07T06:40:00.0242076Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0242164Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0242251Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0242947Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:100:12
2019-11-07T06:40:00.0243013Z    |
2019-11-07T06:40:00.0243013Z    |
2019-11-07T06:40:00.0243067Z LL |     while *let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0243183Z    |
2019-11-07T06:40:00.0243183Z    |
2019-11-07T06:40:00.0243491Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0243580Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0243668Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0243966Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:102:12
2019-11-07T06:40:00.0244035Z    |
2019-11-07T06:40:00.0244035Z    |
2019-11-07T06:40:00.0244321Z LL |     while -let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0244438Z    |
2019-11-07T06:40:00.0244438Z    |
2019-11-07T06:40:00.0244721Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0244782Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0244882Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0245176Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:110:12
2019-11-07T06:40:00.0245410Z    |
2019-11-07T06:40:00.0245410Z    |
2019-11-07T06:40:00.0245496Z LL |     while (let 0 = 0)? {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0245599Z    |
2019-11-07T06:40:00.0245599Z    |
2019-11-07T06:40:00.0245950Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0246013Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0246100Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0246481Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:114:19
2019-11-07T06:40:00.0246535Z    |
2019-11-07T06:40:00.0246535Z    |
2019-11-07T06:40:00.0246590Z LL |     while true || let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0246708Z    |
2019-11-07T06:40:00.0246708Z    |
2019-11-07T06:40:00.0246990Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0247078Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0247267Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0247595Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:115:20
2019-11-07T06:40:00.0247668Z    |
2019-11-07T06:40:00.0247668Z    |
2019-11-07T06:40:00.0247724Z LL |     while (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0247843Z    |
2019-11-07T06:40:00.0247843Z    |
2019-11-07T06:40:00.0248128Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0248191Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0248291Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0248584Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:116:28
2019-11-07T06:40:00.0248664Z    |
2019-11-07T06:40:00.0248664Z    |
2019-11-07T06:40:00.0248731Z LL |     while true && (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0248835Z    |
2019-11-07T06:40:00.0248835Z    |
2019-11-07T06:40:00.0249199Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0249262Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0249362Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0249657Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:117:28
2019-11-07T06:40:00.0249710Z    |
2019-11-07T06:40:00.0249710Z    |
2019-11-07T06:40:00.0249782Z LL |     while true || (true && let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0249895Z    |
2019-11-07T06:40:00.0249895Z    |
2019-11-07T06:40:00.0250179Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0250268Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0250355Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0250671Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:120:15
2019-11-07T06:40:00.0250724Z    |
2019-11-07T06:40:00.0250724Z    |
2019-11-07T06:40:00.0250778Z LL |     while x = let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0250894Z    |
2019-11-07T06:40:00.0250894Z    |
2019-11-07T06:40:00.0251177Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0251256Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0251339Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0251759Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:123:18
2019-11-07T06:40:00.0251830Z    |
2019-11-07T06:40:00.0251830Z    |
2019-11-07T06:40:00.0251894Z LL |     while true..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0252012Z    |
2019-11-07T06:40:00.0252012Z    |
2019-11-07T06:40:00.0252298Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0252360Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0254363Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0254764Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:125:14
2019-11-07T06:40:00.0254817Z    |
2019-11-07T06:40:00.0254817Z    |
2019-11-07T06:40:00.0254891Z LL |     while ..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0254992Z    |
2019-11-07T06:40:00.0254992Z    |
2019-11-07T06:40:00.0255313Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0255506Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0255600Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0255951Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:127:12
2019-11-07T06:40:00.0256003Z    |
2019-11-07T06:40:00.0256003Z    |
2019-11-07T06:40:00.0256058Z LL |     while (let 0 = 0).. {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0256173Z    |
2019-11-07T06:40:00.0256173Z    |
2019-11-07T06:40:00.0256457Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0256537Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0256620Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0256945Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:131:11
2019-11-07T06:40:00.0256998Z    |
2019-11-07T06:40:00.0256998Z    |
2019-11-07T06:40:00.0257060Z LL |     while let Range { start: _, end: _ } = true..true && false {}
2019-11-07T06:40:00.0257185Z    |
2019-11-07T06:40:00.0257185Z    |
2019-11-07T06:40:00.0257471Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0257534Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0257633Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0257928Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:135:11
2019-11-07T06:40:00.0257996Z    |
2019-11-07T06:40:00.0257996Z    |
2019-11-07T06:40:00.0258048Z LL |     while let Range { start: _, end: _ } = true..true || false {}
2019-11-07T06:40:00.0258161Z    |
2019-11-07T06:40:00.0258161Z    |
2019-11-07T06:40:00.0258465Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0258536Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0258637Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0258934Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:142:11
2019-11-07T06:40:00.0258987Z    |
2019-11-07T06:40:00.0258987Z    |
2019-11-07T06:40:00.0259054Z LL |     while let Range { start: F, end } = F..|| true {}
2019-11-07T06:40:00.0259156Z    |
2019-11-07T06:40:00.0259156Z    |
2019-11-07T06:40:00.0259438Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0259518Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0259602Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0260039Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:150:11
2019-11-07T06:40:00.0260093Z    |
2019-11-07T06:40:00.0260093Z    |
2019-11-07T06:40:00.0260155Z LL |     while let Range { start: true, end } = t..&&false {}
2019-11-07T06:40:00.0260275Z    |
2019-11-07T06:40:00.0260275Z    |
2019-11-07T06:40:00.0260562Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0260639Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0260723Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0261016Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:156:22
2019-11-07T06:40:00.0261085Z    |
2019-11-07T06:40:00.0261142Z LL |     while let true = let true = true {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0261142Z LL |     while let true = let true = true {} //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0261198Z    |                      ^^^^^^^^^^^^^^^
2019-11-07T06:40:00.0261273Z    |
2019-11-07T06:40:00.0261560Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0267808Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0267942Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0268380Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:170:6
2019-11-07T06:40:00.0268453Z    |
2019-11-07T06:40:00.0268453Z    |
2019-11-07T06:40:00.0268508Z LL |     &let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0268606Z    |
2019-11-07T06:40:00.0268606Z    |
2019-11-07T06:40:00.0268914Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0268977Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0269078Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0269389Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:172:6
2019-11-07T06:40:00.0269441Z    |
2019-11-07T06:40:00.0269441Z    |
2019-11-07T06:40:00.0269520Z LL |     !let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0269619Z    |
2019-11-07T06:40:00.0269619Z    |
2019-11-07T06:40:00.0269904Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0269986Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0270071Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0270444Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:173:6
2019-11-07T06:40:00.0270499Z    |
2019-11-07T06:40:00.0270499Z    |
2019-11-07T06:40:00.0270551Z LL |     *let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0270662Z    |
2019-11-07T06:40:00.0270662Z    |
2019-11-07T06:40:00.0270950Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0271040Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0271133Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0271429Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:175:6
2019-11-07T06:40:00.0271499Z    |
2019-11-07T06:40:00.0271499Z    |
2019-11-07T06:40:00.0271777Z LL |     -let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0271969Z    |
2019-11-07T06:40:00.0271969Z    |
2019-11-07T06:40:00.0272261Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0272323Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0272796Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0273154Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:183:6
2019-11-07T06:40:00.0273338Z    |
2019-11-07T06:40:00.0273338Z    |
2019-11-07T06:40:00.0273409Z LL |     (let 0 = 0)?; //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0273513Z    |
2019-11-07T06:40:00.0273513Z    |
2019-11-07T06:40:00.0273854Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0273918Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0274002Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0274319Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:187:13
2019-11-07T06:40:00.0274372Z    |
2019-11-07T06:40:00.0274372Z    |
2019-11-07T06:40:00.0274425Z LL |     true || let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0274541Z    |
2019-11-07T06:40:00.0274541Z    |
2019-11-07T06:40:00.0274826Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0274917Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0275079Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0275662Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:188:14
2019-11-07T06:40:00.0275743Z    |
2019-11-07T06:40:00.0275743Z    |
2019-11-07T06:40:00.0275796Z LL |     (true || let 0 = 0); //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0275946Z    |
2019-11-07T06:40:00.0275946Z    |
2019-11-07T06:40:00.0276244Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0276308Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0276408Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0276702Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:189:22
2019-11-07T06:40:00.0276771Z    |
2019-11-07T06:40:00.0276771Z    |
2019-11-07T06:40:00.0276838Z LL |     true && (true || let 0 = 0); //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0276949Z    |
2019-11-07T06:40:00.0276949Z    |
2019-11-07T06:40:00.0277255Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0277317Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0277417Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0277709Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:192:9
2019-11-07T06:40:00.0277763Z    |
2019-11-07T06:40:00.0277763Z    |
2019-11-07T06:40:00.0277834Z LL |     x = let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0277932Z    |
2019-11-07T06:40:00.0277932Z    |
2019-11-07T06:40:00.0278218Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0278307Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0278399Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0278714Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:194:12
2019-11-07T06:40:00.0278767Z    |
2019-11-07T06:40:00.0278767Z    |
2019-11-07T06:40:00.0278820Z LL |     true..(let 0 = 0); //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0278936Z    |
2019-11-07T06:40:00.0278936Z    |
2019-11-07T06:40:00.0279220Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0279282Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0279382Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0279677Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:195:8
2019-11-07T06:40:00.0279748Z    |
2019-11-07T06:40:00.0279748Z    |
2019-11-07T06:40:00.0279927Z LL |     ..(let 0 = 0); //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0280041Z    |
2019-11-07T06:40:00.0280041Z    |
2019-11-07T06:40:00.0280371Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0280435Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0280537Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0280836Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:196:6
2019-11-07T06:40:00.0280889Z    |
2019-11-07T06:40:00.0280889Z    |
2019-11-07T06:40:00.0280961Z LL |     (let 0 = 0)..; //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0281057Z    |
2019-11-07T06:40:00.0281057Z    |
2019-11-07T06:40:00.0281361Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0281424Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0281516Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0281937Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:198:6
2019-11-07T06:40:00.0281998Z    |
2019-11-07T06:40:00.0281998Z    |
2019-11-07T06:40:00.0282049Z LL |     (let Range { start: _, end: _ } = true..true || false);
2019-11-07T06:40:00.0282167Z    |
2019-11-07T06:40:00.0282167Z    |
2019-11-07T06:40:00.0282779Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0282872Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0282957Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0283262Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:202:6
2019-11-07T06:40:00.0283331Z    |
2019-11-07T06:40:00.0283380Z LL |     (let true = let true = true);
2019-11-07T06:40:00.0283380Z LL |     (let true = let true = true);
2019-11-07T06:40:00.0283444Z    |      ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-11-07T06:40:00.0283508Z    |
2019-11-07T06:40:00.0283808Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0283871Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0283972Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0284270Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:202:17
2019-11-07T06:40:00.0284324Z    |
2019-11-07T06:40:00.0284390Z LL |     (let true = let true = true);
2019-11-07T06:40:00.0284390Z LL |     (let true = let true = true);
2019-11-07T06:40:00.0284441Z    |                 ^^^^^^^^^^^^^^^
2019-11-07T06:40:00.0284487Z    |
2019-11-07T06:40:00.0284789Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0284852Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0284936Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0285259Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:207:6
2019-11-07T06:40:00.0285321Z    |
2019-11-07T06:40:00.0285321Z    |
2019-11-07T06:40:00.0285367Z LL |     &let 0 = 0
2019-11-07T06:40:00.0285476Z    |
2019-11-07T06:40:00.0285476Z    |
2019-11-07T06:40:00.0285761Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0285841Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0285924Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0286218Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:218:17
2019-11-07T06:40:00.0286288Z    |
2019-11-07T06:40:00.0286288Z    |
2019-11-07T06:40:00.0286341Z LL |         true && let 1 = 1 //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0286458Z    |
2019-11-07T06:40:00.0286458Z    |
2019-11-07T06:40:00.0286908Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-07T06:40:00.0286983Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-07T06:40:00.0287087Z error: `let` expressions are not supported here
2019-11-07T06:40:00.0287384Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:224:17
---
2019-11-07T06:40:00.0371891Z 
2019-11-07T06:40:00.0371944Z error[E0019]: constant contains unimplemented expression type
2019-11-07T06:40:00.0372267Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:218:25
2019-11-07T06:40:00.0372336Z    |
2019-11-07T06:40:00.0372392Z LL |         true && let 1 = 1 //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0372493Z 
2019-11-07T06:40:00.0372546Z error[E0019]: constant contains unimplemented expression type
2019-11-07T06:40:00.0373117Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:218:21
2019-11-07T06:40:00.0373200Z    |
2019-11-07T06:40:00.0373200Z    |
2019-11-07T06:40:00.0373273Z LL |         true && let 1 = 1 //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0373370Z 
2019-11-07T06:40:00.0373445Z error[E0019]: constant contains unimplemented expression type
2019-11-07T06:40:00.0373796Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:224:25
2019-11-07T06:40:00.0373851Z    |
2019-11-07T06:40:00.0373851Z    |
2019-11-07T06:40:00.0373917Z LL |         true && let 1 = 1 //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0374003Z 
2019-11-07T06:40:00.0374053Z error[E0019]: constant contains unimplemented expression type
2019-11-07T06:40:00.0374365Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:224:21
2019-11-07T06:40:00.0374420Z    |
2019-11-07T06:40:00.0374420Z    |
2019-11-07T06:40:00.0374472Z LL |         true && let 1 = 1 //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0374572Z 
2019-11-07T06:40:00.0374622Z error[E0019]: constant contains unimplemented expression type
2019-11-07T06:40:00.0375078Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:230:25
2019-11-07T06:40:00.0375157Z    |
2019-11-07T06:40:00.0375157Z    |
2019-11-07T06:40:00.0375210Z LL |         true && let 1 = 1 //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0375306Z 
2019-11-07T06:40:00.0375355Z error[E0019]: constant contains unimplemented expression type
2019-11-07T06:40:00.0375652Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:230:21
2019-11-07T06:40:00.0375704Z    |
2019-11-07T06:40:00.0375704Z    |
2019-11-07T06:40:00.0375769Z LL |         true && let 1 = 1 //~ ERROR `let` expressions are not supported here
2019-11-07T06:40:00.0375852Z 
2019-11-07T06:40:00.0375912Z error: aborting due to 112 previous errors
2019-11-07T06:40:00.0375944Z 
2019-11-07T06:40:00.0375996Z Some errors have detailed explanations: E0019, E0277, E0308, E0600, E0614, E0744.
---
2019-11-07T06:40:00.0378765Z test result: FAILED. 9233 passed; 6 failed; 43 ignored; 0 measured; 0 filtered out
2019-11-07T06:40:00.0378804Z 
2019-11-07T06:40:00.0378832Z 
2019-11-07T06:40:00.0378874Z 
2019-11-07T06:40:00.0380532Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-07T06:40:00.0380803Z 
2019-11-07T06:40:00.0380839Z 
2019-11-07T06:40:00.0381145Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-07T06:40:00.0381224Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-07T06:40:00.0381224Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-07T06:40:00.0381287Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-07T06:40:00.0381340Z Build completed unsuccessfully in 1:07:58
2019-11-07T06:40:00.0381404Z == clock drift check ==
2019-11-07T06:40:00.0381455Z   local time: Thu Nov  7 06:40:00 UTC 2019
2019-11-07T06:40:00.3037816Z   network time: Thu, 07 Nov 2019 06:40:00 GMT
2019-11-07T06:40:00.3040840Z == end clock drift check ==
2019-11-07T06:40:01.5918491Z 
2019-11-07T06:40:01.6036908Z ##[error]Bash exited with code '1'.
2019-11-07T06:40:01.6077014Z ##[section]Starting: Checkout
2019-11-07T06:40:01.6078950Z ==============================================================================
2019-11-07T06:40:01.6079028Z Task         : Get sources
2019-11-07T06:40:01.6079082Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
