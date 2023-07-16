plain
2019-10-18T22:46:44.1491872Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-18T22:46:44.1693840Z ##[command]git config gc.auto 0
2019-10-18T22:46:44.1749130Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-18T22:46:44.1828496Z ##[command]git config --get-all http.proxy
2019-10-18T22:46:44.1951503Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65572/merge:refs/remotes/pull/65572/merge
---
2019-10-18T23:47:39.5968409Z .................................................................................................... 1600/9196
2019-10-18T23:47:44.8926547Z .................................................................................................... 1700/9196
2019-10-18T23:47:57.9474736Z .............................i...............i...................................................... 1800/9196
2019-10-18T23:48:05.1780225Z .................................................................................................... 1900/9196
2019-10-18T23:48:19.3128759Z ...................iiiii............................................................................ 2000/9196
2019-10-18T23:48:29.4271777Z .................................................................................................... 2200/9196
2019-10-18T23:48:31.8201922Z .................................................................................................... 2300/9196
2019-10-18T23:48:37.0664551Z .................................................................................................... 2400/9196
2019-10-18T23:48:58.4057524Z .................................................................................................... 2500/9196
---
2019-10-18T23:51:50.4326047Z ......................i...............i............................................................. 4800/9196
2019-10-18T23:52:01.5592828Z .................................................................................................... 4900/9196
2019-10-18T23:52:07.5880236Z .................................................................................................... 5000/9196
2019-10-18T23:52:17.4984390Z .................................................................................................... 5100/9196
2019-10-18T23:52:23.4270127Z ......................ii.ii......................................................................... 5200/9196
2019-10-18T23:52:33.3466846Z .................................................................................................... 5400/9196
2019-10-18T23:52:42.8259923Z ........................................................................................i........... 5500/9196
2019-10-18T23:52:50.3219002Z .................................................................................................... 5600/9196
2019-10-18T23:52:54.9704610Z .................................................................................................... 5700/9196
2019-10-18T23:52:54.9704610Z .................................................................................................... 5700/9196
2019-10-18T23:53:05.4688403Z .....................................................................................ii...i..ii..... 5800/9196
2019-10-18T23:53:30.9318966Z .................................................................................................... 6000/9196
2019-10-18T23:53:38.2139436Z .................................................................................................... 6100/9196
2019-10-18T23:53:42.5881159Z .................................................................................................... 6200/9196
2019-10-18T23:53:42.5881159Z .................................................................................................... 6200/9196
2019-10-18T23:53:55.8915993Z .......i..ii........................................................................................ 6300/9196
2019-10-18T23:54:14.4332836Z ...................................................................i................................ 6500/9196
2019-10-18T23:54:16.3727219Z .................................................................................................... 6600/9196
2019-10-18T23:54:18.6527882Z .........................................i.......................................................... 6700/9196
2019-10-18T23:54:22.2414003Z .................................................................................................... 6800/9196
---
2019-10-18T23:58:20.4004432Z 
2019-10-18T23:58:20.4004610Z 548    = help: the trait `std::ops::Try` is not implemented for `bool`
2019-10-18T23:58:20.4004761Z 549    = note: required by `std::ops::Try::into_result`
2019-10-18T23:58:20.4004894Z 550 
2019-10-18T23:58:20.4005362Z - error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
2019-10-18T23:58:20.4006104Z -    |
2019-10-18T23:58:20.4006104Z -    |
2019-10-18T23:58:20.4006453Z - LL |     if (let 0 = 0)? {}
2019-10-18T23:58:20.4006845Z -    |        ^^^^^^^^^^^^ cannot use the `?` operator in a function that returns `()`
2019-10-18T23:58:20.4008504Z -    = help: the trait `std::ops::Try` is not implemented for `()`
2019-10-18T23:58:20.4009248Z -    = note: required by `std::ops::Try::from_error`
2019-10-18T23:58:20.4009641Z - 
2019-10-18T23:58:20.4009829Z 560 error[E0308]: mismatched types
2019-10-18T23:58:20.4009829Z 560 error[E0308]: mismatched types
2019-10-18T23:58:20.4010201Z 561   --> $DIR/disallowed-positions.rs:56:8
2019-10-18T23:58:20.4010422Z 562    |
2019-10-18T23:58:20.4010559Z 
2019-10-18T23:58:20.4010872Z 736    = help: the trait `std::ops::Try` is not implemented for `bool`
2019-10-18T23:58:20.4011033Z 737    = note: required by `std::ops::Try::into_result`
2019-10-18T23:58:20.4011162Z 738 
2019-10-18T23:58:20.4011579Z - error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
2019-10-18T23:58:20.4012290Z -    |
2019-10-18T23:58:20.4012290Z -    |
2019-10-18T23:58:20.4012651Z - LL |     while (let 0 = 0)? {}
2019-10-18T23:58:20.4013046Z -    |           ^^^^^^^^^^^^ cannot use the `?` operator in a function that returns `()`
2019-10-18T23:58:20.4013769Z -    = help: the trait `std::ops::Try` is not implemented for `()`
2019-10-18T23:58:20.4014149Z -    = note: required by `std::ops::Try::from_error`
2019-10-18T23:58:20.4014461Z - 
2019-10-18T23:58:20.4014670Z 748 error[E0308]: mismatched types
2019-10-18T23:58:20.4014670Z 748 error[E0308]: mismatched types
2019-10-18T23:58:20.4015012Z 749   --> $DIR/disallowed-positions.rs:120:11
2019-10-18T23:58:20.4015178Z 750    |
2019-10-18T23:58:20.4015322Z 
2019-10-18T23:58:20.4015574Z 912    = help: the trait `std::ops::Try` is not implemented for `bool`
2019-10-18T23:58:20.4015716Z 913    = note: required by `std::ops::Try::into_result`
2019-10-18T23:58:20.4015876Z 914 
2019-10-18T23:58:20.4016302Z - error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
2019-10-18T23:58:20.4017026Z -    |
2019-10-18T23:58:20.4017026Z -    |
2019-10-18T23:58:20.4017815Z - LL |     (let 0 = 0)?;
2019-10-18T23:58:20.4018319Z -    |     ^^^^^^^^^^^^ cannot use the `?` operator in a function that returns `()`
2019-10-18T23:58:20.4019122Z -    = help: the trait `std::ops::Try` is not implemented for `()`
2019-10-18T23:58:20.4019696Z -    = note: required by `std::ops::Try::from_error`
2019-10-18T23:58:20.4020136Z - 
2019-10-18T23:58:20.4020325Z 924 error[E0308]: mismatched types
2019-10-18T23:58:20.4020325Z 924 error[E0308]: mismatched types
2019-10-18T23:58:20.4020869Z 925   --> $DIR/disallowed-positions.rs:198:10
2019-10-18T23:58:20.4021043Z 926    |
2019-10-18T23:58:20.4021160Z 
2019-10-18T23:58:20.4021313Z 989 LL |         true && let 1 = 1
2019-10-18T23:58:20.4021580Z 991 
2019-10-18T23:58:20.4021934Z - error: aborting due to 109 previous errors
2019-10-18T23:58:20.4022109Z + error: aborting due to 106 previous errors
2019-10-18T23:58:20.4022241Z 993 
2019-10-18T23:58:20.4022241Z 993 
2019-10-18T23:58:20.4022400Z 994 Some errors have detailed explanations: E0019, E0277, E0308, E0600, E0614.
2019-10-18T23:58:20.4022785Z 995 For more information about an error, try `rustc --explain E0019`.
2019-10-18T23:58:20.4022936Z 
2019-10-18T23:58:20.4023071Z 
2019-10-18T23:58:20.4023215Z The actual stderr differed from the expected stderr.
2019-10-18T23:58:20.4023645Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions/disallowed-positions.stderr
2019-10-18T23:58:20.4078119Z To update references, rerun the tests and pass the `--bless` flag
2019-10-18T23:58:20.4078515Z To only update this specific test, also pass `--test-args rfc-2497-if-let-chains/disallowed-positions.rs`
2019-10-18T23:58:20.4078631Z error: 1 errors occurred comparing output.
2019-10-18T23:58:20.4078681Z status: exit code: 1
2019-10-18T23:58:20.4078681Z status: exit code: 1
2019-10-18T23:58:20.4079516Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions/auxiliary" "-A" "unused"
2019-10-18T23:58:20.4080147Z ------------------------------------------
2019-10-18T23:58:20.4080204Z 
2019-10-18T23:58:20.4080443Z ------------------------------------------
2019-10-18T23:58:20.4080491Z stderr:
2019-10-18T23:58:20.4080491Z stderr:
2019-10-18T23:58:20.4080720Z ------------------------------------------
2019-10-18T23:58:20.4080944Z error: expected one of `,` or `>`, found `&&`
2019-10-18T23:58:20.4081400Z    |
2019-10-18T23:58:20.4081400Z    |
2019-10-18T23:58:20.4081464Z LL |         true && let 1 = 1 //~ ERROR expected one of `,` or `>`, found `&&`
2019-10-18T23:58:20.4081521Z    |              ^^ expected one of `,` or `>` here
2019-10-18T23:58:20.4081610Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4082011Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:32:9
2019-10-18T23:58:20.4082055Z    |
2019-10-18T23:58:20.4082055Z    |
2019-10-18T23:58:20.4082113Z LL |     if &let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4082193Z    |
2019-10-18T23:58:20.4082193Z    |
2019-10-18T23:58:20.4082409Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4082476Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4082719Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4082966Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:35:9
2019-10-18T23:58:20.4083019Z    |
2019-10-18T23:58:20.4083019Z    |
2019-10-18T23:58:20.4083066Z LL |     if !let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4083168Z    |
2019-10-18T23:58:20.4083168Z    |
2019-10-18T23:58:20.4083510Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4083575Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4083666Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4083943Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:36:9
2019-10-18T23:58:20.4084004Z    |
2019-10-18T23:58:20.4084004Z    |
2019-10-18T23:58:20.4084049Z LL |     if *let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4084148Z    |
2019-10-18T23:58:20.4084148Z    |
2019-10-18T23:58:20.4084390Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4084453Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4084542Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4084939Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:38:9
2019-10-18T23:58:20.4084989Z    |
2019-10-18T23:58:20.4084989Z    |
2019-10-18T23:58:20.4085226Z LL |     if -let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4085306Z    |
2019-10-18T23:58:20.4085306Z    |
2019-10-18T23:58:20.4085537Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4085587Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4085654Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4086050Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:46:9
2019-10-18T23:58:20.4086091Z    |
2019-10-18T23:58:20.4086091Z    |
2019-10-18T23:58:20.4086217Z LL |     if (let 0 = 0)? {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4086313Z    |
2019-10-18T23:58:20.4086313Z    |
2019-10-18T23:58:20.4086568Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4086640Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4086710Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4087120Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:50:16
2019-10-18T23:58:20.4087179Z    |
2019-10-18T23:58:20.4087179Z    |
2019-10-18T23:58:20.4087223Z LL |     if true || let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4087711Z    |
2019-10-18T23:58:20.4087711Z    |
2019-10-18T23:58:20.4088008Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4088065Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4088172Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4088433Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:51:17
2019-10-18T23:58:20.4088482Z    |
2019-10-18T23:58:20.4088482Z    |
2019-10-18T23:58:20.4088556Z LL |     if (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4088648Z    |
2019-10-18T23:58:20.4088648Z    |
2019-10-18T23:58:20.4088914Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4088970Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4089057Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4089312Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:52:25
2019-10-18T23:58:20.4089361Z    |
2019-10-18T23:58:20.4089361Z    |
2019-10-18T23:58:20.4089410Z LL |     if true && (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4089518Z    |
2019-10-18T23:58:20.4089518Z    |
2019-10-18T23:58:20.4089767Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4089946Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4090034Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4090333Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:53:25
2019-10-18T23:58:20.4090382Z    |
2019-10-18T23:58:20.4090382Z    |
2019-10-18T23:58:20.4090432Z LL |     if true || (true && let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4090531Z    |
2019-10-18T23:58:20.4090531Z    |
2019-10-18T23:58:20.4090941Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4090988Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4091066Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4091284Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:56:12
2019-10-18T23:58:20.4091330Z    |
2019-10-18T23:58:20.4091330Z    |
2019-10-18T23:58:20.4091377Z LL |     if x = let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4091458Z    |
2019-10-18T23:58:20.4091458Z    |
2019-10-18T23:58:20.4091669Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4091716Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4091794Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4092007Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:59:15
2019-10-18T23:58:20.4092047Z    |
2019-10-18T23:58:20.4092047Z    |
2019-10-18T23:58:20.4092189Z LL |     if true..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4092266Z    |
2019-10-18T23:58:20.4092266Z    |
2019-10-18T23:58:20.4092522Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4092578Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4092642Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4092876Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:61:11
2019-10-18T23:58:20.4092917Z    |
2019-10-18T23:58:20.4092917Z    |
2019-10-18T23:58:20.4092957Z LL |     if ..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4093042Z    |
2019-10-18T23:58:20.4093042Z    |
2019-10-18T23:58:20.4093253Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4093309Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4093379Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4093594Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:63:9
2019-10-18T23:58:20.4093640Z    |
2019-10-18T23:58:20.4093640Z    |
2019-10-18T23:58:20.4093690Z LL |     if (let 0 = 0).. {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4093784Z    |
2019-10-18T23:58:20.4093784Z    |
2019-10-18T23:58:20.4094011Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4094062Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4094146Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4094376Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:67:8
2019-10-18T23:58:20.4094418Z    |
2019-10-18T23:58:20.4094418Z    |
2019-10-18T23:58:20.4094476Z LL |     if let Range { start: _, end: _ } = true..true && false {}
2019-10-18T23:58:20.4094564Z    |
2019-10-18T23:58:20.4094564Z    |
2019-10-18T23:58:20.4094919Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4095046Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4095120Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4095386Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:71:8
2019-10-18T23:58:20.4095429Z    |
2019-10-18T23:58:20.4095429Z    |
2019-10-18T23:58:20.4095472Z LL |     if let Range { start: _, end: _ } = true..true || false {}
2019-10-18T23:58:20.4095568Z    |
2019-10-18T23:58:20.4095568Z    |
2019-10-18T23:58:20.4095800Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4095865Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4095944Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4096198Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:78:8
2019-10-18T23:58:20.4096243Z    |
2019-10-18T23:58:20.4096243Z    |
2019-10-18T23:58:20.4096290Z LL |     if let Range { start: F, end } = F..|| true {}
2019-10-18T23:58:20.4096392Z    |
2019-10-18T23:58:20.4096392Z    |
2019-10-18T23:58:20.4096626Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4096677Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4096759Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4096997Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:86:8
2019-10-18T23:58:20.4097054Z    |
2019-10-18T23:58:20.4097054Z    |
2019-10-18T23:58:20.4097096Z LL |     if let Range { start: true, end } = t..&&false {}
2019-10-18T23:58:20.4097672Z    |
2019-10-18T23:58:20.4097672Z    |
2019-10-18T23:58:20.4098011Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4098080Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4098175Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4098437Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:92:19
2019-10-18T23:58:20.4098486Z    |
2019-10-18T23:58:20.4098554Z LL |     if let true = let true = true {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4098554Z LL |     if let true = let true = true {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4098607Z    |                   ^^^^^^^^^^^^^^^
2019-10-18T23:58:20.4098649Z    |
2019-10-18T23:58:20.4098911Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4098968Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4099052Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4099323Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:96:12
2019-10-18T23:58:20.4099372Z    |
2019-10-18T23:58:20.4099372Z    |
2019-10-18T23:58:20.4099429Z LL |     while &let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4099538Z    |
2019-10-18T23:58:20.4099538Z    |
2019-10-18T23:58:20.4099787Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4099859Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4099935Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4100187Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:99:12
2019-10-18T23:58:20.4100295Z    |
2019-10-18T23:58:20.4100295Z    |
2019-10-18T23:58:20.4100344Z LL |     while !let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4100456Z    |
2019-10-18T23:58:20.4100456Z    |
2019-10-18T23:58:20.4100712Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4101054Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4101150Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4101407Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:100:12
2019-10-18T23:58:20.4101450Z    |
2019-10-18T23:58:20.4101450Z    |
2019-10-18T23:58:20.4101508Z LL |     while *let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4101586Z    |
2019-10-18T23:58:20.4101586Z    |
2019-10-18T23:58:20.4101818Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4101866Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4101956Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4102179Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:102:12
2019-10-18T23:58:20.4102221Z    |
2019-10-18T23:58:20.4102221Z    |
2019-10-18T23:58:20.4102440Z LL |     while -let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4102540Z    |
2019-10-18T23:58:20.4102540Z    |
2019-10-18T23:58:20.4102757Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4102820Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4102886Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4103121Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:110:12
2019-10-18T23:58:20.4103162Z    |
2019-10-18T23:58:20.4103162Z    |
2019-10-18T23:58:20.4103205Z LL |     while (let 0 = 0)? {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4103390Z    |
2019-10-18T23:58:20.4103390Z    |
2019-10-18T23:58:20.4103634Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4103683Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4103769Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4103994Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:114:19
2019-10-18T23:58:20.4104053Z    |
2019-10-18T23:58:20.4104053Z    |
2019-10-18T23:58:20.4104099Z LL |     while true || let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4104200Z    |
2019-10-18T23:58:20.4104200Z    |
2019-10-18T23:58:20.4104439Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4104489Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4104577Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4104819Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:115:20
2019-10-18T23:58:20.4104863Z    |
2019-10-18T23:58:20.4104863Z    |
2019-10-18T23:58:20.4104922Z LL |     while (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4105007Z    |
2019-10-18T23:58:20.4105007Z    |
2019-10-18T23:58:20.4105251Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4105302Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4105373Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4105622Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:116:28
2019-10-18T23:58:20.4105665Z    |
2019-10-18T23:58:20.4105665Z    |
2019-10-18T23:58:20.4105711Z LL |     while true && (true || let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4105816Z    |
2019-10-18T23:58:20.4105816Z    |
2019-10-18T23:58:20.4106054Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4106197Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4106276Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4106547Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:117:28
2019-10-18T23:58:20.4106607Z    |
2019-10-18T23:58:20.4106607Z    |
2019-10-18T23:58:20.4106652Z LL |     while true || (true && let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4106748Z    |
2019-10-18T23:58:20.4106748Z    |
2019-10-18T23:58:20.4107157Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4107209Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4107678Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4107982Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:120:15
2019-10-18T23:58:20.4108032Z    |
2019-10-18T23:58:20.4108032Z    |
2019-10-18T23:58:20.4108108Z LL |     while x = let 0 = 0 {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4108199Z    |
2019-10-18T23:58:20.4108199Z    |
2019-10-18T23:58:20.4108469Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4108526Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4108618Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4108872Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:123:18
2019-10-18T23:58:20.4108919Z    |
2019-10-18T23:58:20.4108919Z    |
2019-10-18T23:58:20.4108969Z LL |     while true..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4109198Z    |
2019-10-18T23:58:20.4109198Z    |
2019-10-18T23:58:20.4109479Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4109559Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4109637Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4109913Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:125:14
2019-10-18T23:58:20.4109961Z    |
2019-10-18T23:58:20.4109961Z    |
2019-10-18T23:58:20.4110011Z LL |     while ..(let 0 = 0) {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4110109Z    |
2019-10-18T23:58:20.4110109Z    |
2019-10-18T23:58:20.4110354Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4110410Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4110505Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4110760Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:127:12
2019-10-18T23:58:20.4110814Z    |
2019-10-18T23:58:20.4110814Z    |
2019-10-18T23:58:20.4111098Z LL |     while (let 0 = 0).. {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4111193Z    |
2019-10-18T23:58:20.4111193Z    |
2019-10-18T23:58:20.4111418Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4111468Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4111543Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4111772Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:131:11
2019-10-18T23:58:20.4111814Z    |
2019-10-18T23:58:20.4111814Z    |
2019-10-18T23:58:20.4111869Z LL |     while let Range { start: _, end: _ } = true..true && false {}
2019-10-18T23:58:20.4111962Z    |
2019-10-18T23:58:20.4111962Z    |
2019-10-18T23:58:20.4112199Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4112331Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4112407Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4112674Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:135:11
2019-10-18T23:58:20.4112718Z    |
2019-10-18T23:58:20.4112718Z    |
2019-10-18T23:58:20.4112760Z LL |     while let Range { start: _, end: _ } = true..true || false {}
2019-10-18T23:58:20.4112853Z    |
2019-10-18T23:58:20.4112853Z    |
2019-10-18T23:58:20.4114513Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4114611Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4114697Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4114971Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:142:11
2019-10-18T23:58:20.4115027Z    |
2019-10-18T23:58:20.4115027Z    |
2019-10-18T23:58:20.4115076Z LL |     while let Range { start: F, end } = F..|| true {}
2019-10-18T23:58:20.4115175Z    |
2019-10-18T23:58:20.4115175Z    |
2019-10-18T23:58:20.4115400Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4115451Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4115534Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4115763Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:150:11
2019-10-18T23:58:20.4115806Z    |
2019-10-18T23:58:20.4115806Z    |
2019-10-18T23:58:20.4115863Z LL |     while let Range { start: true, end } = t..&&false {}
2019-10-18T23:58:20.4116127Z    |
2019-10-18T23:58:20.4116127Z    |
2019-10-18T23:58:20.4116403Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4116463Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4116542Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4116776Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:156:22
2019-10-18T23:58:20.4116820Z    |
2019-10-18T23:58:20.4116865Z LL |     while let true = let true = true {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4116865Z LL |     while let true = let true = true {} //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4117101Z    |                      ^^^^^^^^^^^^^^^
2019-10-18T23:58:20.4117140Z    |
2019-10-18T23:58:20.4117998Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4118076Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4118163Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4118668Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:170:6
2019-10-18T23:58:20.4118725Z    |
2019-10-18T23:58:20.4118725Z    |
2019-10-18T23:58:20.4118783Z LL |     &let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4118882Z    |
2019-10-18T23:58:20.4118882Z    |
2019-10-18T23:58:20.4119137Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4119193Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4119276Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4119526Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:172:6
2019-10-18T23:58:20.4119590Z    |
2019-10-18T23:58:20.4119590Z    |
2019-10-18T23:58:20.4119638Z LL |     !let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4119753Z    |
2019-10-18T23:58:20.4119753Z    |
2019-10-18T23:58:20.4120001Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4120058Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4120305Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4120599Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:173:6
2019-10-18T23:58:20.4120648Z    |
2019-10-18T23:58:20.4120648Z    |
2019-10-18T23:58:20.4120703Z LL |     *let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4120790Z    |
2019-10-18T23:58:20.4120790Z    |
2019-10-18T23:58:20.4121046Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4121101Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4121186Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4121584Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:175:6
2019-10-18T23:58:20.4121626Z    |
2019-10-18T23:58:20.4121626Z    |
2019-10-18T23:58:20.4121834Z LL |     -let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4121929Z    |
2019-10-18T23:58:20.4121929Z    |
2019-10-18T23:58:20.4122329Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4122392Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4122459Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4122686Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:183:6
2019-10-18T23:58:20.4122744Z    |
2019-10-18T23:58:20.4122744Z    |
2019-10-18T23:58:20.4122786Z LL |     (let 0 = 0)?; //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4123019Z    |
2019-10-18T23:58:20.4123019Z    |
2019-10-18T23:58:20.4123270Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4123320Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4123409Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4123643Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:187:13
2019-10-18T23:58:20.4123686Z    |
2019-10-18T23:58:20.4123686Z    |
2019-10-18T23:58:20.4123742Z LL |     true || let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4123822Z    |
2019-10-18T23:58:20.4123822Z    |
2019-10-18T23:58:20.4124075Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4124126Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4124193Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4124434Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:188:14
2019-10-18T23:58:20.4124477Z    |
2019-10-18T23:58:20.4124477Z    |
2019-10-18T23:58:20.4124520Z LL |     (true || let 0 = 0); //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4124617Z    |
2019-10-18T23:58:20.4124617Z    |
2019-10-18T23:58:20.4124839Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4124896Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4124964Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4125191Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:189:22
2019-10-18T23:58:20.4125247Z    |
2019-10-18T23:58:20.4125247Z    |
2019-10-18T23:58:20.4125291Z LL |     true && (true || let 0 = 0); //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4125399Z    |
2019-10-18T23:58:20.4125399Z    |
2019-10-18T23:58:20.4125620Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4125670Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4125825Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4131384Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:192:9
2019-10-18T23:58:20.4131468Z    |
2019-10-18T23:58:20.4131468Z    |
2019-10-18T23:58:20.4131513Z LL |     x = let 0 = 0; //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4131592Z    |
2019-10-18T23:58:20.4131592Z    |
2019-10-18T23:58:20.4131966Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4132017Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4132096Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4132353Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:194:12
2019-10-18T23:58:20.4132397Z    |
2019-10-18T23:58:20.4132397Z    |
2019-10-18T23:58:20.4132454Z LL |     true..(let 0 = 0); //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4132537Z    |
2019-10-18T23:58:20.4132537Z    |
2019-10-18T23:58:20.4132768Z    = note: only supported directly in conditions of `if`- and `while`-expressions
2019-10-18T23:58:20.4132834Z    = note: as well as when nested within `&&` and parenthesis in those conditions
2019-10-18T23:58:20.4132902Z error: `let` expressions are not supported here
2019-10-18T23:58:20.4133145Z   --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:195:8
2019-10-18T23:58:20.4133190Z    |
2019-10-18T23:58:20.4133190Z    |
2019-10-18T23:58:20.4133233Z LL |     ..(let 0 = 0); //~ ERROR `let` expressions are not supported here
2019-10-18T23:58:20.4133321Z    |
2019-10-18T23:58:20.4133321Z    |
2019-10-18T23:58:20.4133543Z    = note: only supported directly in conditions of `if`- and `while`-expressions
---
2019-10-18T23:58:21.1431670Z 
2019-10-18T23:58:21.1431721Z 31    = note: expected type `i32`
2019-10-18T23:58:21.1431757Z 32               found type `()`
2019-10-18T23:58:21.1431790Z 33 
2019-10-18T23:58:21.1432002Z - error[E0277]: the trait bound `(): std::ops::Try` is not satisfied
2019-10-18T23:58:21.1432338Z -    |
2019-10-18T23:58:21.1432338Z -    |
2019-10-18T23:58:21.1432503Z - LL |     let res: () = try { };
2019-10-18T23:58:21.1432874Z -    |
2019-10-18T23:58:21.1433800Z -    = note: required by `std::ops::Try::from_ok`
2019-10-18T23:58:21.1434051Z - 
2019-10-18T23:58:21.1434095Z 42 error[E0277]: the trait bound `i32: std::ops::Try` is not satisfied
---
2019-10-18T23:58:21.1435042Z 
2019-10-18T23:58:21.1435079Z 
2019-10-18T23:58:21.1435117Z The actual stderr differed from the expected stderr.
2019-10-18T23:58:21.1435380Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/try-block-bad-type.stderr
2019-10-18T23:58:21.1435602Z To update references, rerun the tests and pass the `--bless` flag
2019-10-18T23:58:21.1435853Z To only update this specific test, also pass `--test-args try-block/try-block-bad-type.rs`
2019-10-18T23:58:21.1435925Z error: 1 errors occurred comparing output.
2019-10-18T23:58:21.1435981Z status: exit code: 1
2019-10-18T23:58:21.1435981Z status: exit code: 1
2019-10-18T23:58:21.1436646Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-block/try-block-bad-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition" "2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/auxiliary" "-A" "unused"
2019-10-18T23:58:21.1437716Z ------------------------------------------
2019-10-18T23:58:21.1437753Z 
2019-10-18T23:58:21.1437990Z ------------------------------------------
2019-10-18T23:58:21.1438035Z stderr:
2019-10-18T23:58:21.1438035Z stderr:
2019-10-18T23:58:21.1438243Z ------------------------------------------
2019-10-18T23:58:21.1438489Z error[E0277]: `?` couldn't convert the error to `i32`
2019-10-18T23:58:21.1438777Z    |
2019-10-18T23:58:21.1438777Z    |
2019-10-18T23:58:21.1439027Z LL |         Err("")?; //~ ERROR `?` couldn't convert the error
2019-10-18T23:58:21.1439097Z    |                ^ the trait `std::convert::From<&str>` is not implemented for `i32`
2019-10-18T23:58:21.1439144Z    |
2019-10-18T23:58:21.1439217Z    = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
2019-10-18T23:58:21.1439280Z    = help: the following implementations were found:
2019-10-18T23:58:21.1439332Z              <i32 as std::convert::From<bool>>
2019-10-18T23:58:21.1439397Z              <i32 as std::convert::From<i16>>
2019-10-18T23:58:21.1439445Z              <i32 as std::convert::From<i8>>
2019-10-18T23:58:21.1439494Z              <i32 as std::convert::From<std::num::NonZeroI32>>
2019-10-18T23:58:21.1439606Z    = note: required by `std::convert::From::from`
2019-10-18T23:58:21.1439636Z 
2019-10-18T23:58:21.1439636Z 
2019-10-18T23:58:21.1439686Z error[E0271]: type mismatch resolving `<std::result::Result<i32, i32> as std::ops::Try>::Ok == &str`
2019-10-18T23:58:21.1440031Z    |
2019-10-18T23:58:21.1440075Z LL |         "" //~ ERROR type mismatch
2019-10-18T23:58:21.1440075Z LL |         "" //~ ERROR type mismatch
2019-10-18T23:58:21.1440138Z    |         ^^ expected i32, found &str
2019-10-18T23:58:21.1440223Z    = note: expected type `i32`
2019-10-18T23:58:21.1440682Z               found type `&str`
2019-10-18T23:58:21.1440787Z 
2019-10-18T23:58:21.1440787Z 
2019-10-18T23:58:21.1440833Z error[E0271]: type mismatch resolving `<std::result::Result<i32, i32> as std::ops::Try>::Ok == ()`
2019-10-18T23:58:21.1441172Z    |
2019-10-18T23:58:21.1441172Z    |
2019-10-18T23:58:21.1441219Z LL |     let res: Result<i32, i32> = try { }; //~ ERROR type mismatch
2019-10-18T23:58:21.1441266Z    |                                       ^ expected i32, found ()
2019-10-18T23:58:21.1441613Z    = note: expected type `i32`
2019-10-18T23:58:21.1441652Z               found type `()`
2019-10-18T23:58:21.1441687Z 
2019-10-18T23:58:21.1441745Z error[E0277]: the trait bound `i32: std::ops::Try` is not satisfied
2019-10-18T23:58:21.1441745Z error[E0277]: the trait bound `i32: std::ops::Try` is not satisfied
2019-10-18T23:58:21.1441971Z   --> /checkout/src/test/ui/try-block/try-block-bad-type.rs:19:24
2019-10-18T23:58:21.1442013Z    |
2019-10-18T23:58:21.1442078Z LL |     let res: i32 = try { 5 }; //~ ERROR the trait bound `i32: std::ops::Try` is not satisfied
2019-10-18T23:58:21.1442130Z    |                        ^^^^^ the trait `std::ops::Try` is not implemented for `i32`
2019-10-18T23:58:21.1442480Z    = note: required by `std::ops::Try::from_ok`
2019-10-18T23:58:21.1442508Z 
2019-10-18T23:58:21.1442547Z error: aborting due to 4 previous errors
2019-10-18T23:58:21.1442592Z 
---
2019-10-18T23:58:21.1443260Z 
2019-10-18T23:58:21.1443489Z ---- [ui] ui/try-operator-on-main.rs stdout ----
2019-10-18T23:58:21.1443556Z diff of stderr:
2019-10-18T23:58:21.1443582Z 
2019-10-18T23:58:21.1443883Z - error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
2019-10-18T23:58:21.1444111Z -   --> $DIR/try-operator-on-main.rs:9:5
2019-10-18T23:58:21.1444321Z + error[E0277]: `?` couldn't convert the error to `!`
2019-10-18T23:58:21.1444517Z +   --> $DIR/try-operator-on-main.rs:9:31
2019-10-18T23:58:21.1444559Z 3    |
2019-10-18T23:58:21.1444621Z 4 LL |     std::fs::File::open("foo")?;
2019-10-18T23:58:21.1444869Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?` operator in a function that returns `()`
2019-10-18T23:58:21.1444927Z +    |                               ^ the trait `std::convert::From<std::io::Error>` is not implemented for `!`
2019-10-18T23:58:21.1445215Z -    = help: the trait `std::ops::Try` is not implemented for `()`
2019-10-18T23:58:21.1445653Z -    = note: required by `std::ops::Try::from_error`
2019-10-18T23:58:21.1445653Z -    = note: required by `std::ops::Try::from_error`
2019-10-18T23:58:21.1445952Z +    = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
2019-10-18T23:58:21.1446008Z +    = note: required by `std::convert::From::from`
2019-10-18T23:58:21.1446295Z - error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
2019-10-18T23:58:21.1446485Z -   --> $DIR/try-operator-on-main.rs:12:5
2019-10-18T23:58:21.1446638Z -    |
2019-10-18T23:58:21.1447382Z - LL |     ()?;
2019-10-18T23:58:21.1447382Z - LL |     ()?;
2019-10-18T23:58:21.1447655Z -    |     ^^^ the `?` operator cannot be applied to type `()`
2019-10-18T23:58:21.1447832Z -    |
2019-10-18T23:58:21.1448080Z -    = help: the trait `std::ops::Try` is not implemented for `()`
2019-10-18T23:58:21.1448307Z -    = note: required by `std::ops::Try::into_result`
2019-10-18T23:58:21.1448493Z - 
2019-10-18T23:58:21.1448728Z - error[E0277]: the trait bound `(): std::ops::Try` is not satisfied
2019-10-18T23:58:21.1449142Z -    |
2019-10-18T23:58:21.1449142Z -    |
2019-10-18T23:58:21.1449346Z - LL |     try_trait_generic::<()>();
2019-10-18T23:58:21.1449731Z -    |                         ^^ the trait `std::ops::Try` is not implemented for `()`
2019-10-18T23:58:21.1449958Z - ...
2019-10-18T23:58:21.1450169Z - LL | fn try_trait_generic<T: Try>() -> T {
2019-10-18T23:58:21.1450669Z -    |    -----------------    --- required by this bound in `try_trait_generic`
2019-10-18T23:58:21.1451038Z - error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
2019-10-18T23:58:21.1451239Z -   --> $DIR/try-operator-on-main.rs:22:5
2019-10-18T23:58:21.1451390Z -    |
2019-10-18T23:58:21.1451542Z - LL |     ()?;
---
2019-10-18T23:58:21.1452983Z 40 
2019-10-18T23:58:21.1453007Z 
2019-10-18T23:58:21.1453046Z 
2019-10-18T23:58:21.1453083Z The actual stderr differed from the expected stderr.
2019-10-18T23:58:21.1453333Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-operator-on-main/try-operator-on-main.stderr
2019-10-18T23:58:21.1453556Z To update references, rerun the tests and pass the `--bless` flag
2019-10-18T23:58:21.1453769Z To only update this specific test, also pass `--test-args try-operator-on-main.rs`
2019-10-18T23:58:21.1453916Z error: 1 errors occurred comparing output.
2019-10-18T23:58:21.1453975Z status: exit code: 1
2019-10-18T23:58:21.1453975Z status: exit code: 1
2019-10-18T23:58:21.1454610Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-operator-on-main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-operator-on-main" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-operator-on-main/auxiliary" "-A" "unused"
2019-10-18T23:58:21.1454889Z ------------------------------------------
2019-10-18T23:58:21.1454917Z 
2019-10-18T23:58:21.1455114Z ------------------------------------------
2019-10-18T23:58:21.1455153Z stderr:
2019-10-18T23:58:21.1455153Z stderr:
2019-10-18T23:58:21.1455338Z ------------------------------------------
2019-10-18T23:58:21.1455541Z error[E0277]: `?` couldn't convert the error to `!`
2019-10-18T23:58:21.1455775Z    |
2019-10-18T23:58:21.1455775Z    |
2019-10-18T23:58:21.1455842Z LL |     std::fs::File::open("foo")?; //~ ERROR the `?` operator can only
2019-10-18T23:58:21.1455893Z    |                               ^ the trait `std::convert::From<std::io::Error>` is not implemented for `!`
2019-10-18T23:58:21.1455935Z    |
2019-10-18T23:58:21.1455998Z    = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
2019-10-18T23:58:21.1456044Z    = note: required by `std::convert::From::from`
2019-10-18T23:58:21.1456107Z error: aborting due to previous error
2019-10-18T23:58:21.1456149Z 
2019-10-18T23:58:21.1456355Z For more information about this error, try `rustc --explain E0277`.
2019-10-18T23:58:21.1456384Z 
---
2019-10-18T23:58:21.1458676Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-18T23:58:21.1458753Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-18T23:58:21.1458786Z 
2019-10-18T23:58:21.1458812Z 
2019-10-18T23:58:21.1460279Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-18T23:58:21.1461072Z 
2019-10-18T23:58:21.1461098Z 
2019-10-18T23:58:21.1461144Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-18T23:58:21.1461210Z Build completed unsuccessfully in 1:05:00
2019-10-18T23:58:21.1461210Z Build completed unsuccessfully in 1:05:00
2019-10-18T23:58:21.1461249Z == clock drift check ==
2019-10-18T23:58:21.1461287Z   local time: Fri Oct 18 23:58:20 UTC 2019
2019-10-18T23:58:21.1461345Z   network time: Fri, 18 Oct 2019 23:58:20 GMT
2019-10-18T23:58:21.1461383Z == end clock drift check ==
2019-10-18T23:58:21.7231155Z 
2019-10-18T23:58:21.7339728Z ##[error]Bash exited with code '1'.
2019-10-18T23:58:21.7381772Z ##[section]Starting: Checkout
2019-10-18T23:58:21.7383290Z ==============================================================================
2019-10-18T23:58:21.7383368Z Task         : Get sources
2019-10-18T23:58:21.7383406Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
