plain
2019-12-24T17:15:26.8353388Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-24T17:15:26.8545271Z ##[command]git config gc.auto 0
2019-12-24T17:15:26.8606345Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-24T17:15:26.8665869Z ##[command]git config --get-all http.proxy
2019-12-24T17:15:26.8807869Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67592/merge:refs/remotes/pull/67592/merge
---
2019-12-24T18:13:11.9163311Z .................................................................................................... 1600/9452
2019-12-24T18:13:16.6776669Z .................................................................................................... 1700/9452
2019-12-24T18:13:26.5933915Z ...........................................................................................i........ 1800/9452
2019-12-24T18:13:34.0065806Z .................................................................................................... 1900/9452
2019-12-24T18:13:40.3552015Z .............................................................................iiiii.................. 2000/9452
2019-12-24T18:14:01.4717252Z .................................................................................................... 2200/9452
2019-12-24T18:14:03.8305586Z .................................................................................................... 2300/9452
2019-12-24T18:14:06.4068730Z .................................................................................................... 2400/9452
2019-12-24T18:14:15.4726214Z .................................................................................................... 2500/9452
---
2019-12-24T18:17:05.2177991Z ........i...............i........................................................................... 4900/9452
2019-12-24T18:17:14.9117285Z .................................................................................................... 5000/9452
2019-12-24T18:17:19.9368098Z ....................................................i............................................... 5100/9452
2019-12-24T18:17:29.5051397Z .................................................................................................... 5200/9452
2019-12-24T18:17:35.6752963Z ...................ii.ii...........i................................................................ 5300/9452
2019-12-24T18:17:44.8912491Z .....................................................F.............................................. 5500/9452
2019-12-24T18:17:55.7758100Z .................................................................................................... 5600/9452
2019-12-24T18:18:03.3281613Z .i.................................................................................................. 5700/9452
2019-12-24T18:18:08.6794615Z .................................................................................................... 5800/9452
2019-12-24T18:18:08.6794615Z .................................................................................................... 5800/9452
2019-12-24T18:18:18.3713119Z .........................................................................................ii...i..ii. 5900/9452
2019-12-24T18:18:40.2772501Z .................................................................................................... 6100/9452
2019-12-24T18:18:48.3926384Z .........................................................................F.......................... 6200/9452
2019-12-24T18:18:56.2756018Z .................................................................................................... 6300/9452
2019-12-24T18:18:56.2756018Z .................................................................................................... 6300/9452
2019-12-24T18:19:09.2124058Z ................i..ii............................................................................... 6400/9452
2019-12-24T18:19:28.8733420Z .............................................................................................i...... 6600/9452
2019-12-24T18:19:31.0242426Z .................................................................................................... 6700/9452
2019-12-24T18:19:33.2919363Z .............................................................................................i...... 6800/9452
2019-12-24T18:19:36.0290191Z .................................................................................................... 6900/9452
---
2019-12-24T18:21:12.6212103Z .................................................................................................... 7500/9452
2019-12-24T18:21:17.4604946Z .................................................................................................... 7600/9452
2019-12-24T18:21:24.3045084Z .................................................................................................... 7700/9452
2019-12-24T18:21:34.4190459Z .................................................................................................... 7800/9452
2019-12-24T18:21:40.6903812Z .......................iiii......................................................................... 7900/9452
2019-12-24T18:21:55.1283939Z .................................................................................................... 8100/9452
2019-12-24T18:22:05.1177482Z .................................................................................................... 8200/9452
2019-12-24T18:22:18.6490809Z .................................................................................................... 8300/9452
2019-12-24T18:22:25.4946533Z .....................................................FF..F.......................................... 8400/9452
---
2019-12-24T18:24:16.8322190Z diff of stderr:
2019-12-24T18:24:16.8322337Z 
2019-12-24T18:24:16.8322535Z 5    |           ^^^^^^^^ cannot move out of here
2019-12-24T18:24:16.8322693Z 6 ...
2019-12-24T18:24:16.8322843Z 7 LL |         (&[], &[hd, ..]) | (&[hd, ..], &[])
2019-12-24T18:24:16.8323257Z -    |                 -- data moved here
2019-12-24T18:24:16.8323667Z +    |                               -- data moved here
2019-12-24T18:24:16.8323858Z 9 LL |             => println!("one empty"),
2019-12-24T18:24:16.8324040Z 10 LL |         (&[hd1, ..], &[hd2, ..])
2019-12-24T18:24:16.8324585Z 
2019-12-24T18:24:16.8324763Z 19    |           ^^^^^^^^ cannot move out of here
2019-12-24T18:24:16.8324931Z 20 ...
2019-12-24T18:24:16.8324931Z 20 ...
2019-12-24T18:24:16.8325080Z 21 LL |         (&[], &[hd, ..]) | (&[hd, ..], &[])
2019-12-24T18:24:16.8325459Z -    |                 -- data moved here
2019-12-24T18:24:16.8325853Z +    |                               -- data moved here
2019-12-24T18:24:16.8326039Z 23 LL |             => println!("one empty"),
2019-12-24T18:24:16.8326225Z 24 LL |         (&[hd1, ..], &[hd2, ..])
2019-12-24T18:24:16.8326741Z 
2019-12-24T18:24:16.8326893Z 
2019-12-24T18:24:16.8327044Z The actual stderr differed from the expected stderr.
2019-12-24T18:24:16.8327481Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12567/issue-12567.stderr
2019-12-24T18:24:16.8327481Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12567/issue-12567.stderr
2019-12-24T18:24:16.8327926Z To update references, rerun the tests and pass the `--bless` flag
2019-12-24T18:24:16.8328376Z To only update this specific test, also pass `--test-args issues/issue-12567.rs`
2019-12-24T18:24:16.8328723Z error: 1 errors occurred comparing output.
2019-12-24T18:24:16.8328875Z status: exit code: 1
2019-12-24T18:24:16.8328875Z status: exit code: 1
2019-12-24T18:24:16.8329888Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12567.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12567" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12567/auxiliary" "-A" "unused"
2019-12-24T18:24:16.8331604Z ------------------------------------------
2019-12-24T18:24:16.8331805Z 
2019-12-24T18:24:16.8332181Z ------------------------------------------
2019-12-24T18:24:16.8332390Z stderr:
2019-12-24T18:24:16.8332390Z stderr:
2019-12-24T18:24:16.8332736Z ------------------------------------------
2019-12-24T18:24:16.8333391Z error[E0508]: cannot move out of type `[T]`, a non-copy slice
2019-12-24T18:24:16.8333968Z   --> /checkout/src/test/ui/issues/issue-12567.rs:4:11
2019-12-24T18:24:16.8334453Z LL |     match (l1, l2) {
2019-12-24T18:24:16.8334787Z    |           ^^^^^^^^ cannot move out of here
2019-12-24T18:24:16.8334999Z ...
2019-12-24T18:24:16.8334999Z ...
2019-12-24T18:24:16.8335175Z LL |         (&[], &[hd, ..]) | (&[hd, ..], &[])
2019-12-24T18:24:16.8335593Z    |                               -- data moved here
2019-12-24T18:24:16.8335796Z LL |             => println!("one empty"),
2019-12-24T18:24:16.8339023Z LL |         (&[hd1, ..], &[hd2, ..])
2019-12-24T18:24:16.8339674Z    |
2019-12-24T18:24:16.8340112Z    = note: move occurs because these variables have types that don't implement the `Copy` trait
2019-12-24T18:24:16.8340497Z 
2019-12-24T18:24:16.8340497Z 
2019-12-24T18:24:16.8340937Z error[E0508]: cannot move out of type `[T]`, a non-copy slice
2019-12-24T18:24:16.8341350Z   --> /checkout/src/test/ui/issues/issue-12567.rs:4:11
2019-12-24T18:24:16.8341696Z LL |     match (l1, l2) {
2019-12-24T18:24:16.8341868Z    |           ^^^^^^^^ cannot move out of here
2019-12-24T18:24:16.8342011Z ...
2019-12-24T18:24:16.8342011Z ...
2019-12-24T18:24:16.8342186Z LL |         (&[], &[hd, ..]) | (&[hd, ..], &[])
2019-12-24T18:24:16.8342728Z    |                               -- data moved here
2019-12-24T18:24:16.8342918Z LL |             => println!("one empty"),
2019-12-24T18:24:16.8343089Z LL |         (&[hd1, ..], &[hd2, ..])
2019-12-24T18:24:16.8343614Z    |
2019-12-24T18:24:16.8344039Z    = note: move occurs because these variables have types that don't implement the `Copy` trait
2019-12-24T18:24:16.8344223Z 
2019-12-24T18:24:16.8344374Z error: aborting due to 2 previous errors
---
2019-12-24T18:24:16.8356124Z 143 error: variable does not need to be mutable
2019-12-24T18:24:16.8356471Z -   --> $DIR/lint-unused-mut-variables.rs:96:8
2019-12-24T18:24:16.8356676Z +   --> $DIR/lint-unused-mut-variables.rs:99:8
2019-12-24T18:24:16.8356718Z 145    |
2019-12-24T18:24:16.8356893Z - LL |       (mut x, 1) |
2019-12-24T18:24:16.8356952Z + LL |       (mut x, 3) => {
2019-12-24T18:24:16.8357177Z 148    |        |
2019-12-24T18:24:16.8357234Z 149    |        help: remove this `mut`
2019-12-24T18:24:16.8357261Z 
2019-12-24T18:24:16.8357285Z 
2019-12-24T18:24:16.8357285Z 
2019-12-24T18:24:16.8357325Z The actual stderr differed from the expected stderr.
2019-12-24T18:24:16.8357635Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unused-mut-variables/lint-unused-mut-variables.stderr
2019-12-24T18:24:16.8357869Z To update references, rerun the tests and pass the `--bless` flag
2019-12-24T18:24:16.8358115Z To only update this specific test, also pass `--test-args lint/lint-unused-mut-variables.rs`
2019-12-24T18:24:16.8358205Z error: 1 errors occurred comparing output.
2019-12-24T18:24:16.8358245Z status: exit code: 1
2019-12-24T18:24:16.8358245Z status: exit code: 1
2019-12-24T18:24:16.8359193Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-unused-mut-variables.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unused-mut-variables" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unused-mut-variables/auxiliary" "-A" "unused"
2019-12-24T18:24:16.8359737Z ------------------------------------------
2019-12-24T18:24:16.8359769Z 
2019-12-24T18:24:16.8359968Z ------------------------------------------
2019-12-24T18:24:16.8360027Z stderr:
---
2019-12-24T18:24:16.8367369Z 
2019-12-24T18:24:16.8367411Z error: variable does not need to be mutable
2019-12-24T18:24:16.8367779Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:104:14
2019-12-24T18:24:16.8367826Z    |
2019-12-24T18:24:16.8367876Z LL |     let x = |mut y: isize| 10; //~ ERROR: variable does not need to be mutable
2019-12-24T18:24:16.8368312Z    |              |
2019-12-24T18:24:16.8368429Z    |              help: remove this `mut`
2019-12-24T18:24:16.8368457Z 
2019-12-24T18:24:16.8368513Z error: variable does not need to be mutable
---
2019-12-24T18:24:16.8371228Z 
2019-12-24T18:24:16.8371267Z error: variable does not need to be mutable
2019-12-24T18:24:16.8371491Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:77:10
2019-12-24T18:24:16.8371534Z    |
2019-12-24T18:24:16.8371599Z LL |     let (mut a, b) = (1, 2); //~ ERROR: variable does not need to be mutable
2019-12-24T18:24:16.8371820Z    |          |
2019-12-24T18:24:16.8371886Z    |          help: remove this `mut`
2019-12-24T18:24:16.8371914Z 
2019-12-24T18:24:16.8371953Z error: variable does not need to be mutable
2019-12-24T18:24:16.8371953Z error: variable does not need to be mutable
2019-12-24T18:24:16.8372194Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:79:9
2019-12-24T18:24:16.8372238Z    |
2019-12-24T18:24:16.8372282Z LL |     let mut a; //~ ERROR: variable does not need to be mutable
2019-12-24T18:24:16.8372513Z    |         |
2019-12-24T18:24:16.8372553Z    |         help: remove this `mut`
2019-12-24T18:24:16.8372589Z 
2019-12-24T18:24:16.8372647Z error: variable does not need to be mutable
2019-12-24T18:24:16.8372647Z error: variable does not need to be mutable
2019-12-24T18:24:16.8372869Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:83:9
2019-12-24T18:24:16.8372912Z    |
2019-12-24T18:24:16.8372958Z LL |     let mut b; //~ ERROR: variable does not need to be mutable
2019-12-24T18:24:16.8373190Z    |         |
2019-12-24T18:24:16.8373229Z    |         help: remove this `mut`
2019-12-24T18:24:16.8373276Z 
2019-12-24T18:24:16.8373388Z error: variable does not need to be mutable
---
2019-12-24T18:24:16.8374397Z 
2019-12-24T18:24:16.8374438Z error: variable does not need to be mutable
2019-12-24T18:24:16.8374951Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:99:8
2019-12-24T18:24:16.8375015Z    |
2019-12-24T18:24:16.8375058Z LL |       (mut x, 3) => {
2019-12-24T18:24:16.8375305Z    |        |
2019-12-24T18:24:16.8375348Z    |        help: remove this `mut`
2019-12-24T18:24:16.8375377Z 
2019-12-24T18:24:16.8375420Z error: variable does not need to be mutable
2019-12-24T18:24:16.8375420Z error: variable does not need to be mutable
2019-12-24T18:24:16.8375758Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:109:9
2019-12-24T18:24:16.8375805Z    |
2019-12-24T18:24:16.8375896Z LL |     let mut a = &mut 5; //~ ERROR: variable does not need to be mutable
2019-12-24T18:24:16.8376429Z    |         |
2019-12-24T18:24:16.8376472Z    |         help: remove this `mut`
2019-12-24T18:24:16.8376502Z 
2019-12-24T18:24:16.8376655Z error: variable does not need to be mutable
2019-12-24T18:24:16.8376655Z error: variable does not need to be mutable
2019-12-24T18:24:16.8376897Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:114:9
2019-12-24T18:24:16.8376957Z    |
2019-12-24T18:24:16.8377023Z LL |     let mut b = (&mut a,); //~ ERROR: variable does not need to be mutable
2019-12-24T18:24:16.8377259Z    |         |
2019-12-24T18:24:16.8377321Z    |         help: remove this `mut`
2019-12-24T18:24:16.8377350Z 
2019-12-24T18:24:16.8377394Z error: variable does not need to be mutable
2019-12-24T18:24:16.8377394Z error: variable does not need to be mutable
2019-12-24T18:24:16.8377641Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:117:9
2019-12-24T18:24:16.8377704Z    |
2019-12-24T18:24:16.8377752Z LL |     let mut x = &mut 1; //~ ERROR: variable does not need to be mutable
2019-12-24T18:24:16.8378003Z    |         |
2019-12-24T18:24:16.8378046Z    |         help: remove this `mut`
2019-12-24T18:24:16.8378074Z 
2019-12-24T18:24:16.8378116Z error: variable does not need to be mutable
2019-12-24T18:24:16.8378116Z error: variable does not need to be mutable
2019-12-24T18:24:16.8378627Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:129:9
2019-12-24T18:24:16.8378679Z    |
2019-12-24T18:24:16.8378906Z LL |     let mut v : &mut Vec<()> = &mut vec![]; //~ ERROR: variable does not need to be mutable
2019-12-24T18:24:16.8379615Z    |         |
2019-12-24T18:24:16.8379655Z    |         help: remove this `mut`
2019-12-24T18:24:16.8379699Z 
2019-12-24T18:24:16.8379738Z error: variable does not need to be mutable
---
2019-12-24T18:24:16.8380403Z 
2019-12-24T18:24:16.8380443Z error: variable does not need to be mutable
2019-12-24T18:24:16.8380684Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:106:13
2019-12-24T18:24:16.8380736Z    |
2019-12-24T18:24:16.8380780Z LL |     fn what(mut foo: isize) {} //~ ERROR: variable does not need to be mutable
2019-12-24T18:24:16.8381057Z    |             |
2019-12-24T18:24:16.8381097Z    |             help: remove this `mut`
2019-12-24T18:24:16.8381125Z 
2019-12-24T18:24:16.8381181Z error: variable does not need to be mutable
2019-12-24T18:24:16.8381181Z error: variable does not need to be mutable
2019-12-24T18:24:16.8381510Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:124:20
2019-12-24T18:24:16.8381563Z    |
2019-12-24T18:24:16.8381803Z LL |     fn mut_ref_arg(mut arg : &mut [u8]) -> &mut [u8] {
2019-12-24T18:24:16.8382056Z    |                    |
2019-12-24T18:24:16.8382098Z    |                    help: remove this `mut`
2019-12-24T18:24:16.8382145Z 
2019-12-24T18:24:16.8382185Z error: variable does not need to be mutable
2019-12-24T18:24:16.8382185Z error: variable does not need to be mutable
2019-12-24T18:24:16.8382524Z   --> /checkout/src/test/ui/lint/lint-unused-mut-variables.rs:202:9
2019-12-24T18:24:16.8382568Z    |
2019-12-24T18:24:16.8382629Z LL |     let mut b = vec![2]; //~ ERROR: variable does not need to be mutable
2019-12-24T18:24:16.8382848Z    |         |
2019-12-24T18:24:16.8382906Z    |         help: remove this `mut`
2019-12-24T18:24:16.8382945Z    |
2019-12-24T18:24:16.8382984Z note: lint level defined here
---
2019-12-24T18:24:16.8384029Z diff of stderr:
2019-12-24T18:24:16.8384054Z 
2019-12-24T18:24:16.8384097Z 141    |           ^^ help: consider borrowing here: `&*x`
2019-12-24T18:24:16.8384138Z 142 LL |
2019-12-24T18:24:16.8384198Z 143 LL |         Ok(s) | Err(s) => (),
2019-12-24T18:24:16.8384547Z -    |            |
2019-12-24T18:24:16.8384750Z -    |            data moved here
2019-12-24T18:24:16.8384750Z -    |            data moved here
2019-12-24T18:24:16.8385031Z -    |            move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8385287Z +    |                     |
2019-12-24T18:24:16.8385330Z +    |                     data moved here
2019-12-24T18:24:16.8385330Z +    |                     data moved here
2019-12-24T18:24:16.8385383Z +    |                     move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8385493Z 149 error: aborting due to 14 previous errors
2019-12-24T18:24:16.8385539Z 150 
2019-12-24T18:24:16.8385564Z 
2019-12-24T18:24:16.8385605Z 
2019-12-24T18:24:16.8385605Z 
2019-12-24T18:24:16.8385647Z The actual stderr differed from the expected stderr.
2019-12-24T18:24:16.8385920Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/move-errors/move-errors.stderr
2019-12-24T18:24:16.8386161Z To update references, rerun the tests and pass the `--bless` flag
2019-12-24T18:24:16.8386401Z To only update this specific test, also pass `--test-args nll/move-errors.rs`
2019-12-24T18:24:16.8386473Z error: 1 errors occurred comparing output.
2019-12-24T18:24:16.8386534Z status: exit code: 1
2019-12-24T18:24:16.8386534Z status: exit code: 1
2019-12-24T18:24:16.8387295Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/move-errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/move-errors" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/move-errors/auxiliary" "-A" "unused"
2019-12-24T18:24:16.8387658Z ------------------------------------------
2019-12-24T18:24:16.8387691Z 
2019-12-24T18:24:16.8388018Z ------------------------------------------
2019-12-24T18:24:16.8388247Z stderr:
2019-12-24T18:24:16.8388247Z stderr:
2019-12-24T18:24:16.8388479Z ------------------------------------------
2019-12-24T18:24:16.8388548Z error[E0507]: cannot move out of `*a` which is behind a shared reference
2019-12-24T18:24:16.8388999Z    |
2019-12-24T18:24:16.8389057Z LL |     let b = *a;
2019-12-24T18:24:16.8389101Z    |             ^^
2019-12-24T18:24:16.8389142Z    |             |
2019-12-24T18:24:16.8389142Z    |             |
2019-12-24T18:24:16.8389503Z    |             move occurs because `*a` has type `A`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8389573Z    |             help: consider borrowing here: `&*a`
2019-12-24T18:24:16.8389825Z 
2019-12-24T18:24:16.8390097Z error[E0508]: cannot move out of type `[A; 1]`, a non-copy array
2019-12-24T18:24:16.8390392Z    |
2019-12-24T18:24:16.8390442Z LL |     let b = a[0];
2019-12-24T18:24:16.8390502Z    |             ^^^^
2019-12-24T18:24:16.8390545Z    |             |
2019-12-24T18:24:16.8390545Z    |             |
2019-12-24T18:24:16.8390589Z    |             cannot move out of here
2019-12-24T18:24:16.8390642Z    |             move occurs because `a[_]` has type `A`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8390716Z    |             help: consider borrowing here: `&a[0]`
2019-12-24T18:24:16.8390746Z 
2019-12-24T18:24:16.8390792Z error[E0507]: cannot move out of `**r` which is behind a shared reference
2019-12-24T18:24:16.8391098Z    |
2019-12-24T18:24:16.8391098Z    |
2019-12-24T18:24:16.8391139Z LL |     let s = **r;
2019-12-24T18:24:16.8391241Z    |             |
2019-12-24T18:24:16.8391241Z    |             |
2019-12-24T18:24:16.8391292Z    |             move occurs because `**r` has type `A`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8391345Z    |             help: consider borrowing here: `&**r`
2019-12-24T18:24:16.8391445Z error[E0507]: cannot move out of an `Rc`
2019-12-24T18:24:16.8391673Z   --> /checkout/src/test/ui/nll/move-errors.rs:27:13
2019-12-24T18:24:16.8391736Z    |
2019-12-24T18:24:16.8391777Z LL |     let s = *r;
2019-12-24T18:24:16.8391777Z LL |     let s = *r;
2019-12-24T18:24:16.8391819Z    |             ^^
2019-12-24T18:24:16.8391860Z    |             |
2019-12-24T18:24:16.8391929Z    |             move occurs because value has type `A`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8391981Z    |             help: consider borrowing here: `&*r`
2019-12-24T18:24:16.8392018Z 
2019-12-24T18:24:16.8392275Z error[E0508]: cannot move out of type `[A; 1]`, a non-copy array
2019-12-24T18:24:16.8392546Z    |
2019-12-24T18:24:16.8392546Z    |
2019-12-24T18:24:16.8392811Z LL |     let a = [A("".to_string())][0];
2019-12-24T18:24:16.8392897Z    |             |
2019-12-24T18:24:16.8392947Z    |             cannot move out of here
2019-12-24T18:24:16.8393016Z    |             move occurs because value has type `A`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8393016Z    |             move occurs because value has type `A`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8393068Z    |             help: consider borrowing here: `&[A("".to_string())][0]`
2019-12-24T18:24:16.8393160Z error[E0507]: cannot move out of `a.0` which is behind a shared reference
2019-12-24T18:24:16.8393383Z   --> /checkout/src/test/ui/nll/move-errors.rs:38:16
2019-12-24T18:24:16.8393427Z    |
2019-12-24T18:24:16.8393427Z    |
2019-12-24T18:24:16.8393492Z LL |     let A(s) = *a;
2019-12-24T18:24:16.8393714Z    |           -    ^^ help: consider borrowing here: `&*a`
2019-12-24T18:24:16.8394687Z    |           data moved here
2019-12-24T18:24:16.8394687Z    |           data moved here
2019-12-24T18:24:16.8395140Z    |           move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8395364Z error[E0509]: cannot move out of type `D`, which implements the `Drop` trait
2019-12-24T18:24:16.8395707Z   --> /checkout/src/test/ui/nll/move-errors.rs:44:19
2019-12-24T18:24:16.8395756Z    |
2019-12-24T18:24:16.8395756Z    |
2019-12-24T18:24:16.8395797Z LL |     let C(D(s)) = c;
2019-12-24T18:24:16.8396081Z    |             |
2019-12-24T18:24:16.8396125Z    |             data moved here
2019-12-24T18:24:16.8396125Z    |             data moved here
2019-12-24T18:24:16.8396180Z    |             move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8396338Z 
2019-12-24T18:24:16.8396385Z error[E0507]: cannot move out of `*a` which is behind a shared reference
2019-12-24T18:24:16.8396703Z    |
2019-12-24T18:24:16.8396744Z LL |     b = *a;
2019-12-24T18:24:16.8396744Z LL |     b = *a;
2019-12-24T18:24:16.8396797Z    |         ^^ move occurs because `*a` has type `A`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8396830Z 
2019-12-24T18:24:16.8397094Z error[E0508]: cannot move out of type `[B; 1]`, a non-copy array
2019-12-24T18:24:16.8397367Z    |
2019-12-24T18:24:16.8397426Z LL |     match x[0] {
2019-12-24T18:24:16.8397469Z    |           ^^^^
2019-12-24T18:24:16.8397511Z    |           |
2019-12-24T18:24:16.8397511Z    |           |
2019-12-24T18:24:16.8397573Z    |           cannot move out of here
2019-12-24T18:24:16.8397622Z    |           help: consider borrowing here: `&x[0]`
2019-12-24T18:24:16.8397666Z LL |     //~^ ERROR
2019-12-24T18:24:16.8397710Z LL |         B::U(d) => (),
2019-12-24T18:24:16.8397950Z    |              - data moved here
2019-12-24T18:24:16.8397997Z LL |         B::V(s) => (),
2019-12-24T18:24:16.8398197Z    |              - ...and here
2019-12-24T18:24:16.8398525Z    = note: move occurs because these variables have types that don't implement the `Copy` trait
2019-12-24T18:24:16.8398561Z 
2019-12-24T18:24:16.8398609Z error[E0509]: cannot move out of type `D`, which implements the `Drop` trait
2019-12-24T18:24:16.8398862Z   --> /checkout/src/test/ui/nll/move-errors.rs:83:11
2019-12-24T18:24:16.8398862Z   --> /checkout/src/test/ui/nll/move-errors.rs:83:11
2019-12-24T18:24:16.8398908Z    |
2019-12-24T18:24:16.8398948Z LL |     match x {
2019-12-24T18:24:16.8399010Z    |           ^ cannot move out of here
2019-12-24T18:24:16.8399052Z ...
2019-12-24T18:24:16.8399094Z LL |         B::U(D(s)) => (),
2019-12-24T18:24:16.8399352Z    |                |
2019-12-24T18:24:16.8399397Z    |                data moved here
2019-12-24T18:24:16.8399397Z    |                data moved here
2019-12-24T18:24:16.8399477Z    |                move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8399560Z error[E0509]: cannot move out of type `D`, which implements the `Drop` trait
2019-12-24T18:24:16.8399791Z   --> /checkout/src/test/ui/nll/move-errors.rs:92:11
2019-12-24T18:24:16.8399854Z    |
2019-12-24T18:24:16.8399894Z LL |     match x {
2019-12-24T18:24:16.8399894Z LL |     match x {
2019-12-24T18:24:16.8399938Z    |           ^ cannot move out of here
2019-12-24T18:24:16.8400060Z ...
2019-12-24T18:24:16.8400103Z LL |         (D(s), &t) => (),
2019-12-24T18:24:16.8400334Z    |            |
2019-12-24T18:24:16.8400394Z    |            data moved here
2019-12-24T18:24:16.8400394Z    |            data moved here
2019-12-24T18:24:16.8400448Z    |            move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8400484Z 
2019-12-24T18:24:16.8400547Z error[E0507]: cannot move out of `*x.1` which is behind a shared reference
2019-12-24T18:24:16.8400832Z    |
2019-12-24T18:24:16.8401048Z LL |     match x {
2019-12-24T18:24:16.8401091Z    |           ^
2019-12-24T18:24:16.8401130Z ...
2019-12-24T18:24:16.8401130Z ...
2019-12-24T18:24:16.8401169Z LL |         (D(s), &t) => (),
2019-12-24T18:24:16.8401843Z    |                 |
2019-12-24T18:24:16.8401885Z    |                 data moved here
2019-12-24T18:24:16.8402036Z    |                 move occurs because `t` has type `std::string::String`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8402078Z 
2019-12-24T18:24:16.8402078Z 
2019-12-24T18:24:16.8402123Z error[E0509]: cannot move out of type `F`, which implements the `Drop` trait
2019-12-24T18:24:16.8402371Z   --> /checkout/src/test/ui/nll/move-errors.rs:102:11
2019-12-24T18:24:16.8402433Z    |
2019-12-24T18:24:16.8402472Z LL |     match x {
2019-12-24T18:24:16.8402515Z    |           ^ cannot move out of here
2019-12-24T18:24:16.8402815Z LL |     //~^ ERROR
2019-12-24T18:24:16.8402932Z LL |         F(s, mut t) => (),
2019-12-24T18:24:16.8403207Z    |           |
2019-12-24T18:24:16.8403247Z    |           data moved here
2019-12-24T18:24:16.8403285Z    |
2019-12-24T18:24:16.8403533Z    = note: move occurs because these variables have types that don't implement the `Copy` trait
2019-12-24T18:24:16.8403533Z    = note: move occurs because these variables have types that don't implement the `Copy` trait
2019-12-24T18:24:16.8404012Z 
2019-12-24T18:24:16.8404071Z error[E0507]: cannot move out of `x.0` which is behind a shared reference
2019-12-24T18:24:16.8404340Z   --> /checkout/src/test/ui/nll/move-errors.rs:110:11
2019-12-24T18:24:16.8404401Z    |
2019-12-24T18:24:16.8404440Z LL |     match *x {
2019-12-24T18:24:16.8404482Z    |           ^^ help: consider borrowing here: `&*x`
2019-12-24T18:24:16.8404543Z LL |     //~^ ERROR
2019-12-24T18:24:16.8404589Z LL |         Ok(s) | Err(s) => (),
2019-12-24T18:24:16.8405245Z    |                     |
2019-12-24T18:24:16.8405317Z    |                     data moved here
2019-12-24T18:24:16.8405317Z    |                     data moved here
2019-12-24T18:24:16.8405373Z    |                     move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8405487Z error: aborting due to 14 previous errors
2019-12-24T18:24:16.8405516Z 
2019-12-24T18:24:16.8405561Z Some errors have detailed explanations: E0507, E0508, E0509.
2019-12-24T18:24:16.8405815Z For more information about an error, try `rustc --explain E0507`.
---
2019-12-24T18:24:16.8406466Z 
2019-12-24T18:24:16.8406507Z 43 LL |         match e {
2019-12-24T18:24:16.8406653Z 44    |               ^ help: consider borrowing here: `&e`
2019-12-24T18:24:16.8406705Z 45 ...
2019-12-24T18:24:16.8406910Z - LL |             Either::One(_t)
2019-12-24T18:24:16.8407330Z -    |                         |
2019-12-24T18:24:16.8407544Z -    |                         data moved here
2019-12-24T18:24:16.8407544Z -    |                         data moved here
2019-12-24T18:24:16.8407829Z -    |                         move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8407903Z + LL |             | Either::Two(_t) => (),
2019-12-24T18:24:16.8408163Z +    |                           |
2019-12-24T18:24:16.8408226Z +    |                           data moved here
2019-12-24T18:24:16.8408226Z +    |                           data moved here
2019-12-24T18:24:16.8408281Z +    |                           move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8408330Z 51 
2019-12-24T18:24:16.8408398Z 52 error[E0507]: cannot move out of `e.0`, as `e` is a captured variable in an `Fn` closure
2019-12-24T18:24:16.8408655Z 
2019-12-24T18:24:16.8408714Z 109 LL |         match em {
2019-12-24T18:24:16.8408763Z 110    |               ^^ help: consider borrowing here: `&em`
2019-12-24T18:24:16.8408807Z 111 ...
2019-12-24T18:24:16.8408807Z 111 ...
2019-12-24T18:24:16.8409015Z - LL |             Either::One(mut _t)
2019-12-24T18:24:16.8409826Z -    |                         |
2019-12-24T18:24:16.8410149Z -    |                         data moved here
2019-12-24T18:24:16.8410149Z -    |                         data moved here
2019-12-24T18:24:16.8410651Z -    |                         move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8410705Z + LL |             | Either::Two(mut _t) => (),
2019-12-24T18:24:16.8411028Z +    |                           |
2019-12-24T18:24:16.8411073Z +    |                           data moved here
2019-12-24T18:24:16.8411073Z +    |                           data moved here
2019-12-24T18:24:16.8411125Z +    |                           move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8411271Z 117 
2019-12-24T18:24:16.8411319Z 118 error[E0507]: cannot move out of `em.0`, as `em` is a captured variable in an `Fn` closure
2019-12-24T18:24:16.8411774Z 
2019-12-24T18:24:16.8411833Z 175 LL |         match e {
2019-12-24T18:24:16.8411881Z 176    |               ^ help: consider borrowing here: `&e`
2019-12-24T18:24:16.8411935Z 177 ...
2019-12-24T18:24:16.8411935Z 177 ...
2019-12-24T18:24:16.8412158Z - LL |             Either::One(_t)
2019-12-24T18:24:16.8412556Z -    |                         |
2019-12-24T18:24:16.8413127Z -    |                         data moved here
2019-12-24T18:24:16.8413127Z -    |                         data moved here
2019-12-24T18:24:16.8413412Z -    |                         move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8413467Z + LL |             | Either::Two(_t) => (),
2019-12-24T18:24:16.8413745Z +    |                           |
2019-12-24T18:24:16.8413791Z +    |                           data moved here
2019-12-24T18:24:16.8413791Z +    |                           data moved here
2019-12-24T18:24:16.8413845Z +    |                           move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8413913Z 183 
2019-12-24T18:24:16.8413963Z 184 error[E0507]: cannot move out of `e.0`, as `e` is a captured variable in an `FnMut` closure
2019-12-24T18:24:16.8414240Z 
2019-12-24T18:24:16.8414281Z 241 LL |         match em {
2019-12-24T18:24:16.8414329Z 242    |               ^^ help: consider borrowing here: `&em`
2019-12-24T18:24:16.8414391Z 243 ...
2019-12-24T18:24:16.8414391Z 243 ...
2019-12-24T18:24:16.8414751Z - LL |             Either::One(mut _t)
2019-12-24T18:24:16.8415141Z -    |                         |
2019-12-24T18:24:16.8415372Z -    |                         data moved here
2019-12-24T18:24:16.8415372Z -    |                         data moved here
2019-12-24T18:24:16.8415653Z -    |                         move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8415706Z + LL |             | Either::Two(mut _t) => (),
2019-12-24T18:24:16.8435004Z +    |                           |
2019-12-24T18:24:16.8435057Z +    |                           data moved here
2019-12-24T18:24:16.8435057Z +    |                           data moved here
2019-12-24T18:24:16.8435168Z +    |                           move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8435219Z 249 
2019-12-24T18:24:16.8435271Z 250 error[E0507]: cannot move out of `em.0`, as `em` is a captured variable in an `FnMut` closure
2019-12-24T18:24:16.8435827Z 
2019-12-24T18:24:16.8435854Z 
2019-12-24T18:24:16.8435900Z The actual stderr differed from the expected stderr.
2019-12-24T18:24:16.8436265Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-suggest-ref/move-into-closure/move-into-closure.stderr
2019-12-24T18:24:16.8436265Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-suggest-ref/move-into-closure/move-into-closure.stderr
2019-12-24T18:24:16.8436534Z To update references, rerun the tests and pass the `--bless` flag
2019-12-24T18:24:16.8436827Z To only update this specific test, also pass `--test-args suggestions/dont-suggest-ref/move-into-closure.rs`
2019-12-24T18:24:16.8436929Z error: 1 errors occurred comparing output.
2019-12-24T18:24:16.8437154Z status: exit code: 1
2019-12-24T18:24:16.8437154Z status: exit code: 1
2019-12-24T18:24:16.8438308Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/dont-suggest-ref/move-into-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-suggest-ref/move-into-closure" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-suggest-ref/move-into-closure/auxiliary" "-A" "unused"
2019-12-24T18:24:16.8438906Z ------------------------------------------
2019-12-24T18:24:16.8438942Z 
2019-12-24T18:24:16.8439151Z ------------------------------------------
2019-12-24T18:24:16.8439213Z stderr:
2019-12-24T18:24:16.8439213Z stderr:
2019-12-24T18:24:16.8439426Z ------------------------------------------
2019-12-24T18:24:16.8439479Z error[E0507]: cannot move out of `x.0`, as `x` is a captured variable in an `Fn` closure
2019-12-24T18:24:16.8439871Z    |
2019-12-24T18:24:16.8439912Z LL |     let x = X(Y);
2019-12-24T18:24:16.8440146Z    |         - captured outer variable
2019-12-24T18:24:16.8440189Z ...
2019-12-24T18:24:16.8440189Z ...
2019-12-24T18:24:16.8440232Z LL |         let X(_t) = x;
2019-12-24T18:24:16.8440489Z    |               --    ^ help: consider borrowing here: `&x`
2019-12-24T18:24:16.8440583Z    |               data moved here
2019-12-24T18:24:16.8440583Z    |               data moved here
2019-12-24T18:24:16.8440636Z    |               move occurs because `_t` has type `Y`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8440689Z 
2019-12-24T18:24:16.8440737Z error[E0507]: cannot move out of `e.0`, as `e` is a captured variable in an `Fn` closure
2019-12-24T18:24:16.8441063Z    |
2019-12-24T18:24:16.8441063Z    |
2019-12-24T18:24:16.8441105Z LL |     let e = Either::One(X(Y));
2019-12-24T18:24:16.8441367Z ...
2019-12-24T18:24:16.8441367Z ...
2019-12-24T18:24:16.8441411Z LL |         if let Either::One(_t) = e { }
2019-12-24T18:24:16.8441699Z    |                            |
2019-12-24T18:24:16.8441769Z    |                            data moved here
2019-12-24T18:24:16.8441769Z    |                            data moved here
2019-12-24T18:24:16.8441823Z    |                            move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8441859Z 
2019-12-24T18:24:16.8441923Z error[E0507]: cannot move out of `e.0`, as `e` is a captured variable in an `Fn` closure
2019-12-24T18:24:16.8442235Z    |
2019-12-24T18:24:16.8442235Z    |
2019-12-24T18:24:16.8442293Z LL |     let e = Either::One(X(Y));
2019-12-24T18:24:16.8442540Z ...
2019-12-24T18:24:16.8442540Z ...
2019-12-24T18:24:16.8442584Z LL |         while let Either::One(_t) = e { }
2019-12-24T18:24:16.8442891Z    |                               |
2019-12-24T18:24:16.8442937Z    |                               data moved here
2019-12-24T18:24:16.8442937Z    |                               data moved here
2019-12-24T18:24:16.8443016Z    |                               move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8443053Z 
2019-12-24T18:24:16.8443103Z error[E0507]: cannot move out of `e.0`, as `e` is a captured variable in an `Fn` closure
2019-12-24T18:24:16.8443423Z    |
2019-12-24T18:24:16.8443423Z    |
2019-12-24T18:24:16.8443549Z LL |     let e = Either::One(X(Y));
2019-12-24T18:24:16.8443839Z ...
2019-12-24T18:24:16.8443879Z LL |         match e {
2019-12-24T18:24:16.8443945Z    |               ^ help: consider borrowing here: `&e`
2019-12-24T18:24:16.8443986Z ...
2019-12-24T18:24:16.8443986Z ...
2019-12-24T18:24:16.8444027Z LL |             | Either::Two(_t) => (),
2019-12-24T18:24:16.8444288Z    |                           |
2019-12-24T18:24:16.8444333Z    |                           data moved here
2019-12-24T18:24:16.8444333Z    |                           data moved here
2019-12-24T18:24:16.8444462Z    |                           move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8444513Z 
2019-12-24T18:24:16.8444560Z error[E0507]: cannot move out of `e.0`, as `e` is a captured variable in an `Fn` closure
2019-12-24T18:24:16.8445066Z    |
2019-12-24T18:24:16.8445066Z    |
2019-12-24T18:24:16.8445121Z LL |     let e = Either::One(X(Y));
2019-12-24T18:24:16.8445377Z ...
2019-12-24T18:24:16.8445436Z LL |         match e {
2019-12-24T18:24:16.8445483Z    |               ^ help: consider borrowing here: `&e`
2019-12-24T18:24:16.8445525Z ...
2019-12-24T18:24:16.8445525Z ...
2019-12-24T18:24:16.8445588Z LL |             Either::One(_t) => (),
2019-12-24T18:24:16.8445839Z    |                         |
2019-12-24T18:24:16.8445901Z    |                         data moved here
2019-12-24T18:24:16.8445901Z    |                         data moved here
2019-12-24T18:24:16.8445964Z    |                         move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8446000Z 
2019-12-24T18:24:16.8446048Z error[E0507]: cannot move out of `x.0`, as `x` is a captured variable in an `Fn` closure
2019-12-24T18:24:16.8446376Z    |
2019-12-24T18:24:16.8446425Z LL |     let x = X(Y);
2019-12-24T18:24:16.8446650Z    |         - captured outer variable
2019-12-24T18:24:16.8446693Z ...
2019-12-24T18:24:16.8446693Z ...
2019-12-24T18:24:16.8446736Z LL |         let X(mut _t) = x;
2019-12-24T18:24:16.8446985Z    |               ------    ^ help: consider borrowing here: `&x`
2019-12-24T18:24:16.8447077Z    |               data moved here
2019-12-24T18:24:16.8447077Z    |               data moved here
2019-12-24T18:24:16.8447150Z    |               move occurs because `_t` has type `Y`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8447193Z 
2019-12-24T18:24:16.8447243Z error[E0507]: cannot move out of `em.0`, as `em` is a captured variable in an `Fn` closure
2019-12-24T18:24:16.8447570Z    |
2019-12-24T18:24:16.8447570Z    |
2019-12-24T18:24:16.8447614Z LL |     let mut em = Either::One(X(Y));
2019-12-24T18:24:16.8447829Z    |         ------ captured outer variable
2019-12-24T18:24:16.8447889Z ...
2019-12-24T18:24:16.8447942Z LL |         if let Either::One(mut _t) = em { }
2019-12-24T18:24:16.8448419Z    |                            ------    ^^ help: consider borrowing here: `&em`
2019-12-24T18:24:16.8448541Z    |                            data moved here
2019-12-24T18:24:16.8448541Z    |                            data moved here
2019-12-24T18:24:16.8448599Z    |                            move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8448635Z 
2019-12-24T18:24:16.8448702Z error[E0507]: cannot move out of `em.0`, as `em` is a captured variable in an `Fn` closure
2019-12-24T18:24:16.8449053Z    |
2019-12-24T18:24:16.8449053Z    |
2019-12-24T18:24:16.8449117Z LL |     let mut em = Either::One(X(Y));
2019-12-24T18:24:16.8449350Z    |         ------ captured outer variable
2019-12-24T18:24:16.8449395Z ...
2019-12-24T18:24:16.8449458Z LL |         while let Either::One(mut _t) = em { }
2019-12-24T18:24:16.8449805Z    |                               ------    ^^ help: consider borrowing here: `&em`
2019-12-24T18:24:16.8449932Z    |                               data moved here
2019-12-24T18:24:16.8449932Z    |                               data moved here
2019-12-24T18:24:16.8449989Z    |                               move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8450026Z 
2019-12-24T18:24:16.8450075Z error[E0507]: cannot move out of `em.0`, as `em` is a captured variable in an `Fn` closure
2019-12-24T18:24:16.8450504Z    |
2019-12-24T18:24:16.8450504Z    |
2019-12-24T18:24:16.8450549Z LL |     let mut em = Either::One(X(Y));
2019-12-24T18:24:16.8450795Z    |         ------ captured outer variable
2019-12-24T18:24:16.8450884Z LL |         match em {
2019-12-24T18:24:16.8450949Z    |               ^^ help: consider borrowing here: `&em`
2019-12-24T18:24:16.8451004Z ...
2019-12-24T18:24:16.8451004Z ...
2019-12-24T18:24:16.8451050Z LL |             | Either::Two(mut _t) => (),
2019-12-24T18:24:16.8451337Z    |                           |
2019-12-24T18:24:16.8451385Z    |                           data moved here
2019-12-24T18:24:16.8451385Z    |                           data moved here
2019-12-24T18:24:16.8451439Z    |                           move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8451494Z 
2019-12-24T18:24:16.8451543Z error[E0507]: cannot move out of `em.0`, as `em` is a captured variable in an `Fn` closure
2019-12-24T18:24:16.8451891Z    |
2019-12-24T18:24:16.8451891Z    |
2019-12-24T18:24:16.8451937Z LL |     let mut em = Either::One(X(Y));
2019-12-24T18:24:16.8452161Z    |         ------ captured outer variable
2019-12-24T18:24:16.8452268Z LL |         match em {
2019-12-24T18:24:16.8452326Z    |               ^^ help: consider borrowing here: `&em`
2019-12-24T18:24:16.8452370Z ...
2019-12-24T18:24:16.8452370Z ...
2019-12-24T18:24:16.8452433Z LL |             Either::One(mut _t) => (),
2019-12-24T18:24:16.8452695Z    |                         |
2019-12-24T18:24:16.8452759Z    |                         data moved here
2019-12-24T18:24:16.8452759Z    |                         data moved here
2019-12-24T18:24:16.8452814Z    |                         move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8452850Z 
2019-12-24T18:24:16.8452916Z error[E0507]: cannot move out of `x.0`, as `x` is a captured variable in an `FnMut` closure
2019-12-24T18:24:16.8453245Z    |
2019-12-24T18:24:16.8453289Z LL |     let x = X(Y);
2019-12-24T18:24:16.8453519Z    |         - captured outer variable
2019-12-24T18:24:16.8453564Z ...
2019-12-24T18:24:16.8453564Z ...
2019-12-24T18:24:16.8453606Z LL |         let X(_t) = x;
2019-12-24T18:24:16.8453874Z    |               --    ^ help: consider borrowing here: `&x`
2019-12-24T18:24:16.8453968Z    |               data moved here
2019-12-24T18:24:16.8453968Z    |               data moved here
2019-12-24T18:24:16.8454038Z    |               move occurs because `_t` has type `Y`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8454074Z 
2019-12-24T18:24:16.8454124Z error[E0507]: cannot move out of `e.0`, as `e` is a captured variable in an `FnMut` closure
2019-12-24T18:24:16.8454469Z    |
2019-12-24T18:24:16.8454469Z    |
2019-12-24T18:24:16.8454513Z LL |     let e = Either::One(X(Y));
2019-12-24T18:24:16.8454795Z ...
2019-12-24T18:24:16.8454795Z ...
2019-12-24T18:24:16.8454840Z LL |         if let Either::One(_t) = e { }
2019-12-24T18:24:16.8455405Z    |                            |
2019-12-24T18:24:16.8455556Z    |                            data moved here
2019-12-24T18:24:16.8455556Z    |                            data moved here
2019-12-24T18:24:16.8455619Z    |                            move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8455655Z 
2019-12-24T18:24:16.8455722Z error[E0507]: cannot move out of `e.0`, as `e` is a captured variable in an `FnMut` closure
2019-12-24T18:24:16.8456508Z    |
2019-12-24T18:24:16.8456508Z    |
2019-12-24T18:24:16.8456569Z LL |     let e = Either::One(X(Y));
2019-12-24T18:24:16.8457192Z ...
2019-12-24T18:24:16.8457192Z ...
2019-12-24T18:24:16.8457255Z LL |         while let Either::One(_t) = e { }
2019-12-24T18:24:16.8457622Z    |                               |
2019-12-24T18:24:16.8457687Z    |                               data moved here
2019-12-24T18:24:16.8457687Z    |                               data moved here
2019-12-24T18:24:16.8457753Z    |                               move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8457790Z 
2019-12-24T18:24:16.8457857Z error[E0507]: cannot move out of `e.0`, as `e` is a captured variable in an `FnMut` closure
2019-12-24T18:24:16.8458183Z    |
2019-12-24T18:24:16.8458183Z    |
2019-12-24T18:24:16.8458227Z LL |     let e = Either::One(X(Y));
2019-12-24T18:24:16.8458506Z ...
2019-12-24T18:24:16.8458547Z LL |         match e {
2019-12-24T18:24:16.8458613Z    |               ^ help: consider borrowing here: `&e`
2019-12-24T18:24:16.8458654Z ...
2019-12-24T18:24:16.8458654Z ...
2019-12-24T18:24:16.8458698Z LL |             | Either::Two(_t) => (),
2019-12-24T18:24:16.8458972Z    |                           |
2019-12-24T18:24:16.8459020Z    |                           data moved here
2019-12-24T18:24:16.8459020Z    |                           data moved here
2019-12-24T18:24:16.8459099Z    |                           move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8459135Z 
2019-12-24T18:24:16.8459184Z error[E0507]: cannot move out of `e.0`, as `e` is a captured variable in an `FnMut` closure
2019-12-24T18:24:16.8459515Z    |
2019-12-24T18:24:16.8459515Z    |
2019-12-24T18:24:16.8459558Z LL |     let e = Either::One(X(Y));
2019-12-24T18:24:16.8459836Z ...
2019-12-24T18:24:16.8459878Z LL |         match e {
2019-12-24T18:24:16.8459926Z    |               ^ help: consider borrowing here: `&e`
2019-12-24T18:24:16.8459969Z ...
2019-12-24T18:24:16.8459969Z ...
2019-12-24T18:24:16.8460031Z LL |             Either::One(_t) => (),
2019-12-24T18:24:16.8460286Z    |                         |
2019-12-24T18:24:16.8460349Z    |                         data moved here
2019-12-24T18:24:16.8460349Z    |                         data moved here
2019-12-24T18:24:16.8460411Z    |                         move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8460446Z 
2019-12-24T18:24:16.8460512Z error[E0507]: cannot move out of `x.0`, as `x` is a captured variable in an `FnMut` closure
2019-12-24T18:24:16.8460821Z    |
2019-12-24T18:24:16.8460880Z LL |     let x = X(Y);
2019-12-24T18:24:16.8461245Z    |         - captured outer variable
2019-12-24T18:24:16.8461295Z ...
2019-12-24T18:24:16.8461295Z ...
2019-12-24T18:24:16.8461336Z LL |         let X(mut _t) = x;
2019-12-24T18:24:16.8461586Z    |               ------    ^ help: consider borrowing here: `&x`
2019-12-24T18:24:16.8461676Z    |               data moved here
2019-12-24T18:24:16.8461676Z    |               data moved here
2019-12-24T18:24:16.8461744Z    |               move occurs because `_t` has type `Y`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8461777Z 
2019-12-24T18:24:16.8461906Z error[E0507]: cannot move out of `em.0`, as `em` is a captured variable in an `FnMut` closure
2019-12-24T18:24:16.8462254Z    |
2019-12-24T18:24:16.8462254Z    |
2019-12-24T18:24:16.8462297Z LL |     let mut em = Either::One(X(Y));
2019-12-24T18:24:16.8462506Z    |         ------ captured outer variable
2019-12-24T18:24:16.8462565Z ...
2019-12-24T18:24:16.8462609Z LL |         if let Either::One(mut _t) = em { }
2019-12-24T18:24:16.8462949Z    |                            ------    ^^ help: consider borrowing here: `&em`
2019-12-24T18:24:16.8463062Z    |                            data moved here
2019-12-24T18:24:16.8463062Z    |                            data moved here
2019-12-24T18:24:16.8463117Z    |                            move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8463169Z 
2019-12-24T18:24:16.8463225Z error[E0507]: cannot move out of `em.0`, as `em` is a captured variable in an `FnMut` closure
2019-12-24T18:24:16.8463544Z    |
2019-12-24T18:24:16.8463544Z    |
2019-12-24T18:24:16.8463587Z LL |     let mut em = Either::One(X(Y));
2019-12-24T18:24:16.8463796Z    |         ------ captured outer variable
2019-12-24T18:24:16.8463838Z ...
2019-12-24T18:24:16.8463900Z LL |         while let Either::One(mut _t) = em { }
2019-12-24T18:24:16.8464149Z    |                               ------    ^^ help: consider borrowing here: `&em`
2019-12-24T18:24:16.8464271Z    |                               data moved here
2019-12-24T18:24:16.8464271Z    |                               data moved here
2019-12-24T18:24:16.8464324Z    |                               move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8464359Z 
2019-12-24T18:24:16.8464425Z error[E0507]: cannot move out of `em.0`, as `em` is a captured variable in an `FnMut` closure
2019-12-24T18:24:16.8464734Z    |
2019-12-24T18:24:16.8464734Z    |
2019-12-24T18:24:16.8464796Z LL |     let mut em = Either::One(X(Y));
2019-12-24T18:24:16.8465003Z    |         ------ captured outer variable
2019-12-24T18:24:16.8465085Z LL |         match em {
2019-12-24T18:24:16.8465148Z    |               ^^ help: consider borrowing here: `&em`
2019-12-24T18:24:16.8465190Z ...
2019-12-24T18:24:16.8465190Z ...
2019-12-24T18:24:16.8465233Z LL |             | Either::Two(mut _t) => (),
2019-12-24T18:24:16.8465509Z    |                           |
2019-12-24T18:24:16.8465554Z    |                           data moved here
2019-12-24T18:24:16.8465554Z    |                           data moved here
2019-12-24T18:24:16.8465625Z    |                           move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8465660Z 
2019-12-24T18:24:16.8465715Z error[E0507]: cannot move out of `em.0`, as `em` is a captured variable in an `FnMut` closure
2019-12-24T18:24:16.8466030Z    |
2019-12-24T18:24:16.8466030Z    |
2019-12-24T18:24:16.8466071Z LL |     let mut em = Either::One(X(Y));
2019-12-24T18:24:16.8466276Z    |         ------ captured outer variable
2019-12-24T18:24:16.8466375Z LL |         match em {
2019-12-24T18:24:16.8466420Z    |               ^^ help: consider borrowing here: `&em`
2019-12-24T18:24:16.8466548Z ...
2019-12-24T18:24:16.8466548Z ...
2019-12-24T18:24:16.8466592Z LL |             Either::One(mut _t) => (),
2019-12-24T18:24:16.8466837Z    |                         |
2019-12-24T18:24:16.8466898Z    |                         data moved here
2019-12-24T18:24:16.8466898Z    |                         data moved here
2019-12-24T18:24:16.8466951Z    |                         move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8466984Z 
2019-12-24T18:24:16.8467117Z error[E0507]: cannot move out of `em.0`, as `em` is a captured variable in an `FnMut` closure
2019-12-24T18:24:16.8467443Z    |
2019-12-24T18:24:16.8467443Z    |
2019-12-24T18:24:16.8467502Z LL |     let mut em = Either::One(X(Y));
2019-12-24T18:24:16.8467711Z    |         ------ captured outer variable
2019-12-24T18:24:16.8467794Z LL |         match em {
2019-12-24T18:24:16.8467856Z    |               ^^ help: consider borrowing here: `&em`
2019-12-24T18:24:16.8467973Z ...
2019-12-24T18:24:16.8467973Z ...
2019-12-24T18:24:16.8468016Z LL |             Either::One(mut _t) => (),
2019-12-24T18:24:16.8468298Z    |                         |
2019-12-24T18:24:16.8468341Z    |                         data moved here
2019-12-24T18:24:16.8468341Z    |                         data moved here
2019-12-24T18:24:16.8468411Z    |                         move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8468494Z error: aborting due to 21 previous errors
2019-12-24T18:24:16.8468523Z 
2019-12-24T18:24:16.8468774Z For more information about this error, try `rustc --explain E0507`.
2019-12-24T18:24:16.8468806Z 
2019-12-24T18:24:16.8468806Z 
2019-12-24T18:24:16.8469006Z ------------------------------------------
2019-12-24T18:24:16.8469035Z 
2019-12-24T18:24:16.8469075Z 
2019-12-24T18:24:16.8469315Z ---- [ui] ui/suggestions/dont-suggest-ref/duplicate-suggestions.rs stdout ----
2019-12-24T18:24:16.8469362Z diff of stderr:
2019-12-24T18:24:16.8469397Z 
2019-12-24T18:24:16.8469452Z 63    |
2019-12-24T18:24:16.8469495Z 64 LL |     match &(e.clone(), e.clone()) {
2019-12-24T18:24:16.8469729Z - LL |
2019-12-24T18:24:16.8469729Z - LL |
2019-12-24T18:24:16.8469941Z - LL |         &(Either::One(_t), Either::Two(_u))
2019-12-24T18:24:16.8470360Z -    |         |             |                |
2019-12-24T18:24:16.8470608Z -    |         |             |                ...and here
2019-12-24T18:24:16.8470817Z -    |         |             data moved here
2019-12-24T18:24:16.8470817Z -    |         |             data moved here
2019-12-24T18:24:16.8471064Z -    |         help: consider removing the `&`: `(Either::One(_t), Either::Two(_u))`
2019-12-24T18:24:16.8471129Z + ...
2019-12-24T18:24:16.8471174Z + LL |         | &(Either::Two(_t), Either::One(_u)) => (),
2019-12-24T18:24:16.8471510Z +    |                         |
2019-12-24T18:24:16.8471565Z +    |                         data moved here
2019-12-24T18:24:16.8471607Z 73    |
2019-12-24T18:24:16.8471895Z 74    = note: move occurs because these variables have types that don't implement the `Copy` trait
2019-12-24T18:24:16.8471895Z 74    = note: move occurs because these variables have types that don't implement the `Copy` trait
2019-12-24T18:24:16.8471948Z + help: consider removing the `&`
2019-12-24T18:24:16.8471988Z +    |
2019-12-24T18:24:16.8472031Z + LL |         (Either::One(_t), Either::Two(_u))
2019-12-24T18:24:16.8472092Z + LL |
2019-12-24T18:24:16.8472130Z + LL |
2019-12-24T18:24:16.8472182Z + LL |         | &(Either::Two(_t), Either::One(_u)) => (),
2019-12-24T18:24:16.8472280Z 75 
2019-12-24T18:24:16.8472323Z 76 error[E0507]: cannot move out of a shared reference
2019-12-24T18:24:16.8472556Z 77   --> $DIR/duplicate-suggestions.rs:70:11
2019-12-24T18:24:16.8472587Z 
2019-12-24T18:24:16.8472587Z 
2019-12-24T18:24:16.8472626Z 168    |
2019-12-24T18:24:16.8472669Z 169 LL |     match &mut (em.clone(), em.clone()) {
2019-12-24T18:24:16.8472916Z - LL |
2019-12-24T18:24:16.8472916Z - LL |
2019-12-24T18:24:16.8473129Z - LL |         &mut (Either::One(_t), Either::Two(_u))
2019-12-24T18:24:16.8473576Z -    |         |                 |                |
2019-12-24T18:24:16.8473798Z -    |         |                 |                ...and here
2019-12-24T18:24:16.8474025Z -    |         |                 data moved here
2019-12-24T18:24:16.8474025Z -    |         |                 data moved here
2019-12-24T18:24:16.8474352Z -    |         help: consider removing the `&mut`: `(Either::One(_t), Either::Two(_u))`
2019-12-24T18:24:16.8474407Z + ...
2019-12-24T18:24:16.8474452Z + LL |         | &mut (Either::Two(_t), Either::One(_u)) => (),
2019-12-24T18:24:16.8474769Z +    |                             |
2019-12-24T18:24:16.8474815Z +    |                             data moved here
2019-12-24T18:24:16.8474874Z 178    |
2019-12-24T18:24:16.8475137Z 179    = note: move occurs because these variables have types that don't implement the `Copy` trait
2019-12-24T18:24:16.8475137Z 179    = note: move occurs because these variables have types that don't implement the `Copy` trait
2019-12-24T18:24:16.8475262Z + help: consider removing the `&mut`
2019-12-24T18:24:16.8475321Z +    |
2019-12-24T18:24:16.8475365Z + LL |         (Either::One(_t), Either::Two(_u))
2019-12-24T18:24:16.8475406Z + LL |
2019-12-24T18:24:16.8475461Z + LL |
2019-12-24T18:24:16.8475505Z + LL |         | &mut (Either::Two(_t), Either::One(_u)) => (),
2019-12-24T18:24:16.8475585Z 180 
2019-12-24T18:24:16.8475654Z 181 error[E0507]: cannot move out of a mutable reference
2019-12-24T18:24:16.8475894Z 182   --> $DIR/duplicate-suggestions.rs:122:11
2019-12-24T18:24:16.8475925Z 
2019-12-24T18:24:16.8475925Z 
2019-12-24T18:24:16.8475951Z 
2019-12-24T18:24:16.8476010Z The actual stderr differed from the expected stderr.
2019-12-24T18:24:16.8476330Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-suggest-ref/duplicate-suggestions/duplicate-suggestions.stderr
2019-12-24T18:24:16.8476585Z To update references, rerun the tests and pass the `--bless` flag
2019-12-24T18:24:16.8476883Z To only update this specific test, also pass `--test-args suggestions/dont-suggest-ref/duplicate-suggestions.rs`
2019-12-24T18:24:16.8476962Z error: 1 errors occurred comparing output.
2019-12-24T18:24:16.8477023Z status: exit code: 1
2019-12-24T18:24:16.8477023Z status: exit code: 1
2019-12-24T18:24:16.8477919Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/dont-suggest-ref/duplicate-suggestions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-suggest-ref/duplicate-suggestions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-suggest-ref/duplicate-suggestions/auxiliary" "-A" "unused"
2019-12-24T18:24:16.8478260Z ------------------------------------------
2019-12-24T18:24:16.8478309Z 
2019-12-24T18:24:16.8478517Z ------------------------------------------
2019-12-24T18:24:16.8478561Z stderr:
2019-12-24T18:24:16.8478561Z stderr:
2019-12-24T18:24:16.8478760Z ------------------------------------------
2019-12-24T18:24:16.8478825Z error[E0507]: cannot move out of a shared reference
2019-12-24T18:24:16.8479085Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/duplicate-suggestions.rs:39:27
2019-12-24T18:24:16.8479135Z    |
2019-12-24T18:24:16.8479198Z LL |     let &(X(_t), X(_u)) = &(x.clone(), x.clone());
2019-12-24T18:24:16.8479419Z    |         ---------------   ^^^^^^^^^^^^^^^^^^^^^^^
2019-12-24T18:24:16.8479525Z    |         |   |      ...and here
2019-12-24T18:24:16.8479569Z    |         |   data moved here
2019-12-24T18:24:16.8479569Z    |         |   data moved here
2019-12-24T18:24:16.8479616Z    |         help: consider removing the `&`: `(X(_t), X(_u))`
2019-12-24T18:24:16.8479944Z    = note: move occurs because these variables have types that don't implement the `Copy` trait
2019-12-24T18:24:16.8479979Z 
2019-12-24T18:24:16.8480022Z error[E0507]: cannot move out of a shared reference
2019-12-24T18:24:16.8480290Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/duplicate-suggestions.rs:43:50
2019-12-24T18:24:16.8480290Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/duplicate-suggestions.rs:43:50
2019-12-24T18:24:16.8480336Z    |
2019-12-24T18:24:16.8480622Z LL |     if let &(Either::One(_t), Either::Two(_u)) = &(e.clone(), e.clone()) { }
2019-12-24T18:24:16.8480902Z    |            -----------------------------------   ^^^^^^^^^^^^^^^^^^^^^^^
2019-12-24T18:24:16.8480998Z    |            |             |                ...and here
2019-12-24T18:24:16.8481059Z    |            |             data moved here
2019-12-24T18:24:16.8481059Z    |            |             data moved here
2019-12-24T18:24:16.8481109Z    |            help: consider removing the `&`: `(Either::One(_t), Either::Two(_u))`
2019-12-24T18:24:16.8481523Z    = note: move occurs because these variables have types that don't implement the `Copy` trait
2019-12-24T18:24:16.8481557Z 
2019-12-24T18:24:16.8481598Z error[E0507]: cannot move out of a shared reference
2019-12-24T18:24:16.8481841Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/duplicate-suggestions.rs:47:53
2019-12-24T18:24:16.8481841Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/duplicate-suggestions.rs:47:53
2019-12-24T18:24:16.8481902Z    |
2019-12-24T18:24:16.8481956Z LL |     while let &(Either::One(_t), Either::Two(_u)) = &(e.clone(), e.clone()) { }
2019-12-24T18:24:16.8482198Z    |               -----------------------------------   ^^^^^^^^^^^^^^^^^^^^^^^
2019-12-24T18:24:16.8482308Z    |               |             |                ...and here
2019-12-24T18:24:16.8482354Z    |               |             data moved here
2019-12-24T18:24:16.8482354Z    |               |             data moved here
2019-12-24T18:24:16.8482421Z    |               help: consider removing the `&`: `(Either::One(_t), Either::Two(_u))`
2019-12-24T18:24:16.8482743Z    = note: move occurs because these variables have types that don't implement the `Copy` trait
2019-12-24T18:24:16.8482794Z 
2019-12-24T18:24:16.8482839Z error[E0507]: cannot move out of a shared reference
2019-12-24T18:24:16.8483101Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/duplicate-suggestions.rs:51:11
2019-12-24T18:24:16.8483101Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/duplicate-suggestions.rs:51:11
2019-12-24T18:24:16.8483163Z    |
2019-12-24T18:24:16.8483214Z LL |     match &(e.clone(), e.clone()) {
2019-12-24T18:24:16.8483259Z    |           ^^^^^^^^^^^^^^^^^^^^^^^
2019-12-24T18:24:16.8483304Z LL |         //~^ ERROR cannot move
2019-12-24T18:24:16.8483369Z LL |         &(Either::One(_t), Either::Two(_u)) => (),
2019-12-24T18:24:16.8483648Z    |                       |
2019-12-24T18:24:16.8483710Z    |                       data moved here
2019-12-24T18:24:16.8483751Z ...
2019-12-24T18:24:16.8483751Z ...
2019-12-24T18:24:16.8483796Z LL |         &(Either::Two(_t), Either::One(_u)) => (),
2019-12-24T18:24:16.8484102Z    |
2019-12-24T18:24:16.8484364Z    = note: move occurs because these variables have types that don't implement the `Copy` trait
2019-12-24T18:24:16.8484430Z help: consider removing the `&`
2019-12-24T18:24:16.8484472Z    |
2019-12-24T18:24:16.8484472Z    |
2019-12-24T18:24:16.8484517Z LL |         (Either::One(_t), Either::Two(_u)) => (),
2019-12-24T18:24:16.8484635Z help: consider removing the `&`
2019-12-24T18:24:16.8484675Z    |
---
2019-12-24T18:24:16.8577065Z 
2019-12-24T18:24:16.8577106Z 31 LL |     match *r {
2019-12-24T18:24:16.8577170Z 32    |           ^^ help: consider borrowing here: `&*r`
2019-12-24T18:24:16.8577223Z 33 ...
2019-12-24T18:24:16.8577424Z - LL |         Either::One(_t)
2019-12-24T18:24:16.8577839Z -    |                     |
2019-12-24T18:24:16.8578048Z -    |                     data moved here
2019-12-24T18:24:16.8578048Z -    |                     data moved here
2019-12-24T18:24:16.8578789Z -    |                     move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8578898Z + LL |         | Either::Two(_t) => (),
2019-12-24T18:24:16.8579881Z +    |                       |
2019-12-24T18:24:16.8579957Z +    |                       data moved here
2019-12-24T18:24:16.8579957Z +    |                       data moved here
2019-12-24T18:24:16.8580013Z +    |                       move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8580672Z 40 error[E0507]: cannot move out of `r.0` which is behind a shared reference
2019-12-24T18:24:16.8580978Z 41   --> $DIR/simple.rs:57:11
2019-12-24T18:24:16.8581026Z 
2019-12-24T18:24:16.8581085Z 82 LL |     match *rm {
2019-12-24T18:24:16.8581085Z 82 LL |     match *rm {
2019-12-24T18:24:16.8581134Z 83    |           ^^^ help: consider borrowing here: `&*rm`
2019-12-24T18:24:16.8581179Z 84 ...
2019-12-24T18:24:16.8581385Z - LL |         Either::One(_t)
2019-12-24T18:24:16.8581801Z -    |                     |
2019-12-24T18:24:16.8582013Z -    |                     data moved here
2019-12-24T18:24:16.8582013Z -    |                     data moved here
2019-12-24T18:24:16.8582613Z -    |                     move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8582678Z + LL |         | Either::Two(_t) => (),
2019-12-24T18:24:16.8584067Z +    |                       |
2019-12-24T18:24:16.8584113Z +    |                       data moved here
2019-12-24T18:24:16.8584113Z +    |                       data moved here
2019-12-24T18:24:16.8584166Z +    |                       move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8584422Z 91 error[E0507]: cannot move out of `rm.0` which is behind a mutable reference
2019-12-24T18:24:16.8584670Z 92   --> $DIR/simple.rs:85:11
2019-12-24T18:24:16.8584704Z 
2019-12-24T18:24:16.8584704Z 
2019-12-24T18:24:16.8584761Z 145 LL |     match vr[0] {
2019-12-24T18:24:16.8584808Z 146    |           ^^^^^ help: consider borrowing here: `&vr[0]`
2019-12-24T18:24:16.8584851Z 147 ...
2019-12-24T18:24:16.8585063Z - LL |         Either::One(_t)
2019-12-24T18:24:16.8585451Z -    |                     |
2019-12-24T18:24:16.8585670Z -    |                     data moved here
2019-12-24T18:24:16.8585670Z -    |                     data moved here
2019-12-24T18:24:16.8586726Z -    |                     move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8586798Z + LL |         | Either::Two(_t) => (),
2019-12-24T18:24:16.8587134Z +    |                       |
2019-12-24T18:24:16.8587178Z +    |                       data moved here
2019-12-24T18:24:16.8587178Z +    |                       data moved here
2019-12-24T18:24:16.8587246Z +    |                       move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8587313Z 153 
2019-12-24T18:24:16.8587360Z 154 error[E0507]: cannot move out of index of `std::vec::Vec<Either>`
2019-12-24T18:24:16.8587619Z 
2019-12-24T18:24:16.8587619Z 
2019-12-24T18:24:16.8587660Z 196 LL |     match vrm[0] {
2019-12-24T18:24:16.8587714Z 197    |           ^^^^^^ help: consider borrowing here: `&vrm[0]`
2019-12-24T18:24:16.8587775Z 198 ...
2019-12-24T18:24:16.8587967Z - LL |         Either::One(_t)
2019-12-24T18:24:16.8588342Z -    |                     |
2019-12-24T18:24:16.8588563Z -    |                     data moved here
2019-12-24T18:24:16.8588563Z -    |                     data moved here
2019-12-24T18:24:16.8588829Z -    |                     move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8588881Z + LL |         | Either::Two(_t) => (),
2019-12-24T18:24:16.8589144Z +    |                       |
2019-12-24T18:24:16.8589188Z +    |                       data moved here
2019-12-24T18:24:16.8589188Z +    |                       data moved here
2019-12-24T18:24:16.8589258Z +    |                       move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8589305Z 204 
2019-12-24T18:24:16.8589350Z 205 error[E0507]: cannot move out of index of `std::vec::Vec<Either>`
2019-12-24T18:24:16.8589605Z 
2019-12-24T18:24:16.8589643Z 261    |
2019-12-24T18:24:16.8589683Z 262 LL |     match r {
2019-12-24T18:24:16.8589743Z 263    |           ^
2019-12-24T18:24:16.8589743Z 263    |           ^
2019-12-24T18:24:16.8589783Z + ...
2019-12-24T18:24:16.8589825Z + LL |         | &Either::Two(_t) => (),
2019-12-24T18:24:16.8590084Z +    |                        |
2019-12-24T18:24:16.8590129Z +    |                        data moved here
2019-12-24T18:24:16.8590129Z +    |                        data moved here
2019-12-24T18:24:16.8590190Z +    |                        move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8590299Z + help: consider removing the `&`
2019-12-24T18:24:16.8590339Z +    |
2019-12-24T18:24:16.8590339Z +    |
2019-12-24T18:24:16.8590398Z + LL |         Either::One(_t)
2019-12-24T18:24:16.8590438Z 264 LL |
2019-12-24T18:24:16.8590636Z - LL |         &Either::One(_t)
2019-12-24T18:24:16.8591147Z -    |         |            |
2019-12-24T18:24:16.8591390Z -    |         |            data moved here
2019-12-24T18:24:16.8591390Z -    |         |            data moved here
2019-12-24T18:24:16.8591659Z -    |         |            move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8591914Z -    |         help: consider removing the `&`: `Either::One(_t)`
2019-12-24T18:24:16.8591960Z + LL |
2019-12-24T18:24:16.8592002Z + LL |         | &Either::Two(_t) => (),
2019-12-24T18:24:16.8592165Z 271 
2019-12-24T18:24:16.8592212Z 272 error[E0507]: cannot move out of `r.0` which is behind a shared reference
2019-12-24T18:24:16.8592462Z 273   --> $DIR/simple.rs:188:11
2019-12-24T18:24:16.8592492Z 
2019-12-24T18:24:16.8592492Z 
2019-12-24T18:24:16.8592554Z 418    |
2019-12-24T18:24:16.8592596Z 419 LL |     match (&e.clone(),) {
2019-12-24T18:24:16.8593004Z - LL |
2019-12-24T18:24:16.8593004Z - LL |
2019-12-24T18:24:16.8593205Z - LL |         (&Either::One(_t),)
2019-12-24T18:24:16.8593695Z -    |                       |
2019-12-24T18:24:16.8593931Z -    |                       data moved here
2019-12-24T18:24:16.8593931Z -    |                       data moved here
2019-12-24T18:24:16.8594250Z -    |                       move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8594305Z + ...
2019-12-24T18:24:16.8594352Z + LL |         | (&Either::Two(_t),) => (),
2019-12-24T18:24:16.8594810Z +    |                         |
2019-12-24T18:24:16.8594902Z +    |                         data moved here
2019-12-24T18:24:16.8594902Z +    |                         data moved here
2019-12-24T18:24:16.8594960Z +    |                         move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8595095Z 428 error[E0507]: cannot move out of a mutable reference
2019-12-24T18:24:16.8595388Z 429   --> $DIR/simple.rs:272:25
2019-12-24T18:24:16.8595441Z 
2019-12-24T18:24:16.8595484Z 500    |
2019-12-24T18:24:16.8595484Z 500    |
2019-12-24T18:24:16.8595536Z 501 LL |     match &e {
2019-12-24T18:24:16.8595600Z 502    |           ^^
2019-12-24T18:24:16.8595644Z + ...
2019-12-24T18:24:16.8595690Z + LL |         | &Either::Two(_t) => (),
2019-12-24T18:24:16.8596160Z +    |                        |
2019-12-24T18:24:16.8596375Z +    |                        data moved here
2019-12-24T18:24:16.8596375Z +    |                        data moved here
2019-12-24T18:24:16.8596436Z +    |                        move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8596553Z + help: consider removing the `&`
2019-12-24T18:24:16.8596593Z +    |
2019-12-24T18:24:16.8596593Z +    |
2019-12-24T18:24:16.8596652Z + LL |         Either::One(_t)
2019-12-24T18:24:16.8596692Z 503 LL |
2019-12-24T18:24:16.8596933Z - LL |         &Either::One(_t)
2019-12-24T18:24:16.8597539Z -    |         |            |
2019-12-24T18:24:16.8597921Z -    |         |            data moved here
2019-12-24T18:24:16.8597921Z -    |         |            data moved here
2019-12-24T18:24:16.8598243Z -    |         |            move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8598515Z -    |         help: consider removing the `&`: `Either::One(_t)`
2019-12-24T18:24:16.8598562Z + LL |
2019-12-24T18:24:16.8598607Z + LL |         | &Either::Two(_t) => (),
2019-12-24T18:24:16.8598704Z 510 
2019-12-24T18:24:16.8598750Z 511 error[E0507]: cannot move out of a shared reference
2019-12-24T18:24:16.8598974Z 512   --> $DIR/simple.rs:308:11
2019-12-24T18:24:16.8599016Z 
2019-12-24T18:24:16.8599016Z 
2019-12-24T18:24:16.8599056Z 569    |
2019-12-24T18:24:16.8599098Z 570 LL |     match &mut em {
2019-12-24T18:24:16.8599314Z 571    |           ^^^^^^^
2019-12-24T18:24:16.8599359Z + ...
2019-12-24T18:24:16.8599404Z + LL |         | &mut Either::Two(_t) => (),
2019-12-24T18:24:16.8599725Z +    |                            |
2019-12-24T18:24:16.8599870Z +    |                            data moved here
2019-12-24T18:24:16.8599870Z +    |                            data moved here
2019-12-24T18:24:16.8599932Z +    |                            move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8600040Z + help: consider removing the `&mut`
2019-12-24T18:24:16.8600082Z +    |
2019-12-24T18:24:16.8600082Z +    |
2019-12-24T18:24:16.8600143Z + LL |         Either::One(_t)
2019-12-24T18:24:16.8600184Z 572 LL |
2019-12-24T18:24:16.8600416Z - LL |         &mut Either::One(_t)
2019-12-24T18:24:16.8601159Z -    |         |                |
2019-12-24T18:24:16.8602080Z -    |         |                data moved here
2019-12-24T18:24:16.8602080Z -    |         |                data moved here
2019-12-24T18:24:16.8602398Z -    |         |                move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8602662Z -    |         help: consider removing the `&mut`: `Either::One(_t)`
2019-12-24T18:24:16.8602710Z + LL |
2019-12-24T18:24:16.8602754Z + LL |         | &mut Either::Two(_t) => (),
2019-12-24T18:24:16.8602869Z 579 
2019-12-24T18:24:16.8602913Z 580 error[E0507]: cannot move out of a mutable reference
2019-12-24T18:24:16.8603137Z 581   --> $DIR/simple.rs:343:11
2019-12-24T18:24:16.8603169Z 
2019-12-24T18:24:16.8603169Z 
2019-12-24T18:24:16.8603194Z 
2019-12-24T18:24:16.8603238Z The actual stderr differed from the expected stderr.
2019-12-24T18:24:16.8603549Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-suggest-ref/simple/simple.stderr
2019-12-24T18:24:16.8603797Z To update references, rerun the tests and pass the `--bless` flag
2019-12-24T18:24:16.8604056Z To only update this specific test, also pass `--test-args suggestions/dont-suggest-ref/simple.rs`
2019-12-24T18:24:16.8604148Z error: 1 errors occurred comparing output.
2019-12-24T18:24:16.8604192Z status: exit code: 1
2019-12-24T18:24:16.8604192Z status: exit code: 1
2019-12-24T18:24:16.8605319Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-suggest-ref/simple" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-suggest-ref/simple/auxiliary" "-A" "unused"
2019-12-24T18:24:16.8605666Z ------------------------------------------
2019-12-24T18:24:16.8605700Z 
2019-12-24T18:24:16.8605914Z ------------------------------------------
2019-12-24T18:24:16.8605959Z stderr:
2019-12-24T18:24:16.8605959Z stderr:
2019-12-24T18:24:16.8606184Z ------------------------------------------
2019-12-24T18:24:16.8606237Z error[E0507]: cannot move out of `s.0` which is behind a shared reference
2019-12-24T18:24:16.8606492Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:38:17
2019-12-24T18:24:16.8606561Z    |
2019-12-24T18:24:16.8606602Z LL |     let X(_t) = *s;
2019-12-24T18:24:16.8606833Z    |           --    ^^ help: consider borrowing here: `&*s`
2019-12-24T18:24:16.8606943Z    |           data moved here
2019-12-24T18:24:16.8606943Z    |           data moved here
2019-12-24T18:24:16.8606996Z    |           move occurs because `_t` has type `Y`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8607108Z error[E0507]: cannot move out of `r.0` which is behind a shared reference
2019-12-24T18:24:16.8607362Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:42:30
2019-12-24T18:24:16.8607425Z    |
2019-12-24T18:24:16.8607425Z    |
2019-12-24T18:24:16.8607471Z LL |     if let Either::One(_t) = *r { }
2019-12-24T18:24:16.8607715Z    |                        --    ^^ help: consider borrowing here: `&*r`
2019-12-24T18:24:16.8607939Z    |                        data moved here
2019-12-24T18:24:16.8607939Z    |                        data moved here
2019-12-24T18:24:16.8608003Z    |                        move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8608103Z error[E0507]: cannot move out of `r.0` which is behind a shared reference
2019-12-24T18:24:16.8608540Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:46:33
2019-12-24T18:24:16.8608586Z    |
2019-12-24T18:24:16.8608586Z    |
2019-12-24T18:24:16.8608645Z LL |     while let Either::One(_t) = *r { }
2019-12-24T18:24:16.8608978Z    |                           --    ^^ help: consider borrowing here: `&*r`
2019-12-24T18:24:16.8609090Z    |                           data moved here
2019-12-24T18:24:16.8609090Z    |                           data moved here
2019-12-24T18:24:16.8609145Z    |                           move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8609233Z error[E0507]: cannot move out of `r.0` which is behind a shared reference
2019-12-24T18:24:16.8609493Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:50:11
2019-12-24T18:24:16.8609538Z    |
2019-12-24T18:24:16.8609578Z LL |     match *r {
2019-12-24T18:24:16.8609578Z LL |     match *r {
2019-12-24T18:24:16.8609641Z    |           ^^ help: consider borrowing here: `&*r`
2019-12-24T18:24:16.8609682Z ...
2019-12-24T18:24:16.8609724Z LL |         | Either::Two(_t) => (),
2019-12-24T18:24:16.8609983Z    |                       |
2019-12-24T18:24:16.8610034Z    |                       data moved here
2019-12-24T18:24:16.8610034Z    |                       data moved here
2019-12-24T18:24:16.8610087Z    |                       move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8610186Z error[E0507]: cannot move out of `r.0` which is behind a shared reference
2019-12-24T18:24:16.8610429Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:57:11
2019-12-24T18:24:16.8610493Z    |
2019-12-24T18:24:16.8610533Z LL |     match *r {
2019-12-24T18:24:16.8610533Z LL |     match *r {
2019-12-24T18:24:16.8610585Z    |           ^^ help: consider borrowing here: `&*r`
2019-12-24T18:24:16.8610645Z ...
2019-12-24T18:24:16.8610688Z LL |         Either::One(_t) => (),
2019-12-24T18:24:16.8610925Z    |                     |
2019-12-24T18:24:16.8610984Z    |                     data moved here
2019-12-24T18:24:16.8610984Z    |                     data moved here
2019-12-24T18:24:16.8611036Z    |                     move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8611078Z 
2019-12-24T18:24:16.8611142Z error[E0507]: cannot move out of `sm.0` which is behind a mutable reference
2019-12-24T18:24:16.8611463Z    |
2019-12-24T18:24:16.8611463Z    |
2019-12-24T18:24:16.8611522Z LL |     let X(_t) = *sm;
2019-12-24T18:24:16.8611767Z    |           --    ^^^ help: consider borrowing here: `&*sm`
2019-12-24T18:24:16.8611861Z    |           data moved here
2019-12-24T18:24:16.8611861Z    |           data moved here
2019-12-24T18:24:16.8611939Z    |           move occurs because `_t` has type `Y`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8612023Z error[E0507]: cannot move out of `rm.0` which is behind a mutable reference
2019-12-24T18:24:16.8612299Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:70:30
2019-12-24T18:24:16.8612348Z    |
2019-12-24T18:24:16.8612348Z    |
2019-12-24T18:24:16.8612393Z LL |     if let Either::One(_t) = *rm { }
2019-12-24T18:24:16.8612668Z    |                        --    ^^^ help: consider borrowing here: `&*rm`
2019-12-24T18:24:16.8612774Z    |                        data moved here
2019-12-24T18:24:16.8612774Z    |                        data moved here
2019-12-24T18:24:16.8612851Z    |                        move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8612935Z error[E0507]: cannot move out of `rm.0` which is behind a mutable reference
2019-12-24T18:24:16.8613264Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:74:33
2019-12-24T18:24:16.8613338Z    |
2019-12-24T18:24:16.8613338Z    |
2019-12-24T18:24:16.8613383Z LL |     while let Either::One(_t) = *rm { }
2019-12-24T18:24:16.8613670Z    |                           --    ^^^ help: consider borrowing here: `&*rm`
2019-12-24T18:24:16.8613784Z    |                           data moved here
2019-12-24T18:24:16.8613784Z    |                           data moved here
2019-12-24T18:24:16.8613840Z    |                           move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8614007Z error[E0507]: cannot move out of `rm.0` which is behind a mutable reference
2019-12-24T18:24:16.8614286Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:78:11
2019-12-24T18:24:16.8614333Z    |
2019-12-24T18:24:16.8614393Z LL |     match *rm {
2019-12-24T18:24:16.8614393Z LL |     match *rm {
2019-12-24T18:24:16.8614440Z    |           ^^^ help: consider borrowing here: `&*rm`
2019-12-24T18:24:16.8614484Z ...
2019-12-24T18:24:16.8614556Z LL |         | Either::Two(_t) => (),
2019-12-24T18:24:16.8614815Z    |                       |
2019-12-24T18:24:16.8614878Z    |                       data moved here
2019-12-24T18:24:16.8614878Z    |                       data moved here
2019-12-24T18:24:16.8614935Z    |                       move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8615019Z error[E0507]: cannot move out of `rm.0` which is behind a mutable reference
2019-12-24T18:24:16.8615308Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:85:11
2019-12-24T18:24:16.8615356Z    |
2019-12-24T18:24:16.8615398Z LL |     match *rm {
2019-12-24T18:24:16.8615398Z LL |     match *rm {
2019-12-24T18:24:16.8615463Z    |           ^^^ help: consider borrowing here: `&*rm`
2019-12-24T18:24:16.8615507Z ...
2019-12-24T18:24:16.8615551Z LL |         Either::One(_t) => (),
2019-12-24T18:24:16.8615819Z    |                     |
2019-12-24T18:24:16.8615872Z    |                     data moved here
2019-12-24T18:24:16.8615872Z    |                     data moved here
2019-12-24T18:24:16.8616145Z    |                     move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8616261Z error[E0507]: cannot move out of `rm.0` which is behind a mutable reference
2019-12-24T18:24:16.8616568Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:93:11
2019-12-24T18:24:16.8616635Z    |
2019-12-24T18:24:16.8616678Z LL |     match *rm {
2019-12-24T18:24:16.8616678Z LL |     match *rm {
2019-12-24T18:24:16.8616726Z    |           ^^^ help: consider borrowing here: `&*rm`
2019-12-24T18:24:16.8616797Z ...
2019-12-24T18:24:16.8616843Z LL |         Either::One(_t) => (),
2019-12-24T18:24:16.8617104Z    |                     |
2019-12-24T18:24:16.8617167Z    |                     data moved here
2019-12-24T18:24:16.8617167Z    |                     data moved here
2019-12-24T18:24:16.8617223Z    |                     move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8617331Z error[E0507]: cannot move out of index of `std::vec::Vec<X>`
2019-12-24T18:24:16.8617592Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:102:17
2019-12-24T18:24:16.8617640Z    |
2019-12-24T18:24:16.8617640Z    |
2019-12-24T18:24:16.8617683Z LL |     let X(_t) = vs[0];
2019-12-24T18:24:16.8617949Z    |           --    ^^^^^ help: consider borrowing here: `&vs[0]`
2019-12-24T18:24:16.8618042Z    |           data moved here
2019-12-24T18:24:16.8618042Z    |           data moved here
2019-12-24T18:24:16.8618111Z    |           move occurs because `_t` has type `Y`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8618154Z 
2019-12-24T18:24:16.8618372Z error[E0507]: cannot move out of index of `std::vec::Vec<Either>`
2019-12-24T18:24:16.8618682Z    |
2019-12-24T18:24:16.8618682Z    |
2019-12-24T18:24:16.8618724Z LL |     if let Either::One(_t) = vr[0] { }
2019-12-24T18:24:16.8619086Z    |                        --    ^^^^^ help: consider borrowing here: `&vr[0]`
2019-12-24T18:24:16.8619187Z    |                        data moved here
2019-12-24T18:24:16.8619187Z    |                        data moved here
2019-12-24T18:24:16.8619240Z    |                        move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8619281Z 
2019-12-24T18:24:16.8619344Z error[E0507]: cannot move out of index of `std::vec::Vec<Either>`
2019-12-24T18:24:16.8619745Z    |
2019-12-24T18:24:16.8619745Z    |
2019-12-24T18:24:16.8619806Z LL |     while let Either::One(_t) = vr[0] { }
2019-12-24T18:24:16.8620067Z    |                           --    ^^^^^ help: consider borrowing here: `&vr[0]`
2019-12-24T18:24:16.8620177Z    |                           data moved here
2019-12-24T18:24:16.8620177Z    |                           data moved here
2019-12-24T18:24:16.8620229Z    |                           move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8620270Z 
2019-12-24T18:24:16.8620442Z error[E0507]: cannot move out of index of `std::vec::Vec<Either>`
2019-12-24T18:24:16.8620718Z    |
2019-12-24T18:24:16.8620718Z    |
2019-12-24T18:24:16.8620756Z LL |     match vr[0] {
2019-12-24T18:24:16.8620818Z    |           ^^^^^ help: consider borrowing here: `&vr[0]`
2019-12-24T18:24:16.8620859Z ...
2019-12-24T18:24:16.8620900Z LL |         | Either::Two(_t) => (),
2019-12-24T18:24:16.8621233Z    |                       |
2019-12-24T18:24:16.8621275Z    |                       data moved here
2019-12-24T18:24:16.8621275Z    |                       data moved here
2019-12-24T18:24:16.8621343Z    |                       move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8621376Z 
2019-12-24T18:24:16.8621419Z error[E0507]: cannot move out of index of `std::vec::Vec<Either>`
2019-12-24T18:24:16.8621721Z    |
2019-12-24T18:24:16.8621721Z    |
2019-12-24T18:24:16.8621760Z LL |     match vr[0] {
2019-12-24T18:24:16.8621804Z    |           ^^^^^ help: consider borrowing here: `&vr[0]`
2019-12-24T18:24:16.8621900Z ...
2019-12-24T18:24:16.8621941Z LL |         Either::One(_t) => (),
2019-12-24T18:24:16.8622185Z    |                     |
2019-12-24T18:24:16.8622228Z    |                     data moved here
2019-12-24T18:24:16.8622228Z    |                     data moved here
2019-12-24T18:24:16.8622278Z    |                     move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8622376Z error[E0507]: cannot move out of index of `std::vec::Vec<X>`
2019-12-24T18:24:16.8622608Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:130:17
2019-12-24T18:24:16.8622651Z    |
2019-12-24T18:24:16.8622651Z    |
2019-12-24T18:24:16.8622708Z LL |     let X(_t) = vsm[0];
2019-12-24T18:24:16.8622937Z    |           --    ^^^^^^ help: consider borrowing here: `&vsm[0]`
2019-12-24T18:24:16.8623042Z    |           data moved here
2019-12-24T18:24:16.8623042Z    |           data moved here
2019-12-24T18:24:16.8623091Z    |           move occurs because `_t` has type `Y`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8623123Z 
2019-12-24T18:24:16.8623165Z error[E0507]: cannot move out of index of `std::vec::Vec<Either>`
2019-12-24T18:24:16.8623460Z    |
2019-12-24T18:24:16.8623460Z    |
2019-12-24T18:24:16.8623509Z LL |     if let Either::One(_t) = vrm[0] { }
2019-12-24T18:24:16.8623765Z    |                        --    ^^^^^^ help: consider borrowing here: `&vrm[0]`
2019-12-24T18:24:16.8623856Z    |                        data moved here
2019-12-24T18:24:16.8623856Z    |                        data moved here
2019-12-24T18:24:16.8623925Z    |                        move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8623957Z 
2019-12-24T18:24:16.8624064Z error[E0507]: cannot move out of index of `std::vec::Vec<Either>`
2019-12-24T18:24:16.8624383Z    |
2019-12-24T18:24:16.8624383Z    |
2019-12-24T18:24:16.8624423Z LL |     while let Either::One(_t) = vrm[0] { }
2019-12-24T18:24:16.8624662Z    |                           --    ^^^^^^ help: consider borrowing here: `&vrm[0]`
2019-12-24T18:24:16.8624768Z    |                           data moved here
2019-12-24T18:24:16.8624768Z    |                           data moved here
2019-12-24T18:24:16.8641352Z    |                           move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8641409Z 
2019-12-24T18:24:16.8641459Z error[E0507]: cannot move out of index of `std::vec::Vec<Either>`
2019-12-24T18:24:16.8641917Z    |
2019-12-24T18:24:16.8641917Z    |
2019-12-24T18:24:16.8641956Z LL |     match vrm[0] {
2019-12-24T18:24:16.8642015Z    |           ^^^^^^ help: consider borrowing here: `&vrm[0]`
2019-12-24T18:24:16.8642073Z ...
2019-12-24T18:24:16.8642114Z LL |         | Either::Two(_t) => (),
2019-12-24T18:24:16.8642511Z    |                       |
2019-12-24T18:24:16.8642553Z    |                       data moved here
2019-12-24T18:24:16.8642553Z    |                       data moved here
2019-12-24T18:24:16.8642603Z    |                       move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8642649Z 
2019-12-24T18:24:16.8642701Z error[E0507]: cannot move out of index of `std::vec::Vec<Either>`
2019-12-24T18:24:16.8643024Z    |
2019-12-24T18:24:16.8643024Z    |
2019-12-24T18:24:16.8643081Z LL |     match vrm[0] {
2019-12-24T18:24:16.8643124Z    |           ^^^^^^ help: consider borrowing here: `&vrm[0]`
2019-12-24T18:24:16.8643163Z ...
2019-12-24T18:24:16.8643222Z LL |         Either::One(_t) => (),
2019-12-24T18:24:16.8643455Z    |                     |
2019-12-24T18:24:16.8643514Z    |                     data moved here
2019-12-24T18:24:16.8643514Z    |                     data moved here
2019-12-24T18:24:16.8643562Z    |                     move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8643593Z 
2019-12-24T18:24:16.8643633Z error[E0507]: cannot move out of index of `std::vec::Vec<Either>`
2019-12-24T18:24:16.8643928Z    |
2019-12-24T18:24:16.8643928Z    |
2019-12-24T18:24:16.8643964Z LL |     match vrm[0] {
2019-12-24T18:24:16.8644024Z    |           ^^^^^^ help: consider borrowing here: `&vrm[0]`
2019-12-24T18:24:16.8644063Z ...
2019-12-24T18:24:16.8644101Z LL |         Either::One(_t) => (),
2019-12-24T18:24:16.8644340Z    |                     |
2019-12-24T18:24:16.8644380Z    |                     data moved here
2019-12-24T18:24:16.8644380Z    |                     data moved here
2019-12-24T18:24:16.8644434Z    |                     move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8644525Z error[E0507]: cannot move out of `s.0` which is behind a shared reference
2019-12-24T18:24:16.8644752Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:168:18
2019-12-24T18:24:16.8644990Z    |
2019-12-24T18:24:16.8644990Z    |
2019-12-24T18:24:16.8645032Z LL |     let &X(_t) = s;
2019-12-24T18:24:16.8645224Z    |         ------   ^
2019-12-24T18:24:16.8645340Z    |         |  data moved here
2019-12-24T18:24:16.8645340Z    |         |  data moved here
2019-12-24T18:24:16.8645394Z    |         |  move occurs because `_t` has type `Y`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8645448Z    |         help: consider removing the `&`: `X(_t)`
2019-12-24T18:24:16.8645547Z error[E0507]: cannot move out of `r.0` which is behind a shared reference
2019-12-24T18:24:16.8645803Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:172:31
2019-12-24T18:24:16.8646006Z    |
2019-12-24T18:24:16.8646006Z    |
2019-12-24T18:24:16.8646062Z LL |     if let &Either::One(_t) = r { }
2019-12-24T18:24:16.8646305Z    |            ----------------   ^
2019-12-24T18:24:16.8646418Z    |            |            data moved here
2019-12-24T18:24:16.8646418Z    |            |            data moved here
2019-12-24T18:24:16.8646474Z    |            |            move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8646530Z    |            help: consider removing the `&`: `Either::One(_t)`
2019-12-24T18:24:16.8646707Z error[E0507]: cannot move out of `r.0` which is behind a shared reference
2019-12-24T18:24:16.8646980Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:176:34
2019-12-24T18:24:16.8647047Z    |
2019-12-24T18:24:16.8647047Z    |
2019-12-24T18:24:16.8647091Z LL |     while let &Either::One(_t) = r { }
2019-12-24T18:24:16.8647308Z    |               ----------------   ^
2019-12-24T18:24:16.8647428Z    |               |            data moved here
2019-12-24T18:24:16.8647428Z    |               |            data moved here
2019-12-24T18:24:16.8647484Z    |               |            move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8647557Z    |               help: consider removing the `&`: `Either::One(_t)`
2019-12-24T18:24:16.8647635Z error[E0507]: cannot move out of `r.0` which is behind a shared reference
2019-12-24T18:24:16.8647888Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:180:11
2019-12-24T18:24:16.8648120Z    |
2019-12-24T18:24:16.8648161Z LL |     match r {
2019-12-24T18:24:16.8648161Z LL |     match r {
2019-12-24T18:24:16.8648365Z    |           ^
2019-12-24T18:24:16.8648420Z ...
2019-12-24T18:24:16.8648459Z LL |         | &Either::Two(_t) => (),
2019-12-24T18:24:16.8648685Z    |                        |
2019-12-24T18:24:16.8648749Z    |                        data moved here
2019-12-24T18:24:16.8648749Z    |                        data moved here
2019-12-24T18:24:16.8648805Z    |                        move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8648904Z help: consider removing the `&`
2019-12-24T18:24:16.8648941Z    |
2019-12-24T18:24:16.8648941Z    |
2019-12-24T18:24:16.8648978Z LL |         Either::One(_t)
2019-12-24T18:24:16.8649037Z LL |         //~^ HELP consider removing the `&`
2019-12-24T18:24:16.8649080Z LL |         //~| SUGGESTION Either::One(_t)
2019-12-24T18:24:16.8649122Z LL |         | &Either::Two(_t) => (),
2019-12-24T18:24:16.8649206Z 
2019-12-24T18:24:16.8649248Z error[E0507]: cannot move out of `r.0` which is behind a shared reference
2019-12-24T18:24:16.8649479Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:188:11
2019-12-24T18:24:16.8649540Z    |
2019-12-24T18:24:16.8649540Z    |
2019-12-24T18:24:16.8649577Z LL |     match r {
2019-12-24T18:24:16.8649614Z    |           ^
2019-12-24T18:24:16.8649672Z LL |         //~^ ERROR cannot move
2019-12-24T18:24:16.8649714Z LL |         &Either::One(_t) => (),
2019-12-24T18:24:16.8649945Z    |         |            |
2019-12-24T18:24:16.8650005Z    |         |            data moved here
2019-12-24T18:24:16.8650005Z    |         |            data moved here
2019-12-24T18:24:16.8650054Z    |         |            move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8650103Z    |         help: consider removing the `&`: `Either::One(_t)`
2019-12-24T18:24:16.8650191Z error[E0507]: cannot move out of `r.0` which is behind a shared reference
2019-12-24T18:24:16.8650425Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:195:11
2019-12-24T18:24:16.8650486Z    |
2019-12-24T18:24:16.8650523Z LL |     match r {
2019-12-24T18:24:16.8650523Z LL |     match r {
2019-12-24T18:24:16.8650561Z    |           ^
2019-12-24T18:24:16.8650601Z LL |         //~^ ERROR cannot move
2019-12-24T18:24:16.8650660Z LL |         &Either::One(_t) => (),
2019-12-24T18:24:16.8650883Z    |         |            |
2019-12-24T18:24:16.8651009Z    |         |            data moved here
2019-12-24T18:24:16.8651009Z    |         |            data moved here
2019-12-24T18:24:16.8651064Z    |         |            move occurs because `_t` has type `X`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8651111Z    |         help: consider removing the `&`: `Either::One(_t)`
2019-12-24T18:24:16.8651158Z 
2019-12-24T18:24:16.8651201Z error[E0507]: cannot move out of `sm.0` which is behind a mutable reference
2019-12-24T18:24:16.8651598Z    |
2019-12-24T18:24:16.8651598Z    |
2019-12-24T18:24:16.8651659Z LL |     let &mut X(_t) = sm;
2019-12-24T18:24:16.8651876Z    |         ----------   ^^
2019-12-24T18:24:16.8651981Z    |         |      data moved here
2019-12-24T18:24:16.8651981Z    |         |      data moved here
2019-12-24T18:24:16.8652031Z    |         |      move occurs because `_t` has type `Y`, which does not implement the `Copy` trait
2019-12-24T18:24:16.8652081Z    |         help: consider removing the `&mut`: `X(_t)`
2019-12-24T18:24:16.8652183Z error[E0507]: cannot move out of `rm.0` which is behind a mutable reference
2019-12-24T18:24:16.8652433Z   --> /checkout/src/test/ui/suggestions/dont-suggest-ref/simple.rs:211:35
2019-12-24T18:24:16.8652477Z    |
---
2019-12-24T18:24:17.8442828Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-24T18:24:17.8443310Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-24T18:24:17.8443504Z 
2019-12-24T18:24:17.8443590Z 
2019-12-24T18:24:17.8445578Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-24T18:24:17.8445935Z 
2019-12-24T18:24:17.8445995Z 
2019-12-24T18:24:18.5057661Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-24T18:24:18.5057778Z Build completed unsuccessfully in 1:02:17
2019-12-24T18:24:18.5057778Z Build completed unsuccessfully in 1:02:17
2019-12-24T18:24:18.5057877Z == clock drift check ==
2019-12-24T18:24:18.5057923Z   local time: Tue Dec 24 18:24:16 UTC 2019
2019-12-24T18:24:18.5057974Z   network time: Tue, 24 Dec 2019 18:24:17 GMT
2019-12-24T18:24:18.5058037Z == end clock drift check ==
2019-12-24T18:24:18.5058068Z 
2019-12-24T18:24:18.5157097Z ##[error]Bash exited with code '1'.
2019-12-24T18:24:18.5198396Z ##[section]Starting: Checkout
2019-12-24T18:24:18.5200152Z ==============================================================================
2019-12-24T18:24:18.5200229Z Task         : Get sources
2019-12-24T18:24:18.5200277Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
