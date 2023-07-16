plain
2019-12-22T23:49:40.7548632Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-22T23:49:40.7568835Z ##[command]git config gc.auto 0
2019-12-22T23:49:40.7572778Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-22T23:49:40.7577104Z ##[command]git config --get-all http.proxy
2019-12-22T23:49:40.7583494Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67545/merge:refs/remotes/pull/67545/merge
---
2019-12-23T00:38:56.9495214Z .................................................................................................... 1600/9429
2019-12-23T00:39:01.0345654Z .................................................................................................... 1700/9429
2019-12-23T00:39:10.1743001Z .....................................................................................i.............. 1800/9429
2019-12-23T00:39:17.2542689Z .................................................................................................... 1900/9429
2019-12-23T00:39:23.4016262Z ................................F......................................iiiii........................ 2000/9429
2019-12-23T00:39:41.1079999Z .................................................................................................... 2200/9429
2019-12-23T00:39:43.1302615Z .................................................................................................... 2300/9429
2019-12-23T00:39:45.5083319Z .................................................................................................... 2400/9429
2019-12-23T00:39:56.5188364Z .................................................................................................... 2500/9429
---
2019-12-23T00:42:27.5638135Z ..i...............i................................................................................. 4900/9429
2019-12-23T00:42:36.3804908Z .................................................................................................... 5000/9429
2019-12-23T00:42:40.7374444Z ..............................................i..................................................... 5100/9429
2019-12-23T00:42:49.1724767Z .................................................................................................... 5200/9429
2019-12-23T00:42:54.3736611Z .............ii.ii...........i...................................................................... 5300/9429
2019-12-23T00:43:02.3869059Z .................................................................................................... 5500/9429
2019-12-23T00:43:11.9337665Z ...............................................................................................i.... 5600/9429
2019-12-23T00:43:19.0300696Z .................................................................................................... 5700/9429
2019-12-23T00:43:23.4271689Z .................................................................................................... 5800/9429
2019-12-23T00:43:23.4271689Z .................................................................................................... 5800/9429
2019-12-23T00:43:31.8265591Z ...................................................................................ii...i..ii....... 5900/9429
2019-12-23T00:43:51.7319675Z .................................................................................................... 6100/9429
2019-12-23T00:43:55.5258431Z .................................................................................................... 6200/9429
2019-12-23T00:43:59.0434732Z .................................................................................................... 6300/9429
2019-12-23T00:43:59.0434732Z .................................................................................................... 6300/9429
2019-12-23T00:44:10.3455155Z ..........i..ii..................................................................................... 6400/9429
2019-12-23T00:44:26.5820546Z ......................................................................................i............. 6600/9429
2019-12-23T00:44:28.4152988Z .................................................................................................... 6700/9429
2019-12-23T00:44:30.3853605Z ......................................................................................i............. 6800/9429
2019-12-23T00:44:32.9646712Z .................................................................................................... 6900/9429
---
2019-12-23T00:45:55.4301961Z .................................................................................................... 7500/9429
2019-12-23T00:45:59.7583981Z .................................................................................................... 7600/9429
2019-12-23T00:46:05.7084353Z .................................................................................................... 7700/9429
2019-12-23T00:46:14.9173582Z .................................................................................................... 7800/9429
2019-12-23T00:46:20.3796878Z iiii................................................................................................ 7900/9429
2019-12-23T00:46:32.6509699Z .................................................................................................... 8100/9429
2019-12-23T00:46:42.5523979Z .................................................................................................... 8200/9429
2019-12-23T00:46:52.6741120Z .................................................................................................... 8300/9429
2019-12-23T00:46:57.4254023Z .................................................................................................... 8400/9429
---
2019-12-23T00:48:34.3678832Z 
2019-12-23T00:48:34.3679818Z ---- [ui] ui/destructuring-assignment/note-unsupported.rs stdout ----
2019-12-23T00:48:34.3680022Z diff of stderr:
2019-12-23T00:48:34.3680163Z 
2019-12-23T00:48:34.3680572Z 1 error[E0070]: invalid left-hand side of assignment
2019-12-23T00:48:34.3680946Z -   --> $DIR/destructuring-assignment.rs:6:12
2019-12-23T00:48:34.3681306Z +   --> $DIR/note-unsupported.rs:6:12
2019-12-23T00:48:34.3681647Z 4 LL |     (a, b) = (3, 4);
2019-12-23T00:48:34.3681968Z 5    |     ------ ^
2019-12-23T00:48:34.3682136Z 
2019-12-23T00:48:34.3682581Z 10    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:48:34.3682581Z 10    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:48:34.3682760Z 11 
2019-12-23T00:48:34.3682964Z 12 error[E0368]: binary assignment operation `+=` cannot be applied to type `({integer}, {integer})`
2019-12-23T00:48:34.3683509Z -   --> $DIR/destructuring-assignment.rs:7:5
2019-12-23T00:48:34.3683880Z +   --> $DIR/note-unsupported.rs:7:5
2019-12-23T00:48:34.3684050Z 14    |
2019-12-23T00:48:34.3684199Z 15 LL |     (a, b) += (3, 4);
2019-12-23T00:48:34.3684691Z 
2019-12-23T00:48:34.3684691Z 
2019-12-23T00:48:34.3684868Z 20    = note: an implementation of `std::ops::AddAssign` might be missing for `({integer}, {integer})`
2019-12-23T00:48:34.3685037Z 21 
2019-12-23T00:48:34.3685393Z 22 error[E0067]: invalid left-hand side of assignment
2019-12-23T00:48:34.3685756Z -   --> $DIR/destructuring-assignment.rs:7:12
2019-12-23T00:48:34.3686124Z +   --> $DIR/note-unsupported.rs:7:12
2019-12-23T00:48:34.3686294Z 24    |
2019-12-23T00:48:34.3686442Z 25 LL |     (a, b) += (3, 4);
2019-12-23T00:48:34.3686924Z 
2019-12-23T00:48:34.3687316Z 31    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:48:34.3687511Z 32 
2019-12-23T00:48:34.3687511Z 32 
2019-12-23T00:48:34.3687866Z 33 error[E0070]: invalid left-hand side of assignment
2019-12-23T00:48:34.3688232Z -   --> $DIR/destructuring-assignment.rs:10:12
2019-12-23T00:48:34.3693560Z +   --> $DIR/note-unsupported.rs:10:12
2019-12-23T00:48:34.3693911Z 35    |
2019-12-23T00:48:34.3694332Z 36 LL |     [a, b] = [3, 4];
2019-12-23T00:48:34.3694894Z 
2019-12-23T00:48:34.3695282Z 42    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:48:34.3695476Z 43 
2019-12-23T00:48:34.3695476Z 43 
2019-12-23T00:48:34.3695638Z 44 error[E0368]: binary assignment operation `+=` cannot be applied to type `[{integer}; 2]`
2019-12-23T00:48:34.3695991Z -   --> $DIR/destructuring-assignment.rs:11:5
2019-12-23T00:48:34.3696989Z +   --> $DIR/note-unsupported.rs:11:5
2019-12-23T00:48:34.3697187Z 46    |
2019-12-23T00:48:34.3697355Z 47 LL |     [a, b] += [3, 4];
2019-12-23T00:48:34.3697863Z 
2019-12-23T00:48:34.3697863Z 
2019-12-23T00:48:34.3698022Z 52    = note: an implementation of `std::ops::AddAssign` might be missing for `[{integer}; 2]`
2019-12-23T00:48:34.3698191Z 53 
2019-12-23T00:48:34.3698544Z 54 error[E0067]: invalid left-hand side of assignment
2019-12-23T00:48:34.3698905Z -   --> $DIR/destructuring-assignment.rs:11:12
2019-12-23T00:48:34.3699277Z +   --> $DIR/note-unsupported.rs:11:12
2019-12-23T00:48:34.3699444Z 56    |
2019-12-23T00:48:34.3699590Z 57 LL |     [a, b] += [3, 4];
2019-12-23T00:48:34.3700075Z 
2019-12-23T00:48:34.3700457Z 63    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:48:34.3700641Z 64 
2019-12-23T00:48:34.3700641Z 64 
2019-12-23T00:48:34.3700993Z 65 error[E0070]: invalid left-hand side of assignment
2019-12-23T00:48:34.3701370Z -   --> $DIR/destructuring-assignment.rs:16:22
2019-12-23T00:48:34.3701881Z +   --> $DIR/note-unsupported.rs:16:22
2019-12-23T00:48:34.3702062Z 67    |
2019-12-23T00:48:34.3702230Z 68 LL |     S { x: a, y: b } = s;
2019-12-23T00:48:34.3702726Z 
2019-12-23T00:48:34.3703109Z 74    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:48:34.3703348Z 75 
2019-12-23T00:48:34.3703348Z 75 
2019-12-23T00:48:34.3703577Z 76 error[E0368]: binary assignment operation `+=` cannot be applied to type `S`
2019-12-23T00:48:34.3704029Z -   --> $DIR/destructuring-assignment.rs:17:5
2019-12-23T00:48:34.3706330Z +   --> $DIR/note-unsupported.rs:17:5
2019-12-23T00:48:34.3706404Z 78    |
2019-12-23T00:48:34.3706453Z 79 LL |     S { x: a, y: b } += s;
2019-12-23T00:48:34.3706715Z 
2019-12-23T00:48:34.3706769Z 84    = note: an implementation of `std::ops::AddAssign` might be missing for `S`
2019-12-23T00:48:34.3706817Z 85 
2019-12-23T00:48:34.3706817Z 85 
2019-12-23T00:48:34.3707050Z 86 error[E0067]: invalid left-hand side of assignment
2019-12-23T00:48:34.3707306Z -   --> $DIR/destructuring-assignment.rs:17:22
2019-12-23T00:48:34.3707652Z +   --> $DIR/note-unsupported.rs:17:22
2019-12-23T00:48:34.3707700Z 88    |
2019-12-23T00:48:34.3707762Z 89 LL |     S { x: a, y: b } += s;
2019-12-23T00:48:34.3708004Z 
2019-12-23T00:48:34.3708266Z 95    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:48:34.3708333Z 96 
2019-12-23T00:48:34.3708333Z 96 
2019-12-23T00:48:34.3708564Z 97 error[E0070]: invalid left-hand side of assignment
2019-12-23T00:48:34.3708790Z -   --> $DIR/destructuring-assignment.rs:20:21
2019-12-23T00:48:34.3709027Z +   --> $DIR/note-unsupported.rs:20:21
2019-12-23T00:48:34.3709076Z 99    |
2019-12-23T00:48:34.3709123Z 100 LL |     S { x: a, ..s } = S { x: 3, y: 4 };
2019-12-23T00:48:34.3709380Z 
2019-12-23T00:48:34.3709642Z 106    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:48:34.3710612Z 107 
2019-12-23T00:48:34.3710612Z 107 
2019-12-23T00:48:34.3715737Z 108 error[E0070]: invalid left-hand side of assignment
2019-12-23T00:48:34.3716082Z -   --> $DIR/destructuring-assignment.rs:24:17
2019-12-23T00:48:34.3716301Z +   --> $DIR/note-unsupported.rs:24:17
2019-12-23T00:48:34.3716380Z 110    |
2019-12-23T00:48:34.3716427Z 111 LL |     ((a, b), c) = ((3, 4), 5);
2019-12-23T00:48:34.3716667Z 
2019-12-23T00:48:34.3716712Z 
2019-12-23T00:48:34.3716761Z The actual stderr differed from the expected stderr.
2019-12-23T00:48:34.3716761Z The actual stderr differed from the expected stderr.
2019-12-23T00:48:34.3717089Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/note-unsupported/note-unsupported.stderr
2019-12-23T00:48:34.3717365Z To update references, rerun the tests and pass the `--bless` flag
2019-12-23T00:48:34.3717658Z To only update this specific test, also pass `--test-args destructuring-assignment/note-unsupported.rs`
2019-12-23T00:48:34.3717768Z error: 1 errors occurred comparing output.
2019-12-23T00:48:34.3717815Z status: exit code: 1
2019-12-23T00:48:34.3717815Z status: exit code: 1
2019-12-23T00:48:34.3718715Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/destructuring-assignment/note-unsupported.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/note-unsupported" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/note-unsupported/auxiliary" "-A" "unused"
2019-12-23T00:48:34.3719055Z ------------------------------------------
2019-12-23T00:48:34.3719105Z 
2019-12-23T00:48:34.3719444Z ------------------------------------------
2019-12-23T00:48:34.3719496Z stderr:
2019-12-23T00:48:34.3719496Z stderr:
2019-12-23T00:48:34.3719737Z ------------------------------------------
2019-12-23T00:48:34.3719979Z error[E0070]: invalid left-hand side of assignment
2019-12-23T00:48:34.3720237Z   --> /checkout/src/test/ui/destructuring-assignment/note-unsupported.rs:6:12
2019-12-23T00:48:34.3720306Z    |
2019-12-23T00:48:34.3720561Z LL |     (a, b) = (3, 4); //~ ERROR invalid left-hand side of assignment
2019-12-23T00:48:34.3720807Z    |     |
2019-12-23T00:48:34.3720869Z    |     cannot assign to this expression
2019-12-23T00:48:34.3720915Z    |
2019-12-23T00:48:34.3720964Z    = note: destructuring assignments are not currently supported
2019-12-23T00:48:34.3720964Z    = note: destructuring assignments are not currently supported
2019-12-23T00:48:34.3721243Z    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:48:34.3721281Z 
2019-12-23T00:48:34.3721335Z error[E0368]: binary assignment operation `+=` cannot be applied to type `({integer}, {integer})`
2019-12-23T00:48:34.3721616Z   --> /checkout/src/test/ui/destructuring-assignment/note-unsupported.rs:7:5
2019-12-23T00:48:34.3721729Z    |
2019-12-23T00:48:34.3721989Z LL |     (a, b) += (3, 4); //~ ERROR invalid left-hand side of assignment
2019-12-23T00:48:34.3722259Z    |     |
2019-12-23T00:48:34.3722259Z    |     |
2019-12-23T00:48:34.3722308Z    |     cannot use `+=` on type `({integer}, {integer})`
2019-12-23T00:48:34.3722354Z    |
2019-12-23T00:48:34.3722424Z    = note: an implementation of `std::ops::AddAssign` might be missing for `({integer}, {integer})`
2019-12-23T00:48:34.3722463Z 
2019-12-23T00:48:34.3722694Z error[E0067]: invalid left-hand side of assignment
2019-12-23T00:48:34.3722968Z   --> /checkout/src/test/ui/destructuring-assignment/note-unsupported.rs:7:12
2019-12-23T00:48:34.3723018Z    |
2019-12-23T00:48:34.3723266Z LL |     (a, b) += (3, 4); //~ ERROR invalid left-hand side of assignment
2019-12-23T00:48:34.3723526Z    |     |
2019-12-23T00:48:34.3723580Z    |     cannot assign to this expression
2019-12-23T00:48:34.3723623Z    |
2019-12-23T00:48:34.3723694Z    = note: destructuring assignments are not currently supported
2019-12-23T00:48:34.3723694Z    = note: destructuring assignments are not currently supported
2019-12-23T00:48:34.3723955Z    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:48:34.3723992Z 
2019-12-23T00:48:34.3724232Z error[E0070]: invalid left-hand side of assignment
2019-12-23T00:48:34.3724492Z   --> /checkout/src/test/ui/destructuring-assignment/note-unsupported.rs:10:12
2019-12-23T00:48:34.3724544Z    |
2019-12-23T00:48:34.3724803Z LL |     [a, b] = [3, 4]; //~ ERROR invalid left-hand side of assignment
2019-12-23T00:48:34.3725151Z    |     |
2019-12-23T00:48:34.3725194Z    |     cannot assign to this expression
2019-12-23T00:48:34.3725250Z    |
2019-12-23T00:48:34.3725294Z    = note: destructuring assignments are not currently supported
2019-12-23T00:48:34.3725294Z    = note: destructuring assignments are not currently supported
2019-12-23T00:48:34.3725542Z    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:48:34.3725592Z 
2019-12-23T00:48:34.3725640Z error[E0368]: binary assignment operation `+=` cannot be applied to type `[{integer}; 2]`
2019-12-23T00:48:34.3725886Z   --> /checkout/src/test/ui/destructuring-assignment/note-unsupported.rs:11:5
2019-12-23T00:48:34.3725947Z    |
2019-12-23T00:48:34.3726176Z LL |     [a, b] += [3, 4]; //~ ERROR invalid left-hand side of assignment
2019-12-23T00:48:34.3726408Z    |     |
2019-12-23T00:48:34.3726408Z    |     |
2019-12-23T00:48:34.3726468Z    |     cannot use `+=` on type `[{integer}; 2]`
2019-12-23T00:48:34.3726510Z    |
2019-12-23T00:48:34.3726557Z    = note: an implementation of `std::ops::AddAssign` might be missing for `[{integer}; 2]`
2019-12-23T00:48:34.3726603Z 
2019-12-23T00:48:34.3726812Z error[E0067]: invalid left-hand side of assignment
2019-12-23T00:48:34.3727049Z   --> /checkout/src/test/ui/destructuring-assignment/note-unsupported.rs:11:12
2019-12-23T00:48:34.3727096Z    |
2019-12-23T00:48:34.3727393Z LL |     [a, b] += [3, 4]; //~ ERROR invalid left-hand side of assignment
2019-12-23T00:48:34.3727635Z    |     |
2019-12-23T00:48:34.3727695Z    |     cannot assign to this expression
2019-12-23T00:48:34.3727735Z    |
2019-12-23T00:48:34.3727779Z    = note: destructuring assignments are not currently supported
2019-12-23T00:48:34.3727779Z    = note: destructuring assignments are not currently supported
2019-12-23T00:48:34.3728035Z    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:48:34.3728071Z 
2019-12-23T00:48:34.3728280Z error[E0070]: invalid left-hand side of assignment
2019-12-23T00:48:34.3728518Z   --> /checkout/src/test/ui/destructuring-assignment/note-unsupported.rs:16:22
2019-12-23T00:48:34.3728578Z    |
2019-12-23T00:48:34.3728808Z LL |     S { x: a, y: b } = s; //~ ERROR invalid left-hand side of assignment
2019-12-23T00:48:34.3729057Z    |     |
2019-12-23T00:48:34.3729100Z    |     cannot assign to this expression
2019-12-23T00:48:34.3729147Z    |
2019-12-23T00:48:34.3729208Z    = note: destructuring assignments are not currently supported
2019-12-23T00:48:34.3729208Z    = note: destructuring assignments are not currently supported
2019-12-23T00:48:34.3729502Z    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:48:34.3729536Z 
2019-12-23T00:48:34.3729582Z error[E0368]: binary assignment operation `+=` cannot be applied to type `S`
2019-12-23T00:48:34.3729837Z   --> /checkout/src/test/ui/destructuring-assignment/note-unsupported.rs:17:5
2019-12-23T00:48:34.3729882Z    |
2019-12-23T00:48:34.3730112Z LL |     S { x: a, y: b } += s; //~ ERROR invalid left-hand side of assignment
2019-12-23T00:48:34.3730369Z    |     |
2019-12-23T00:48:34.3730369Z    |     |
2019-12-23T00:48:34.3730411Z    |     cannot use `+=` on type `S`
2019-12-23T00:48:34.3730514Z    = note: an implementation of `std::ops::AddAssign` might be missing for `S`
2019-12-23T00:48:34.3730544Z 
2019-12-23T00:48:34.3730544Z 
2019-12-23T00:48:34.3730753Z error[E0067]: invalid left-hand side of assignment
2019-12-23T00:48:34.3731016Z   --> /checkout/src/test/ui/destructuring-assignment/note-unsupported.rs:17:22
2019-12-23T00:48:34.3731069Z    |
2019-12-23T00:48:34.3731300Z LL |     S { x: a, y: b } += s; //~ ERROR invalid left-hand side of assignment
2019-12-23T00:48:34.3731555Z    |     |
2019-12-23T00:48:34.3731598Z    |     cannot assign to this expression
2019-12-23T00:48:34.3731654Z    |
2019-12-23T00:48:34.3731700Z    = note: destructuring assignments are not currently supported
2019-12-23T00:48:34.3731700Z    = note: destructuring assignments are not currently supported
2019-12-23T00:48:34.3731941Z    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:48:34.3731975Z 
2019-12-23T00:48:34.3732199Z error[E0070]: invalid left-hand side of assignment
2019-12-23T00:48:34.3732438Z   --> /checkout/src/test/ui/destructuring-assignment/note-unsupported.rs:20:21
2019-12-23T00:48:34.3732483Z    |
2019-12-23T00:48:34.3732744Z LL |     S { x: a, ..s } = S { x: 3, y: 4 }; //~ ERROR invalid left-hand side of assignment
2019-12-23T00:48:34.3732988Z    |     |
2019-12-23T00:48:34.3733053Z    |     cannot assign to this expression
2019-12-23T00:48:34.3733094Z    |
2019-12-23T00:48:34.3733137Z    = note: destructuring assignments are not currently supported
2019-12-23T00:48:34.3733137Z    = note: destructuring assignments are not currently supported
2019-12-23T00:48:34.3733380Z    = note: for more information, see https://github.com/rust-lang/rfcs/issues/372
2019-12-23T00:48:34.3733430Z 
2019-12-23T00:48:34.3733640Z error[E0070]: invalid left-hand side of assignment
2019-12-23T00:48:34.3733879Z   --> /checkout/src/test/ui/destructuring-assignment/note-unsupported.rs:24:17
2019-12-23T00:48:34.3733941Z    |
2019-12-23T00:48:34.3734179Z LL |     ((a, b), c) = ((3, 4), 5); //~ ERROR invalid left-hand side of assignment
2019-12-23T00:48:34.3734425Z    |     |
2019-12-23T00:48:34.3734468Z    |     cannot assign to this expression
2019-12-23T00:48:34.3734508Z    |
2019-12-23T00:48:34.3734594Z    = note: destructuring assignments are not currently supported
---
2019-12-23T00:48:34.3736436Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-23T00:48:34.3736490Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-23T00:48:34.3737345Z 
2019-12-23T00:48:34.3786956Z 
2019-12-23T00:48:34.3789209Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-23T00:48:34.3789775Z 
2019-12-23T00:48:34.3789878Z 
2019-12-23T00:48:34.3789980Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-23T00:48:34.3790031Z Build completed unsuccessfully in 0:53:24
2019-12-23T00:48:34.3790031Z Build completed unsuccessfully in 0:53:24
2019-12-23T00:48:34.3794574Z == clock drift check ==
2019-12-23T00:48:34.3809941Z   local time: Mon Dec 23 00:48:34 UTC 2019
2019-12-23T00:48:34.6590259Z   network time: Mon, 23 Dec 2019 00:48:34 GMT
2019-12-23T00:48:34.6595084Z == end clock drift check ==
2019-12-23T00:48:35.6449041Z 
2019-12-23T00:48:35.6527155Z ##[error]Bash exited with code '1'.
2019-12-23T00:48:35.6576412Z ##[section]Starting: Checkout
2019-12-23T00:48:35.6578028Z ==============================================================================
2019-12-23T00:48:35.6578082Z Task         : Get sources
2019-12-23T00:48:35.6578133Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
