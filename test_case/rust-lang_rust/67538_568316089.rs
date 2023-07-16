plain
2019-12-22T23:10:27.2762691Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-22T23:10:27.3007891Z ##[command]git config gc.auto 0
2019-12-22T23:10:27.3078884Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-22T23:10:27.3142019Z ##[command]git config --get-all http.proxy
2019-12-22T23:10:27.3318490Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67538/merge:refs/remotes/pull/67538/merge
---
2019-12-23T00:08:59.7822222Z .................................................................................................... 1600/9428
2019-12-23T00:09:04.1784597Z .................................................................................................... 1700/9428
2019-12-23T00:09:14.0698663Z .....................................................................................i.............. 1800/9428
2019-12-23T00:09:21.3676510Z .................................................................................................... 1900/9428
2019-12-23T00:09:27.7995457Z ................................F......................................iiiii........................ 2000/9428
2019-12-23T00:09:48.3784093Z .................................................................................................... 2200/9428
2019-12-23T00:09:50.6560011Z .................................................................................................... 2300/9428
2019-12-23T00:09:53.1735989Z .................................................................................................... 2400/9428
2019-12-23T00:10:05.3942873Z .................................................................................................... 2500/9428
---
2019-12-23T00:12:49.3807417Z ..i...............i................................................................................. 4900/9428
2019-12-23T00:12:58.9852947Z .................................................................................................... 5000/9428
2019-12-23T00:13:03.5887555Z ..............................................i..................................................... 5100/9428
2019-12-23T00:13:12.8929186Z .................................................................................................... 5200/9428
2019-12-23T00:13:18.5495266Z .............ii.ii...........i...................................................................... 5300/9428
2019-12-23T00:13:27.2357343Z .................................................................................................... 5500/9428
2019-12-23T00:13:37.7855840Z ...............................................................................................i.... 5600/9428
2019-12-23T00:13:45.9638705Z .................................................................................................... 5700/9428
2019-12-23T00:13:50.5114648Z .................................................................................................... 5800/9428
2019-12-23T00:13:50.5114648Z .................................................................................................... 5800/9428
2019-12-23T00:13:59.7836124Z ...................................................................................ii...i..ii....... 5900/9428
2019-12-23T00:14:21.4020555Z .................................................................................................... 6100/9428
2019-12-23T00:14:28.9747537Z .................................................................................................... 6200/9428
2019-12-23T00:14:36.3638114Z .................................................................................................... 6300/9428
2019-12-23T00:14:36.3638114Z .................................................................................................... 6300/9428
2019-12-23T00:14:50.9656498Z ..........i..ii..................................................................................... 6400/9428
2019-12-23T00:15:09.4643233Z ......................................................................................i............. 6600/9428
2019-12-23T00:15:11.4684530Z .................................................................................................... 6700/9428
2019-12-23T00:15:13.6360163Z ......................................................................................i............. 6800/9428
2019-12-23T00:15:16.2067403Z .................................................................................................... 6900/9428
---
2019-12-23T00:16:45.8006165Z .................................................................................................... 7400/9428
2019-12-23T00:16:50.4193709Z .................................................................................................... 7500/9428
2019-12-23T00:16:55.3226927Z .................................................................................................... 7600/9428
2019-12-23T00:17:01.9559086Z .................................................................................................... 7700/9428
2019-12-23T00:17:12.1836759Z ...................................................................................................i 7800/9428
2019-12-23T00:17:18.5875570Z iii................................................................................................. 7900/9428
2019-12-23T00:17:32.2912053Z .................................................................................................... 8100/9428
2019-12-23T00:17:43.4759959Z .................................................................................................... 8200/9428
2019-12-23T00:17:54.8242015Z .................................................................................................... 8300/9428
2019-12-23T00:18:00.2919103Z .................................................................................................... 8400/9428
---
2019-12-23T00:19:48.9245549Z 
2019-12-23T00:19:48.9246297Z ---- [ui] ui/destructuring-assignment/note-unsupported.rs stdout ----
2019-12-23T00:19:48.9246393Z diff of stderr:
2019-12-23T00:19:48.9246428Z 
2019-12-23T00:19:48.9249329Z 1 error[E0070]: invalid left-hand side of assignment
2019-12-23T00:19:48.9249589Z -   --> $DIR/destructuring-assignment.rs:6:12
2019-12-23T00:19:48.9249813Z +   --> $DIR/note-unsupported.rs:6:12
2019-12-23T00:19:48.9249905Z 4 LL |     (a, b) = (3, 4);
2019-12-23T00:19:48.9250080Z 5    |     ------ ^
2019-12-23T00:19:48.9250109Z 
2019-12-23T00:19:48.9250339Z 10    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:19:48.9250339Z 10    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:19:48.9250401Z 11 
2019-12-23T00:19:48.9250623Z 12 error[E0368]: binary assignment operation `+=` cannot be applied to type `({integer}, {integer})`
2019-12-23T00:19:48.9250877Z -   --> $DIR/destructuring-assignment.rs:7:5
2019-12-23T00:19:48.9251080Z +   --> $DIR/note-unsupported.rs:7:5
2019-12-23T00:19:48.9251120Z 14    |
2019-12-23T00:19:48.9251158Z 15 LL |     (a, b) += (3, 4);
2019-12-23T00:19:48.9251377Z 
2019-12-23T00:19:48.9251377Z 
2019-12-23T00:19:48.9251422Z 20    = note: an implementation of `std::ops::AddAssign` might be missing for `({integer}, {integer})`
2019-12-23T00:19:48.9251463Z 21 
2019-12-23T00:19:48.9251822Z 22 error[E0067]: invalid left-hand side of assignment
2019-12-23T00:19:48.9252018Z -   --> $DIR/destructuring-assignment.rs:7:12
2019-12-23T00:19:48.9252205Z +   --> $DIR/note-unsupported.rs:7:12
2019-12-23T00:19:48.9252260Z 24    |
2019-12-23T00:19:48.9252297Z 25 LL |     (a, b) += (3, 4);
2019-12-23T00:19:48.9252496Z 
2019-12-23T00:19:48.9252738Z 31    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:19:48.9252791Z 32 
2019-12-23T00:19:48.9252791Z 32 
2019-12-23T00:19:48.9253026Z 33 error[E0070]: invalid left-hand side of assignment
2019-12-23T00:19:48.9253259Z -   --> $DIR/destructuring-assignment.rs:10:12
2019-12-23T00:19:48.9253474Z +   --> $DIR/note-unsupported.rs:10:12
2019-12-23T00:19:48.9253515Z 35    |
2019-12-23T00:19:48.9253566Z 36 LL |     [a, b] = [3, 4];
2019-12-23T00:19:48.9253788Z 
2019-12-23T00:19:48.9254037Z 42    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:19:48.9254105Z 43 
2019-12-23T00:19:48.9254105Z 43 
2019-12-23T00:19:48.9254150Z 44 error[E0368]: binary assignment operation `+=` cannot be applied to type `[{integer}; 2]`
2019-12-23T00:19:48.9254370Z -   --> $DIR/destructuring-assignment.rs:11:5
2019-12-23T00:19:48.9254589Z +   --> $DIR/note-unsupported.rs:11:5
2019-12-23T00:19:48.9255243Z 46    |
2019-12-23T00:19:48.9255503Z 47 LL |     [a, b] += [3, 4];
2019-12-23T00:19:48.9255864Z 
2019-12-23T00:19:48.9255864Z 
2019-12-23T00:19:48.9255928Z 52    = note: an implementation of `std::ops::AddAssign` might be missing for `[{integer}; 2]`
2019-12-23T00:19:48.9255978Z 53 
2019-12-23T00:19:48.9256257Z 54 error[E0067]: invalid left-hand side of assignment
2019-12-23T00:19:48.9256511Z -   --> $DIR/destructuring-assignment.rs:11:12
2019-12-23T00:19:48.9256748Z +   --> $DIR/note-unsupported.rs:11:12
2019-12-23T00:19:48.9256811Z 56    |
2019-12-23T00:19:48.9256856Z 57 LL |     [a, b] += [3, 4];
2019-12-23T00:19:48.9257117Z 
2019-12-23T00:19:48.9257422Z 63    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:19:48.9257475Z 64 
2019-12-23T00:19:48.9257475Z 64 
2019-12-23T00:19:48.9257725Z 65 error[E0070]: invalid left-hand side of assignment
2019-12-23T00:19:48.9257990Z -   --> $DIR/destructuring-assignment.rs:16:22
2019-12-23T00:19:48.9258230Z +   --> $DIR/note-unsupported.rs:16:22
2019-12-23T00:19:48.9258277Z 67    |
2019-12-23T00:19:48.9258331Z 68 LL |     S { x: a, y: b } = s;
2019-12-23T00:19:48.9258820Z 
2019-12-23T00:19:48.9259050Z 74    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:19:48.9259107Z 75 
2019-12-23T00:19:48.9259107Z 75 
2019-12-23T00:19:48.9259151Z 76 error[E0368]: binary assignment operation `+=` cannot be applied to type `S`
2019-12-23T00:19:48.9259365Z -   --> $DIR/destructuring-assignment.rs:17:5
2019-12-23T00:19:48.9259571Z +   --> $DIR/note-unsupported.rs:17:5
2019-12-23T00:19:48.9259633Z 78    |
2019-12-23T00:19:48.9259672Z 79 LL |     S { x: a, y: b } += s;
2019-12-23T00:19:48.9259918Z 
2019-12-23T00:19:48.9259962Z 84    = note: an implementation of `std::ops::AddAssign` might be missing for `S`
2019-12-23T00:19:48.9260003Z 85 
2019-12-23T00:19:48.9260003Z 85 
2019-12-23T00:19:48.9260224Z 86 error[E0067]: invalid left-hand side of assignment
2019-12-23T00:19:48.9260461Z -   --> $DIR/destructuring-assignment.rs:17:22
2019-12-23T00:19:48.9260777Z +   --> $DIR/note-unsupported.rs:17:22
2019-12-23T00:19:48.9260830Z 88    |
2019-12-23T00:19:48.9260884Z 89 LL |     S { x: a, y: b } += s;
2019-12-23T00:19:48.9261257Z 
2019-12-23T00:19:48.9261627Z 95    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:19:48.9261693Z 96 
2019-12-23T00:19:48.9261693Z 96 
2019-12-23T00:19:48.9261937Z 97 error[E0070]: invalid left-hand side of assignment
2019-12-23T00:19:48.9262174Z -   --> $DIR/destructuring-assignment.rs:20:21
2019-12-23T00:19:48.9262536Z +   --> $DIR/note-unsupported.rs:20:21
2019-12-23T00:19:48.9262583Z 99    |
2019-12-23T00:19:48.9262628Z 100 LL |     S { x: a, ..s } = S { x: 3, y: 4 };
2019-12-23T00:19:48.9263048Z 
2019-12-23T00:19:48.9263690Z 106    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:19:48.9263748Z 107 
2019-12-23T00:19:48.9263748Z 107 
2019-12-23T00:19:48.9264032Z 108 error[E0070]: invalid left-hand side of assignment
2019-12-23T00:19:48.9264262Z -   --> $DIR/destructuring-assignment.rs:24:17
2019-12-23T00:19:48.9264481Z +   --> $DIR/note-unsupported.rs:24:17
2019-12-23T00:19:48.9264542Z 110    |
2019-12-23T00:19:48.9264586Z 111 LL |     ((a, b), c) = ((3, 4), 5);
2019-12-23T00:19:48.9264821Z 
2019-12-23T00:19:48.9264860Z 
2019-12-23T00:19:48.9264906Z The actual stderr differed from the expected stderr.
2019-12-23T00:19:48.9264906Z The actual stderr differed from the expected stderr.
2019-12-23T00:19:48.9265712Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/note-unsupported/note-unsupported.stderr
2019-12-23T00:19:48.9266007Z To update references, rerun the tests and pass the `--bless` flag
2019-12-23T00:19:48.9266296Z To only update this specific test, also pass `--test-args destructuring-assignment/note-unsupported.rs`
2019-12-23T00:19:48.9266389Z error: 1 errors occurred comparing output.
2019-12-23T00:19:48.9266434Z status: exit code: 1
2019-12-23T00:19:48.9266434Z status: exit code: 1
2019-12-23T00:19:48.9269045Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/destructuring-assignment/note-unsupported.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/note-unsupported" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/note-unsupported/auxiliary" "-A" "unused"
2019-12-23T00:19:48.9270615Z ------------------------------------------
2019-12-23T00:19:48.9270684Z 
2019-12-23T00:19:48.9273282Z ------------------------------------------
2019-12-23T00:19:48.9273486Z stderr:
2019-12-23T00:19:48.9273486Z stderr:
2019-12-23T00:19:48.9276494Z ------------------------------------------
2019-12-23T00:19:48.9278948Z error[E0070]: invalid left-hand side of assignment
2019-12-23T00:19:48.9281385Z   --> /checkout/src/test/ui/destructuring-assignment/note-unsupported.rs:6:12
2019-12-23T00:19:48.9281468Z    |
2019-12-23T00:19:48.9282857Z LL |     (a, b) = (3, 4); //~ ERROR invalid left-hand side of assignment
2019-12-23T00:19:48.9285197Z    |     |
2019-12-23T00:19:48.9285264Z    |     cannot assign to this expression
2019-12-23T00:19:48.9285321Z    |
2019-12-23T00:19:48.9285368Z    = note: destructuring assignments are not currently supported
2019-12-23T00:19:48.9285368Z    = note: destructuring assignments are not currently supported
2019-12-23T00:19:48.9285717Z    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:19:48.9285759Z 
2019-12-23T00:19:48.9285809Z error[E0368]: binary assignment operation `+=` cannot be applied to type `({integer}, {integer})`
2019-12-23T00:19:48.9286085Z   --> /checkout/src/test/ui/destructuring-assignment/note-unsupported.rs:7:5
2019-12-23T00:19:48.9286277Z    |
2019-12-23T00:19:48.9286571Z LL |     (a, b) += (3, 4); //~ ERROR invalid left-hand side of assignment
2019-12-23T00:19:48.9286837Z    |     |
2019-12-23T00:19:48.9286837Z    |     |
2019-12-23T00:19:48.9286883Z    |     cannot use `+=` on type `({integer}, {integer})`
2019-12-23T00:19:48.9286925Z    |
2019-12-23T00:19:48.9286991Z    = note: an implementation of `std::ops::AddAssign` might be missing for `({integer}, {integer})`
2019-12-23T00:19:48.9287029Z 
2019-12-23T00:19:48.9287382Z error[E0067]: invalid left-hand side of assignment
2019-12-23T00:19:48.9287655Z   --> /checkout/src/test/ui/destructuring-assignment/note-unsupported.rs:7:12
2019-12-23T00:19:48.9287704Z    |
2019-12-23T00:19:48.9287950Z LL |     (a, b) += (3, 4); //~ ERROR invalid left-hand side of assignment
2019-12-23T00:19:48.9288326Z    |     |
2019-12-23T00:19:48.9288365Z    |     cannot assign to this expression
2019-12-23T00:19:48.9288403Z    |
2019-12-23T00:19:48.9288468Z    = note: destructuring assignments are not currently supported
2019-12-23T00:19:48.9288468Z    = note: destructuring assignments are not currently supported
2019-12-23T00:19:48.9288715Z    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:19:48.9288750Z 
2019-12-23T00:19:48.9288973Z error[E0070]: invalid left-hand side of assignment
2019-12-23T00:19:48.9289214Z   --> /checkout/src/test/ui/destructuring-assignment/note-unsupported.rs:10:12
2019-12-23T00:19:48.9289258Z    |
2019-12-23T00:19:48.9289499Z LL |     [a, b] = [3, 4]; //~ ERROR invalid left-hand side of assignment
2019-12-23T00:19:48.9289732Z    |     |
2019-12-23T00:19:48.9289772Z    |     cannot assign to this expression
2019-12-23T00:19:48.9289825Z    |
2019-12-23T00:19:48.9289868Z    = note: destructuring assignments are not currently supported
2019-12-23T00:19:48.9289868Z    = note: destructuring assignments are not currently supported
2019-12-23T00:19:48.9290113Z    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:19:48.9290159Z 
2019-12-23T00:19:48.9290211Z error[E0368]: binary assignment operation `+=` cannot be applied to type `[{integer}; 2]`
2019-12-23T00:19:48.9290453Z   --> /checkout/src/test/ui/destructuring-assignment/note-unsupported.rs:11:5
2019-12-23T00:19:48.9290509Z    |
2019-12-23T00:19:48.9290738Z LL |     [a, b] += [3, 4]; //~ ERROR invalid left-hand side of assignment
2019-12-23T00:19:48.9290966Z    |     |
2019-12-23T00:19:48.9290966Z    |     |
2019-12-23T00:19:48.9291021Z    |     cannot use `+=` on type `[{integer}; 2]`
2019-12-23T00:19:48.9291067Z    |
2019-12-23T00:19:48.9291113Z    = note: an implementation of `std::ops::AddAssign` might be missing for `[{integer}; 2]`
2019-12-23T00:19:48.9291158Z 
2019-12-23T00:19:48.9291481Z error[E0067]: invalid left-hand side of assignment
2019-12-23T00:19:48.9291704Z   --> /checkout/src/test/ui/destructuring-assignment/note-unsupported.rs:11:12
2019-12-23T00:19:48.9291745Z    |
2019-12-23T00:19:48.9291973Z LL |     [a, b] += [3, 4]; //~ ERROR invalid left-hand side of assignment
2019-12-23T00:19:48.9292189Z    |     |
2019-12-23T00:19:48.9292240Z    |     cannot assign to this expression
2019-12-23T00:19:48.9292276Z    |
2019-12-23T00:19:48.9292315Z    = note: destructuring assignments are not currently supported
2019-12-23T00:19:48.9292315Z    = note: destructuring assignments are not currently supported
2019-12-23T00:19:48.9292556Z    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:19:48.9292587Z 
2019-12-23T00:19:48.9292783Z error[E0070]: invalid left-hand side of assignment
2019-12-23T00:19:48.9293014Z   --> /checkout/src/test/ui/destructuring-assignment/note-unsupported.rs:16:22
2019-12-23T00:19:48.9293068Z    |
2019-12-23T00:19:48.9293283Z LL |     S { x: a, y: b } = s; //~ ERROR invalid left-hand side of assignment
2019-12-23T00:19:48.9293515Z    |     |
2019-12-23T00:19:48.9293553Z    |     cannot assign to this expression
2019-12-23T00:19:48.9293588Z    |
2019-12-23T00:19:48.9293713Z    = note: destructuring assignments are not currently supported
2019-12-23T00:19:48.9293713Z    = note: destructuring assignments are not currently supported
2019-12-23T00:19:48.9293971Z    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:19:48.9294002Z 
2019-12-23T00:19:48.9294042Z error[E0368]: binary assignment operation `+=` cannot be applied to type `S`
2019-12-23T00:19:48.9294281Z   --> /checkout/src/test/ui/destructuring-assignment/note-unsupported.rs:17:5
2019-12-23T00:19:48.9294323Z    |
2019-12-23T00:19:48.9294667Z LL |     S { x: a, y: b } += s; //~ ERROR invalid left-hand side of assignment
2019-12-23T00:19:48.9295686Z    |     |
2019-12-23T00:19:48.9295686Z    |     |
2019-12-23T00:19:48.9295913Z    |     cannot use `+=` on type `S`
2019-12-23T00:19:48.9296027Z    = note: an implementation of `std::ops::AddAssign` might be missing for `S`
2019-12-23T00:19:48.9296059Z 
2019-12-23T00:19:48.9296059Z 
2019-12-23T00:19:48.9296347Z error[E0067]: invalid left-hand side of assignment
2019-12-23T00:19:48.9296621Z   --> /checkout/src/test/ui/destructuring-assignment/note-unsupported.rs:17:22
2019-12-23T00:19:48.9296681Z    |
2019-12-23T00:19:48.9297127Z LL |     S { x: a, y: b } += s; //~ ERROR invalid left-hand side of assignment
2019-12-23T00:19:48.9297442Z    |     |
2019-12-23T00:19:48.9297485Z    |     cannot assign to this expression
2019-12-23T00:19:48.9297538Z    |
2019-12-23T00:19:48.9298286Z    = note: destructuring assignments are not currently supported
2019-12-23T00:19:48.9298286Z    = note: destructuring assignments are not currently supported
2019-12-23T00:19:48.9298878Z    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:19:48.9298930Z 
2019-12-23T00:19:48.9299383Z error[E0070]: invalid left-hand side of assignment
2019-12-23T00:19:48.9299864Z   --> /checkout/src/test/ui/destructuring-assignment/note-unsupported.rs:20:21
2019-12-23T00:19:48.9299919Z    |
2019-12-23T00:19:48.9300197Z LL |     S { x: a, ..s } = S { x: 3, y: 4 }; //~ ERROR invalid left-hand side of assignment
2019-12-23T00:19:48.9300784Z    |     |
2019-12-23T00:19:48.9300854Z    |     cannot assign to this expression
2019-12-23T00:19:48.9300895Z    |
2019-12-23T00:19:48.9301054Z    = note: destructuring assignments are not currently supported
2019-12-23T00:19:48.9301054Z    = note: destructuring assignments are not currently supported
2019-12-23T00:19:48.9301513Z    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:19:48.9301695Z 
2019-12-23T00:19:48.9302088Z error[E0070]: invalid left-hand side of assignment
2019-12-23T00:19:48.9302402Z   --> /checkout/src/test/ui/destructuring-assignment/note-unsupported.rs:24:17
2019-12-23T00:19:48.9302479Z    |
2019-12-23T00:19:48.9302729Z LL |     ((a, b), c) = ((3, 4), 5); //~ ERROR invalid left-hand side of assignment
2019-12-23T00:19:48.9303180Z    |     |
2019-12-23T00:19:48.9304127Z    |     cannot assign to this expression
2019-12-23T00:19:48.9304299Z    |
2019-12-23T00:19:48.9304341Z    = note: destructuring assignments are not currently supported
---
2019-12-23T00:19:48.9307007Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-23T00:19:48.9307209Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-23T00:19:48.9307252Z 
2019-12-23T00:19:48.9307277Z 
2019-12-23T00:19:48.9315628Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-23T00:19:48.9317540Z 
2019-12-23T00:19:48.9317749Z 
2019-12-23T00:19:48.9319366Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-23T00:19:48.9319576Z Build completed unsuccessfully in 1:02:29
2019-12-23T00:19:48.9319576Z Build completed unsuccessfully in 1:02:29
2019-12-23T00:19:48.9372141Z == clock drift check ==
2019-12-23T00:19:48.9395524Z   local time: Mon Dec 23 00:19:48 UTC 2019
2019-12-23T00:19:49.4790861Z   network time: Mon, 23 Dec 2019 00:19:49 GMT
2019-12-23T00:19:49.4790978Z == end clock drift check ==
2019-12-23T00:19:50.3290867Z 
2019-12-23T00:19:50.3382233Z ##[error]Bash exited with code '1'.
2019-12-23T00:19:50.3428788Z ##[section]Starting: Checkout
2019-12-23T00:19:50.3430339Z ==============================================================================
2019-12-23T00:19:50.3430386Z Task         : Get sources
2019-12-23T00:19:50.3430440Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
