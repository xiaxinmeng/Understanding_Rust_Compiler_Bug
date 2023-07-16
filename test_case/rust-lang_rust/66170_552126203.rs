plain
2019-11-09T17:31:38.1552373Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-09T17:31:38.1787706Z ##[command]git config gc.auto 0
2019-11-09T17:31:38.1877646Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-09T17:31:38.1945285Z ##[command]git config --get-all http.proxy
2019-11-09T17:31:38.2099190Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66170/merge:refs/remotes/pull/66170/merge
---
2019-11-09T18:31:02.5274302Z .................................................................................................... 1500/9220
2019-11-09T18:31:08.5962487Z .................................................................................................... 1600/9220
2019-11-09T18:31:18.0333784Z .................................................................................................i.. 1700/9220
2019-11-09T18:31:26.4423786Z .................................................................................................... 1800/9220
2019-11-09T18:31:33.4727405Z .................................................................................iiiii.............. 1900/9220
2019-11-09T18:31:55.0422061Z .................................................................................................... 2100/9220
2019-11-09T18:31:57.4800830Z .................................................................................................... 2200/9220
2019-11-09T18:32:00.1384098Z .................................................................................................... 2300/9220
2019-11-09T18:32:14.1362096Z .................................................................................................... 2400/9220
---
2019-11-09T18:35:05.8574758Z ...........................................................................i...............i........ 4700/9220
2019-11-09T18:35:13.6565526Z .................................................................................................... 4800/9220
2019-11-09T18:35:22.3861269Z .................................................................................................... 4900/9220
2019-11-09T18:35:27.7166279Z .................................................................................................... 5000/9220
2019-11-09T18:35:39.2714379Z .............................................................................ii.ii...........i...... 5100/9220
2019-11-09T18:35:48.1601551Z ............i....................................................................................... 5300/9220
2019-11-09T18:35:58.7436685Z .................................................................................................... 5400/9220
2019-11-09T18:36:05.3392670Z ...........................................................i........................................ 5500/9220
2019-11-09T18:36:12.6504129Z .................................................................................................... 5600/9220
2019-11-09T18:36:12.6504129Z .................................................................................................... 5600/9220
2019-11-09T18:36:22.6375001Z .................................................................................................... 5700/9220
2019-11-09T18:36:30.0364858Z ............................................ii...i..ii...........i.................................. 5800/9220
2019-11-09T18:36:52.9895717Z .................................................................................................... 6000/9220
2019-11-09T18:37:00.6702918Z .................................................................................................... 6100/9220
2019-11-09T18:37:00.6702918Z .................................................................................................... 6100/9220
2019-11-09T18:37:06.7860610Z ...............................................................i..ii................................ 6200/9220
2019-11-09T18:37:35.4043597Z .................................................................................................... 6400/9220
2019-11-09T18:37:37.5419298Z ...............................i.................................................................... 6500/9220
2019-11-09T18:37:39.8885474Z .................................................................................................... 6600/9220
2019-11-09T18:37:42.3742959Z ...............i.................................................................................... 6700/9220
---
2019-11-09T18:42:34.1416738Z 
2019-11-09T18:42:34.1418301Z ---- [ui] ui/rfc-2497-if-let-chains/disallowed-positions.rs stdout ----
2019-11-09T18:42:34.1418574Z diff of stderr:
2019-11-09T18:42:34.1418714Z 
2019-11-09T18:42:34.1418871Z 1 error: expected one of `,` or `>`, found `&&`
2019-11-09T18:42:34.1419656Z +   --> $DIR/disallowed-positions.rs:239:14
2019-11-09T18:42:34.1419834Z 3    |
2019-11-09T18:42:34.1419834Z 3    |
2019-11-09T18:42:34.1420003Z 4 LL |         true && let 1 = 1
2019-11-09T18:42:34.1420152Z 5    |              ^^ expected one of `,` or `>`
2019-11-09T18:42:34.1420281Z 
2019-11-09T18:42:34.1420707Z 482    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1421446Z 484 error: `let` expressions are not supported here
2019-11-09T18:42:34.1421937Z -   --> $DIR/disallowed-positions.rs:225:17
2019-11-09T18:42:34.1422291Z +   --> $DIR/disallowed-positions.rs:223:17
2019-11-09T18:42:34.1422453Z 486    |
2019-11-09T18:42:34.1422453Z 486    |
2019-11-09T18:42:34.1422604Z 487 LL |         true && let 1 = 1
2019-11-09T18:42:34.1423124Z 
2019-11-09T18:42:34.1423124Z 
2019-11-09T18:42:34.1423292Z 491    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1423740Z 493 error: `let` expressions are not supported here
2019-11-09T18:42:34.1424913Z -   --> $DIR/disallowed-positions.rs:232:17
2019-11-09T18:42:34.1425349Z +   --> $DIR/disallowed-positions.rs:228:17
2019-11-09T18:42:34.1425543Z 495    |
2019-11-09T18:42:34.1425543Z 495    |
2019-11-09T18:42:34.1425674Z 496 LL |         true && let 1 = 1
2019-11-09T18:42:34.1425772Z 
2019-11-09T18:42:34.1425850Z 520    |                 ^^^^^^^^^
2019-11-09T18:42:34.1425885Z 521 
2019-11-09T18:42:34.1425951Z 522 error[E0744]: `match` is not allowed in a `const`
2019-11-09T18:42:34.1425951Z 522 error[E0744]: `match` is not allowed in a `const`
2019-11-09T18:42:34.1426222Z -   --> $DIR/disallowed-positions.rs:225:17
2019-11-09T18:42:34.1426561Z +   --> $DIR/disallowed-positions.rs:223:17
2019-11-09T18:42:34.1426791Z 524    |
2019-11-09T18:42:34.1426948Z 525 LL |         true && let 1 = 1
2019-11-09T18:42:34.1427224Z 
2019-11-09T18:42:34.1427275Z 527 
2019-11-09T18:42:34.1427752Z 528 error[E0744]: `match` is not allowed in a `const`
2019-11-09T18:42:34.1428057Z -   --> $DIR/disallowed-positions.rs:232:17
2019-11-09T18:42:34.1428057Z -   --> $DIR/disallowed-positions.rs:232:17
2019-11-09T18:42:34.1428862Z +   --> $DIR/disallowed-positions.rs:228:17
2019-11-09T18:42:34.1429081Z 530    |
2019-11-09T18:42:34.1429170Z 531 LL |         true && let 1 = 1
2019-11-09T18:42:34.1429281Z 
2019-11-09T18:42:34.1429330Z 971    = help: the trait `std::ops::Try` is not implemented for `{integer}`
2019-11-09T18:42:34.1429407Z 972    = note: required by `std::ops::Try::into_result`
2019-11-09T18:42:34.1429452Z 973 
2019-11-09T18:42:34.1429452Z 973 
2019-11-09T18:42:34.1429744Z - error[E0019]: constant contains unimplemented expression type
2019-11-09T18:42:34.1430185Z -   --> $DIR/disallowed-positions.rs:218:25
2019-11-09T18:42:34.1430541Z -    |
2019-11-09T18:42:34.1430800Z - LL |         true && let 1 = 1
2019-11-09T18:42:34.1431610Z + error: aborting due to 106 previous errors
2019-11-09T18:42:34.1431766Z 979 
2019-11-09T18:42:34.1432040Z - error[E0019]: constant contains unimplemented expression type
2019-11-09T18:42:34.1432405Z -   --> $DIR/disallowed-positions.rs:218:21
2019-11-09T18:42:34.1432405Z -   --> $DIR/disallowed-positions.rs:218:21
2019-11-09T18:42:34.1432719Z -    |
2019-11-09T18:42:34.1432982Z - LL |         true && let 1 = 1
2019-11-09T18:42:34.1433641Z - 
2019-11-09T18:42:34.1433855Z - error[E0019]: constant contains unimplemented expression type
2019-11-09T18:42:34.1434033Z -   --> $DIR/disallowed-positions.rs:225:25
2019-11-09T18:42:34.1434310Z -    |
2019-11-09T18:42:34.1434310Z -    |
2019-11-09T18:42:34.1434543Z - LL |         true && let 1 = 1
2019-11-09T18:42:34.1435314Z - 
2019-11-09T18:42:34.1435680Z - error[E0019]: constant contains unimplemented expression type
2019-11-09T18:42:34.1436014Z -   --> $DIR/disallowed-positions.rs:225:21
2019-11-09T18:42:34.1436310Z -    |
2019-11-09T18:42:34.1436310Z -    |
2019-11-09T18:42:34.1436649Z - LL |         true && let 1 = 1
2019-11-09T18:42:34.1437881Z - 
2019-11-09T18:42:34.1438362Z - error[E0019]: constant contains unimplemented expression type
2019-11-09T18:42:34.1438589Z -   --> $DIR/disallowed-positions.rs:232:25
2019-11-09T18:42:34.1438936Z -    |
2019-11-09T18:42:34.1438936Z -    |
2019-11-09T18:42:34.1439332Z - LL |         true && let 1 = 1
2019-11-09T18:42:34.1440270Z - 
2019-11-09T18:42:34.1440678Z - error[E0019]: constant contains unimplemented expression type
2019-11-09T18:42:34.1441238Z -   --> $DIR/disallowed-positions.rs:232:21
2019-11-09T18:42:34.1441584Z -    |
2019-11-09T18:42:34.1441584Z -    |
2019-11-09T18:42:34.1441927Z - LL |         true && let 1 = 1
2019-11-09T18:42:34.1442601Z - 
2019-11-09T18:42:34.1442951Z - error: aborting due to 112 previous errors
2019-11-09T18:42:34.1443502Z - 
2019-11-09T18:42:34.1443969Z - Some errors have detailed explanations: E0019, E0277, E0308, E0600, E0614, E0744.
---
2019-11-09T18:42:34.1445236Z 
2019-11-09T18:42:34.1445275Z 
2019-11-09T18:42:34.1445459Z The actual stderr differed from the expected stderr.
2019-11-09T18:42:34.1445817Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions/disallowed-positions.stderr
2019-11-09T18:42:34.1446203Z To update references, rerun the tests and pass the `--bless` flag
2019-11-09T18:42:34.1446640Z To only update this specific test, also pass `--test-args rfc-2497-if-let-chains/disallowed-positions.rs`
2019-11-09T18:42:34.1447140Z error: 1 errors occurred comparing output.
2019-11-09T18:42:34.1447202Z status: exit code: 1
2019-11-09T18:42:34.1447202Z status: exit code: 1
2019-11-09T18:42:34.1448776Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions/auxiliary" "-A" "unused"
2019-11-09T18:42:34.1449365Z ------------------------------------------
2019-11-09T18:42:34.1449443Z 
2019-11-09T18:42:34.1449698Z ------------------------------------------
2019-11-09T18:42:34.1449895Z stderr:
2019-11-09T18:42:34.1449895Z stderr:
2019-11-09T18:42:34.1450267Z ------------------------------------------
2019-11-09T18:42:34.1450319Z error: expected one of `,` or `>`, found `&&`
2019-11-09T18:42:34.1451420Z    |
2019-11-09T18:42:34.1451420Z    |
2019-11-09T18:42:34.1451582Z LL |         true && let 1 = 1 //~ ERROR expected one of `,` or `>`, found `&&`
2019-11-09T18:42:34.1451693Z    |              ^^ expected one of `,` or `>`
2019-11-09T18:42:34.1451789Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1452081Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:32:9
2019-11-09T18:42:34.1452243Z    |
2019-11-09T18:42:34.1452243Z    |
2019-11-09T18:42:34.1452294Z LL |     if &let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1452391Z    |
2019-11-09T18:42:34.1452391Z    |
2019-11-09T18:42:34.1452644Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1452711Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1452778Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1453023Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:35:9
2019-11-09T18:42:34.1453236Z    |
2019-11-09T18:42:34.1453236Z    |
2019-11-09T18:42:34.1453491Z LL |     if !let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1453855Z    |
2019-11-09T18:42:34.1453855Z    |
2019-11-09T18:42:34.1454166Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1454367Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1454549Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1454865Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:36:9
2019-11-09T18:42:34.1454910Z    |
2019-11-09T18:42:34.1454910Z    |
2019-11-09T18:42:34.1454951Z LL |     if *let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1455454Z    |
2019-11-09T18:42:34.1455454Z    |
2019-11-09T18:42:34.1455743Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1455934Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1456123Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1456433Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:38:9
2019-11-09T18:42:34.1456477Z    |
2019-11-09T18:42:34.1456477Z    |
2019-11-09T18:42:34.1456695Z LL |     if -let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1457191Z    |
2019-11-09T18:42:34.1457191Z    |
2019-11-09T18:42:34.1457970Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1458198Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1458446Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1458783Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:46:9
2019-11-09T18:42:34.1458966Z    |
2019-11-09T18:42:34.1458966Z    |
2019-11-09T18:42:34.1459068Z LL |     if (let 0 = 0)? {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1459171Z    |
2019-11-09T18:42:34.1459171Z    |
2019-11-09T18:42:34.1459601Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1459802Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1460025Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1460367Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:50:16
2019-11-09T18:42:34.1460418Z    |
2019-11-09T18:42:34.1460418Z    |
2019-11-09T18:42:34.1460626Z LL |     if true || let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1460760Z    |
2019-11-09T18:42:34.1460760Z    |
2019-11-09T18:42:34.1461402Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1461595Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1461792Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1462109Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:51:17
2019-11-09T18:42:34.1462275Z    |
2019-11-09T18:42:34.1462275Z    |
2019-11-09T18:42:34.1462559Z LL |     if (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1462884Z    |
2019-11-09T18:42:34.1462884Z    |
2019-11-09T18:42:34.1463182Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1463362Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1463604Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1463866Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:52:25
2019-11-09T18:42:34.1464031Z    |
2019-11-09T18:42:34.1464031Z    |
2019-11-09T18:42:34.1464185Z LL |     if true && (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1464721Z    |
2019-11-09T18:42:34.1464721Z    |
2019-11-09T18:42:34.1465103Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1465284Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1465473Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1465728Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:53:25
2019-11-09T18:42:34.1465911Z    |
2019-11-09T18:42:34.1465911Z    |
2019-11-09T18:42:34.1466041Z LL |     if true || (true && let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1466175Z    |
2019-11-09T18:42:34.1466175Z    |
2019-11-09T18:42:34.1466445Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1466610Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1466804Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1467342Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:56:12
2019-11-09T18:42:34.1467918Z    |
2019-11-09T18:42:34.1467918Z    |
2019-11-09T18:42:34.1468095Z LL |     if x = let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1468241Z    |
2019-11-09T18:42:34.1468241Z    |
2019-11-09T18:42:34.1468603Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1468662Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1468963Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1469268Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:59:15
2019-11-09T18:42:34.1469478Z    |
2019-11-09T18:42:34.1469478Z    |
2019-11-09T18:42:34.1469644Z LL |     if true..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1469795Z    |
2019-11-09T18:42:34.1469795Z    |
2019-11-09T18:42:34.1470124Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1470343Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1470506Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1470830Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:61:11
2019-11-09T18:42:34.1471031Z    |
2019-11-09T18:42:34.1471031Z    |
2019-11-09T18:42:34.1471365Z LL |     if ..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1471736Z    |
2019-11-09T18:42:34.1471736Z    |
2019-11-09T18:42:34.1472187Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1472375Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1472543Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1472848Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:63:9
2019-11-09T18:42:34.1473012Z    |
2019-11-09T18:42:34.1473012Z    |
2019-11-09T18:42:34.1473198Z LL |     if (let 0 = 0).. {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1473305Z    |
2019-11-09T18:42:34.1473305Z    |
2019-11-09T18:42:34.1473589Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1473810Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1473987Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1474524Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:67:8
2019-11-09T18:42:34.1474568Z    |
2019-11-09T18:42:34.1474568Z    |
2019-11-09T18:42:34.1474742Z LL |     if let Range { start: _, end: _ } = true..true && false {}
2019-11-09T18:42:34.1475015Z    |
2019-11-09T18:42:34.1475015Z    |
2019-11-09T18:42:34.1475307Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1475532Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1475708Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1476212Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:71:8
2019-11-09T18:42:34.1476256Z    |
2019-11-09T18:42:34.1476256Z    |
2019-11-09T18:42:34.1476297Z LL |     if let Range { start: _, end: _ } = true..true || false {}
2019-11-09T18:42:34.1476401Z    |
2019-11-09T18:42:34.1476401Z    |
2019-11-09T18:42:34.1476802Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1477234Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1477354Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1478439Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:78:8
2019-11-09T18:42:34.1478655Z    |
2019-11-09T18:42:34.1478655Z    |
2019-11-09T18:42:34.1478768Z LL |     if let Range { start: F, end } = F..|| true {}
2019-11-09T18:42:34.1478889Z    |
2019-11-09T18:42:34.1478889Z    |
2019-11-09T18:42:34.1479325Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1479590Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1479778Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1480097Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:86:8
2019-11-09T18:42:34.1480290Z    |
2019-11-09T18:42:34.1480290Z    |
2019-11-09T18:42:34.1480435Z LL |     if let Range { start: true, end } = t..&&false {}
2019-11-09T18:42:34.1480593Z    |
2019-11-09T18:42:34.1480593Z    |
2019-11-09T18:42:34.1480928Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1481140Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1481334Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1481800Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:92:19
2019-11-09T18:42:34.1482131Z    |
2019-11-09T18:42:34.1482297Z LL |     if let true = let true = true {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1482297Z LL |     if let true = let true = true {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1482342Z    |                   ^^^^^^^^^^^^^^^
2019-11-09T18:42:34.1482513Z    |
2019-11-09T18:42:34.1482960Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1483139Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1483477Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1483934Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:96:12
2019-11-09T18:42:34.1483976Z    |
2019-11-09T18:42:34.1483976Z    |
2019-11-09T18:42:34.1484164Z LL |     while &let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1484499Z    |
2019-11-09T18:42:34.1484499Z    |
2019-11-09T18:42:34.1484934Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1485149Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1485231Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1485479Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:99:12
2019-11-09T18:42:34.1485520Z    |
2019-11-09T18:42:34.1485520Z    |
2019-11-09T18:42:34.1485574Z LL |     while !let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1485647Z    |
2019-11-09T18:42:34.1485647Z    |
2019-11-09T18:42:34.1485983Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1486048Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1486407Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1486682Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:100:12
2019-11-09T18:42:34.1486849Z    |
2019-11-09T18:42:34.1486849Z    |
2019-11-09T18:42:34.1487179Z LL |     while *let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1487772Z    |
2019-11-09T18:42:34.1487772Z    |
2019-11-09T18:42:34.1488382Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1488607Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1488781Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1489135Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:102:12
2019-11-09T18:42:34.1489333Z    |
2019-11-09T18:42:34.1489333Z    |
2019-11-09T18:42:34.1489756Z LL |     while -let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1490079Z    |
2019-11-09T18:42:34.1490079Z    |
2019-11-09T18:42:34.1490422Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1490615Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1490769Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1491210Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:110:12
2019-11-09T18:42:34.1491250Z    |
2019-11-09T18:42:34.1491250Z    |
2019-11-09T18:42:34.1491590Z LL |     while (let 0 = 0)? {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1491799Z    |
2019-11-09T18:42:34.1491799Z    |
2019-11-09T18:42:34.1492063Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1492248Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1492453Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1492729Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:114:19
2019-11-09T18:42:34.1492771Z    |
2019-11-09T18:42:34.1492771Z    |
2019-11-09T18:42:34.1492955Z LL |     while true || let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1493122Z    |
2019-11-09T18:42:34.1493122Z    |
2019-11-09T18:42:34.1493414Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1493578Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1493776Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1494048Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:115:20
2019-11-09T18:42:34.1494224Z    |
2019-11-09T18:42:34.1494224Z    |
2019-11-09T18:42:34.1494365Z LL |     while (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1494598Z    |
2019-11-09T18:42:34.1494598Z    |
2019-11-09T18:42:34.1494867Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1495065Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1495259Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1495535Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:116:28
2019-11-09T18:42:34.1495712Z    |
2019-11-09T18:42:34.1495712Z    |
2019-11-09T18:42:34.1495841Z LL |     while true && (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1495956Z    |
2019-11-09T18:42:34.1495956Z    |
2019-11-09T18:42:34.1496463Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1496633Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1496768Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1497279Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:117:28
2019-11-09T18:42:34.1497336Z    |
2019-11-09T18:42:34.1497336Z    |
2019-11-09T18:42:34.1497892Z LL |     while true || (true && let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1498155Z    |
2019-11-09T18:42:34.1498155Z    |
2019-11-09T18:42:34.1498515Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1498716Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1498908Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1499250Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:120:15
2019-11-09T18:42:34.1499453Z    |
2019-11-09T18:42:34.1499453Z    |
2019-11-09T18:42:34.1499644Z LL |     while x = let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1499790Z    |
2019-11-09T18:42:34.1499790Z    |
2019-11-09T18:42:34.1500114Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1500322Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1500480Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1500804Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:123:18
2019-11-09T18:42:34.1501344Z    |
2019-11-09T18:42:34.1501344Z    |
2019-11-09T18:42:34.1502013Z LL |     while true..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1502216Z    |
2019-11-09T18:42:34.1502216Z    |
2019-11-09T18:42:34.1502549Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1502614Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1502927Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1503236Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:125:14
2019-11-09T18:42:34.1503279Z    |
2019-11-09T18:42:34.1503279Z    |
2019-11-09T18:42:34.1503479Z LL |     while ..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1503576Z    |
2019-11-09T18:42:34.1503576Z    |
2019-11-09T18:42:34.1503824Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1503886Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1503949Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1504178Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:127:12
2019-11-09T18:42:34.1504234Z    |
2019-11-09T18:42:34.1504234Z    |
2019-11-09T18:42:34.1504398Z LL |     while (let 0 = 0).. {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1504855Z    |
2019-11-09T18:42:34.1504855Z    |
2019-11-09T18:42:34.1505295Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1505479Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1505655Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1505947Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:131:11
2019-11-09T18:42:34.1505989Z    |
2019-11-09T18:42:34.1505989Z    |
2019-11-09T18:42:34.1506152Z LL |     while let Range { start: _, end: _ } = true..true && false {}
2019-11-09T18:42:34.1506600Z    |
2019-11-09T18:42:34.1506600Z    |
2019-11-09T18:42:34.1506909Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1506973Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1507226Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1508131Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:135:11
2019-11-09T18:42:34.1508346Z    |
2019-11-09T18:42:34.1508346Z    |
2019-11-09T18:42:34.1508462Z LL |     while let Range { start: _, end: _ } = true..true || false {}
2019-11-09T18:42:34.1508576Z    |
2019-11-09T18:42:34.1508576Z    |
2019-11-09T18:42:34.1509008Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1509218Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1509360Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1509727Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:142:11
2019-11-09T18:42:34.1509910Z    |
2019-11-09T18:42:34.1509910Z    |
2019-11-09T18:42:34.1510065Z LL |     while let Range { start: F, end } = F..|| true {}
2019-11-09T18:42:34.1510237Z    |
2019-11-09T18:42:34.1510237Z    |
2019-11-09T18:42:34.1510558Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1510771Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1510901Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1511360Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:150:11
2019-11-09T18:42:34.1511510Z    |
2019-11-09T18:42:34.1511510Z    |
2019-11-09T18:42:34.1511602Z LL |     while let Range { start: true, end } = t..&&false {}
2019-11-09T18:42:34.1511809Z    |
2019-11-09T18:42:34.1511809Z    |
2019-11-09T18:42:34.1512211Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1512382Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1512582Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1512837Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:156:22
2019-11-09T18:42:34.1512876Z    |
2019-11-09T18:42:34.1513066Z LL |     while let true = let true = true {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1513066Z LL |     while let true = let true = true {} //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1513177Z    |                      ^^^^^^^^^^^^^^^
2019-11-09T18:42:34.1513231Z    |
2019-11-09T18:42:34.1513515Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1513679Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1513876Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1514163Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:170:6
2019-11-09T18:42:34.1514204Z    |
2019-11-09T18:42:34.1514204Z    |
2019-11-09T18:42:34.1514382Z LL |     &let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1514900Z    |
2019-11-09T18:42:34.1514900Z    |
2019-11-09T18:42:34.1515277Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1515454Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1515645Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1515917Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:172:6
2019-11-09T18:42:34.1515958Z    |
2019-11-09T18:42:34.1515958Z    |
2019-11-09T18:42:34.1516134Z LL |     !let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1516295Z    |
2019-11-09T18:42:34.1516295Z    |
2019-11-09T18:42:34.1516585Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1516742Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1517243Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1518873Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:173:6
2019-11-09T18:42:34.1518936Z    |
2019-11-09T18:42:34.1518936Z    |
2019-11-09T18:42:34.1519005Z LL |     *let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1519306Z    |
2019-11-09T18:42:34.1519306Z    |
2019-11-09T18:42:34.1519731Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1519936Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1520137Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1520475Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:175:6
2019-11-09T18:42:34.1520682Z    |
2019-11-09T18:42:34.1520682Z    |
2019-11-09T18:42:34.1521214Z LL |     -let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1521756Z    |
2019-11-09T18:42:34.1521756Z    |
2019-11-09T18:42:34.1522006Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1522183Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1522380Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1522669Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:183:6
2019-11-09T18:42:34.1522725Z    |
2019-11-09T18:42:34.1522725Z    |
2019-11-09T18:42:34.1522764Z LL |     (let 0 = 0)?; //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1522835Z    |
2019-11-09T18:42:34.1522835Z    |
2019-11-09T18:42:34.1523057Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1523254Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1523519Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1523803Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:187:13
2019-11-09T18:42:34.1523983Z    |
2019-11-09T18:42:34.1523983Z    |
2019-11-09T18:42:34.1524110Z LL |     true || let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1524235Z    |
2019-11-09T18:42:34.1524235Z    |
2019-11-09T18:42:34.1524485Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1524532Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1524610Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1524989Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:188:14
2019-11-09T18:42:34.1525161Z    |
2019-11-09T18:42:34.1525161Z    |
2019-11-09T18:42:34.1525269Z LL |     (true || let 0 = 0); //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1525400Z    |
2019-11-09T18:42:34.1525400Z    |
2019-11-09T18:42:34.1525785Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1525985Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1526214Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1526490Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:189:22
2019-11-09T18:42:34.1526546Z    |
2019-11-09T18:42:34.1526546Z    |
2019-11-09T18:42:34.1526587Z LL |     true && (true || let 0 = 0); //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1526662Z    |
2019-11-09T18:42:34.1526662Z    |
2019-11-09T18:42:34.1526883Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1527215Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1527296Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1528071Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:192:9
2019-11-09T18:42:34.1528124Z    |
2019-11-09T18:42:34.1528124Z    |
2019-11-09T18:42:34.1528394Z LL |     x = let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1528622Z    |
2019-11-09T18:42:34.1528622Z    |
2019-11-09T18:42:34.1528970Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1529169Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1529318Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1529637Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:194:12
2019-11-09T18:42:34.1529812Z    |
2019-11-09T18:42:34.1529812Z    |
2019-11-09T18:42:34.1529922Z LL |     true..(let 0 = 0); //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1530069Z    |
2019-11-09T18:42:34.1530069Z    |
2019-11-09T18:42:34.1530392Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1530449Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1530711Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1531195Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:195:8
2019-11-09T18:42:34.1531257Z    |
2019-11-09T18:42:34.1531257Z    |
2019-11-09T18:42:34.1531303Z LL |     ..(let 0 = 0); //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1531384Z    |
2019-11-09T18:42:34.1531384Z    |
2019-11-09T18:42:34.1531636Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1531833Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1532030Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1532529Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:196:6
2019-11-09T18:42:34.1532861Z    |
2019-11-09T18:42:34.1532861Z    |
2019-11-09T18:42:34.1533128Z LL |     (let 0 = 0)..; //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1533236Z    |
2019-11-09T18:42:34.1533236Z    |
2019-11-09T18:42:34.1533499Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1533547Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1533624Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1534002Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:198:6
2019-11-09T18:42:34.1534162Z    |
2019-11-09T18:42:34.1534162Z    |
2019-11-09T18:42:34.1534453Z LL |     (let Range { start: _, end: _ } = true..true || false);
2019-11-09T18:42:34.1534562Z    |
2019-11-09T18:42:34.1534562Z    |
2019-11-09T18:42:34.1534853Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1535327Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1535480Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1535780Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:202:6
2019-11-09T18:42:34.1535950Z    |
2019-11-09T18:42:34.1536097Z LL |     (let true = let true = true);
2019-11-09T18:42:34.1536097Z LL |     (let true = let true = true);
2019-11-09T18:42:34.1536160Z    |      ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-11-09T18:42:34.1536196Z    |
2019-11-09T18:42:34.1536500Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1536548Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1536740Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1537915Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:202:17
2019-11-09T18:42:34.1538127Z    |
2019-11-09T18:42:34.1538317Z LL |     (let true = let true = true);
2019-11-09T18:42:34.1538317Z LL |     (let true = let true = true);
2019-11-09T18:42:34.1538365Z    |                 ^^^^^^^^^^^^^^^
2019-11-09T18:42:34.1538437Z    |
2019-11-09T18:42:34.1538807Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1538865Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1539209Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1539547Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:207:6
2019-11-09T18:42:34.1539596Z    |
2019-11-09T18:42:34.1539596Z    |
2019-11-09T18:42:34.1539792Z LL |     &let 0 = 0
2019-11-09T18:42:34.1539997Z    |
2019-11-09T18:42:34.1539997Z    |
2019-11-09T18:42:34.1540352Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1540543Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1540778Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1541628Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:218:17
2019-11-09T18:42:34.1541673Z    |
2019-11-09T18:42:34.1541673Z    |
2019-11-09T18:42:34.1541842Z LL |         true && let 1 = 1 //~ ERROR `let` expressions are not supported here
2019-11-09T18:42:34.1572094Z    |
2019-11-09T18:42:34.1572094Z    |
2019-11-09T18:42:34.1572608Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-11-09T18:42:34.1572797Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-11-09T18:42:34.1573061Z error: `let` expressions are not supported here
2019-11-09T18:42:34.1573416Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:223:17
---
2019-11-09T18:42:34.1667314Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-09T18:42:34.1668045Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-09T18:42:34.1668123Z 
2019-11-09T18:42:34.1668186Z 
2019-11-09T18:42:34.1669870Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-09T18:42:34.1670358Z 
2019-11-09T18:42:34.1670440Z 
2019-11-09T18:42:34.1670489Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-09T18:42:34.1670568Z Build completed unsuccessfully in 1:04:29
2019-11-09T18:42:34.1670568Z Build completed unsuccessfully in 1:04:29
2019-11-09T18:42:34.1670655Z == clock drift check ==
2019-11-09T18:42:34.1670700Z   local time: Sat Nov  9 18:42:33 UTC 2019
2019-11-09T18:42:34.1670774Z   network time: Sat, 09 Nov 2019 18:42:33 GMT
2019-11-09T18:42:34.1670859Z == end clock drift check ==
2019-11-09T18:42:34.3761745Z 
2019-11-09T18:42:34.3898613Z ##[error]Bash exited with code '1'.
2019-11-09T18:42:34.3943922Z ##[section]Starting: Checkout
2019-11-09T18:42:34.3946389Z ==============================================================================
2019-11-09T18:42:34.3946440Z Task         : Get sources
2019-11-09T18:42:34.3946499Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
