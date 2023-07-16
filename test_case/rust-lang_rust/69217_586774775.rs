plain
2020-02-16T23:22:22.1739070Z ========================== Starting Command Output ===========================
2020-02-16T23:22:22.1759154Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8005cdba-bbec-4b36-84c8-977ffbb92b4b.sh
2020-02-16T23:22:23.1653741Z 
2020-02-16T23:22:23.1716308Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-16T23:22:23.1722923Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69217/merge to s
2020-02-16T23:22:23.1724554Z Task         : Get sources
2020-02-16T23:22:23.1724625Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T23:22:23.1724654Z Version      : 1.0.0
2020-02-16T23:22:23.1724682Z Author       : Microsoft
---
2020-02-16T23:22:29.0822700Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-16T23:22:29.1311807Z ##[command]git config gc.auto 0
2020-02-16T23:22:29.1390901Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-16T23:22:29.1438402Z ##[command]git config --get-all http.proxy
2020-02-16T23:22:29.1589064Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69217/merge:refs/remotes/pull/69217/merge
---
2020-02-17T00:22:38.1850894Z .................................................................................................... 1700/9650
2020-02-17T00:22:43.1335854Z .................................................................................................... 1800/9650
2020-02-17T00:22:55.2804145Z ..................................i................................................................. 1900/9650
2020-02-17T00:23:02.9826354Z ...................................................................................F................ 2000/9650
2020-02-17T00:23:17.6872254Z ........................iiiii....................................................................... 2100/9650
2020-02-17T00:23:27.6104553Z ............................F....................................................................... 2300/9650
2020-02-17T00:23:30.0993394Z .................................................................................................... 2400/9650
2020-02-17T00:23:34.7700891Z .................................................F.................................................. 2500/9650
2020-02-17T00:23:55.7433112Z .................................................................................................... 2600/9650
---
2020-02-17T00:27:14.4581820Z ....................................i............................................................... 5500/9650
2020-02-17T00:27:20.3110361Z .................................................................................................... 5600/9650
2020-02-17T00:27:30.9432042Z .......................................................................................i............ 5700/9650
2020-02-17T00:27:38.9370872Z .................................................................................................... 5800/9650
2020-02-17T00:27:44.2624786Z ...................................................................................F.i.............. 5900/9650
2020-02-17T00:27:54.5908389Z ...............................................................................ii...i..ii........... 6000/9650
2020-02-17T00:28:07.3602276Z i................................................................................................... 6100/9650
2020-02-17T00:28:24.0624770Z .................................................................................................... 6300/9650
2020-02-17T00:28:32.0092870Z .................................................................................................... 6400/9650
2020-02-17T00:28:32.0092870Z .................................................................................................... 6400/9650
2020-02-17T00:28:48.5389314Z .......i..ii........................................................................................ 6500/9650
2020-02-17T00:29:08.7587573Z ...............................................................................................i.... 6700/9650
2020-02-17T00:29:11.0232791Z .................................................................................................... 6800/9650
2020-02-17T00:29:13.2647131Z .................................................................................................... 6900/9650
2020-02-17T00:29:15.6578572Z .....i.............................................................................................. 7000/9650
---
2020-02-17T00:30:50.8886834Z .................................................................................................... 7600/9650
2020-02-17T00:30:55.5705548Z .................................................................................................... 7700/9650
2020-02-17T00:31:01.5850572Z .................................................................................................... 7800/9650
2020-02-17T00:31:08.2552435Z .................................F.................................................................. 7900/9650
2020-02-17T00:31:18.0995684Z .......................................................................................iiiiiii.i.... 8000/9650
2020-02-17T00:31:34.8541546Z ...........................i......i................................................................. 8200/9650
2020-02-17T00:31:39.9524978Z .................................................................................................... 8300/9650
2020-02-17T00:31:51.6784463Z .................................................................................................... 8400/9650
2020-02-17T00:32:03.4898465Z .................................................................................................... 8500/9650
---
2020-02-17T00:34:02.8296820Z diff of stderr:
2020-02-17T00:34:02.8296945Z 
2020-02-17T00:34:02.8297488Z 16    |     ------^^^^^^^^^^
2020-02-17T00:34:02.8297903Z 17    |     |
2020-02-17T00:34:02.8298063Z 18    |     cannot use `+=` on type `({integer}, {integer})`
2020-02-17T00:34:02.8298644Z -    |
2020-02-17T00:34:02.8299077Z -    = note: an implementation of `std::ops::AddAssign` might be missing for `({integer}, {integer})`
2020-02-17T00:34:02.8299254Z 21 
2020-02-17T00:34:02.8299594Z 22 error[E0067]: invalid left-hand side of assignment
2020-02-17T00:34:02.8300454Z 
2020-02-17T00:34:02.8300931Z 48    |     ------^^^^^^^^^^
2020-02-17T00:34:02.8301330Z 49    |     |
2020-02-17T00:34:02.8301330Z 49    |     |
2020-02-17T00:34:02.8301639Z 50    |     cannot use `+=` on type `[{integer}; 2]`
2020-02-17T00:34:02.8302026Z -    |
2020-02-17T00:34:02.8302468Z -    = note: an implementation of `std::ops::AddAssign` might be missing for `[{integer}; 2]`
2020-02-17T00:34:02.8302675Z 53 
2020-02-17T00:34:02.8303087Z 54 error[E0067]: invalid left-hand side of assignment
2020-02-17T00:34:02.8303709Z 
2020-02-17T00:34:02.8303841Z 
2020-02-17T00:34:02.8304034Z The actual stderr differed from the expected stderr.
2020-02-17T00:34:02.8304839Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/note-unsupported/note-unsupported.stderr
2020-02-17T00:34:02.8304839Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/note-unsupported/note-unsupported.stderr
2020-02-17T00:34:02.8305220Z To update references, rerun the tests and pass the `--bless` flag
2020-02-17T00:34:02.8305632Z To only update this specific test, also pass `--test-args destructuring-assignment/note-unsupported.rs`
2020-02-17T00:34:02.8305974Z error: 1 errors occurred comparing output.
2020-02-17T00:34:02.8306110Z status: exit code: 1
2020-02-17T00:34:02.8306110Z status: exit code: 1
2020-02-17T00:34:02.8306986Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/destructuring-assignment/note-unsupported.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/note-unsupported" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/note-unsupported/auxiliary"
2020-02-17T00:34:02.8307548Z ------------------------------------------
2020-02-17T00:34:02.8307710Z 
2020-02-17T00:34:02.8308020Z ------------------------------------------
2020-02-17T00:34:02.8308174Z stderr:
2020-02-17T00:34:02.8308174Z stderr:
2020-02-17T00:34:02.8308528Z ------------------------------------------
2020-02-17T00:34:02.8308894Z error[E0070]: invalid left-hand side of assignment
2020-02-17T00:34:02.8309475Z    |
2020-02-17T00:34:02.8309475Z    |
2020-02-17T00:34:02.8309819Z LL |     (a, b) = (3, 4); //~ ERROR invalid left-hand side of assignment
2020-02-17T00:34:02.8310333Z    |     |
2020-02-17T00:34:02.8310479Z    |     cannot assign to this expression
2020-02-17T00:34:02.8310602Z    |
2020-02-17T00:34:02.8310900Z    = note: destructuring assignments are not currently supported
2020-02-17T00:34:02.8310900Z    = note: destructuring assignments are not currently supported
2020-02-17T00:34:02.8311334Z    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2020-02-17T00:34:02.8311706Z 
2020-02-17T00:34:02.8311872Z error[E0368]: binary assignment operation `+=` cannot be applied to type `({integer}, {integer})`
2020-02-17T00:34:02.8312639Z    |
2020-02-17T00:34:02.8312639Z    |
2020-02-17T00:34:02.8313060Z LL |     (a, b) += (3, 4); //~ ERROR invalid left-hand side of assignment
2020-02-17T00:34:02.8313679Z    |     |
2020-02-17T00:34:02.8313679Z    |     |
2020-02-17T00:34:02.8313840Z    |     cannot use `+=` on type `({integer}, {integer})`
2020-02-17T00:34:02.8313975Z 
2020-02-17T00:34:02.8314701Z error[E0067]: invalid left-hand side of assignment
2020-02-17T00:34:02.8315975Z    |
2020-02-17T00:34:02.8315975Z    |
2020-02-17T00:34:02.8316297Z LL |     (a, b) += (3, 4); //~ ERROR invalid left-hand side of assignment
2020-02-17T00:34:02.8316791Z    |     |
2020-02-17T00:34:02.8317094Z    |     cannot assign to this expression
2020-02-17T00:34:02.8317218Z    |
2020-02-17T00:34:02.8317345Z    = note: destructuring assignments are not currently supported
2020-02-17T00:34:02.8317345Z    = note: destructuring assignments are not currently supported
2020-02-17T00:34:02.8318154Z    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2020-02-17T00:34:02.8318363Z 
2020-02-17T00:34:02.8318803Z error[E0070]: invalid left-hand side of assignment
2020-02-17T00:34:02.8319567Z    |
2020-02-17T00:34:02.8319567Z    |
2020-02-17T00:34:02.8321904Z LL |     [a, b] = [3, 4]; //~ ERROR invalid left-hand side of assignment
2020-02-17T00:34:02.8322773Z    |     |
2020-02-17T00:34:02.8322911Z    |     cannot assign to this expression
2020-02-17T00:34:02.8323055Z    |
2020-02-17T00:34:02.8323191Z    = note: destructuring assignments are not currently supported
2020-02-17T00:34:02.8323191Z    = note: destructuring assignments are not currently supported
2020-02-17T00:34:02.8323568Z    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2020-02-17T00:34:02.8323749Z 
2020-02-17T00:34:02.8324228Z error[E0368]: binary assignment operation `+=` cannot be applied to type `[{integer}; 2]`
2020-02-17T00:34:02.8324758Z    |
2020-02-17T00:34:02.8324758Z    |
2020-02-17T00:34:02.8325114Z LL |     [a, b] += [3, 4]; //~ ERROR invalid left-hand side of assignment
2020-02-17T00:34:02.8325604Z    |     |
2020-02-17T00:34:02.8325604Z    |     |
2020-02-17T00:34:02.8325733Z    |     cannot use `+=` on type `[{integer}; 2]`
2020-02-17T00:34:02.8325841Z 
2020-02-17T00:34:02.8326152Z error[E0067]: invalid left-hand side of assignment
2020-02-17T00:34:02.8327480Z    |
2020-02-17T00:34:02.8327480Z    |
2020-02-17T00:34:02.8328149Z LL |     [a, b] += [3, 4]; //~ ERROR invalid left-hand side of assignment
2020-02-17T00:34:02.8328668Z    |     |
2020-02-17T00:34:02.8328799Z    |     cannot assign to this expression
2020-02-17T00:34:02.8329219Z    |
2020-02-17T00:34:02.8330063Z    = note: destructuring assignments are not currently supported
2020-02-17T00:34:02.8330063Z    = note: destructuring assignments are not currently supported
2020-02-17T00:34:02.8339584Z    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2020-02-17T00:34:02.8339839Z 
2020-02-17T00:34:02.8340262Z error[E0070]: invalid left-hand side of assignment
2020-02-17T00:34:02.8341192Z    |
2020-02-17T00:34:02.8341192Z    |
2020-02-17T00:34:02.8341600Z LL |     S { x: a, y: b } = s; //~ ERROR invalid left-hand side of assignment
2020-02-17T00:34:02.8342373Z    |     |
2020-02-17T00:34:02.8342522Z    |     cannot assign to this expression
2020-02-17T00:34:02.8342681Z    |
2020-02-17T00:34:02.8342834Z    = note: destructuring assignments are not currently supported
2020-02-17T00:34:02.8342834Z    = note: destructuring assignments are not currently supported
2020-02-17T00:34:02.8343244Z    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2020-02-17T00:34:02.8343426Z 
2020-02-17T00:34:02.8343591Z error[E0368]: binary assignment operation `+=` cannot be applied to type `S`
2020-02-17T00:34:02.8344696Z    |
2020-02-17T00:34:02.8344696Z    |
2020-02-17T00:34:02.8345061Z LL |     S { x: a, y: b } += s; //~ ERROR invalid left-hand side of assignment
2020-02-17T00:34:02.8345580Z    |     |
2020-02-17T00:34:02.8345580Z    |     |
2020-02-17T00:34:02.8345712Z    |     cannot use `+=` on type `S`
2020-02-17T00:34:02.8345986Z    = note: an implementation of `std::ops::AddAssign` might be missing for `S`
2020-02-17T00:34:02.8346323Z 
2020-02-17T00:34:02.8346323Z 
2020-02-17T00:34:02.8346691Z error[E0067]: invalid left-hand side of assignment
2020-02-17T00:34:02.8347285Z    |
2020-02-17T00:34:02.8347285Z    |
2020-02-17T00:34:02.8347936Z LL |     S { x: a, y: b } += s; //~ ERROR invalid left-hand side of assignment
2020-02-17T00:34:02.8348471Z    |     |
2020-02-17T00:34:02.8348726Z    |     cannot assign to this expression
2020-02-17T00:34:02.8348895Z    |
2020-02-17T00:34:02.8349050Z    = note: destructuring assignments are not currently supported
2020-02-17T00:34:02.8349050Z    = note: destructuring assignments are not currently supported
2020-02-17T00:34:02.8349438Z    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2020-02-17T00:34:02.8349599Z 
2020-02-17T00:34:02.8349974Z error[E0070]: invalid left-hand side of assignment
2020-02-17T00:34:02.8351242Z    |
2020-02-17T00:34:02.8351242Z    |
2020-02-17T00:34:02.8351712Z LL |     S { x: a, ..s } = S { x: 3, y: 4 }; //~ ERROR invalid left-hand side of assignment
2020-02-17T00:34:02.8352311Z    |     |
2020-02-17T00:34:02.8352479Z    |     cannot assign to this expression
2020-02-17T00:34:02.8352625Z    |
2020-02-17T00:34:02.8352771Z    = note: destructuring assignments are not currently supported
2020-02-17T00:34:02.8352771Z    = note: destructuring assignments are not currently supported
2020-02-17T00:34:02.8353187Z    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2020-02-17T00:34:02.8353523Z 
2020-02-17T00:34:02.8353928Z error[E0070]: invalid left-hand side of assignment
2020-02-17T00:34:02.8359835Z    |
2020-02-17T00:34:02.8359835Z    |
2020-02-17T00:34:02.8360115Z LL |     ((a, b), c) = ((3, 4), 5); //~ ERROR invalid left-hand side of assignment
2020-02-17T00:34:02.8360369Z    |     |
2020-02-17T00:34:02.8360409Z    |     cannot assign to this expression
2020-02-17T00:34:02.8360448Z    |
2020-02-17T00:34:02.8360507Z    = note: destructuring assignments are not currently supported
---
2020-02-17T00:34:02.8362395Z diff of stderr:
2020-02-17T00:34:02.8362423Z 
2020-02-17T00:34:02.8362635Z 5    |     -----------------^^^^^
2020-02-17T00:34:02.8362692Z 6    |     |
2020-02-17T00:34:02.8362757Z 7    |     cannot use `+=` on type `std::collections::LinkedList<_>`
2020-02-17T00:34:02.8363238Z -    = note: an implementation of `std::ops::AddAssign` might be missing for `std::collections::LinkedList<_>`
2020-02-17T00:34:02.8363313Z 10 
2020-02-17T00:34:02.8363313Z 10 
2020-02-17T00:34:02.8363551Z 11 error[E0067]: invalid left-hand side of assignment
2020-02-17T00:34:02.8363760Z 12   --> $DIR/E0067.rs:4:23
2020-02-17T00:34:02.8363834Z 
2020-02-17T00:34:02.8363890Z The actual stderr differed from the expected stderr.
2020-02-17T00:34:02.8364189Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0067/E0067.stderr
2020-02-17T00:34:02.8364189Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0067/E0067.stderr
2020-02-17T00:34:02.8364457Z To update references, rerun the tests and pass the `--bless` flag
2020-02-17T00:34:02.8364872Z To only update this specific test, also pass `--test-args error-codes/E0067.rs`
2020-02-17T00:34:02.8364981Z error: 1 errors occurred comparing output.
2020-02-17T00:34:02.8365198Z status: exit code: 1
2020-02-17T00:34:02.8365198Z status: exit code: 1
2020-02-17T00:34:02.8366168Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0067.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0067" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0067/auxiliary"
2020-02-17T00:34:02.8366575Z ------------------------------------------
2020-02-17T00:34:02.8366612Z 
2020-02-17T00:34:02.8366874Z ------------------------------------------
2020-02-17T00:34:02.8366923Z stderr:
2020-02-17T00:34:02.8366923Z stderr:
2020-02-17T00:34:02.8367156Z ------------------------------------------
2020-02-17T00:34:02.8367443Z error[E0368]: binary assignment operation `+=` cannot be applied to type `std::collections::LinkedList<_>`
2020-02-17T00:34:02.8367878Z    |
2020-02-17T00:34:02.8367878Z    |
2020-02-17T00:34:02.8367935Z LL |     LinkedList::new() += 1; //~ ERROR E0368
2020-02-17T00:34:02.8368161Z    |     |
2020-02-17T00:34:02.8368161Z    |     |
2020-02-17T00:34:02.8368218Z    |     cannot use `+=` on type `std::collections::LinkedList<_>`
2020-02-17T00:34:02.8368255Z 
2020-02-17T00:34:02.8368459Z error[E0067]: invalid left-hand side of assignment
2020-02-17T00:34:02.8368721Z    |
2020-02-17T00:34:02.8368721Z    |
2020-02-17T00:34:02.8368760Z LL |     LinkedList::new() += 1; //~ ERROR E0368
2020-02-17T00:34:02.8368996Z    |     |
2020-02-17T00:34:02.8369035Z    |     cannot assign to this expression
2020-02-17T00:34:02.8369069Z 
2020-02-17T00:34:02.8369107Z error: aborting due to 2 previous errors
---
2020-02-17T00:34:02.8370294Z 25    |     cannot use `+=` on type `&str`
2020-02-17T00:34:02.8370452Z -    |
2020-02-17T00:34:02.8370676Z -    = note: an implementation of `std::ops::AddAssign` might be missing for `&str`
2020-02-17T00:34:02.8370736Z 28 
2020-02-17T00:34:02.8370784Z 29 error[E0599]: no method named `z` found for reference `&str` in the current scope
2020-02-17T00:34:02.8371428Z 
2020-02-17T00:34:02.8371453Z 
2020-02-17T00:34:02.8371678Z The actual stderr differed from the expected stderr.
2020-02-17T00:34:02.8371963Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-festival/error-festival.stderr
2020-02-17T00:34:02.8371963Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-festival/error-festival.stderr
2020-02-17T00:34:02.8372221Z To update references, rerun the tests and pass the `--bless` flag
2020-02-17T00:34:02.8372498Z To only update this specific test, also pass `--test-args error-festival.rs`
2020-02-17T00:34:02.8372674Z error: 1 errors occurred comparing output.
2020-02-17T00:34:02.8372722Z status: exit code: 1
2020-02-17T00:34:02.8372722Z status: exit code: 1
2020-02-17T00:34:02.8373509Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-festival.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-festival" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-festival/auxiliary"
2020-02-17T00:34:02.8374141Z ------------------------------------------
2020-02-17T00:34:02.8374175Z 
2020-02-17T00:34:02.8374811Z ------------------------------------------
2020-02-17T00:34:02.8374862Z stderr:
---
2020-02-17T00:34:02.8376430Z    |
2020-02-17T00:34:02.8376477Z LL |     const FOO: u32 = 0;
2020-02-17T00:34:02.8376518Z    |     ^^^^^^^^^^^^^^^^^^^
2020-02-17T00:34:02.8376559Z 
2020-02-17T00:34:02.8376642Z error[E0368]: binary assignment operation `+=` cannot be applied to type `&str`
2020-02-17T00:34:02.8376920Z    |
2020-02-17T00:34:02.8376959Z LL |     x += 2;
2020-02-17T00:34:02.8377130Z    |     -^^^^^
2020-02-17T00:34:02.8377170Z    |     |
2020-02-17T00:34:02.8377170Z    |     |
2020-02-17T00:34:02.8377236Z    |     cannot use `+=` on type `&str`
2020-02-17T00:34:02.8377263Z 
2020-02-17T00:34:02.8377308Z error[E0599]: no method named `z` found for reference `&str` in the current scope
2020-02-17T00:34:02.8377582Z    |
2020-02-17T00:34:02.8377619Z LL |     x.z();
2020-02-17T00:34:02.8377659Z    |       ^ method not found in `&str`
2020-02-17T00:34:02.8377704Z 
2020-02-17T00:34:02.8377704Z 
2020-02-17T00:34:02.8377754Z error[E0600]: cannot apply unary operator `!` to type `Question`
2020-02-17T00:34:02.8377969Z   --> /checkout/src/test/ui/error-festival.rs:19:5
2020-02-17T00:34:02.8378024Z    |
2020-02-17T00:34:02.8378064Z LL |     !Question::Yes;
2020-02-17T00:34:02.8378162Z    |
2020-02-17T00:34:02.8378162Z    |
2020-02-17T00:34:02.8378206Z    = note: an implementation of `std::ops::Not` might be missing for `Question`
2020-02-17T00:34:02.8378236Z 
2020-02-17T00:34:02.8378276Z error[E0604]: only `u8` can be cast as `char`, not `u32`
2020-02-17T00:34:02.8378556Z    |
2020-02-17T00:34:02.8378594Z LL |     0u32 as char;
2020-02-17T00:34:02.8378715Z    |     ^^^^^^^^^^^^
2020-02-17T00:34:02.8378741Z 
---
2020-02-17T00:34:02.8379449Z 
2020-02-17T00:34:02.8379491Z error[E0054]: cannot cast as `bool`
2020-02-17T00:34:02.8379724Z   --> /checkout/src/test/ui/error-festival.rs:33:24
2020-02-17T00:34:02.8379767Z    |
2020-02-17T00:34:02.8379885Z LL |     let x_is_nonzero = x as bool;
2020-02-17T00:34:02.8379948Z    |                        ^^^^^^^^^ help: compare with zero instead: `x != 0`
2020-02-17T00:34:02.8379978Z 
2020-02-17T00:34:02.8380019Z error[E0606]: casting `&u8` as `u32` is invalid
2020-02-17T00:34:02.8380318Z    |
2020-02-17T00:34:02.8380318Z    |
2020-02-17T00:34:02.8380357Z LL |     let y: u32 = x as u32;
2020-02-17T00:34:02.8380543Z    |                  -^^^^^^^
2020-02-17T00:34:02.8380600Z    |                  |
2020-02-17T00:34:02.8380707Z    |                  cannot cast `&u8` as `u32`
2020-02-17T00:34:02.8380758Z    |                  help: dereference the expression: `*x`
2020-02-17T00:34:02.8380789Z 
2020-02-17T00:34:02.8381038Z error[E0607]: cannot cast thin pointer `*const u8` to fat pointer `*const [u8]`
2020-02-17T00:34:02.8381350Z    |
2020-02-17T00:34:02.8381409Z LL |     v as *const [u8];
2020-02-17T00:34:02.8381464Z    |     ^^^^^^^^^^^^^^^^
2020-02-17T00:34:02.8381491Z 
---
2020-02-17T00:34:02.8382507Z ---- [ui] ui/minus-string.rs stdout ----
2020-02-17T00:34:02.8382573Z diff of stderr:
2020-02-17T00:34:02.8382601Z 
2020-02-17T00:34:02.8382642Z 3    |
2020-02-17T00:34:02.8382877Z 4 LL | fn main() { -"foo".to_string(); }
2020-02-17T00:34:02.8383126Z 5    |             ^^^^^^^^^^^^^^^^^^ cannot apply unary operator `-`
2020-02-17T00:34:02.8383580Z -    = note: an implementation of `std::ops::Neg` might be missing for `std::string::String`
2020-02-17T00:34:02.8383657Z 8 
2020-02-17T00:34:02.8383702Z 9 error: aborting due to previous error
2020-02-17T00:34:02.8383745Z 10 
2020-02-17T00:34:02.8383745Z 10 
2020-02-17T00:34:02.8383791Z 
2020-02-17T00:34:02.8383818Z 
2020-02-17T00:34:02.8383865Z The actual stderr differed from the expected stderr.
2020-02-17T00:34:02.8384469Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/minus-string/minus-string.stderr
2020-02-17T00:34:02.8384720Z To update references, rerun the tests and pass the `--bless` flag
2020-02-17T00:34:02.8384972Z To only update this specific test, also pass `--test-args minus-string.rs`
2020-02-17T00:34:02.8385063Z error: 1 errors occurred comparing output.
2020-02-17T00:34:02.8385107Z status: exit code: 1
2020-02-17T00:34:02.8385107Z status: exit code: 1
2020-02-17T00:34:02.8385827Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/minus-string.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/minus-string" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/minus-string/auxiliary"
2020-02-17T00:34:02.8386163Z ------------------------------------------
2020-02-17T00:34:02.8386195Z 
2020-02-17T00:34:02.8386435Z ------------------------------------------
2020-02-17T00:34:02.8386479Z stderr:
2020-02-17T00:34:02.8386479Z stderr:
2020-02-17T00:34:02.8386694Z ------------------------------------------
2020-02-17T00:34:02.8386959Z error[E0600]: cannot apply unary operator `-` to type `std::string::String`
2020-02-17T00:34:02.8387184Z   --> /checkout/src/test/ui/minus-string.rs:3:13
2020-02-17T00:34:02.8387230Z    |
2020-02-17T00:34:02.8387453Z LL | fn main() { -"foo".to_string(); }
2020-02-17T00:34:02.8387813Z    |             ^^^^^^^^^^^^^^^^^^ cannot apply unary operator `-`
2020-02-17T00:34:02.8387889Z error: aborting due to previous error
2020-02-17T00:34:02.8387933Z 
2020-02-17T00:34:02.8388173Z For more information about this error, try `rustc --explain E0600`.
2020-02-17T00:34:02.8388205Z 
2020-02-17T00:34:02.8388205Z 
2020-02-17T00:34:02.8388414Z ------------------------------------------
2020-02-17T00:34:02.8388459Z 
2020-02-17T00:34:02.8388484Z 
2020-02-17T00:34:02.8388798Z ---- [ui] ui/rfc-2497-if-let-chains/disallowed-positions.rs stdout ----
2020-02-17T00:34:02.8388856Z diff of stderr:
2020-02-17T00:34:02.8388899Z 
2020-02-17T00:34:02.8388938Z 560    |
2020-02-17T00:34:02.8389173Z 561 LL |     if -let 0 = 0 {}
2020-02-17T00:34:02.8389421Z 562    |        ^^^^^^^^^^ cannot apply unary operator `-`
2020-02-17T00:34:02.8389844Z -    = note: an implementation of `std::ops::Neg` might be missing for `bool`
2020-02-17T00:34:02.8389891Z 565 
2020-02-17T00:34:02.8389965Z 566 error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
2020-02-17T00:34:02.8390195Z 567   --> $DIR/disallowed-positions.rs:46:8
2020-02-17T00:34:02.8390195Z 567   --> $DIR/disallowed-positions.rs:46:8
2020-02-17T00:34:02.8390225Z 
2020-02-17T00:34:02.8390280Z 748    |
2020-02-17T00:34:02.8390484Z 749 LL |     while -let 0 = 0 {}
2020-02-17T00:34:02.8390716Z 750    |           ^^^^^^^^^^ cannot apply unary operator `-`
2020-02-17T00:34:02.8391672Z -    = note: an implementation of `std::ops::Neg` might be missing for `bool`
2020-02-17T00:34:02.8391731Z 753 
2020-02-17T00:34:02.8391784Z 754 error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
2020-02-17T00:34:02.8392040Z 755   --> $DIR/disallowed-positions.rs:110:11
2020-02-17T00:34:02.8392040Z 755   --> $DIR/disallowed-positions.rs:110:11
2020-02-17T00:34:02.8392073Z 
2020-02-17T00:34:02.8392114Z 927    |
2020-02-17T00:34:02.8392329Z 928 LL |     -let 0 = 0;
2020-02-17T00:34:02.8392564Z 929    |     ^^^^^^^^^^ cannot apply unary operator `-`
2020-02-17T00:34:02.8393011Z -    = note: an implementation of `std::ops::Neg` might be missing for `bool`
2020-02-17T00:34:02.8393079Z 932 
2020-02-17T00:34:02.8393130Z 933 error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
2020-02-17T00:34:02.8393360Z 934   --> $DIR/disallowed-positions.rs:183:5
2020-02-17T00:34:02.8393360Z 934   --> $DIR/disallowed-positions.rs:183:5
2020-02-17T00:34:02.8393409Z 
2020-02-17T00:34:02.8393437Z 
2020-02-17T00:34:02.8393483Z The actual stderr differed from the expected stderr.
2020-02-17T00:34:02.8393829Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions/disallowed-positions.stderr
2020-02-17T00:34:02.8394106Z To update references, rerun the tests and pass the `--bless` flag
2020-02-17T00:34:02.8395094Z To only update this specific test, also pass `--test-args rfc-2497-if-let-chains/disallowed-positions.rs`
2020-02-17T00:34:02.8395191Z error: 1 errors occurred comparing output.
2020-02-17T00:34:02.8395242Z status: exit code: 1
2020-02-17T00:34:02.8395242Z status: exit code: 1
2020-02-17T00:34:02.8396203Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions/auxiliary"
2020-02-17T00:34:02.8396539Z ------------------------------------------
2020-02-17T00:34:02.8396571Z 
2020-02-17T00:34:02.8396771Z ------------------------------------------
2020-02-17T00:34:02.8396812Z stderr:
2020-02-17T00:34:02.8396812Z stderr:
2020-02-17T00:34:02.8397023Z ------------------------------------------
2020-02-17T00:34:02.8397187Z error: expected one of `,` or `>`, found `&&`
2020-02-17T00:34:02.8397521Z    |
2020-02-17T00:34:02.8397521Z    |
2020-02-17T00:34:02.8397569Z LL |         true && let 1 = 1 //~ ERROR expected one of `,` or `>`, found `&&`
2020-02-17T00:34:02.8397615Z    |              ^^ expected one of `,` or `>`
2020-02-17T00:34:02.8397700Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8398018Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:32:9
2020-02-17T00:34:02.8398076Z    |
2020-02-17T00:34:02.8398076Z    |
2020-02-17T00:34:02.8398139Z LL |     if &let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8398226Z    |
2020-02-17T00:34:02.8398226Z    |
2020-02-17T00:34:02.8398520Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8398576Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8398660Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8398935Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:35:9
2020-02-17T00:34:02.8398982Z    |
2020-02-17T00:34:02.8398982Z    |
2020-02-17T00:34:02.8399028Z LL |     if !let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8399130Z    |
2020-02-17T00:34:02.8399130Z    |
2020-02-17T00:34:02.8399388Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8399459Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8399532Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8399807Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:36:9
2020-02-17T00:34:02.8399854Z    |
2020-02-17T00:34:02.8399854Z    |
2020-02-17T00:34:02.8399900Z LL |     if *let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8400013Z    |
2020-02-17T00:34:02.8400013Z    |
2020-02-17T00:34:02.8400267Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8400321Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8400412Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8400672Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:38:9
2020-02-17T00:34:02.8400733Z    |
2020-02-17T00:34:02.8400733Z    |
2020-02-17T00:34:02.8401159Z LL |     if -let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8401253Z    |
2020-02-17T00:34:02.8401253Z    |
2020-02-17T00:34:02.8401535Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8401592Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8401697Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8401966Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:46:9
2020-02-17T00:34:02.8402016Z    |
2020-02-17T00:34:02.8402016Z    |
2020-02-17T00:34:02.8402082Z LL |     if (let 0 = 0)? {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8402177Z    |
2020-02-17T00:34:02.8402177Z    |
2020-02-17T00:34:02.8402463Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8402540Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8402622Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8402926Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:50:16
2020-02-17T00:34:02.8402980Z    |
2020-02-17T00:34:02.8402980Z    |
2020-02-17T00:34:02.8403032Z LL |     if true || let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8403233Z    |
2020-02-17T00:34:02.8403233Z    |
2020-02-17T00:34:02.8403529Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8403590Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8403690Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8404042Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:51:17
2020-02-17T00:34:02.8404122Z    |
2020-02-17T00:34:02.8404122Z    |
2020-02-17T00:34:02.8404173Z LL |     if (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8404288Z    |
2020-02-17T00:34:02.8404288Z    |
2020-02-17T00:34:02.8404740Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8404794Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8405049Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8405297Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:52:25
2020-02-17T00:34:02.8405342Z    |
2020-02-17T00:34:02.8405342Z    |
2020-02-17T00:34:02.8405404Z LL |     if true && (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8405490Z    |
2020-02-17T00:34:02.8405490Z    |
2020-02-17T00:34:02.8405745Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8405797Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8405866Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8406126Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:53:25
2020-02-17T00:34:02.8406171Z    |
2020-02-17T00:34:02.8406171Z    |
2020-02-17T00:34:02.8406217Z LL |     if true || (true && let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8406325Z    |
2020-02-17T00:34:02.8406325Z    |
2020-02-17T00:34:02.8406560Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8406630Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8406699Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8406944Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:56:12
2020-02-17T00:34:02.8407005Z    |
2020-02-17T00:34:02.8407005Z    |
2020-02-17T00:34:02.8407048Z LL |     if x = let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8407148Z    |
2020-02-17T00:34:02.8407148Z    |
2020-02-17T00:34:02.8407382Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8407433Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8407526Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8407765Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:59:15
2020-02-17T00:34:02.8407823Z    |
2020-02-17T00:34:02.8407823Z    |
2020-02-17T00:34:02.8407868Z LL |     if true..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8408125Z    |
2020-02-17T00:34:02.8408125Z    |
2020-02-17T00:34:02.8408374Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8408424Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8408507Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8408740Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:61:11
2020-02-17T00:34:02.8408782Z    |
2020-02-17T00:34:02.8408782Z    |
2020-02-17T00:34:02.8408838Z LL |     if ..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8409001Z    |
2020-02-17T00:34:02.8409001Z    |
2020-02-17T00:34:02.8409248Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8409315Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8409381Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8409807Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:63:9
2020-02-17T00:34:02.8409861Z    |
2020-02-17T00:34:02.8409861Z    |
2020-02-17T00:34:02.8409906Z LL |     if (let 0 = 0).. {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8410003Z    |
2020-02-17T00:34:02.8410003Z    |
2020-02-17T00:34:02.8410272Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8410326Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8410424Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8410675Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:67:8
2020-02-17T00:34:02.8410906Z    |
2020-02-17T00:34:02.8410906Z    |
2020-02-17T00:34:02.8410955Z LL |     if let Range { start: _, end: _ } = true..true && false {}
2020-02-17T00:34:02.8411069Z    |
2020-02-17T00:34:02.8411069Z    |
2020-02-17T00:34:02.8411327Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8411393Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8411485Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8411751Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:71:8
2020-02-17T00:34:02.8411801Z    |
2020-02-17T00:34:02.8411801Z    |
2020-02-17T00:34:02.8411866Z LL |     if let Range { start: _, end: _ } = true..true || false {}
2020-02-17T00:34:02.8411975Z    |
2020-02-17T00:34:02.8411975Z    |
2020-02-17T00:34:02.8412273Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8412335Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8412417Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8412715Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:78:8
2020-02-17T00:34:02.8412776Z    |
2020-02-17T00:34:02.8412776Z    |
2020-02-17T00:34:02.8412827Z LL |     if let Range { start: F, end } = F..|| true {}
2020-02-17T00:34:02.8412942Z    |
2020-02-17T00:34:02.8412942Z    |
2020-02-17T00:34:02.8413221Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8413296Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8413389Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8413674Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:86:8
2020-02-17T00:34:02.8413949Z    |
2020-02-17T00:34:02.8413949Z    |
2020-02-17T00:34:02.8413996Z LL |     if let Range { start: true, end } = t..&&false {}
2020-02-17T00:34:02.8414260Z    |
2020-02-17T00:34:02.8414260Z    |
2020-02-17T00:34:02.8414648Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8414707Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8414792Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8415026Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:92:19
2020-02-17T00:34:02.8415069Z    |
2020-02-17T00:34:02.8415131Z LL |     if let true = let true = true {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8415131Z LL |     if let true = let true = true {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8415261Z    |                   ^^^^^^^^^^^^^^^
2020-02-17T00:34:02.8415298Z    |
2020-02-17T00:34:02.8415566Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8415616Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8415701Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8415931Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:96:12
2020-02-17T00:34:02.8416046Z    |
2020-02-17T00:34:02.8416046Z    |
2020-02-17T00:34:02.8416111Z LL |     while &let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8416190Z    |
2020-02-17T00:34:02.8416190Z    |
2020-02-17T00:34:02.8416438Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8416504Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8416580Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8416829Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:99:12
2020-02-17T00:34:02.8417050Z    |
2020-02-17T00:34:02.8417050Z    |
2020-02-17T00:34:02.8417093Z LL |     while !let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8417189Z    |
2020-02-17T00:34:02.8417189Z    |
2020-02-17T00:34:02.8417420Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8417479Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8417565Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8417806Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:100:12
2020-02-17T00:34:02.8417865Z    |
2020-02-17T00:34:02.8417865Z    |
2020-02-17T00:34:02.8417909Z LL |     while *let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8418014Z    |
2020-02-17T00:34:02.8418014Z    |
2020-02-17T00:34:02.8418248Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8418299Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8418385Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8418771Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:102:12
2020-02-17T00:34:02.8418813Z    |
2020-02-17T00:34:02.8418813Z    |
2020-02-17T00:34:02.8419056Z LL |     while -let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8419137Z    |
2020-02-17T00:34:02.8419137Z    |
2020-02-17T00:34:02.8419377Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8419427Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8419493Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8419751Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:110:12
2020-02-17T00:34:02.8419792Z    |
2020-02-17T00:34:02.8419792Z    |
2020-02-17T00:34:02.8419835Z LL |     while (let 0 = 0)? {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8419929Z    |
2020-02-17T00:34:02.8419929Z    |
2020-02-17T00:34:02.8420151Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8420223Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8420292Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8420724Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:114:19
2020-02-17T00:34:02.8420788Z    |
2020-02-17T00:34:02.8420788Z    |
2020-02-17T00:34:02.8420838Z LL |     while true || let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8421034Z    |
2020-02-17T00:34:02.8421034Z    |
2020-02-17T00:34:02.8421317Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8421375Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8421469Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8421733Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:115:20
2020-02-17T00:34:02.8421782Z    |
2020-02-17T00:34:02.8421782Z    |
2020-02-17T00:34:02.8421922Z LL |     while (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8422022Z    |
2020-02-17T00:34:02.8422022Z    |
2020-02-17T00:34:02.8422321Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8422378Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8422469Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8422748Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:116:28
2020-02-17T00:34:02.8422796Z    |
2020-02-17T00:34:02.8422796Z    |
2020-02-17T00:34:02.8422848Z LL |     while true && (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8422958Z    |
2020-02-17T00:34:02.8422958Z    |
2020-02-17T00:34:02.8423213Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8423295Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8423375Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8423994Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:117:28
2020-02-17T00:34:02.8424035Z    |
2020-02-17T00:34:02.8424035Z    |
2020-02-17T00:34:02.8424079Z LL |     while true || (true && let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8424187Z    |
2020-02-17T00:34:02.8424187Z    |
2020-02-17T00:34:02.8424413Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8424463Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8424545Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8424773Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:120:15
2020-02-17T00:34:02.8424837Z    |
2020-02-17T00:34:02.8424837Z    |
2020-02-17T00:34:02.8424881Z LL |     while x = let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8424978Z    |
2020-02-17T00:34:02.8424978Z    |
2020-02-17T00:34:02.8425208Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8425256Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8425346Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8425577Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:123:18
2020-02-17T00:34:02.8425620Z    |
2020-02-17T00:34:02.8425620Z    |
2020-02-17T00:34:02.8425677Z LL |     while true..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8425758Z    |
2020-02-17T00:34:02.8425758Z    |
2020-02-17T00:34:02.8426001Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8426058Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8426125Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8426375Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:125:14
2020-02-17T00:34:02.8426417Z    |
2020-02-17T00:34:02.8426417Z    |
2020-02-17T00:34:02.8426461Z LL |     while ..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8426636Z    |
2020-02-17T00:34:02.8426636Z    |
2020-02-17T00:34:02.8426884Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8426951Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8427018Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8427247Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:127:12
2020-02-17T00:34:02.8427303Z    |
2020-02-17T00:34:02.8427303Z    |
2020-02-17T00:34:02.8427410Z LL |     while (let 0 = 0).. {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8427514Z    |
2020-02-17T00:34:02.8427514Z    |
2020-02-17T00:34:02.8427763Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8427813Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8427905Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8428139Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:131:11
2020-02-17T00:34:02.8428182Z    |
2020-02-17T00:34:02.8428182Z    |
2020-02-17T00:34:02.8428239Z LL |     while let Range { start: _, end: _ } = true..true && false {}
2020-02-17T00:34:02.8428323Z    |
2020-02-17T00:34:02.8428323Z    |
2020-02-17T00:34:02.8428743Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8428803Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8428889Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8429306Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:135:11
2020-02-17T00:34:02.8429350Z    |
2020-02-17T00:34:02.8429350Z    |
2020-02-17T00:34:02.8429652Z LL |     while let Range { start: _, end: _ } = true..true || false {}
2020-02-17T00:34:02.8429768Z    |
2020-02-17T00:34:02.8429768Z    |
2020-02-17T00:34:02.8430021Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8431049Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8431142Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8431532Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:142:11
2020-02-17T00:34:02.8431596Z    |
2020-02-17T00:34:02.8431596Z    |
2020-02-17T00:34:02.8431643Z LL |     while let Range { start: F, end } = F..|| true {}
2020-02-17T00:34:02.8431753Z    |
2020-02-17T00:34:02.8431753Z    |
2020-02-17T00:34:02.8432017Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8432073Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8432177Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8432444Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:150:11
2020-02-17T00:34:02.8432508Z    |
2020-02-17T00:34:02.8432508Z    |
2020-02-17T00:34:02.8432556Z LL |     while let Range { start: true, end } = t..&&false {}
2020-02-17T00:34:02.8432668Z    |
2020-02-17T00:34:02.8432668Z    |
2020-02-17T00:34:02.8432924Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8432990Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8433084Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8433351Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:156:22
2020-02-17T00:34:02.8433398Z    |
2020-02-17T00:34:02.8433465Z LL |     while let true = let true = true {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8433465Z LL |     while let true = let true = true {} //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8433657Z    |                      ^^^^^^^^^^^^^^^
2020-02-17T00:34:02.8433699Z    |
2020-02-17T00:34:02.8434013Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8434070Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8434147Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8435362Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:170:6
2020-02-17T00:34:02.8435537Z    |
2020-02-17T00:34:02.8435537Z    |
2020-02-17T00:34:02.8435588Z LL |     &let 0 = 0; //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8435687Z    |
2020-02-17T00:34:02.8435687Z    |
2020-02-17T00:34:02.8435951Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8436016Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8436094Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8436331Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:172:6
2020-02-17T00:34:02.8436390Z    |
2020-02-17T00:34:02.8436390Z    |
2020-02-17T00:34:02.8436431Z LL |     !let 0 = 0; //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8436525Z    |
2020-02-17T00:34:02.8436525Z    |
2020-02-17T00:34:02.8436751Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8436808Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8436890Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8437122Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:173:6
2020-02-17T00:34:02.8437164Z    |
2020-02-17T00:34:02.8437164Z    |
2020-02-17T00:34:02.8437220Z LL |     *let 0 = 0; //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8437305Z    |
2020-02-17T00:34:02.8437305Z    |
2020-02-17T00:34:02.8437550Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8437599Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8437681Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8437910Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:175:6
2020-02-17T00:34:02.8437952Z    |
2020-02-17T00:34:02.8437952Z    |
2020-02-17T00:34:02.8438342Z LL |     -let 0 = 0; //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8438495Z    |
2020-02-17T00:34:02.8438495Z    |
2020-02-17T00:34:02.8438733Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8438799Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8438868Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8439138Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:183:6
2020-02-17T00:34:02.8439181Z    |
2020-02-17T00:34:02.8439181Z    |
2020-02-17T00:34:02.8439224Z LL |     (let 0 = 0)?; //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8439321Z    |
2020-02-17T00:34:02.8439321Z    |
2020-02-17T00:34:02.8439551Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8439602Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8439694Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8439935Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:187:13
2020-02-17T00:34:02.8439993Z    |
2020-02-17T00:34:02.8439993Z    |
2020-02-17T00:34:02.8440037Z LL |     true || let 0 = 0; //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8440135Z    |
2020-02-17T00:34:02.8440135Z    |
2020-02-17T00:34:02.8440572Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8440737Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8440827Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8441110Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:188:14
2020-02-17T00:34:02.8441154Z    |
2020-02-17T00:34:02.8441154Z    |
2020-02-17T00:34:02.8441213Z LL |     (true || let 0 = 0); //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8441372Z    |
2020-02-17T00:34:02.8441372Z    |
2020-02-17T00:34:02.8441652Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8441705Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8441773Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8442031Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:189:22
2020-02-17T00:34:02.8442083Z    |
2020-02-17T00:34:02.8442083Z    |
2020-02-17T00:34:02.8442128Z LL |     true && (true || let 0 = 0); //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8442228Z    |
2020-02-17T00:34:02.8442228Z    |
2020-02-17T00:34:02.8442461Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8442527Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8445031Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8445414Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:192:9
2020-02-17T00:34:02.8446966Z    |
2020-02-17T00:34:02.8446966Z    |
2020-02-17T00:34:02.8447016Z LL |     x = let 0 = 0; //~ ERROR `let` expressions are not supported here
2020-02-17T00:34:02.8447115Z    |
2020-02-17T00:34:02.8447115Z    |
2020-02-17T00:34:02.8447615Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2020-02-17T00:34:02.8447678Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2020-02-17T00:34:02.8447765Z error: `let` expressions are not supported here
2020-02-17T00:34:02.8448004Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:194:12
2020-02-17T00:34:02.8448047Z    |
2020-02-17T00:34:02.8448047Z    |
2020-02-17T00:34:02.8448104Z LL |     true..(let 0 = 0); //~ ERROR `let` expressions are not supported here
---
2020-02-17T00:34:02.8518850Z 
2020-02-17T00:34:02.8518874Z 
2020-02-17T00:34:02.8518924Z The actual stderr differed from the expected stderr.
2020-02-17T00:34:02.8519193Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unop-neg-bool/unop-neg-bool.stderr
2020-02-17T00:34:02.8519436Z To update references, rerun the tests and pass the `--bless` flag
2020-02-17T00:34:02.8519680Z To only update this specific test, also pass `--test-args unop-neg-bool.rs`
2020-02-17T00:34:02.8519771Z error: 1 errors occurred comparing output.
2020-02-17T00:34:02.8519903Z status: exit code: 1
2020-02-17T00:34:02.8519903Z status: exit code: 1
2020-02-17T00:34:02.8520725Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unop-neg-bool.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unop-neg-bool" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unop-neg-bool/auxiliary"
2020-02-17T00:34:02.8521273Z ------------------------------------------
2020-02-17T00:34:02.8521324Z 
2020-02-17T00:34:02.8521547Z ------------------------------------------
2020-02-17T00:34:02.8521593Z stderr:
2020-02-17T00:34:02.8521593Z stderr:
2020-02-17T00:34:02.8521807Z ------------------------------------------
2020-02-17T00:34:02.8522083Z error[E0600]: cannot apply unary operator `-` to type `bool`
2020-02-17T00:34:02.8522316Z   --> /checkout/src/test/ui/unop-neg-bool.rs:2:5
2020-02-17T00:34:02.8522365Z    |
2020-02-17T00:34:02.8522630Z LL |     -true; //~ ERROR cannot apply unary operator `-` to type `bool`
2020-02-17T00:34:02.8522856Z    |     ^^^^^ cannot apply unary operator `-`
2020-02-17T00:34:02.8522948Z error: aborting due to previous error
2020-02-17T00:34:02.8522979Z 
2020-02-17T00:34:02.8523234Z For more information about this error, try `rustc --explain E0600`.
2020-02-17T00:34:02.8523268Z 
---
2020-02-17T00:34:02.8525783Z test result: FAILED. 9590 passed; 6 failed; 54 ignored; 0 measured; 0 filtered out
2020-02-17T00:34:02.8525817Z 
2020-02-17T00:34:02.8525843Z 
2020-02-17T00:34:02.8525867Z 
2020-02-17T00:34:02.8527253Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-17T00:34:02.8527524Z 
2020-02-17T00:34:02.8527566Z 
2020-02-17T00:34:02.8527974Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-17T00:34:02.8528027Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-17T00:34:02.8528027Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-17T00:34:02.8528219Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-17T00:34:02.8528263Z Build completed unsuccessfully in 1:04:30
2020-02-17T00:34:02.8528482Z == clock drift check ==
2020-02-17T00:34:02.8528538Z   local time: Mon Feb 17 00:34:02 UTC 2020
2020-02-17T00:34:03.1187991Z   network time: Mon, 17 Feb 2020 00:34:03 GMT
2020-02-17T00:34:03.1193207Z == end clock drift check ==
2020-02-17T00:34:03.5969314Z 
2020-02-17T00:34:03.6062880Z ##[error]Bash exited with code '1'.
2020-02-17T00:34:03.6073766Z ##[section]Finishing: Run build
2020-02-17T00:34:03.6102831Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69217/merge to s
2020-02-17T00:34:03.6104601Z Task         : Get sources
2020-02-17T00:34:03.6104646Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-17T00:34:03.6104719Z Version      : 1.0.0
2020-02-17T00:34:03.6104757Z Author       : Microsoft
2020-02-17T00:34:03.6104757Z Author       : Microsoft
2020-02-17T00:34:03.6104800Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-17T00:34:03.6104864Z ==============================================================================
2020-02-17T00:34:04.0380721Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-17T00:34:04.0420631Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69217/merge to s
2020-02-17T00:34:04.0533089Z Cleaning up task key
2020-02-17T00:34:04.0533898Z Start cleaning up orphan processes.
2020-02-17T00:34:04.0642020Z Terminate orphan process: pid (6649) (python)
2020-02-17T00:34:04.1039004Z ##[section]Finishing: Finalize Job
