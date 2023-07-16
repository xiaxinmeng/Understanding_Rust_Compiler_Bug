plain
2019-07-20T06:19:06.6282788Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-20T06:19:06.6459682Z ##[command]git config gc.auto 0
2019-07-20T06:19:06.6530979Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-20T06:19:06.6596007Z ##[command]git config --get-all http.proxy
2019-07-20T06:19:06.6732060Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/60951/merge:refs/remotes/pull/60951/merge
---
2019-07-20T06:19:40.7953895Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-20T06:19:40.7953944Z 
2019-07-20T06:19:40.7954177Z   git checkout -b <new-branch-name>
2019-07-20T06:19:40.7954209Z 
2019-07-20T06:19:40.7954257Z HEAD is now at 1c20e99e9 Merge fd352b02e17d78f03345ebc3ffabe02b0cb04fd1 into e9d22273283dce210b26362aa0dcc3fc10bf7e81
2019-07-20T06:19:40.8086948Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-20T06:19:40.8089549Z ==============================================================================
2019-07-20T06:19:40.8089599Z Task         : Bash
2019-07-20T06:19:40.8089638Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-20T07:17:35.0404645Z .................................................................................................... 200/5839
2019-07-20T07:17:39.2516606Z .................................................................................................... 300/5839
2019-07-20T07:17:43.1294043Z .................................................................................................... 400/5839
2019-07-20T07:17:47.0088602Z .................................................................................................... 500/5839
2019-07-20T07:17:50.9838985Z ........................................................................i........................... 600/5839
2019-07-20T07:18:00.3007225Z .................................................................................................... 800/5839
2019-07-20T07:18:00.3007225Z .................................................................................................... 800/5839
2019-07-20T07:18:06.0156694Z ..........FFF....................................................................................... 900/5839
2019-07-20T07:18:11.1800562Z ...................................................................................................i 1000/5839
2019-07-20T07:18:16.9801335Z ...........i........................................................................................ 1100/5839
2019-07-20T07:18:21.0278138Z .............................iiiii.................................................................. 1200/5839
2019-07-20T07:18:27.2253200Z .................................................................................................... 1400/5839
2019-07-20T07:18:30.0486735Z .................................................................................................... 1500/5839
2019-07-20T07:18:33.9326550Z .................................................................................................... 1600/5839
2019-07-20T07:18:36.6416946Z .................................................................................................... 1700/5839
2019-07-20T07:18:36.6416946Z .................................................................................................... 1700/5839
2019-07-20T07:18:40.2171420Z ...................................................................i................................ 1800/5839
2019-07-20T07:18:49.0614147Z .................................................................................................... 2000/5839
2019-07-20T07:18:53.5679722Z .................................................................................................... 2100/5839
2019-07-20T07:18:57.4644232Z .................................................................................................... 2200/5839
2019-07-20T07:18:57.4644232Z .................................................................................................... 2200/5839
2019-07-20T07:19:01.3882500Z ..................................................i................................................. 2300/5839
2019-07-20T07:19:11.4050534Z .................................................................................................... 2500/5839
2019-07-20T07:19:15.7244675Z .................................................................................................... 2600/5839
2019-07-20T07:19:20.9255994Z .................................................................................................... 2700/5839
2019-07-20T07:19:25.1223260Z .................................................................................................... 2800/5839
2019-07-20T07:19:25.1223260Z .................................................................................................... 2800/5839
2019-07-20T07:19:29.6406109Z .................................................................................................... 2900/5839
2019-07-20T07:19:34.9786195Z .................................................................................................... 3000/5839
2019-07-20T07:19:39.5655575Z .................................................................................................... 3100/5839
2019-07-20T07:19:44.9977303Z .................................................................................................... 3200/5839
2019-07-20T07:19:48.6430998Z .................................................................................................... 3300/5839
2019-07-20T07:19:52.5362994Z .................................................................................................... 3400/5839
2019-07-20T07:19:57.9624623Z .................................................................................................... 3500/5839
2019-07-20T07:20:01.9470524Z ................i................................................................................... 3600/5839
2019-07-20T07:20:06.1885684Z ..........................................................................................ii...i..ii 3700/5839
2019-07-20T07:20:15.3369853Z .................................................................................................... 3900/5839
2019-07-20T07:20:19.3850831Z .................................................................................................... 4000/5839
2019-07-20T07:20:19.3850831Z .................................................................................................... 4000/5839
2019-07-20T07:20:23.2188878Z ....ii.............................................................................................. 4100/5839
2019-07-20T07:20:25.3631365Z .........................i.......................................................................... 4200/5839
2019-07-20T07:20:27.5203955Z ...........................................................................................i........ 4300/5839
2019-07-20T07:20:35.4319216Z .................................................................................................... 4500/5839
2019-07-20T07:20:53.0497427Z .................................................................................................... 4600/5839
2019-07-20T07:20:56.7049045Z .................................................................................................... 4700/5839
2019-07-20T07:21:00.7760566Z .................................................................................................... 4800/5839
---
2019-07-20T07:21:35.8705675Z .................................................................................................... 5400/5839
2019-07-20T07:21:40.0056520Z .................................................................................................... 5500/5839
2019-07-20T07:21:44.3005101Z .................................................................................................... 5600/5839
2019-07-20T07:21:47.6031615Z .................................................................................................... 5700/5839
2019-07-20T07:21:50.6745648Z ...............................................................................i.................... 5800/5839
2019-07-20T07:21:52.1573911Z failures:
2019-07-20T07:21:52.1629436Z 
2019-07-20T07:21:52.1630670Z ---- [ui] ui/consts/const-eval/const_panic.rs stdout ----
2019-07-20T07:21:52.1630743Z diff of stderr:
2019-07-20T07:21:52.1630743Z diff of stderr:
2019-07-20T07:21:52.1630780Z 
2019-07-20T07:21:52.1630826Z 4 LL | pub const Z: () = panic!("cheese");
2019-07-20T07:21:52.1631297Z 5    | ------------------^^^^^^^^^^^^^^^^-
2019-07-20T07:21:52.1631357Z 6    |                   |
2019-07-20T07:21:52.1631667Z -    |                   the evaluated program panicked at 'cheese', $DIR/const_panic.rs:4:19
2019-07-20T07:21:52.1631755Z +    |                   the evaluated program panicked
2019-07-20T07:21:52.1632005Z 9    = note: `#[deny(const_err)]` on by default
2019-07-20T07:21:52.1632409Z 10    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-20T07:21:52.1632470Z 
2019-07-20T07:21:52.1632470Z 
2019-07-20T07:21:52.1632520Z 15 LL | pub const Y: () = unreachable!();
2019-07-20T07:21:52.1632967Z 16    | ------------------^^^^^^^^^^^^^^-
2019-07-20T07:21:52.1633028Z 17    |                   |
2019-07-20T07:21:52.1633496Z -    |                   the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic.rs:7:19
2019-07-20T07:21:52.1633573Z +    |                   the evaluated program panicked
2019-07-20T07:21:52.1634102Z 20    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-20T07:21:52.1634350Z 21 
2019-07-20T07:21:52.1634380Z 
2019-07-20T07:21:52.1634380Z 
2019-07-20T07:21:52.1634422Z 25 LL | pub const X: () = unimplemented!();
2019-07-20T07:21:52.1634671Z 26    | ------------------^^^^^^^^^^^^^^^^-
2019-07-20T07:21:52.1634749Z 27    |                   |
2019-07-20T07:21:52.1635177Z -    |                   the evaluated program panicked at 'not yet implemented', $DIR/const_panic.rs:10:19
2019-07-20T07:21:52.1635230Z +    |                   the evaluated program panicked
2019-07-20T07:21:52.1635948Z 30    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-20T07:21:52.1636002Z 31 
2019-07-20T07:21:52.1636029Z 
2019-07-20T07:21:52.1636076Z 
2019-07-20T07:21:52.1636076Z 
2019-07-20T07:21:52.1636120Z The actual stderr differed from the expected stderr.
2019-07-20T07:21:52.1636425Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic/const_panic.stderr
2019-07-20T07:21:52.1636693Z To update references, rerun the tests and pass the `--bless` flag
2019-07-20T07:21:52.1636995Z To only update this specific test, also pass `--test-args consts/const-eval/const_panic.rs`
2019-07-20T07:21:52.1637089Z error: 1 errors occurred comparing output.
2019-07-20T07:21:52.1637156Z status: exit code: 1
2019-07-20T07:21:52.1637156Z status: exit code: 1
2019-07-20T07:21:52.1638273Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic/auxiliary" "-A" "unused"
2019-07-20T07:21:52.1638621Z ------------------------------------------
2019-07-20T07:21:52.1638657Z 
2019-07-20T07:21:52.1639217Z ------------------------------------------
2019-07-20T07:21:52.1639261Z stderr:
2019-07-20T07:21:52.1639261Z stderr:
2019-07-20T07:21:52.1639598Z ------------------------------------------
2019-07-20T07:21:52.1639666Z error: any use of this value will cause an error
2019-07-20T07:21:52.1640375Z    |
2019-07-20T07:21:52.1640375Z    |
2019-07-20T07:21:52.1640445Z LL | pub const Z: () = panic!("cheese");
2019-07-20T07:21:52.1640672Z    | ------------------^^^^^^^^^^^^^^^^-
2019-07-20T07:21:52.1640787Z    |                   the evaluated program panicked
2019-07-20T07:21:52.1640833Z    |
2019-07-20T07:21:52.1640879Z    = note: `#[deny(const_err)]` on by default
2019-07-20T07:21:52.1641214Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-20T07:21:52.1641214Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-20T07:21:52.1641279Z 
2019-07-20T07:21:52.1641327Z error: any use of this value will cause an error
2019-07-20T07:21:52.1641655Z    |
2019-07-20T07:21:52.1641655Z    |
2019-07-20T07:21:52.1641699Z LL | pub const Y: () = unreachable!();
2019-07-20T07:21:52.1641920Z    | ------------------^^^^^^^^^^^^^^-
2019-07-20T07:21:52.1642034Z    |                   the evaluated program panicked
2019-07-20T07:21:52.1642077Z    |
2019-07-20T07:21:52.1642411Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-20T07:21:52.1642453Z 
2019-07-20T07:21:52.1642453Z 
2019-07-20T07:21:52.1642499Z error: any use of this value will cause an error
2019-07-20T07:21:52.1642926Z    |
2019-07-20T07:21:52.1642926Z    |
2019-07-20T07:21:52.1642969Z LL | pub const X: () = unimplemented!();
2019-07-20T07:21:52.1643214Z    | ------------------^^^^^^^^^^^^^^^^-
2019-07-20T07:21:52.1643331Z    |                   the evaluated program panicked
2019-07-20T07:21:52.1643383Z    |
2019-07-20T07:21:52.1643870Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-20T07:21:52.1643910Z 
---
2019-07-20T07:21:52.1644655Z 
2019-07-20T07:21:52.1645034Z ---- [ui] ui/consts/const-eval/const_panic_libcore.rs stdout ----
2019-07-20T07:21:52.1645099Z diff of stderr:
2019-07-20T07:21:52.1645127Z 
2019-07-20T07:21:52.1645452Z 4 LL | const Z: () = panic!("cheese");
2019-07-20T07:21:52.1645668Z 5    | --------------^^^^^^^^^^^^^^^^-
2019-07-20T07:21:52.1645733Z 6    |               |
2019-07-20T07:21:52.1646001Z -    |               the evaluated program panicked at 'cheese', $DIR/const_panic_libcore.rs:5:15
2019-07-20T07:21:52.1646067Z +    |               the evaluated program panicked
2019-07-20T07:21:52.1646174Z 9    = note: `#[deny(const_err)]` on by default
2019-07-20T07:21:52.1646489Z 10    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-20T07:21:52.1646548Z 
2019-07-20T07:21:52.1646548Z 
2019-07-20T07:21:52.1646592Z 15 LL | const Y: () = unreachable!();
2019-07-20T07:21:52.1646805Z 16    | --------------^^^^^^^^^^^^^^-
2019-07-20T07:21:52.1646870Z 17    |               |
2019-07-20T07:21:52.1647176Z -    |               the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic_libcore.rs:8:15
2019-07-20T07:21:52.1647236Z +    |               the evaluated program panicked
2019-07-20T07:21:52.1647610Z 20    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-20T07:21:52.1647761Z 21 
2019-07-20T07:21:52.1647788Z 
2019-07-20T07:21:52.1647788Z 
2019-07-20T07:21:52.1647850Z 25 LL | const X: () = unimplemented!();
2019-07-20T07:21:52.1648087Z 26    | --------------^^^^^^^^^^^^^^^^-
2019-07-20T07:21:52.1648134Z 27    |               |
2019-07-20T07:21:52.1648437Z -    |               the evaluated program panicked at 'not yet implemented', $DIR/const_panic_libcore.rs:11:15
2019-07-20T07:21:52.1648492Z +    |               the evaluated program panicked
2019-07-20T07:21:52.1648860Z 30    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-20T07:21:52.1648922Z 31 
2019-07-20T07:21:52.1648949Z 
2019-07-20T07:21:52.1648975Z 
2019-07-20T07:21:52.1648975Z 
2019-07-20T07:21:52.1649036Z The actual stderr differed from the expected stderr.
2019-07-20T07:21:52.1649350Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore/const_panic_libcore.stderr
2019-07-20T07:21:52.1649977Z To update references, rerun the tests and pass the `--bless` flag
2019-07-20T07:21:52.1650311Z To only update this specific test, also pass `--test-args consts/const-eval/const_panic_libcore.rs`
2019-07-20T07:21:52.1650575Z error: 1 errors occurred comparing output.
2019-07-20T07:21:52.1650620Z status: exit code: 1
2019-07-20T07:21:52.1650620Z status: exit code: 1
2019-07-20T07:21:52.1651537Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore/auxiliary" "-A" "unused"
2019-07-20T07:21:52.1651916Z ------------------------------------------
2019-07-20T07:21:52.1651952Z 
2019-07-20T07:21:52.1652176Z ------------------------------------------
2019-07-20T07:21:52.1652242Z stderr:
2019-07-20T07:21:52.1652242Z stderr:
2019-07-20T07:21:52.1652461Z ------------------------------------------
2019-07-20T07:21:52.1652512Z error: any use of this value will cause an error
2019-07-20T07:21:52.1652839Z    |
2019-07-20T07:21:52.1652839Z    |
2019-07-20T07:21:52.1652885Z LL | const Z: () = panic!("cheese");
2019-07-20T07:21:52.1653121Z    | --------------^^^^^^^^^^^^^^^^-
2019-07-20T07:21:52.1653225Z    |               the evaluated program panicked
2019-07-20T07:21:52.1653268Z    |
2019-07-20T07:21:52.1653331Z    = note: `#[deny(const_err)]` on by default
2019-07-20T07:21:52.1653653Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-20T07:21:52.1653653Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-20T07:21:52.1653706Z 
2019-07-20T07:21:52.1653773Z error: any use of this value will cause an error
2019-07-20T07:21:52.1654082Z    |
2019-07-20T07:21:52.1654082Z    |
2019-07-20T07:21:52.1654147Z LL | const Y: () = unreachable!();
2019-07-20T07:21:52.1654361Z    | --------------^^^^^^^^^^^^^^-
2019-07-20T07:21:52.1654477Z    |               the evaluated program panicked
2019-07-20T07:21:52.1654521Z    |
2019-07-20T07:21:52.1658182Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-20T07:21:52.1658288Z 
2019-07-20T07:21:52.1658288Z 
2019-07-20T07:21:52.1658367Z error: any use of this value will cause an error
2019-07-20T07:21:52.1658702Z    |
2019-07-20T07:21:52.1658702Z    |
2019-07-20T07:21:52.1658907Z LL | const X: () = unimplemented!();
2019-07-20T07:21:52.1659153Z    | --------------^^^^^^^^^^^^^^^^-
2019-07-20T07:21:52.1659269Z    |               the evaluated program panicked
2019-07-20T07:21:52.1659312Z    |
2019-07-20T07:21:52.1659631Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-20T07:21:52.1659675Z 
---
2019-07-20T07:21:52.1660431Z 
2019-07-20T07:21:52.1660684Z ---- [ui] ui/consts/const-eval/const_panic_libcore_main.rs stdout ----
2019-07-20T07:21:52.1660735Z diff of stderr:
2019-07-20T07:21:52.1660783Z 
2019-07-20T07:21:52.1660827Z 4 LL | const Z: () = panic!("cheese");
2019-07-20T07:21:52.1661050Z 5    | --------------^^^^^^^^^^^^^^^^-
2019-07-20T07:21:52.1661106Z 6    |               |
2019-07-20T07:21:52.1661408Z -    |               the evaluated program panicked at 'cheese', $DIR/const_panic_libcore_main.rs:9:15
2019-07-20T07:21:52.1661466Z +    |               the evaluated program panicked
2019-07-20T07:21:52.1661576Z 9    = note: `#[deny(const_err)]` on by default
2019-07-20T07:21:52.1661900Z 10    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-20T07:21:52.1661942Z 
2019-07-20T07:21:52.1661942Z 
2019-07-20T07:21:52.1662008Z 15 LL | const Y: () = unreachable!();
2019-07-20T07:21:52.1662326Z 16    | --------------^^^^^^^^^^^^^^-
2019-07-20T07:21:52.1662381Z 17    |               |
2019-07-20T07:21:52.1662740Z -    |               the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic_libcore_main.rs:12:15
2019-07-20T07:21:52.1662800Z +    |               the evaluated program panicked
2019-07-20T07:21:52.1663194Z 20    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-20T07:21:52.1663249Z 21 
2019-07-20T07:21:52.1663280Z 
2019-07-20T07:21:52.1663280Z 
2019-07-20T07:21:52.1663323Z 25 LL | const X: () = unimplemented!();
2019-07-20T07:21:52.1663563Z 26    | --------------^^^^^^^^^^^^^^^^-
2019-07-20T07:21:52.1663612Z 27    |               |
2019-07-20T07:21:52.1663907Z -    |               the evaluated program panicked at 'not yet implemented', $DIR/const_panic_libcore_main.rs:15:15
2019-07-20T07:21:52.1663991Z +    |               the evaluated program panicked
2019-07-20T07:21:52.1664356Z 30    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-20T07:21:52.1664430Z 31 
2019-07-20T07:21:52.1664458Z 
2019-07-20T07:21:52.1664493Z 
2019-07-20T07:21:52.1664493Z 
2019-07-20T07:21:52.1664538Z The actual stderr differed from the expected stderr.
2019-07-20T07:21:52.1664892Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main/const_panic_libcore_main.stderr
2019-07-20T07:21:52.1665150Z To update references, rerun the tests and pass the `--bless` flag
2019-07-20T07:21:52.1665473Z To only update this specific test, also pass `--test-args consts/const-eval/const_panic_libcore_main.rs`
2019-07-20T07:21:52.1665581Z error: 1 errors occurred comparing output.
2019-07-20T07:21:52.1665628Z status: exit code: 1
2019-07-20T07:21:52.1665628Z status: exit code: 1
2019-07-20T07:21:52.1666581Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main/auxiliary" "-A" "unused"
2019-07-20T07:21:52.1667013Z ------------------------------------------
2019-07-20T07:21:52.1667049Z 
2019-07-20T07:21:52.1667269Z ------------------------------------------
2019-07-20T07:21:52.1667313Z stderr:
2019-07-20T07:21:52.1667313Z stderr:
2019-07-20T07:21:52.1675625Z ------------------------------------------
2019-07-20T07:21:52.1675701Z error: any use of this value will cause an error
2019-07-20T07:21:52.1676203Z    |
2019-07-20T07:21:52.1676203Z    |
2019-07-20T07:21:52.1676413Z LL | const Z: () = panic!("cheese");
2019-07-20T07:21:52.1676632Z    | --------------^^^^^^^^^^^^^^^^-
2019-07-20T07:21:52.1676751Z    |               the evaluated program panicked
2019-07-20T07:21:52.1676793Z    |
2019-07-20T07:21:52.1676836Z    = note: `#[deny(const_err)]` on by default
2019-07-20T07:21:52.1677158Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-20T07:21:52.1677158Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-20T07:21:52.1677202Z 
2019-07-20T07:21:52.1677254Z error: any use of this value will cause an error
2019-07-20T07:21:52.1677971Z    |
2019-07-20T07:21:52.1677971Z    |
2019-07-20T07:21:52.1678011Z LL | const Y: () = unreachable!();
2019-07-20T07:21:52.1678387Z    | --------------^^^^^^^^^^^^^^-
2019-07-20T07:21:52.1678486Z    |               the evaluated program panicked
2019-07-20T07:21:52.1678546Z    |
2019-07-20T07:21:52.1678863Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-20T07:21:52.1678912Z 
2019-07-20T07:21:52.1678912Z 
2019-07-20T07:21:52.1678956Z error: any use of this value will cause an error
2019-07-20T07:21:52.1679298Z    |
2019-07-20T07:21:52.1679298Z    |
2019-07-20T07:21:52.1679340Z LL | const X: () = unimplemented!();
2019-07-20T07:21:52.1679573Z    | --------------^^^^^^^^^^^^^^^^-
2019-07-20T07:21:52.1679663Z    |               the evaluated program panicked
2019-07-20T07:21:52.1679723Z    |
2019-07-20T07:21:52.1680714Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-20T07:21:52.1680763Z 
---
2019-07-20T07:21:52.1684330Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-20T07:21:52.1684395Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-20T07:21:52.1684444Z 
2019-07-20T07:21:52.1684468Z 
2019-07-20T07:21:52.1685814Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-20T07:21:52.1686230Z 
2019-07-20T07:21:52.1686258Z 
2019-07-20T07:21:52.1686300Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-20T07:21:52.1686365Z Build completed unsuccessfully in 0:55:54
2019-07-20T07:21:52.1686365Z Build completed unsuccessfully in 0:55:54
2019-07-20T07:21:53.3243412Z ##[error]Bash exited with code '1'.
2019-07-20T07:21:53.3281195Z ##[section]Starting: Checkout
2019-07-20T07:21:53.3283007Z ==============================================================================
2019-07-20T07:21:53.3283085Z Task         : Get sources
2019-07-20T07:21:53.3283135Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
