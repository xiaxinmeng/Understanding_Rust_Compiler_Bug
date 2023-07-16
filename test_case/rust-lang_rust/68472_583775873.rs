plain
2020-02-08T19:43:26.0253999Z ========================== Starting Command Output ===========================
2020-02-08T19:43:26.0255705Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5be6cea0-8034-4c89-ac6b-edd8a37b0429.sh
2020-02-08T19:43:26.0255785Z 
2020-02-08T19:43:26.0257974Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-08T19:43:26.0263980Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68472/merge to s
2020-02-08T19:43:26.0265539Z Task         : Get sources
2020-02-08T19:43:26.0265621Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-08T19:43:26.0265653Z Version      : 1.0.0
2020-02-08T19:43:26.0265683Z Author       : Microsoft
---
2020-02-08T19:43:27.0072439Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-08T19:43:27.0083888Z ##[command]git config gc.auto 0
2020-02-08T19:43:27.0086670Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-08T19:43:27.0088339Z ##[command]git config --get-all http.proxy
2020-02-08T19:43:27.0095598Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68472/merge:refs/remotes/pull/68472/merge
---
2020-02-08T20:41:31.0395897Z .................................................................................................... 1700/9611
2020-02-08T20:41:36.0948272Z .................................................................................................... 1800/9611
2020-02-08T20:41:48.0661238Z ...............................i.................................................................... 1900/9611
2020-02-08T20:41:55.7481496Z .................................................................................................... 2000/9611
2020-02-08T20:42:09.8335909Z .....................iiiii.......................................................................... 2100/9611
2020-02-08T20:42:19.6318784Z .................................................................................................... 2300/9611
2020-02-08T20:42:22.0907782Z .................................................................................................... 2400/9611
2020-02-08T20:42:26.8443711Z .................................................................................................... 2500/9611
2020-02-08T20:42:47.5839769Z .................................................................................................... 2600/9611
---
2020-02-08T20:45:25.3021968Z .........................................................................i...............i.......... 4900/9611
2020-02-08T20:45:32.9192163Z .................................................................................................... 5000/9611
2020-02-08T20:45:40.9925017Z .................................................................................................... 5100/9611
2020-02-08T20:45:45.5916119Z ................i................................................................................... 5200/9611
2020-02-08T20:45:56.3737484Z ..........................................................................................ii.ii..... 5300/9611
2020-02-08T20:46:00.7245267Z ...i...i............................................................................................ 5400/9611
2020-02-08T20:46:10.7055873Z .................................................................................................... 5600/9611
2020-02-08T20:46:21.2274291Z ..............................................................................i..................... 5700/9611
2020-02-08T20:46:28.9046092Z .................................................................................................... 5800/9611
2020-02-08T20:46:35.1642550Z .........................................F.......................................................... 5900/9611
2020-02-08T20:46:35.1642550Z .........................................F.......................................................... 5900/9611
2020-02-08T20:46:45.2697924Z ............................................F........................ii...i..ii...........i......... 6000/9611
2020-02-08T20:47:06.7373497Z .................................................................................................... 6200/9611
2020-02-08T20:47:14.4566742Z .................................................................................................... 6300/9611
2020-02-08T20:47:22.6721962Z .................................................................................................i.. 6400/9611
2020-02-08T20:47:36.0895755Z ii.................................................................................................. 6500/9611
---
2020-02-08T20:49:39.9664027Z .................................................................................................... 7600/9611
2020-02-08T20:49:45.0695674Z .................................................................................................... 7700/9611
2020-02-08T20:49:50.3853844Z .................................................................................................... 7800/9611
2020-02-08T20:50:00.0269108Z .................................................................................................... 7900/9611
2020-02-08T20:50:08.9849482Z .......................................................iiiiiii.i.................................... 8000/9611
2020-02-08T20:50:24.0940753Z ..i................................................................................................. 8200/9611
2020-02-08T20:50:29.7414045Z .................................................................................................... 8300/9611
2020-02-08T20:50:45.2751222Z .................................................................................................... 8400/9611
2020-02-08T20:50:53.8213436Z .................................................................................................... 8500/9611
---
2020-02-08T20:52:52.8623331Z 
2020-02-08T20:52:52.8624042Z ---- [ui] ui/issues/issue-31173.rs stdout ----
2020-02-08T20:52:52.8624342Z diff of stderr:
2020-02-08T20:52:52.8624579Z 
2020-02-08T20:52:52.8625186Z 14    |          ^^^^^^^ method not found in `std::iter::Cloned<std::iter::TakeWhile<&mut std::vec::IntoIter<u8>, [closure@$DIR/issue-31173.rs:6:39: 9:6 found_e:_]>>`
2020-02-08T20:52:52.8625744Z 16    = note: the method `collect` exists but the following trait bounds were not satisfied:
2020-02-08T20:52:52.8625744Z 16    = note: the method `collect` exists but the following trait bounds were not satisfied:
2020-02-08T20:52:52.8626302Z -            `&mut std::iter::Cloned<std::iter::TakeWhile<&mut std::vec::IntoIter<u8>, [closure@$DIR/issue-31173.rs:6:39: 9:6 found_e:_]>> : std::iter::Iterator`
2020-02-08T20:52:52.8626930Z 18            `std::iter::Cloned<std::iter::TakeWhile<&mut std::vec::IntoIter<u8>, [closure@$DIR/issue-31173.rs:6:39: 9:6 found_e:_]>> : std::iter::Iterator`
2020-02-08T20:52:52.8627460Z 20 error: aborting due to 2 previous errors
2020-02-08T20:52:52.8627652Z 
2020-02-08T20:52:52.8627840Z 
2020-02-08T20:52:52.8628070Z The actual stderr differed from the expected stderr.
2020-02-08T20:52:52.8628070Z The actual stderr differed from the expected stderr.
2020-02-08T20:52:52.8628654Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31173/issue-31173.stderr
2020-02-08T20:52:52.8629226Z To update references, rerun the tests and pass the `--bless` flag
2020-02-08T20:52:52.8629796Z To only update this specific test, also pass `--test-args issues/issue-31173.rs`
2020-02-08T20:52:52.8630283Z error: 1 errors occurred comparing output.
2020-02-08T20:52:52.8630499Z status: exit code: 1
2020-02-08T20:52:52.8630499Z status: exit code: 1
2020-02-08T20:52:52.8631668Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-31173.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31173" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31173/auxiliary" "-A" "unused"
2020-02-08T20:52:52.8632783Z ------------------------------------------
2020-02-08T20:52:52.8633053Z 
2020-02-08T20:52:52.8633518Z ------------------------------------------
2020-02-08T20:52:52.8633779Z stderr:
2020-02-08T20:52:52.8633779Z stderr:
2020-02-08T20:52:52.8634185Z ------------------------------------------
2020-02-08T20:52:52.8634813Z error[E0271]: type mismatch resolving `<std::iter::TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6 found_e:_]> as std::iter::Iterator>::Item == &_`
2020-02-08T20:52:52.8636269Z    |
2020-02-08T20:52:52.8636637Z LL |         .cloned()
2020-02-08T20:52:52.8636786Z    |          ^^^^^^ expected `u8`, found reference
2020-02-08T20:52:52.8637098Z    |
2020-02-08T20:52:52.8637098Z    |
2020-02-08T20:52:52.8637263Z    = note:   expected type `u8`
2020-02-08T20:52:52.8637403Z            found reference `&_`
2020-02-08T20:52:52.8637523Z 
2020-02-08T20:52:52.8638676Z error[E0599]: no method named `collect` found for struct `std::iter::Cloned<std::iter::TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6 found_e:_]>>` in the current scope
2020-02-08T20:52:52.8639278Z    |
2020-02-08T20:52:52.8639418Z LL |         .collect(); //~ ERROR no method named `collect`
2020-02-08T20:52:52.8639418Z LL |         .collect(); //~ ERROR no method named `collect`
2020-02-08T20:52:52.8641620Z    |          ^^^^^^^ method not found in `std::iter::Cloned<std::iter::TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6 found_e:_]>>`
2020-02-08T20:52:52.8642162Z    = note: the method `collect` exists but the following trait bounds were not satisfied:
2020-02-08T20:52:52.8642162Z    = note: the method `collect` exists but the following trait bounds were not satisfied:
2020-02-08T20:52:52.8642662Z            `std::iter::Cloned<std::iter::TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6 found_e:_]>> : std::iter::Iterator`
2020-02-08T20:52:52.8642978Z error: aborting due to 2 previous errors
2020-02-08T20:52:52.8643103Z 
2020-02-08T20:52:52.8643262Z Some errors have detailed explanations: E0271, E0599.
2020-02-08T20:52:52.8643616Z For more information about an error, try `rustc --explain E0271`.
---
2020-02-08T20:52:52.8652355Z 36    |
2020-02-08T20:52:52.8652741Z -    = note: the method `take` exists but the following trait bounds were not satisfied:
2020-02-08T20:52:52.8653088Z -            `&mut Foo : std::iter::Iterator`
2020-02-08T20:52:52.8653257Z 39    = help: items from traits can only be used if the trait is implemented and in scope
2020-02-08T20:52:52.8653453Z 40    = note: the following traits define an item `take`, perhaps you need to implement one of them:
2020-02-08T20:52:52.8653599Z 41            candidate #1: `std::io::Read`
2020-02-08T20:52:52.8653846Z 
2020-02-08T20:52:52.8653979Z The actual stderr differed from the expected stderr.
2020-02-08T20:52:52.8653979Z The actual stderr differed from the expected stderr.
2020-02-08T20:52:52.8654390Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-call-err-msg/method-call-err-msg.stderr
2020-02-08T20:52:52.8654773Z To update references, rerun the tests and pass the `--bless` flag
2020-02-08T20:52:52.8655277Z To only update this specific test, also pass `--test-args methods/method-call-err-msg.rs`
2020-02-08T20:52:52.8655586Z error: 1 errors occurred comparing output.
2020-02-08T20:52:52.8655720Z status: exit code: 1
2020-02-08T20:52:52.8655720Z status: exit code: 1
2020-02-08T20:52:52.8656750Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-call-err-msg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-call-err-msg" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-call-err-msg/auxiliary" "-A" "unused"
2020-02-08T20:52:52.8657445Z ------------------------------------------
2020-02-08T20:52:52.8657594Z 
2020-02-08T20:52:52.8657909Z ------------------------------------------
2020-02-08T20:52:52.8658094Z stderr:
2020-02-08T20:52:52.8658094Z stderr:
2020-02-08T20:52:52.8658403Z ------------------------------------------
2020-02-08T20:52:52.8658573Z error[E0061]: this function takes 0 parameters but 1 parameter was supplied
2020-02-08T20:52:52.8658930Z   --> /checkout/src/test/ui/methods/method-call-err-msg.rs:12:7
2020-02-08T20:52:52.8659116Z    |
2020-02-08T20:52:52.8659423Z LL |     fn zero(self) -> Foo { self }
2020-02-08T20:52:52.8659777Z    |     -------------------- defined here
2020-02-08T20:52:52.8659933Z ...
2020-02-08T20:52:52.8660089Z LL |     x.zero(0)   //~ ERROR this function takes 0 parameters but 1 parameter was supplied
2020-02-08T20:52:52.8660343Z 
2020-02-08T20:52:52.8660494Z error[E0061]: this function takes 1 parameter but 0 parameters were supplied
2020-02-08T20:52:52.8660833Z   --> /checkout/src/test/ui/methods/method-call-err-msg.rs:13:7
2020-02-08T20:52:52.8660991Z    |
2020-02-08T20:52:52.8660991Z    |
2020-02-08T20:52:52.8661317Z LL |     fn one(self, _: isize) -> Foo { self }
2020-02-08T20:52:52.8661836Z ...
2020-02-08T20:52:52.8661836Z ...
2020-02-08T20:52:52.8661994Z LL |      .one()     //~ ERROR this function takes 1 parameter but 0 parameters were supplied
2020-02-08T20:52:52.8662268Z 
2020-02-08T20:52:52.8662424Z error[E0061]: this function takes 2 parameters but 1 parameter was supplied
2020-02-08T20:52:52.8662765Z   --> /checkout/src/test/ui/methods/method-call-err-msg.rs:14:7
2020-02-08T20:52:52.8662926Z    |
2020-02-08T20:52:52.8662926Z    |
2020-02-08T20:52:52.8663258Z LL |     fn two(self, _: isize, _: isize) -> Foo { self }
2020-02-08T20:52:52.8663762Z ...
2020-02-08T20:52:52.8663762Z ...
2020-02-08T20:52:52.8663920Z LL |      .two(0);   //~ ERROR this function takes 2 parameters but 1 parameter was supplied
2020-02-08T20:52:52.8664170Z 
2020-02-08T20:52:52.8664321Z error[E0599]: no method named `take` found for struct `Foo` in the current scope
2020-02-08T20:52:52.8664657Z   --> /checkout/src/test/ui/methods/method-call-err-msg.rs:18:7
2020-02-08T20:52:52.8664831Z    |
2020-02-08T20:52:52.8664831Z    |
2020-02-08T20:52:52.8664962Z LL | pub struct Foo;
2020-02-08T20:52:52.8665304Z    | --------------- method `take` not found for this
2020-02-08T20:52:52.8665482Z ...
2020-02-08T20:52:52.8665618Z LL |      .take()    //~ ERROR no method named `take` found
2020-02-08T20:52:52.8665896Z    |
2020-02-08T20:52:52.8666076Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-02-08T20:52:52.8666076Z    = help: items from traits can only be used if the trait is implemented and in scope
2020-02-08T20:52:52.8666234Z    = note: the following traits define an item `take`, perhaps you need to implement one of them:
2020-02-08T20:52:52.8666378Z            candidate #1: `std::io::Read`
2020-02-08T20:52:52.8666538Z            candidate #2: `std::iter::Iterator`
2020-02-08T20:52:52.8666822Z error: aborting due to 4 previous errors
2020-02-08T20:52:52.8666935Z 
2020-02-08T20:52:52.8667071Z Some errors have detailed explanations: E0061, E0599.
2020-02-08T20:52:52.8667448Z For more information about an error, try `rustc --explain E0061`.
2020-02-08T20:52:52.8667448Z For more information about an error, try `rustc --explain E0061`.
2020-02-08T20:52:52.8667721Z 
2020-02-08T20:52:52.8668175Z ------------------------------------------
2020-02-08T20:52:52.8668346Z 
2020-02-08T20:52:52.8668484Z 
2020-02-08T20:52:52.8668843Z ---- [ui] ui/mismatched_types/issue-36053-2.rs stdout ----
2020-02-08T20:52:52.8669010Z diff of stderr:
2020-02-08T20:52:52.8669127Z 
2020-02-08T20:52:52.8669612Z 5    |                                                       ^^^^^ method not found in `std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@$DIR/issue-36053-2.rs:7:39: 7:53]>`
2020-02-08T20:52:52.8669949Z 7    = note: the method `count` exists but the following trait bounds were not satisfied:
2020-02-08T20:52:52.8669949Z 7    = note: the method `count` exists but the following trait bounds were not satisfied:
2020-02-08T20:52:52.8670370Z -            `&mut std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@$DIR/issue-36053-2.rs:7:39: 7:53]> : std::iter::Iterator`
2020-02-08T20:52:52.8671067Z 9            `std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@$DIR/issue-36053-2.rs:7:39: 7:53]> : std::iter::Iterator`
2020-02-08T20:52:52.8671410Z 11 error[E0631]: type mismatch in closure arguments
2020-02-08T20:52:52.8671530Z 
2020-02-08T20:52:52.8672019Z 
2020-02-08T20:52:52.8672337Z The actual stderr differed from the expected stderr.
2020-02-08T20:52:52.8672337Z The actual stderr differed from the expected stderr.
2020-02-08T20:52:52.8673076Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2/issue-36053-2.stderr
2020-02-08T20:52:52.8673496Z To update references, rerun the tests and pass the `--bless` flag
2020-02-08T20:52:52.8674127Z To only update this specific test, also pass `--test-args mismatched_types/issue-36053-2.rs`
2020-02-08T20:52:52.8674421Z error: 1 errors occurred comparing output.
2020-02-08T20:52:52.8674550Z status: exit code: 1
2020-02-08T20:52:52.8674550Z status: exit code: 1
2020-02-08T20:52:52.8675507Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/issue-36053-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2/auxiliary" "-A" "unused"
2020-02-08T20:52:52.8676065Z ------------------------------------------
2020-02-08T20:52:52.8676204Z 
2020-02-08T20:52:52.8676504Z ------------------------------------------
2020-02-08T20:52:52.8676682Z stderr:
2020-02-08T20:52:52.8676682Z stderr:
2020-02-08T20:52:52.8677006Z ------------------------------------------
2020-02-08T20:52:52.8677499Z error[E0599]: no method named `count` found for struct `std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@/checkout/src/test/ui/mismatched_types/issue-36053-2.rs:7:39: 7:53]>` in the current scope
2020-02-08T20:52:52.8679454Z    |
2020-02-08T20:52:52.8679454Z    |
2020-02-08T20:52:52.8681067Z LL |     once::<&str>("str").fuse().filter(|a: &str| true).count();
2020-02-08T20:52:52.8681609Z    |                                                       ^^^^^ method not found in `std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@/checkout/src/test/ui/mismatched_types/issue-36053-2.rs:7:39: 7:53]>`
2020-02-08T20:52:52.8681750Z    = note: the method `count` exists but the following trait bounds were not satisfied:
2020-02-08T20:52:52.8681750Z    = note: the method `count` exists but the following trait bounds were not satisfied:
2020-02-08T20:52:52.8682122Z            `std::iter::Filter<std::iter::Fuse<std::iter::Once<&str>>, [closure@/checkout/src/test/ui/mismatched_types/issue-36053-2.rs:7:39: 7:53]> : std::iter::Iterator`
2020-02-08T20:52:52.8682393Z error[E0631]: type mismatch in closure arguments
2020-02-08T20:52:52.8682812Z   --> /checkout/src/test/ui/mismatched_types/issue-36053-2.rs:7:32
2020-02-08T20:52:52.8682999Z    |
2020-02-08T20:52:52.8682999Z    |
2020-02-08T20:52:52.8683429Z LL |     once::<&str>("str").fuse().filter(|a: &str| true).count();
2020-02-08T20:52:52.8683897Z    |                                ^^^^^^ -------------- found signature of `for<'r> fn(&'r str) -> _`
2020-02-08T20:52:52.8683945Z    |                                |
2020-02-08T20:52:52.8684522Z    |                                expected signature of `for<'r> fn(&'r &str) -> _`
2020-02-08T20:52:52.8684594Z error: aborting due to 2 previous errors
2020-02-08T20:52:52.8684620Z 
2020-02-08T20:52:52.8684677Z Some errors have detailed explanations: E0599, E0631.
2020-02-08T20:52:52.8685075Z For more information about an error, try `rustc --explain E0599`.
---
2020-02-08T20:52:52.8687548Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-08T20:52:52.8687616Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-08T20:52:52.8687646Z 
2020-02-08T20:52:52.8687669Z 
2020-02-08T20:52:52.8689482Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-08T20:52:52.8689684Z 
2020-02-08T20:52:52.8689720Z 
2020-02-08T20:52:52.8689789Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-08T20:52:52.8689838Z Build completed unsuccessfully in 1:03:16
2020-02-08T20:52:52.8689838Z Build completed unsuccessfully in 1:03:16
2020-02-08T20:52:52.8705932Z == clock drift check ==
2020-02-08T20:52:52.8732151Z   local time: Sat Feb  8 20:52:52 UTC 2020
2020-02-08T20:52:53.1429399Z   network time: Sat, 08 Feb 2020 20:52:53 GMT
2020-02-08T20:52:53.1434848Z == end clock drift check ==
2020-02-08T20:52:53.6034823Z 
2020-02-08T20:52:53.6136021Z ##[error]Bash exited with code '1'.
2020-02-08T20:52:53.6147456Z ##[section]Finishing: Run build
2020-02-08T20:52:53.6174371Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68472/merge to s
2020-02-08T20:52:53.6176133Z Task         : Get sources
2020-02-08T20:52:53.6176177Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-08T20:52:53.6176225Z Version      : 1.0.0
2020-02-08T20:52:53.6176417Z Author       : Microsoft
2020-02-08T20:52:53.6176417Z Author       : Microsoft
2020-02-08T20:52:53.6176461Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-08T20:52:53.6176508Z ==============================================================================
2020-02-08T20:52:54.0473711Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-08T20:52:54.0514484Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68472/merge to s
2020-02-08T20:52:54.0624064Z Cleaning up task key
2020-02-08T20:52:54.0624945Z Start cleaning up orphan processes.
2020-02-08T20:52:54.0748378Z Terminate orphan process: pid (12302) (python)
2020-02-08T20:52:54.0997928Z ##[section]Finishing: Finalize Job
