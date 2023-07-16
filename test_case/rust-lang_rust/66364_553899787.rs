plain
2019-11-14T12:38:58.5273172Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-14T12:38:58.5465497Z ##[command]git config gc.auto 0
2019-11-14T12:38:58.5570656Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-14T12:38:58.5625929Z ##[command]git config --get-all http.proxy
2019-11-14T12:38:58.5794933Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66364/merge:refs/remotes/pull/66364/merge
---
2019-11-14T13:45:48.3442333Z .................................................................................................... 1500/9241
2019-11-14T13:45:55.2355266Z .................................................................................................... 1600/9241
2019-11-14T13:46:05.9478446Z .................................................................................................... 1700/9241
2019-11-14T13:46:15.8108248Z ....i............................................................................................... 1800/9241
2019-11-14T13:46:23.6089383Z ........................................................................................iiiii....... 1900/9241
2019-11-14T13:46:48.8013154Z .................................................................................................... 2100/9241
2019-11-14T13:46:51.5906237Z .................................................................................................... 2200/9241
2019-11-14T13:46:54.5231293Z .................................................................................................... 2300/9241
2019-11-14T13:47:02.5593260Z .................................................................................................... 2400/9241
---
2019-11-14T13:50:28.8206195Z .......................................................................................iF........... 4700/9241
2019-11-14T13:50:36.7635236Z ...i................................................................................................ 4800/9241
2019-11-14T13:50:47.7029271Z .................................................................................................... 4900/9241
2019-11-14T13:50:54.0903235Z .................................................................................................... 5000/9241
2019-11-14T13:51:06.6208508Z ..........................................................................................ii.ii..... 5100/9241
2019-11-14T13:51:16.4392818Z .........................i.......................................................................... 5300/9241
2019-11-14T13:51:26.6835323Z .................................................................................................... 5400/9241
2019-11-14T13:51:37.0940204Z ........................................................................i........................... 5500/9241
2019-11-14T13:51:45.9952524Z ....................................................F............................................... 5600/9241
2019-11-14T13:51:45.9952524Z ....................................................F............................................... 5600/9241
2019-11-14T13:51:54.4125425Z .................................................................................................... 5700/9241
2019-11-14T13:52:05.9749294Z ..........................................................ii...i..ii...........i.................... 5800/9241
2019-11-14T13:52:31.9202676Z .................................................................................................... 6000/9241
2019-11-14T13:52:41.4474729Z .................................................................................................... 6100/9241
2019-11-14T13:52:41.4474729Z .................................................................................................... 6100/9241
2019-11-14T13:52:50.0343478Z .............................................................................i..ii.................. 6200/9241
2019-11-14T13:53:23.0941304Z .................................................................................................... 6400/9241
2019-11-14T13:53:26.4754169Z .............................................i...................................................... 6500/9241
2019-11-14T13:53:29.1817181Z .................................................................................................... 6600/9241
2019-11-14T13:53:31.9778724Z ...............................i.................................................................... 6700/9241
---
2019-11-14T13:59:08.9161744Z diff of stderr:
2019-11-14T13:59:08.9163070Z 
2019-11-14T13:59:08.9163667Z 2   --> $DIR/mismatched-types.rs:2:20
2019-11-14T13:59:08.9163975Z 3    |
2019-11-14T13:59:08.9164235Z 4 LL |     let b: &[u8] = include_str!("file.txt");
2019-11-14T13:59:08.9165062Z +    |                    ^^^^^^^^^^^^^^^^^^^^^^^^
2019-11-14T13:59:08.9165296Z +    |                    |
2019-11-14T13:59:08.9165515Z +    |                    expected slice, found str
2019-11-14T13:59:08.9165763Z +    |                    in this macro invocation
2019-11-14T13:59:08.9165763Z +    |                    in this macro invocation
2019-11-14T13:59:08.9166345Z 6    |
2019-11-14T13:59:08.9166875Z 7    = note: expected type `&[u8]`
2019-11-14T13:59:08.9167487Z 8               found type `&'static str`
2019-11-14T13:59:08.9167963Z 
2019-11-14T13:59:08.9168455Z 11   --> $DIR/mismatched-types.rs:3:19
2019-11-14T13:59:08.9168769Z 12    |
2019-11-14T13:59:08.9168995Z 13 LL |     let s: &str = include_bytes!("file.txt");
2019-11-14T13:59:08.9169521Z -    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected str, found array of 0 elements
2019-11-14T13:59:08.9170582Z +    |                   |
2019-11-14T13:59:08.9170830Z +    |                   expected str, found array of 0 elements
2019-11-14T13:59:08.9171060Z +    |                   in this macro invocation
2019-11-14T13:59:08.9171280Z 15    |
2019-11-14T13:59:08.9171280Z 15    |
2019-11-14T13:59:08.9171515Z 16    = note: expected type `&str`
2019-11-14T13:59:08.9172031Z 17               found type `&'static [u8; 0]`
2019-11-14T13:59:08.9172496Z 
2019-11-14T13:59:08.9173034Z The actual stderr differed from the expected stderr.
2019-11-14T13:59:08.9173741Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/include-macros/mismatched-types/mismatched-types.stderr
2019-11-14T13:59:08.9173741Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/include-macros/mismatched-types/mismatched-types.stderr
2019-11-14T13:59:08.9174342Z To update references, rerun the tests and pass the `--bless` flag
2019-11-14T13:59:08.9175421Z To only update this specific test, also pass `--test-args include-macros/mismatched-types.rs`
2019-11-14T13:59:08.9175841Z error: 1 errors occurred comparing output.
2019-11-14T13:59:08.9175986Z status: exit code: 1
2019-11-14T13:59:08.9175986Z status: exit code: 1
2019-11-14T13:59:08.9177208Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/include-macros/mismatched-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/include-macros/mismatched-types" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/include-macros/mismatched-types/auxiliary" "-A" "unused"
2019-11-14T13:59:08.9177961Z ------------------------------------------
2019-11-14T13:59:08.9178410Z 
2019-11-14T13:59:08.9178977Z ------------------------------------------
2019-11-14T13:59:08.9179186Z stderr:
2019-11-14T13:59:08.9179186Z stderr:
2019-11-14T13:59:08.9179536Z ------------------------------------------
2019-11-14T13:59:08.9179713Z error[E0308]: mismatched types
2019-11-14T13:59:08.9180104Z   --> /checkout/src/test/ui/include-macros/mismatched-types.rs:2:20
2019-11-14T13:59:08.9180288Z    |
2019-11-14T13:59:08.9180441Z LL |     let b: &[u8] = include_str!("file.txt");    //~ ERROR mismatched types
2019-11-14T13:59:08.9180741Z    |                    |
2019-11-14T13:59:08.9180880Z    |                    expected slice, found str
2019-11-14T13:59:08.9181042Z    |                    in this macro invocation
2019-11-14T13:59:08.9181175Z    |
2019-11-14T13:59:08.9181175Z    |
2019-11-14T13:59:08.9181342Z    = note: expected type `&[u8]`
2019-11-14T13:59:08.9181694Z               found type `&'static str`
2019-11-14T13:59:08.9181852Z 
2019-11-14T13:59:08.9182013Z error[E0308]: mismatched types
2019-11-14T13:59:08.9183099Z   --> /checkout/src/test/ui/include-macros/mismatched-types.rs:3:19
2019-11-14T13:59:08.9183882Z    |
2019-11-14T13:59:08.9184099Z LL |     let s: &str = include_bytes!("file.txt");   //~ ERROR mismatched types
2019-11-14T13:59:08.9184388Z    |                   |
2019-11-14T13:59:08.9184545Z    |                   expected str, found array of 0 elements
2019-11-14T13:59:08.9184692Z    |                   in this macro invocation
2019-11-14T13:59:08.9184826Z    |
2019-11-14T13:59:08.9184826Z    |
2019-11-14T13:59:08.9184994Z    = note: expected type `&str`
2019-11-14T13:59:08.9185435Z               found type `&'static [u8; 0]`
2019-11-14T13:59:08.9185752Z error: aborting due to 2 previous errors
2019-11-14T13:59:08.9186312Z 
2019-11-14T13:59:08.9186747Z For more information about this error, try `rustc --explain E0308`.
2019-11-14T13:59:08.9186916Z 
---
2019-11-14T13:59:08.9199579Z diff of stderr:
2019-11-14T13:59:08.9199610Z 
2019-11-14T13:59:08.9200147Z 2   --> $DIR/issue-48364.rs:2:21
2019-11-14T13:59:08.9200230Z 3    |
2019-11-14T13:59:08.9200276Z 4 LL |     b"".starts_with(stringify!(foo))
2019-11-14T13:59:08.9200844Z +    |                     ^^^^^^^^^^^^^^^
2019-11-14T13:59:08.9200894Z +    |                     |
2019-11-14T13:59:08.9200941Z +    |                     expected slice, found str
2019-11-14T13:59:08.9201007Z +    |                     in this macro invocation
2019-11-14T13:59:08.9201007Z +    |                     in this macro invocation
2019-11-14T13:59:08.9201052Z 6    |
2019-11-14T13:59:08.9201113Z 7    = note: expected type `&[u8]`
2019-11-14T13:59:08.9201617Z 8               found type `&'static str`
2019-11-14T13:59:08.9201685Z 
2019-11-14T13:59:08.9201711Z 
2019-11-14T13:59:08.9201770Z The actual stderr differed from the expected stderr.
2019-11-14T13:59:08.9202136Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-48364/issue-48364.stderr
2019-11-14T13:59:08.9206911Z To update references, rerun the tests and pass the `--bless` flag
2019-11-14T13:59:08.9207510Z To only update this specific test, also pass `--test-args issues/issue-48364.rs`
2019-11-14T13:59:08.9207632Z error: 1 errors occurred comparing output.
2019-11-14T13:59:08.9207679Z status: exit code: 1
2019-11-14T13:59:08.9207679Z status: exit code: 1
2019-11-14T13:59:08.9208483Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-48364.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-48364" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-48364/auxiliary" "-A" "unused"
2019-11-14T13:59:08.9208828Z ------------------------------------------
2019-11-14T13:59:08.9211954Z 
2019-11-14T13:59:08.9212364Z ------------------------------------------
2019-11-14T13:59:08.9212415Z stderr:
2019-11-14T13:59:08.9212415Z stderr:
2019-11-14T13:59:08.9212632Z ------------------------------------------
2019-11-14T13:59:08.9212708Z error[E0308]: mismatched types
2019-11-14T13:59:08.9214923Z   --> /checkout/src/test/ui/issues/issue-48364.rs:2:21
2019-11-14T13:59:08.9214987Z    |
2019-11-14T13:59:08.9215059Z LL |     b"".starts_with(stringify!(foo))
2019-11-14T13:59:08.9215151Z    |                     |
2019-11-14T13:59:08.9215233Z    |                     expected slice, found str
2019-11-14T13:59:08.9215282Z    |                     in this macro invocation
2019-11-14T13:59:08.9215326Z    |
---
2019-11-14T13:59:08.9217068Z diff of stderr:
2019-11-14T13:59:08.9217114Z 
2019-11-14T13:59:08.9217331Z 67   --> $DIR/macros-nonfatal-errors.rs:27:5
2019-11-14T13:59:08.9217376Z 68    |
2019-11-14T13:59:08.9217632Z 69 LL |     include_str!("i'd be quite surprised if a file with this name existed");
2019-11-14T13:59:08.9218933Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ in this macro invocation
2019-11-14T13:59:08.9218985Z 71 
2019-11-14T13:59:08.9219059Z 72 error: argument must be a string literal
2019-11-14T13:59:08.9219294Z 73   --> $DIR/macros-nonfatal-errors.rs:28:20
2019-11-14T13:59:08.9219294Z 73   --> $DIR/macros-nonfatal-errors.rs:28:20
2019-11-14T13:59:08.9219328Z 
2019-11-14T13:59:08.9219557Z 79   --> $DIR/macros-nonfatal-errors.rs:29:5
2019-11-14T13:59:08.9219603Z 80    |
2019-11-14T13:59:08.9219865Z 81 LL |     include_bytes!("i'd be quite surprised if a file with this name existed");
2019-11-14T13:59:08.9220203Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ in this macro invocation
2019-11-14T13:59:08.9220259Z 83 
2019-11-14T13:59:08.9220259Z 83 
2019-11-14T13:59:08.9220323Z 84 error: trace_macros! accepts only `true` or `false`
2019-11-14T13:59:08.9220580Z 
2019-11-14T13:59:08.9220613Z 
2019-11-14T13:59:08.9220677Z The actual stderr differed from the expected stderr.
2019-11-14T13:59:08.9221004Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macros-nonfatal-errors/macros-nonfatal-errors.stderr
2019-11-14T13:59:08.9221004Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macros-nonfatal-errors/macros-nonfatal-errors.stderr
2019-11-14T13:59:08.9221249Z To update references, rerun the tests and pass the `--bless` flag
2019-11-14T13:59:08.9221533Z To only update this specific test, also pass `--test-args macros/macros-nonfatal-errors.rs`
2019-11-14T13:59:08.9221613Z error: 1 errors occurred comparing output.
2019-11-14T13:59:08.9221658Z status: exit code: 1
2019-11-14T13:59:08.9221658Z status: exit code: 1
2019-11-14T13:59:08.9222568Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macros-nonfatal-errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macros-nonfatal-errors" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macros-nonfatal-errors/auxiliary" "-A" "unused"
2019-11-14T13:59:08.9222908Z ------------------------------------------
2019-11-14T13:59:08.9222940Z 
2019-11-14T13:59:08.9223852Z ------------------------------------------
2019-11-14T13:59:08.9223934Z stderr:
2019-11-14T13:59:08.9223934Z stderr:
2019-11-14T13:59:08.9224209Z ------------------------------------------
2019-11-14T13:59:08.9224263Z error[E0665]: `Default` cannot be derived for enums, only structs
2019-11-14T13:59:08.9224581Z    |
2019-11-14T13:59:08.9224639Z LL | #[derive(Default)] //~ ERROR
2019-11-14T13:59:08.9224700Z    |          ^^^^^^^
2019-11-14T13:59:08.9224730Z 
2019-11-14T13:59:08.9224730Z 
2019-11-14T13:59:08.9230567Z error: inline assembly must be a string literal
2019-11-14T13:59:08.9239534Z   --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:13:10
2019-11-14T13:59:08.9239676Z    |
2019-11-14T13:59:08.9239724Z LL |     asm!(invalid); //~ ERROR
2019-11-14T13:59:08.9239805Z 
2019-11-14T13:59:08.9239805Z 
2019-11-14T13:59:08.9239870Z error: concat_idents! requires ident args.
2019-11-14T13:59:08.9240304Z    |
2019-11-14T13:59:08.9240304Z    |
2019-11-14T13:59:08.9240369Z LL |     concat_idents!("not", "idents"); //~ ERROR
2019-11-14T13:59:08.9240449Z 
2019-11-14T13:59:08.9240494Z error: argument must be a string literal
2019-11-14T13:59:08.9240761Z   --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:17:17
2019-11-14T13:59:08.9240953Z    |
2019-11-14T13:59:08.9240953Z    |
2019-11-14T13:59:08.9240998Z LL |     option_env!(invalid); //~ ERROR
2019-11-14T13:59:08.9253883Z 
2019-11-14T13:59:08.9253947Z error: expected string literal
2019-11-14T13:59:08.9254610Z   --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:18:10
2019-11-14T13:59:08.9254699Z    |
2019-11-14T13:59:08.9254699Z    |
2019-11-14T13:59:08.9254745Z LL |     env!(invalid); //~ ERROR
2019-11-14T13:59:08.9254839Z 
2019-11-14T13:59:08.9254883Z error: expected string literal
2019-11-14T13:59:08.9255143Z   --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:19:10
2019-11-14T13:59:08.9255191Z    |
2019-11-14T13:59:08.9255191Z    |
2019-11-14T13:59:08.9255253Z LL |     env!(foo, abr, baz); //~ ERROR
2019-11-14T13:59:08.9255328Z 
2019-11-14T13:59:08.9255328Z 
2019-11-14T13:59:08.9255376Z error: environment variable `RUST_HOPEFULLY_THIS_DOESNT_EXIST` not defined
2019-11-14T13:59:08.9255698Z    |
2019-11-14T13:59:08.9255698Z    |
2019-11-14T13:59:08.9255746Z LL |     env!("RUST_HOPEFULLY_THIS_DOESNT_EXIST"); //~ ERROR
2019-11-14T13:59:08.9255851Z 
2019-11-14T13:59:08.9255896Z error: format argument must be a string literal
2019-11-14T13:59:08.9256606Z   --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:22:13
2019-11-14T13:59:08.9256669Z    |
2019-11-14T13:59:08.9256669Z    |
2019-11-14T13:59:08.9256713Z LL |     format!(invalid); //~ ERROR
2019-11-14T13:59:08.9256819Z    |
2019-11-14T13:59:08.9256866Z help: you might be missing a string literal to format with
2019-11-14T13:59:08.9256910Z    |
2019-11-14T13:59:08.9256972Z LL |     format!("{}", invalid); //~ ERROR
2019-11-14T13:59:08.9256972Z LL |     format!("{}", invalid); //~ ERROR
2019-11-14T13:59:08.9257016Z    |             ^^^^^
2019-11-14T13:59:08.9257045Z 
2019-11-14T13:59:08.9257088Z error: argument must be a string literal
2019-11-14T13:59:08.9257403Z   --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:24:14
2019-11-14T13:59:08.9257462Z    |
2019-11-14T13:59:08.9257506Z LL |     include!(invalid); //~ ERROR
2019-11-14T13:59:08.9257604Z 
2019-11-14T13:59:08.9257648Z error: argument must be a string literal
2019-11-14T13:59:08.9257892Z   --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:26:18
2019-11-14T13:59:08.9257957Z    |
2019-11-14T13:59:08.9257957Z    |
2019-11-14T13:59:08.9258000Z LL |     include_str!(invalid); //~ ERROR
2019-11-14T13:59:08.9258047Z    |                  ^^^^^^^
2019-11-14T13:59:08.9258093Z 
2019-11-14T13:59:08.9258416Z error: couldn't read /checkout/src/test/ui/macros/i'd be quite surprised if a file with this name existed: No such file or directory (os error 2)
2019-11-14T13:59:08.9258738Z    |
2019-11-14T13:59:08.9258738Z    |
2019-11-14T13:59:08.9259001Z LL |     include_str!("i'd be quite surprised if a file with this name existed"); //~ ERROR
2019-11-14T13:59:08.9259123Z 
2019-11-14T13:59:08.9259341Z error: argument must be a string literal
2019-11-14T13:59:08.9259623Z   --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:28:20
2019-11-14T13:59:08.9259671Z    |
2019-11-14T13:59:08.9259671Z    |
2019-11-14T13:59:08.9259735Z LL |     include_bytes!(invalid); //~ ERROR
2019-11-14T13:59:08.9259781Z    |                    ^^^^^^^
2019-11-14T13:59:08.9259811Z 
2019-11-14T13:59:08.9260473Z error: couldn't read /checkout/src/test/ui/macros/i'd be quite surprised if a file with this name existed: No such file or directory (os error 2)
2019-11-14T13:59:08.9260834Z    |
2019-11-14T13:59:08.9260834Z    |
2019-11-14T13:59:08.9261111Z LL |     include_bytes!("i'd be quite surprised if a file with this name existed"); //~ ERROR
2019-11-14T13:59:08.9261341Z 
2019-11-14T13:59:08.9261341Z 
2019-11-14T13:59:08.9261386Z error: trace_macros! accepts only `true` or `false`
2019-11-14T13:59:08.9261723Z    |
2019-11-14T13:59:08.9261723Z    |
2019-11-14T13:59:08.9261767Z LL |     trace_macros!(invalid); //~ ERROR
2019-11-14T13:59:08.9261859Z 
2019-11-14T13:59:08.9261902Z error: aborting due to 14 previous errors
2019-11-14T13:59:08.9261931Z 
2019-11-14T13:59:08.9261957Z 
---
2019-11-14T13:59:08.9263802Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-14T13:59:08.9263862Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-14T13:59:08.9263894Z 
2019-11-14T13:59:08.9263921Z 
2019-11-14T13:59:08.9265483Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-14T13:59:08.9265720Z 
2019-11-14T13:59:08.9266093Z 
2019-11-14T13:59:08.9266149Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-14T13:59:08.9266199Z Build completed unsuccessfully in 1:13:32
2019-11-14T13:59:08.9266199Z Build completed unsuccessfully in 1:13:32
2019-11-14T13:59:08.9311282Z == clock drift check ==
2019-11-14T13:59:08.9325806Z   local time: Thu Nov 14 13:59:08 UTC 2019
2019-11-14T13:59:09.0833826Z   network time: Thu, 14 Nov 2019 13:59:09 GMT
2019-11-14T13:59:09.0839835Z == end clock drift check ==
2019-11-14T13:59:09.9687498Z 
2019-11-14T13:59:09.9759640Z ##[error]Bash exited with code '1'.
2019-11-14T13:59:09.9799475Z ##[section]Starting: Checkout
2019-11-14T13:59:09.9801248Z ==============================================================================
2019-11-14T13:59:09.9801303Z Task         : Get sources
2019-11-14T13:59:09.9801370Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
