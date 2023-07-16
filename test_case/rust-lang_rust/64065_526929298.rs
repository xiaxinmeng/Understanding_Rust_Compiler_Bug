plain
2019-09-01T14:29:00.0939677Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-01T14:29:00.1144721Z ##[command]git config gc.auto 0
2019-09-01T14:29:00.1216140Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-01T14:29:00.1290607Z ##[command]git config --get-all http.proxy
2019-09-01T14:29:00.1448972Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64065/merge:refs/remotes/pull/64065/merge
---
2019-09-01T15:30:04.0144583Z .................................................................................................... 1400/8984
2019-09-01T15:30:10.5815867Z .................................................................................................... 1500/8984
2019-09-01T15:30:16.1723757Z .................................................................................................... 1600/8984
2019-09-01T15:30:29.0162626Z .................................................i...............i.................................. 1700/8984
2019-09-01T15:30:37.1611855Z ..............................................FFFF........F......................................... 1800/8984
2019-09-01T15:30:51.3685788Z ........................................iiiii....................................................... 1900/8984
2019-09-01T15:31:02.3354837Z .................................................................................................... 2100/8984
2019-09-01T15:31:04.8829894Z .................................................................................................... 2200/8984
2019-09-01T15:31:08.9934646Z .................................................................................................... 2300/8984
2019-09-01T15:31:16.5315940Z .................................................................................................... 2400/8984
---
2019-09-01T15:34:14.5066425Z ...........................i...............i........................................................ 4700/8984
2019-09-01T15:34:26.5046393Z .................................................................................................... 4800/8984
2019-09-01T15:34:32.8021730Z .................................................................................................... 4900/8984
2019-09-01T15:34:43.7032944Z .................................................................................................... 5000/8984
2019-09-01T15:34:49.3797966Z ........ii.ii....................................................................................... 5100/8984
2019-09-01T15:35:02.3460097Z .................................................................................................... 5300/8984
2019-09-01T15:35:10.4841907Z .......................................................................i............................ 5400/8984
2019-09-01T15:35:17.5798154Z .................................................................................................... 5500/8984
2019-09-01T15:35:24.2141498Z .................................................................................................... 5600/8984
2019-09-01T15:35:24.2141498Z .................................................................................................... 5600/8984
2019-09-01T15:35:34.6302213Z .................................................................ii...i..ii...........i............. 5700/8984
2019-09-01T15:35:59.7233814Z .................................................................................................... 5900/8984
2019-09-01T15:36:09.2186660Z .................................................................................................... 6000/8984
2019-09-01T15:36:09.2186660Z .................................................................................................... 6000/8984
2019-09-01T15:36:15.9662542Z ...................................................................i..ii............................ 6100/8984
2019-09-01T15:36:44.2739481Z .................................................................................................... 6300/8984
2019-09-01T15:36:46.2900479Z ......................i............................................................................. 6400/8984
2019-09-01T15:36:48.4275434Z ..............................................................................................i..... 6500/8984
2019-09-01T15:36:50.9994259Z .................................................................................................... 6600/8984
---
2019-09-01T15:40:43.6598310Z ---- [ui] ui/derives/derives-span-PartialEq-enum.rs stdout ----
2019-09-01T15:40:43.6598524Z diff of stderr:
2019-09-01T15:40:43.6598561Z 
2019-09-01T15:40:43.6598604Z 6    |
2019-09-01T15:40:43.6598655Z 7    = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
2019-09-01T15:40:43.6598725Z 8 
2019-09-01T15:40:43.6599194Z - error[E0369]: binary operation `!=` cannot be applied to type `Error`
2019-09-01T15:40:43.6600017Z -    |
2019-09-01T15:40:43.6600186Z - LL |      Error
2019-09-01T15:40:43.6600354Z -    |      ^^^^^
2019-09-01T15:40:43.6600529Z -    |
2019-09-01T15:40:43.6600529Z -    |
2019-09-01T15:40:43.6600749Z -    = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
2019-09-01T15:40:43.6601118Z - error: aborting due to 2 previous errors
2019-09-01T15:40:43.6601161Z + error: aborting due to previous error
2019-09-01T15:40:43.6601194Z 18 
2019-09-01T15:40:43.6601406Z 19 For more information about this error, try `rustc --explain E0369`.
2019-09-01T15:40:43.6601406Z 19 For more information about this error, try `rustc --explain E0369`.
2019-09-01T15:40:43.6601582Z 20 
2019-09-01T15:40:43.6601613Z 
2019-09-01T15:40:43.6601634Z 
2019-09-01T15:40:43.6601671Z The actual stderr differed from the expected stderr.
2019-09-01T15:40:43.6602001Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-enum/derives-span-PartialEq-enum.stderr
2019-09-01T15:40:43.6602222Z To update references, rerun the tests and pass the `--bless` flag
2019-09-01T15:40:43.6602487Z To only update this specific test, also pass `--test-args derives/derives-span-PartialEq-enum.rs`
2019-09-01T15:40:43.6602553Z error: 1 errors occurred comparing output.
2019-09-01T15:40:43.6602589Z status: exit code: 1
2019-09-01T15:40:43.6602589Z status: exit code: 1
2019-09-01T15:40:43.6603235Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-PartialEq-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-enum" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-enum/auxiliary" "-A" "unused"
2019-09-01T15:40:43.6603518Z ------------------------------------------
2019-09-01T15:40:43.6603554Z 
2019-09-01T15:40:43.6603744Z ------------------------------------------
2019-09-01T15:40:43.6603797Z stderr:
2019-09-01T15:40:43.6603797Z stderr:
2019-09-01T15:40:43.6603980Z ------------------------------------------
2019-09-01T15:40:43.6604023Z error[E0369]: binary operation `==` cannot be applied to type `Error`
2019-09-01T15:40:43.6604478Z    |
2019-09-01T15:40:43.6604512Z LL |      Error //~ ERROR
2019-09-01T15:40:43.6604565Z    |      ^^^^^
2019-09-01T15:40:43.6604600Z    |
2019-09-01T15:40:43.6604600Z    |
2019-09-01T15:40:43.6604648Z    = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
2019-09-01T15:40:43.6604727Z error: aborting due to previous error
2019-09-01T15:40:43.6604751Z 
2019-09-01T15:40:43.6605136Z For more information about this error, try `rustc --explain E0369`.
2019-09-01T15:40:43.6605164Z 
2019-09-01T15:40:43.6605164Z 
2019-09-01T15:40:43.6605365Z ------------------------------------------
2019-09-01T15:40:43.6605398Z 
2019-09-01T15:40:43.6605419Z 
2019-09-01T15:40:43.6606114Z ---- [ui] ui/derives/derives-span-PartialEq-enum-struct-variant.rs stdout ----
2019-09-01T15:40:43.6606192Z diff of stderr:
2019-09-01T15:40:43.6606221Z 
2019-09-01T15:40:43.6606260Z 6    |
2019-09-01T15:40:43.6606309Z 7    = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
2019-09-01T15:40:43.6606374Z 8 
2019-09-01T15:40:43.6606664Z - error[E0369]: binary operation `!=` cannot be applied to type `Error`
2019-09-01T15:40:43.6607160Z -    |
2019-09-01T15:40:43.6607160Z -    |
2019-09-01T15:40:43.6607376Z - LL |      x: Error
2019-09-01T15:40:43.6607793Z -    |
2019-09-01T15:40:43.6607793Z -    |
2019-09-01T15:40:43.6608094Z -    = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
2019-09-01T15:40:43.6608532Z - error: aborting due to 2 previous errors
2019-09-01T15:40:43.6608599Z + error: aborting due to previous error
2019-09-01T15:40:43.6608763Z 18 
2019-09-01T15:40:43.6609066Z 19 For more information about this error, try `rustc --explain E0369`.
2019-09-01T15:40:43.6609066Z 19 For more information about this error, try `rustc --explain E0369`.
2019-09-01T15:40:43.6609133Z 20 
2019-09-01T15:40:43.6609160Z 
2019-09-01T15:40:43.6609355Z 
2019-09-01T15:40:43.6609392Z The actual stderr differed from the expected stderr.
2019-09-01T15:40:43.6609870Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-enum-struct-variant/derives-span-PartialEq-enum-struct-variant.stderr
2019-09-01T15:40:43.6610157Z To update references, rerun the tests and pass the `--bless` flag
2019-09-01T15:40:43.6610441Z To only update this specific test, also pass `--test-args derives/derives-span-PartialEq-enum-struct-variant.rs`
2019-09-01T15:40:43.6610524Z error: 1 errors occurred comparing output.
2019-09-01T15:40:43.6610558Z status: exit code: 1
2019-09-01T15:40:43.6610558Z status: exit code: 1
2019-09-01T15:40:43.6611203Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-PartialEq-enum-struct-variant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-enum-struct-variant" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-enum-struct-variant/auxiliary" "-A" "unused"
2019-09-01T15:40:43.6611495Z ------------------------------------------
2019-09-01T15:40:43.6611523Z 
2019-09-01T15:40:43.6611704Z ------------------------------------------
2019-09-01T15:40:43.6611755Z stderr:
2019-09-01T15:40:43.6611755Z stderr:
2019-09-01T15:40:43.6611935Z ------------------------------------------
2019-09-01T15:40:43.6611978Z error[E0369]: binary operation `==` cannot be applied to type `Error`
2019-09-01T15:40:43.6612252Z    |
2019-09-01T15:40:43.6612252Z    |
2019-09-01T15:40:43.6612302Z LL |      x: Error //~ ERROR
2019-09-01T15:40:43.6612366Z    |
2019-09-01T15:40:43.6612366Z    |
2019-09-01T15:40:43.6612421Z    = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
2019-09-01T15:40:43.6612480Z error: aborting due to previous error
2019-09-01T15:40:43.6612502Z 
2019-09-01T15:40:43.6612736Z For more information about this error, try `rustc --explain E0369`.
2019-09-01T15:40:43.6612764Z 
2019-09-01T15:40:43.6612764Z 
2019-09-01T15:40:43.6612945Z ------------------------------------------
2019-09-01T15:40:43.6612970Z 
2019-09-01T15:40:43.6612990Z 
2019-09-01T15:40:43.6613202Z ---- [ui] ui/derives/derives-span-PartialEq-struct.rs stdout ----
2019-09-01T15:40:43.6613241Z diff of stderr:
2019-09-01T15:40:43.6613263Z 
2019-09-01T15:40:43.6613309Z 6    |
2019-09-01T15:40:43.6613526Z 7    = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
2019-09-01T15:40:43.6613569Z 8 
2019-09-01T15:40:43.6613779Z - error[E0369]: binary operation `!=` cannot be applied to type `Error`
2019-09-01T15:40:43.6614155Z -    |
2019-09-01T15:40:43.6614155Z -    |
2019-09-01T15:40:43.6614323Z - LL |     x: Error
2019-09-01T15:40:43.6614669Z -    |
2019-09-01T15:40:43.6614669Z -    |
2019-09-01T15:40:43.6614885Z -    = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
2019-09-01T15:40:43.6615260Z - error: aborting due to 2 previous errors
2019-09-01T15:40:43.6615300Z + error: aborting due to previous error
2019-09-01T15:40:43.6615333Z 18 
2019-09-01T15:40:43.6615560Z 19 For more information about this error, try `rustc --explain E0369`.
2019-09-01T15:40:43.6615560Z 19 For more information about this error, try `rustc --explain E0369`.
2019-09-01T15:40:43.6615936Z 20 
2019-09-01T15:40:43.6615967Z 
2019-09-01T15:40:43.6615992Z 
2019-09-01T15:40:43.6616057Z The actual stderr differed from the expected stderr.
2019-09-01T15:40:43.6616576Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-struct/derives-span-PartialEq-struct.stderr
2019-09-01T15:40:43.6616852Z To update references, rerun the tests and pass the `--bless` flag
2019-09-01T15:40:43.6617208Z To only update this specific test, also pass `--test-args derives/derives-span-PartialEq-struct.rs`
2019-09-01T15:40:43.6617297Z error: 1 errors occurred comparing output.
2019-09-01T15:40:43.6617363Z status: exit code: 1
2019-09-01T15:40:43.6617363Z status: exit code: 1
2019-09-01T15:40:43.6618263Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-PartialEq-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-struct" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-struct/auxiliary" "-A" "unused"
2019-09-01T15:40:43.6618690Z ------------------------------------------
2019-09-01T15:40:43.6618728Z 
2019-09-01T15:40:43.6619003Z ------------------------------------------
2019-09-01T15:40:43.6619052Z stderr:
2019-09-01T15:40:43.6619052Z stderr:
2019-09-01T15:40:43.6619419Z ------------------------------------------
2019-09-01T15:40:43.6619487Z error[E0369]: binary operation `==` cannot be applied to type `Error`
2019-09-01T15:40:43.6619747Z    |
2019-09-01T15:40:43.6619747Z    |
2019-09-01T15:40:43.6619798Z LL |     x: Error //~ ERROR
2019-09-01T15:40:43.6619866Z    |
2019-09-01T15:40:43.6619866Z    |
2019-09-01T15:40:43.6619905Z    = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
2019-09-01T15:40:43.6619990Z error: aborting due to previous error
2019-09-01T15:40:43.6620013Z 
2019-09-01T15:40:43.6620227Z For more information about this error, try `rustc --explain E0369`.
2019-09-01T15:40:43.6620339Z 
2019-09-01T15:40:43.6620339Z 
2019-09-01T15:40:43.6621880Z ------------------------------------------
2019-09-01T15:40:43.6621938Z 
2019-09-01T15:40:43.6621959Z 
2019-09-01T15:40:43.6622226Z ---- [ui] ui/derives/derives-span-PartialEq-tuple-struct.rs stdout ----
2019-09-01T15:40:43.6622269Z diff of stderr:
2019-09-01T15:40:43.6622307Z 
2019-09-01T15:40:43.6622340Z 6    |
2019-09-01T15:40:43.6643628Z 7    = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
2019-09-01T15:40:43.6643683Z 8 
2019-09-01T15:40:43.6644288Z - error[E0369]: binary operation `!=` cannot be applied to type `Error`
2019-09-01T15:40:43.6644728Z -    |
2019-09-01T15:40:43.6644905Z - LL |     Error
2019-09-01T15:40:43.6645256Z -    |     ^^^^^
2019-09-01T15:40:43.6645434Z -    |
2019-09-01T15:40:43.6645434Z -    |
2019-09-01T15:40:43.6646168Z -    = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
2019-09-01T15:40:43.6646653Z - error: aborting due to 2 previous errors
2019-09-01T15:40:43.6646705Z + error: aborting due to previous error
2019-09-01T15:40:43.6646747Z 18 
2019-09-01T15:40:43.6647026Z 19 For more information about this error, try `rustc --explain E0369`.
2019-09-01T15:40:43.6647026Z 19 For more information about this error, try `rustc --explain E0369`.
2019-09-01T15:40:43.6647075Z 20 
2019-09-01T15:40:43.6647104Z 
2019-09-01T15:40:43.6647130Z 
2019-09-01T15:40:43.6647196Z The actual stderr differed from the expected stderr.
2019-09-01T15:40:43.6647556Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-tuple-struct/derives-span-PartialEq-tuple-struct.stderr
2019-09-01T15:40:43.6647832Z To update references, rerun the tests and pass the `--bless` flag
2019-09-01T15:40:43.6648159Z To only update this specific test, also pass `--test-args derives/derives-span-PartialEq-tuple-struct.rs`
2019-09-01T15:40:43.6648424Z error: 1 errors occurred comparing output.
2019-09-01T15:40:43.6648484Z status: exit code: 1
2019-09-01T15:40:43.6648484Z status: exit code: 1
2019-09-01T15:40:43.6649723Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-PartialEq-tuple-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-tuple-struct" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-tuple-struct/auxiliary" "-A" "unused"
2019-09-01T15:40:43.6650060Z ------------------------------------------
2019-09-01T15:40:43.6650091Z 
2019-09-01T15:40:43.6650301Z ------------------------------------------
2019-09-01T15:40:43.6650349Z stderr:
2019-09-01T15:40:43.6650349Z stderr:
2019-09-01T15:40:43.6650547Z ------------------------------------------
2019-09-01T15:40:43.6650592Z error[E0369]: binary operation `==` cannot be applied to type `Error`
2019-09-01T15:40:43.6650880Z    |
2019-09-01T15:40:43.6650916Z LL |     Error //~ ERROR
2019-09-01T15:40:43.6651127Z    |     ^^^^^
2019-09-01T15:40:43.6651160Z    |
2019-09-01T15:40:43.6651160Z    |
2019-09-01T15:40:43.6651368Z    = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
2019-09-01T15:40:43.6651443Z error: aborting due to previous error
2019-09-01T15:40:43.6651466Z 
2019-09-01T15:40:43.6651676Z For more information about this error, try `rustc --explain E0369`.
2019-09-01T15:40:43.6651715Z 
2019-09-01T15:40:43.6651715Z 
2019-09-01T15:40:43.6651899Z ------------------------------------------
2019-09-01T15:40:43.6651925Z 
2019-09-01T15:40:43.6651945Z 
2019-09-01T15:40:43.6652165Z ---- [ui] ui/derives/deriving-no-inner-impl-error-message.rs stdout ----
2019-09-01T15:40:43.6652205Z diff of stderr:
2019-09-01T15:40:43.6652228Z 
2019-09-01T15:40:43.6652259Z 6    |
2019-09-01T15:40:43.6652305Z 7    = note: an implementation of `std::cmp::PartialEq` might be missing for `NoCloneOrEq`
2019-09-01T15:40:43.6652518Z 8 
2019-09-01T15:40:43.6652742Z - error[E0369]: binary operation `!=` cannot be applied to type `NoCloneOrEq`
2019-09-01T15:40:43.6653316Z -    |
2019-09-01T15:40:43.6653316Z -    |
2019-09-01T15:40:43.6653508Z - LL |     x: NoCloneOrEq
2019-09-01T15:40:43.6653873Z -    |
2019-09-01T15:40:43.6653873Z -    |
2019-09-01T15:40:43.6654108Z -    = note: an implementation of `std::cmp::PartialEq` might be missing for `NoCloneOrEq`
2019-09-01T15:40:43.6654276Z - 
2019-09-01T15:40:43.6654335Z 17 error[E0277]: the trait bound `NoCloneOrEq: std::clone::Clone` is not satisfied
2019-09-01T15:40:43.6654601Z 19    |
2019-09-01T15:40:43.6654639Z 
2019-09-01T15:40:43.6654671Z 22    |
2019-09-01T15:40:43.6654709Z 23    = note: required by `std::clone::Clone::clone`
---
2019-09-01T15:40:43.6655316Z 28 For more information about an error, try `rustc --explain E0277`.
2019-09-01T15:40:43.6655508Z 
2019-09-01T15:40:43.6655530Z 
2019-09-01T15:40:43.6655734Z The actual stderr differed from the expected stderr.
2019-09-01T15:40:43.6656320Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-no-inner-impl-error-message/deriving-no-inner-impl-error-message.stderr
2019-09-01T15:40:43.6656592Z To update references, rerun the tests and pass the `--bless` flag
2019-09-01T15:40:43.6657071Z To only update this specific test, also pass `--test-args derives/deriving-no-inner-impl-error-message.rs`
2019-09-01T15:40:43.6657154Z error: 1 errors occurred comparing output.
2019-09-01T15:40:43.6657208Z status: exit code: 1
2019-09-01T15:40:43.6657208Z status: exit code: 1
2019-09-01T15:40:43.6658097Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/deriving-no-inner-impl-error-message.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-no-inner-impl-error-message" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-no-inner-impl-error-message/auxiliary" "-A" "unused"
2019-09-01T15:40:43.6658487Z ------------------------------------------
2019-09-01T15:40:43.6658523Z 
2019-09-01T15:40:43.6658765Z ------------------------------------------
2019-09-01T15:40:43.6658812Z stderr:
2019-09-01T15:40:43.6658812Z stderr:
2019-09-01T15:40:43.6659043Z ------------------------------------------
2019-09-01T15:40:43.6659279Z error[E0369]: binary operation `==` cannot be applied to type `NoCloneOrEq`
2019-09-01T15:40:43.6659558Z    |
2019-09-01T15:40:43.6659558Z    |
2019-09-01T15:40:43.6659625Z LL |     x: NoCloneOrEq //~ ERROR binary operation `==` cannot be applied to type `NoCloneOrEq`
2019-09-01T15:40:43.6659702Z    |
2019-09-01T15:40:43.6659702Z    |
2019-09-01T15:40:43.6659742Z    = note: an implementation of `std::cmp::PartialEq` might be missing for `NoCloneOrEq`
2019-09-01T15:40:43.6659782Z 
2019-09-01T15:40:43.6659821Z error[E0277]: the trait bound `NoCloneOrEq: std::clone::Clone` is not satisfied
2019-09-01T15:40:43.6660122Z    |
2019-09-01T15:40:43.6660122Z    |
2019-09-01T15:40:43.6660157Z LL |     x: NoCloneOrEq
2019-09-01T15:40:43.6660231Z    |     ^^^^^^^^^^^^^^ the trait `std::clone::Clone` is not implemented for `NoCloneOrEq`
2019-09-01T15:40:43.6660316Z    = note: required by `std::clone::Clone::clone`
2019-09-01T15:40:43.6660342Z 
2019-09-01T15:40:43.6660377Z error: aborting due to 2 previous errors
2019-09-01T15:40:43.6660410Z 
---
2019-09-01T15:40:43.6694615Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-01T15:40:43.6694713Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-01T15:40:43.6694745Z 
2019-09-01T15:40:43.6700190Z 
2019-09-01T15:40:43.6702605Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-01T15:40:43.6702883Z 
2019-09-01T15:40:43.6702907Z 
2019-09-01T15:40:43.6702958Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-01T15:40:43.6702997Z Build completed unsuccessfully in 1:04:56
2019-09-01T15:40:43.6702997Z Build completed unsuccessfully in 1:04:56
2019-09-01T15:40:43.6737772Z == clock drift check ==
2019-09-01T15:40:43.6753079Z   local time: Sun Sep  1 15:40:43 UTC 2019
2019-09-01T15:40:43.8252013Z   network time: Sun, 01 Sep 2019 15:40:43 GMT
2019-09-01T15:40:43.8253947Z == end clock drift check ==
2019-09-01T15:40:44.6425023Z ##[error]Bash exited with code '1'.
2019-09-01T15:40:44.6463697Z ##[section]Starting: Checkout
2019-09-01T15:40:44.6465154Z ==============================================================================
2019-09-01T15:40:44.6465197Z Task         : Get sources
2019-09-01T15:40:44.6465248Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
