plain
2019-11-08T11:03:56.7663771Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-08T11:03:56.7860878Z ##[command]git config gc.auto 0
2019-11-08T11:03:56.7911773Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-08T11:03:56.7951762Z ##[command]git config --get-all http.proxy
2019-11-08T11:03:56.8104385Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66213/merge:refs/remotes/pull/66213/merge
---
2019-11-08T11:57:57.7567370Z .................................................................................................... 1600/9289
2019-11-08T11:58:02.8267857Z .................................................................................................... 1700/9289
2019-11-08T11:58:13.9763641Z ................................................................i................................... 1800/9289
2019-11-08T11:58:21.1974832Z .................................................................................................... 1900/9289
2019-11-08T11:58:34.3335650Z ................................................iiiii............................................... 2000/9289
2019-11-08T11:58:43.7839369Z .................................................................................................... 2200/9289
2019-11-08T11:58:46.0820616Z .................................................................................................... 2300/9289
2019-11-08T11:58:49.3776007Z .................................................................................................... 2400/9289
2019-11-08T11:59:10.0874312Z .................................................................................................... 2500/9289
---
2019-11-08T12:01:41.1073808Z ............................................i...............i....................................... 4800/9289
2019-11-08T12:01:49.3740868Z .................................................................................................... 4900/9289
2019-11-08T12:01:55.8985564Z .................................................................................................... 5000/9289
2019-11-08T12:02:01.7860203Z .................................................................................................... 5100/9289
2019-11-08T12:02:10.7348737Z ..............................................ii.ii...........i..................................... 5200/9289
2019-11-08T12:02:19.6265590Z .................................................................................................... 5400/9289
2019-11-08T12:02:28.8170327Z .................................................................................................... 5500/9289
2019-11-08T12:02:35.5244518Z ............................i....................................................................... 5600/9289
2019-11-08T12:02:41.3369371Z .................................................................................................... 5700/9289
2019-11-08T12:02:41.3369371Z .................................................................................................... 5700/9289
2019-11-08T12:02:52.0498372Z .................................................................................................... 5800/9289
2019-11-08T12:03:02.7670761Z .............ii...i..ii...........i................................................................. 5900/9289
2019-11-08T12:03:19.4396484Z .................................................................................................... 6100/9289
2019-11-08T12:03:27.0462834Z .................................................................................................... 6200/9289
2019-11-08T12:03:27.0462834Z .................................................................................................... 6200/9289
2019-11-08T12:03:39.8604685Z ................................i..ii............................................................... 6300/9289
2019-11-08T12:03:58.4453647Z .................................................................................................... 6500/9289
2019-11-08T12:04:00.4351133Z i................................................................................................... 6600/9289
2019-11-08T12:04:02.6030294Z ....................................................................................i............... 6700/9289
2019-11-08T12:04:05.0269335Z .................................................................................................... 6800/9289
---
2019-11-08T12:08:53.0429059Z  finished in 5.260
2019-11-08T12:08:53.0599714Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-08T12:08:53.2021256Z 
2019-11-08T12:08:53.2021767Z running 156 tests
2019-11-08T12:08:56.0189685Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/156
2019-11-08T12:08:57.8054288Z .i.i..iiii..............i.........iii.i.........ii......
2019-11-08T12:08:57.8054817Z 
2019-11-08T12:08:57.8058165Z  finished in 4.745
2019-11-08T12:08:57.8234186Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-08T12:08:57.9554544Z 
---
2019-11-08T12:08:59.8549611Z  finished in 2.031
2019-11-08T12:08:59.8728428Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-08T12:09:00.0092133Z 
2019-11-08T12:09:00.0092760Z running 9 tests
2019-11-08T12:09:00.0095494Z iiiiiiiii
2019-11-08T12:09:00.0096221Z 
2019-11-08T12:09:00.0096400Z  finished in 0.136
2019-11-08T12:09:00.0292943Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-08T12:09:00.1844062Z 
---
2019-11-08T12:09:19.0965398Z  finished in 19.067
2019-11-08T12:09:19.1160932Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-08T12:09:19.2718301Z 
2019-11-08T12:09:19.2719181Z running 123 tests
2019-11-08T12:09:40.7504964Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-11-08T12:09:44.9342385Z i.i.i......iii.i.....ii
2019-11-08T12:09:44.9342763Z 
2019-11-08T12:09:44.9343701Z  finished in 25.818
2019-11-08T12:09:44.9351426Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-08T12:09:44.9355309Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-11-08T12:20:33.4468585Z 
2019-11-08T12:20:33.4469651Z    Doc-tests core
2019-11-08T12:20:37.9748624Z 
2019-11-08T12:20:37.9749563Z running 2418 tests
2019-11-08T12:20:47.7226499Z ......iiiii......................................................................................... 100/2418
2019-11-08T12:20:57.3847799Z ................................................................................ii.................. 200/2418
2019-11-08T12:21:19.9094670Z ..i................................................................................................. 400/2418
2019-11-08T12:21:19.9094670Z ..i................................................................................................. 400/2418
2019-11-08T12:21:29.5050606Z ..................................................i..i.................iiii......................... 500/2418
2019-11-08T12:21:47.1386280Z .................................................................................................... 700/2418
2019-11-08T12:21:56.5858419Z .................................................................................................... 800/2418
2019-11-08T12:22:05.9306359Z .................................................................................................... 900/2418
2019-11-08T12:22:15.3517867Z .................................................................................................... 1000/2418
---
2019-11-08T12:25:49.2725113Z ............................................... 300/763
2019-11-08T12:25:49.2746350Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:854:13
2019-11-08T12:25:49.3438088Z .................................................................................................... 400/763
2019-11-08T12:25:51.4176159Z .................................................................................................... 500/763
2019-11-08T12:25:51.4376243Z ....................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-11-08T12:25:51.4392359Z ....thread 'thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:1165:5
2019-11-08T12:25:51.4403548Z .<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-11-08T12:25:51.4431596Z ......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-11-08T12:25:51.6451161Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-11-08T12:25:51.6470814Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:1165:5
2019-11-08T12:25:51.6490408Z .thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', .src/libcore/result.rs.:1165.:5
2019-11-08T12:25:51.6501330Z ...thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-11-08T12:25:53.6968579Z ....................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:629:13
2019-11-08T12:25:53.6982458Z .........thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:584:13
2019-11-08T12:25:53.6985409Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:561:13
2019-11-08T12:25:53.6985714Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:689:13
---
2019-11-08T12:26:02.8296807Z 
2019-11-08T12:26:02.8316433Z running 1000 tests
2019-11-08T12:26:21.8785078Z i................................................................................................... 100/1000
2019-11-08T12:26:32.9336796Z .................................................................................................... 200/1000
2019-11-08T12:26:40.5592377Z ...................iii......i......i...i......i..................................................... 300/1000
2019-11-08T12:26:45.6413796Z .................................................................................................... 400/1000
2019-11-08T12:26:52.8991076Z ...........................................i..i.................................ii.................. 500/1000
2019-11-08T12:27:06.4846954Z .................................................................................................... 700/1000
2019-11-08T12:27:06.4846954Z .................................................................................................... 700/1000
2019-11-08T12:27:14.3832288Z ..........................iiii...................................................................... 800/1000
2019-11-08T12:27:28.4486431Z .................................................................................................... 900/1000
2019-11-08T12:27:35.6847391Z ................................................iiii................................................ 1000/1000
2019-11-08T12:27:35.6848122Z test result: ok. 980 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2019-11-08T12:27:35.6848171Z 
2019-11-08T12:27:35.6898625Z  finished in 178.935
2019-11-08T12:27:35.6910727Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2019-11-08T12:43:44.7049437Z  finished in 38.260
2019-11-08T12:43:44.7365861Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-08T12:43:44.8735818Z 
2019-11-08T12:43:44.8736879Z running 203 tests
2019-11-08T12:44:13.7680008Z ....................i...ii...................................................................i...... 100/203
2019-11-08T12:44:46.6065149Z .................................iiii.......i...........iiii.iii.................................... 200/203
2019-11-08T12:44:46.9305296Z i..
2019-11-08T12:44:46.9306895Z 
2019-11-08T12:44:46.9312790Z  finished in 62.195
2019-11-08T12:44:46.9323399Z doc tests for: /checkout/src/doc/rustdoc/src/command-line-arguments.md
2019-11-08T12:44:46.9396716Z doc tests for: /checkout/src/doc/rustdoc/src/documentation-tests.md
---
2019-11-08T12:45:11.0299913Z failures:
2019-11-08T12:45:11.0300124Z 
2019-11-08T12:45:11.0300755Z ---- [ui] rustdoc-ui/intra-links-warning-crlf.rs stdout ----
2019-11-08T12:45:11.0301031Z 
2019-11-08T12:45:11.0301710Z error: /checkout/src/test/rustdoc-ui/intra-links-warning-crlf.rs:8: unexpected warning: '8:6: 8:11: `[error]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]'
2019-11-08T12:45:11.0302036Z 
2019-11-08T12:45:11.0302585Z error: /checkout/src/test/rustdoc-ui/intra-links-warning-crlf.rs:12: unexpected warning: '12:11: 12:17: `[error1]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]'
2019-11-08T12:45:11.0302811Z 
2019-11-08T12:45:11.0303466Z error: /checkout/src/test/rustdoc-ui/intra-links-warning-crlf.rs:14: unexpected warning: '14:11: 14:17: `[error2]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]'
2019-11-08T12:45:11.0303678Z 
2019-11-08T12:45:11.0304205Z error: /checkout/src/test/rustdoc-ui/intra-links-warning-crlf.rs:21: unexpected warning: '21:20: 21:25: `[error]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]'
2019-11-08T12:45:11.0304301Z error: 4 unexpected errors found, 0 expected errors not found
2019-11-08T12:45:11.0304338Z status: exit code: 0
2019-11-08T12:45:11.0304338Z status: exit code: 0
2019-11-08T12:45:11.0304946Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-links-warning-crlf.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning-crlf" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning-crlf/auxiliary"
2019-11-08T12:45:11.0305020Z unexpected errors (from JSON output): [
2019-11-08T12:45:11.0305108Z         line_num: 8,
2019-11-08T12:45:11.0305141Z         kind: Some(
2019-11-08T12:45:11.0305192Z             Warning,
2019-11-08T12:45:11.0305224Z         ),
2019-11-08T12:45:11.0305224Z         ),
2019-11-08T12:45:11.0305264Z         msg: "8:6: 8:11: `[error]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]",
2019-11-08T12:45:11.0305351Z     Error {
2019-11-08T12:45:11.0305384Z         line_num: 12,
2019-11-08T12:45:11.0305440Z         kind: Some(
2019-11-08T12:45:11.0305475Z             Warning,
2019-11-08T12:45:11.0305475Z             Warning,
2019-11-08T12:45:11.0305506Z         ),
2019-11-08T12:45:11.0305546Z         msg: "12:11: 12:17: `[error1]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]",
2019-11-08T12:45:11.0305628Z     Error {
2019-11-08T12:45:11.0305666Z         line_num: 14,
2019-11-08T12:45:11.0305712Z         kind: Some(
2019-11-08T12:45:11.0305745Z             Warning,
2019-11-08T12:45:11.0305745Z             Warning,
2019-11-08T12:45:11.0305776Z         ),
2019-11-08T12:45:11.0305830Z         msg: "14:11: 14:17: `[error2]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]",
2019-11-08T12:45:11.0305900Z     Error {
2019-11-08T12:45:11.0305932Z         line_num: 21,
2019-11-08T12:45:11.0305981Z         kind: Some(
2019-11-08T12:45:11.0306013Z             Warning,
2019-11-08T12:45:11.0306013Z             Warning,
2019-11-08T12:45:11.0306045Z         ),
2019-11-08T12:45:11.0306198Z         msg: "21:20: 21:25: `[error]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]",
2019-11-08T12:45:11.0306273Z ]
2019-11-08T12:45:11.0306294Z 
2019-11-08T12:45:11.0306576Z thread '[ui] rustdoc-ui/intra-links-warning-crlf.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-11-08T12:45:11.0306624Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-08T12:45:11.0306624Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-08T12:45:11.0306657Z 
2019-11-08T12:45:11.0307221Z ---- [ui] rustdoc-ui/intra-links-warning.rs stdout ----
2019-11-08T12:45:11.0307253Z 
2019-11-08T12:45:11.0307532Z error: /checkout/src/test/rustdoc-ui/intra-links-warning.rs:3: unexpected warning: '3:23: 3:31: `[Foo::baz]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]'
2019-11-08T12:45:11.0307569Z 
2019-11-08T12:45:11.0307864Z error: /checkout/src/test/rustdoc-ui/intra-links-warning.rs:3: unexpected warning: '3:35: 3:43: `[Bar::foo]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]'
2019-11-08T12:45:11.0307906Z 
2019-11-08T12:45:11.0308180Z error: /checkout/src/test/rustdoc-ui/intra-links-warning.rs:4: unexpected warning: '4:13: 4:23: `[Uniooon::X]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]'
2019-11-08T12:45:11.0308228Z 
2019-11-08T12:45:11.0308500Z error: /checkout/src/test/rustdoc-ui/intra-links-warning.rs:4: unexpected warning: '4:30: 4:36: `[Qux::Z]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]'
2019-11-08T12:45:11.0308626Z 
2019-11-08T12:45:11.0308933Z error: /checkout/src/test/rustdoc-ui/intra-links-warning.rs:6: unexpected warning: '6:14: 6:24: `[Uniooon::X]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]'
2019-11-08T12:45:11.0308965Z 
2019-11-08T12:45:11.0309232Z error: /checkout/src/test/rustdoc-ui/intra-links-warning.rs:6: unexpected warning: '6:31: 6:37: `[Qux::Z]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]'
2019-11-08T12:45:11.0309264Z 
2019-11-08T12:45:11.0309840Z error: /checkout/src/test/rustdoc-ui/intra-links-warning.rs:8: unexpected warning: '8:13: 8:18: `[Qux:Y]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]'
2019-11-08T12:45:11.0309883Z 
2019-11-08T12:45:11.0310175Z error: /checkout/src/test/rustdoc-ui/intra-links-warning.rs:51: unexpected warning: '51:30: 51:35: `[error]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]'
2019-11-08T12:45:11.0310232Z 
2019-11-08T12:45:11.0310505Z error: /checkout/src/test/rustdoc-ui/intra-links-warning.rs:57: unexpected warning: '57:30: 57:35: `[error]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]'
2019-11-08T12:45:11.0310538Z 
2019-11-08T12:45:11.0310825Z error: /checkout/src/test/rustdoc-ui/intra-links-warning.rs:61: unexpected warning: '61:1: 61:31: `[error]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]'
2019-11-08T12:45:11.0310857Z 
2019-11-08T12:45:11.0311132Z error: /checkout/src/test/rustdoc-ui/intra-links-warning.rs:64: unexpected warning: '64:1: 64:49: `[error]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]'
2019-11-08T12:45:11.0311165Z 
2019-11-08T12:45:11.0311450Z error: /checkout/src/test/rustdoc-ui/intra-links-warning.rs:67: unexpected warning: '67:1: 69:12: `[error]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]'
2019-11-08T12:45:11.0311482Z 
2019-11-08T12:45:11.0311759Z error: /checkout/src/test/rustdoc-ui/intra-links-warning.rs:73: unexpected warning: '73:11: 73:17: `[error1]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]'
2019-11-08T12:45:11.0312001Z 
2019-11-08T12:45:11.0312354Z error: /checkout/src/test/rustdoc-ui/intra-links-warning.rs:75: unexpected warning: '75:11: 75:17: `[error2]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]'
2019-11-08T12:45:11.0312386Z 
2019-11-08T12:45:11.0312739Z error: /checkout/src/test/rustdoc-ui/intra-links-warning.rs:14: unexpected warning: '14:10: 14:14: `[BarA]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]'
2019-11-08T12:45:11.0312798Z 
2019-11-08T12:45:11.0313284Z error: /checkout/src/test/rustdoc-ui/intra-links-warning.rs:20: unexpected warning: '20:9: 20:13: `[BarB]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]'
2019-11-08T12:45:11.0313317Z 
2019-11-08T12:45:11.0313797Z error: /checkout/src/test/rustdoc-ui/intra-links-warning.rs:27: unexpected warning: '27:6: 27:10: `[BarC]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]'
2019-11-08T12:45:11.0313840Z 
2019-11-08T12:45:11.0314135Z error: /checkout/src/test/rustdoc-ui/intra-links-warning.rs:38: unexpected warning: '38:1: 38:36: `[BarD]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]'
2019-11-08T12:45:11.0314168Z 
2019-11-08T12:45:11.0314844Z error: /checkout/src/test/rustdoc-ui/intra-links-warning.rs:43: unexpected warning: '43:9: 43:20: `[BarF]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]'
2019-11-08T12:45:11.0315155Z error: 19 unexpected errors found, 0 expected errors not found
2019-11-08T12:45:11.0315247Z status: exit code: 0
2019-11-08T12:45:11.0315247Z status: exit code: 0
2019-11-08T12:45:11.0315949Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-links-warning.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning/auxiliary"
2019-11-08T12:45:11.0316307Z unexpected errors (from JSON output): [
2019-11-08T12:45:11.0316565Z         line_num: 3,
2019-11-08T12:45:11.0316619Z         kind: Some(
2019-11-08T12:45:11.0316657Z             Warning,
2019-11-08T12:45:11.0316694Z         ),
2019-11-08T12:45:11.0316694Z         ),
2019-11-08T12:45:11.0316767Z         msg: "3:23: 3:31: `[Foo::baz]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]",
2019-11-08T12:45:11.0317071Z     Error {
2019-11-08T12:45:11.0317130Z         line_num: 3,
2019-11-08T12:45:11.0317169Z         kind: Some(
2019-11-08T12:45:11.0317206Z             Warning,
2019-11-08T12:45:11.0317206Z             Warning,
2019-11-08T12:45:11.0317242Z         ),
2019-11-08T12:45:11.0317476Z         msg: "3:35: 3:43: `[Bar::foo]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]",
2019-11-08T12:45:11.0317552Z     Error {
2019-11-08T12:45:11.0317604Z         line_num: 4,
2019-11-08T12:45:11.0317641Z         kind: Some(
2019-11-08T12:45:11.0317677Z             Warning,
2019-11-08T12:45:11.0317677Z             Warning,
2019-11-08T12:45:11.0317728Z         ),
2019-11-08T12:45:11.0317773Z         msg: "4:13: 4:23: `[Uniooon::X]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]",
2019-11-08T12:45:11.0317850Z     Error {
2019-11-08T12:45:11.0317968Z         line_num: 4,
2019-11-08T12:45:11.0318004Z         kind: Some(
2019-11-08T12:45:11.0318040Z             Warning,
2019-11-08T12:45:11.0318040Z             Warning,
2019-11-08T12:45:11.0318093Z         ),
2019-11-08T12:45:11.0318137Z         msg: "4:30: 4:36: `[Qux::Z]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]",
2019-11-08T12:45:11.0318465Z     Error {
2019-11-08T12:45:11.0318528Z         line_num: 6,
2019-11-08T12:45:11.0318730Z         kind: Some(
2019-11-08T12:45:11.0318765Z             Warning,
2019-11-08T12:45:11.0318765Z             Warning,
2019-11-08T12:45:11.0318815Z         ),
2019-11-08T12:45:11.0318857Z         msg: "6:14: 6:24: `[Uniooon::X]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]",
2019-11-08T12:45:11.0318947Z     Error {
2019-11-08T12:45:11.0318980Z         line_num: 6,
2019-11-08T12:45:11.0319014Z         kind: Some(
2019-11-08T12:45:11.0319065Z             Warning,
2019-11-08T12:45:11.0319065Z             Warning,
2019-11-08T12:45:11.0319098Z         ),
2019-11-08T12:45:11.0319226Z         msg: "6:31: 6:37: `[Qux::Z]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]",
2019-11-08T12:45:11.0319319Z     Error {
2019-11-08T12:45:11.0319353Z         line_num: 8,
2019-11-08T12:45:11.0319387Z         kind: Some(
2019-11-08T12:45:11.0319440Z             Warning,
2019-11-08T12:45:11.0319440Z             Warning,
2019-11-08T12:45:11.0319473Z         ),
2019-11-08T12:45:11.0319522Z         msg: "8:13: 8:18: `[Qux:Y]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]",
2019-11-08T12:45:11.0319609Z     Error {
2019-11-08T12:45:11.0319643Z         line_num: 51,
2019-11-08T12:45:11.0319676Z         kind: Some(
2019-11-08T12:45:11.0319727Z             Warning,
2019-11-08T12:45:11.0319727Z             Warning,
2019-11-08T12:45:11.0319761Z         ),
2019-11-08T12:45:11.0319802Z         msg: "51:30: 51:35: `[error]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]",
2019-11-08T12:45:11.0319889Z     Error {
2019-11-08T12:45:11.0319922Z         line_num: 57,
2019-11-08T12:45:11.0319979Z         kind: Some(
2019-11-08T12:45:11.0320013Z             Warning,
2019-11-08T12:45:11.0320013Z             Warning,
2019-11-08T12:45:11.0320046Z         ),
2019-11-08T12:45:11.0320088Z         msg: "57:30: 57:35: `[error]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]",
2019-11-08T12:45:11.0320174Z     Error {
2019-11-08T12:45:11.0320268Z         line_num: 61,
2019-11-08T12:45:11.0320319Z         kind: Some(
2019-11-08T12:45:11.0320352Z             Warning,
2019-11-08T12:45:11.0320352Z             Warning,
2019-11-08T12:45:11.0320385Z         ),
2019-11-08T12:45:11.0320442Z         msg: "61:1: 61:31: `[error]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]",
2019-11-08T12:45:11.0320512Z     Error {
2019-11-08T12:45:11.0320545Z         line_num: 64,
2019-11-08T12:45:11.0320595Z         kind: Some(
2019-11-08T12:45:11.0320789Z             Warning,
2019-11-08T12:45:11.0320789Z             Warning,
2019-11-08T12:45:11.0320823Z         ),
2019-11-08T12:45:11.0320886Z         msg: "64:1: 64:49: `[error]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]",
2019-11-08T12:45:11.0321116Z     Error {
2019-11-08T12:45:11.0321307Z         line_num: 67,
2019-11-08T12:45:11.0321342Z         kind: Some(
2019-11-08T12:45:11.0321374Z             Warning,
2019-11-08T12:45:11.0321374Z             Warning,
2019-11-08T12:45:11.0321405Z         ),
2019-11-08T12:45:11.0321464Z         msg: "67:1: 69:12: `[error]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]",
2019-11-08T12:45:11.0321629Z     Error {
2019-11-08T12:45:11.0321678Z         line_num: 73,
2019-11-08T12:45:11.0321710Z         kind: Some(
2019-11-08T12:45:11.0321741Z             Warning,
2019-11-08T12:45:11.0321741Z             Warning,
2019-11-08T12:45:11.0321787Z         ),
2019-11-08T12:45:11.0321828Z         msg: "73:11: 73:17: `[error1]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]",
2019-11-08T12:45:11.0321893Z     Error {
2019-11-08T12:45:11.0322122Z         line_num: 75,
2019-11-08T12:45:11.0322163Z         kind: Some(
2019-11-08T12:45:11.0322196Z             Warning,
2019-11-08T12:45:11.0322196Z             Warning,
2019-11-08T12:45:11.0322244Z         ),
2019-11-08T12:45:11.0322284Z         msg: "75:11: 75:17: `[error2]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]",
2019-11-08T12:45:11.0322548Z     Error {
2019-11-08T12:45:11.0322588Z         line_num: 14,
2019-11-08T12:45:11.0322622Z         kind: Some(
2019-11-08T12:45:11.0322655Z             Warning,
2019-11-08T12:45:11.0322655Z             Warning,
2019-11-08T12:45:11.0322885Z         ),
2019-11-08T12:45:11.0322928Z         msg: "14:10: 14:14: `[BarA]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]",
2019-11-08T12:45:11.0323821Z     Error {
2019-11-08T12:45:11.0324047Z         line_num: 20,
2019-11-08T12:45:11.0324288Z         kind: Some(
2019-11-08T12:45:11.0324358Z             Warning,
2019-11-08T12:45:11.0324358Z             Warning,
2019-11-08T12:45:11.0324395Z         ),
2019-11-08T12:45:11.0324534Z         msg: "20:9: 20:13: `[BarB]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]",
2019-11-08T12:45:11.0324634Z     Error {
2019-11-08T12:45:11.0324671Z         line_num: 27,
2019-11-08T12:45:11.0324709Z         kind: Some(
2019-11-08T12:45:11.0324763Z             Warning,
2019-11-08T12:45:11.0324763Z             Warning,
2019-11-08T12:45:11.0324800Z         ),
2019-11-08T12:45:11.0324847Z         msg: "27:6: 27:10: `[BarC]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]",
2019-11-08T12:45:11.0324948Z     Error {
2019-11-08T12:45:11.0324985Z         line_num: 38,
2019-11-08T12:45:11.0325024Z         kind: Some(
2019-11-08T12:45:11.0325078Z             Warning,
2019-11-08T12:45:11.0325078Z             Warning,
2019-11-08T12:45:11.0325114Z         ),
2019-11-08T12:45:11.0325161Z         msg: "38:1: 38:36: `[BarD]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]",
2019-11-08T12:45:11.0325254Z     Error {
2019-11-08T12:45:11.0325291Z         line_num: 43,
2019-11-08T12:45:11.0325350Z         kind: Some(
2019-11-08T12:45:11.0325387Z             Warning,
2019-11-08T12:45:11.0325387Z             Warning,
2019-11-08T12:45:11.0325423Z         ),
2019-11-08T12:45:11.0325468Z         msg: "43:9: 43:20: `[BarF]` cannot be resolved, ignoring it... [intra_doc_link_resolution_failure]",
2019-11-08T12:45:11.0325724Z ]
2019-11-08T12:45:11.0325749Z 
2019-11-08T12:45:11.0326381Z thread '[ui] rustdoc-ui/intra-links-warning.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-11-08T12:45:11.0326416Z 
2019-11-08T12:45:11.0326416Z 
2019-11-08T12:45:11.0327347Z ---- [ui] rustdoc-ui/invalid-syntax.rs stdout ----
2019-11-08T12:45:11.0327391Z 
2019-11-08T12:45:11.0328412Z error: /checkout/src/test/rustdoc-ui/invalid-syntax.rs:3: unexpected warning: '3:5: 5:8: could not parse code block as Rust code'
2019-11-08T12:45:11.0328453Z 
2019-11-08T12:45:11.0328723Z error: /checkout/src/test/rustdoc-ui/invalid-syntax.rs:8: unexpected warning: '8:5: 12:8: could not parse code block as Rust code'
2019-11-08T12:45:11.0328757Z 
2019-11-08T12:45:11.0329288Z error: /checkout/src/test/rustdoc-ui/invalid-syntax.rs:19: unexpected warning: '19:5: 21:8: could not parse code block as Rust code'
2019-11-08T12:45:11.0329332Z 
2019-11-08T12:45:11.0329749Z error: /checkout/src/test/rustdoc-ui/invalid-syntax.rs:32: unexpected warning: '32:5: 34:8: could not parse code block as Rust code'
2019-11-08T12:45:11.0329920Z 
2019-11-08T12:45:11.0330264Z error: /checkout/src/test/rustdoc-ui/invalid-syntax.rs:41: unexpected warning: '41:9: 42:11: could not parse code block as Rust code'
2019-11-08T12:45:11.0330298Z 
2019-11-08T12:45:11.0331580Z error: /checkout/src/test/rustdoc-ui/invalid-syntax.rs:55: unexpected warning: '55:9: 55:12: could not parse code block as Rust code'
2019-11-08T12:45:11.0331826Z 
2019-11-08T12:45:11.0332122Z error: /checkout/src/test/rustdoc-ui/invalid-syntax.rs:58: unexpected warning: '58:5: 60:8: could not parse code block as Rust code'
2019-11-08T12:45:11.0332316Z 
2019-11-08T12:45:11.0332922Z error: /checkout/src/test/rustdoc-ui/invalid-syntax.rs:63: unexpected warning: '63:1: 65:15: doc comment contains an invalid Rust code block'
2019-11-08T12:45:11.0332979Z 
2019-11-08T12:45:11.0333229Z error: /checkout/src/test/rustdoc-ui/invalid-syntax.rs:68: unexpected warning: '68:5: 69:8: Rust code block is empty'
2019-11-08T12:45:11.0333260Z 
2019-11-08T12:45:11.0333510Z error: /checkout/src/test/rustdoc-ui/invalid-syntax.rs:72: unexpected warning: '72:5: 75:8: Rust code block is empty'
2019-11-08T12:45:11.0333547Z 
2019-11-08T12:45:11.0333973Z error: /checkout/src/test/rustdoc-ui/invalid-syntax.rs:82: unexpected warning: '82:9: 82:15: could not parse code block as Rust code'
2019-11-08T12:45:11.0334063Z error: 11 unexpected errors found, 0 expected errors not found
2019-11-08T12:45:11.0334102Z status: exit code: 0
2019-11-08T12:45:11.0334102Z status: exit code: 0
2019-11-08T12:45:11.0334991Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/invalid-syntax.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-syntax" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-syntax/auxiliary"
2019-11-08T12:45:11.0335096Z unexpected errors (from JSON output): [
2019-11-08T12:45:11.0335200Z         line_num: 3,
2019-11-08T12:45:11.0335237Z         kind: Some(
2019-11-08T12:45:11.0335273Z             Warning,
2019-11-08T12:45:11.0335325Z         ),
2019-11-08T12:45:11.0335325Z         ),
2019-11-08T12:45:11.0335366Z         msg: "3:5: 5:8: could not parse code block as Rust code",
2019-11-08T12:45:11.0335458Z     Error {
2019-11-08T12:45:11.0335495Z         line_num: 8,
2019-11-08T12:45:11.0335531Z         kind: Some(
2019-11-08T12:45:11.0335567Z             Warning,
2019-11-08T12:45:11.0335567Z             Warning,
2019-11-08T12:45:11.0335619Z         ),
2019-11-08T12:45:11.0335666Z         msg: "8:5: 12:8: could not parse code block as Rust code",
2019-11-08T12:45:11.0335757Z     Error {
2019-11-08T12:45:11.0335793Z         line_num: 19,
2019-11-08T12:45:11.0335829Z         kind: Some(
2019-11-08T12:45:11.0335866Z             Warning,
2019-11-08T12:45:11.0335866Z             Warning,
2019-11-08T12:45:11.0335918Z         ),
2019-11-08T12:45:11.0335958Z         msg: "19:5: 21:8: could not parse code block as Rust code",
2019-11-08T12:45:11.0336588Z     Error {
2019-11-08T12:45:11.0336635Z         line_num: 32,
2019-11-08T12:45:11.0336669Z         kind: Some(
2019-11-08T12:45:11.0336721Z             Warning,
2019-11-08T12:45:11.0336721Z             Warning,
2019-11-08T12:45:11.0336754Z         ),
2019-11-08T12:45:11.0336791Z         msg: "32:5: 34:8: could not parse code block as Rust code",
2019-11-08T12:45:11.0336875Z     Error {
2019-11-08T12:45:11.0336909Z         line_num: 41,
2019-11-08T12:45:11.0336942Z         kind: Some(
2019-11-08T12:45:11.0336993Z             Warning,
2019-11-08T12:45:11.0336993Z             Warning,
2019-11-08T12:45:11.0337406Z         ),
2019-11-08T12:45:11.0337628Z         msg: "41:9: 42:11: could not parse code block as Rust code",
2019-11-08T12:45:11.0337710Z     Error {
2019-11-08T12:45:11.0337829Z         line_num: 55,
2019-11-08T12:45:11.0337870Z         kind: Some(
2019-11-08T12:45:11.0337920Z             Warning,
2019-11-08T12:45:11.0337920Z             Warning,
2019-11-08T12:45:11.0337958Z         ),
2019-11-08T12:45:11.0337993Z         msg: "55:9: 55:12: could not parse code block as Rust code",
2019-11-08T12:45:11.0338073Z     Error {
2019-11-08T12:45:11.0338105Z         line_num: 58,
2019-11-08T12:45:11.0338136Z         kind: Some(
2019-11-08T12:45:11.0338186Z             Warning,
2019-11-08T12:45:11.0338186Z             Warning,
2019-11-08T12:45:11.0338217Z         ),
2019-11-08T12:45:11.0338252Z         msg: "58:5: 60:8: could not parse code block as Rust code",
2019-11-08T12:45:11.0338334Z     Error {
2019-11-08T12:45:11.0338366Z         line_num: 63,
2019-11-08T12:45:11.0338419Z         kind: Some(
2019-11-08T12:45:11.0338450Z             Warning,
2019-11-08T12:45:11.0338450Z             Warning,
2019-11-08T12:45:11.0338481Z         ),
2019-11-08T12:45:11.0338516Z         msg: "63:1: 65:15: doc comment contains an invalid Rust code block",
2019-11-08T12:45:11.0338596Z     Error {
2019-11-08T12:45:11.0338627Z         line_num: 68,
2019-11-08T12:45:11.0338680Z         kind: Some(
2019-11-08T12:45:11.0338711Z             Warning,
2019-11-08T12:45:11.0338711Z             Warning,
2019-11-08T12:45:11.0338743Z         ),
2019-11-08T12:45:11.0338793Z         msg: "68:5: 69:8: Rust code block is empty",
2019-11-08T12:45:11.0338856Z     Error {
2019-11-08T12:45:11.0338888Z         line_num: 72,
2019-11-08T12:45:11.0338935Z         kind: Some(
2019-11-08T12:45:11.0338967Z             Warning,
2019-11-08T12:45:11.0338967Z             Warning,
2019-11-08T12:45:11.0338997Z         ),
2019-11-08T12:45:11.0339048Z         msg: "72:5: 75:8: Rust code block is empty",
2019-11-08T12:45:11.0339110Z     Error {
2019-11-08T12:45:11.0339142Z         line_num: 82,
2019-11-08T12:45:11.0339263Z         kind: Some(
2019-11-08T12:45:11.0339303Z             Warning,
2019-11-08T12:45:11.0339303Z             Warning,
2019-11-08T12:45:11.0339334Z         ),
2019-11-08T12:45:11.0339386Z         msg: "82:9: 82:15: could not parse code block as Rust code",
2019-11-08T12:45:11.0339450Z ]
2019-11-08T12:45:11.0339471Z 
2019-11-08T12:45:11.0339810Z thread '[ui] rustdoc-ui/invalid-syntax.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-11-08T12:45:11.0339843Z 
---
2019-11-08T12:45:11.0340725Z 
2019-11-08T12:45:11.0340929Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-08T12:45:11.0359334Z 
2019-11-08T12:45:11.0359407Z 
2019-11-08T12:45:11.0360882Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-08T12:45:11.0361255Z 
2019-11-08T12:45:11.0361277Z 
2019-11-08T12:45:11.0361332Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-08T12:45:11.0361374Z Build completed unsuccessfully in 1:35:03
2019-11-08T12:45:11.0361374Z Build completed unsuccessfully in 1:35:03
2019-11-08T12:45:11.0399603Z == clock drift check ==
2019-11-08T12:45:11.0413098Z   local time: Fri Nov  8 12:45:11 UTC 2019
2019-11-08T12:45:11.1163987Z   network time: Fri, 08 Nov 2019 12:45:11 GMT
2019-11-08T12:45:11.1164114Z == end clock drift check ==
2019-11-08T12:45:12.6749039Z 
2019-11-08T12:45:12.6859159Z ##[error]Bash exited with code '1'.
2019-11-08T12:45:12.6895521Z ##[section]Starting: Checkout
2019-11-08T12:45:12.6896992Z ==============================================================================
2019-11-08T12:45:12.6897049Z Task         : Get sources
2019-11-08T12:45:12.6897087Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
