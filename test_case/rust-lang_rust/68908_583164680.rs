plain
2020-02-06T22:29:08.6749743Z ========================== Starting Command Output ===========================
2020-02-06T22:29:08.6753243Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/61e668a9-b1f2-444c-bef8-235e07cbaf3c.sh
2020-02-06T22:29:08.6753347Z 
2020-02-06T22:29:08.6756041Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-06T22:29:08.6762130Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68908/merge to s
2020-02-06T22:29:08.6763731Z Task         : Get sources
2020-02-06T22:29:08.6763804Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-06T22:29:08.6763836Z Version      : 1.0.0
2020-02-06T22:29:08.6763867Z Author       : Microsoft
---
2020-02-06T22:29:09.5810139Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-06T22:29:09.5891864Z ##[command]git config gc.auto 0
2020-02-06T22:29:09.5962683Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-06T22:29:09.6040770Z ##[command]git config --get-all http.proxy
2020-02-06T22:29:09.6180069Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68908/merge:refs/remotes/pull/68908/merge
---
2020-02-06T23:27:03.0698915Z .................................................................................................... 1700/9595
2020-02-06T23:27:07.8321579Z .................................................................................................... 1800/9595
2020-02-06T23:27:20.1747241Z .............................i...................................................................... 1900/9595
2020-02-06T23:27:27.1961870Z .................................................................................................... 2000/9595
2020-02-06T23:27:41.1888754Z ...................iiiii............................................................................ 2100/9595
2020-02-06T23:27:50.7933210Z .................................................................................................... 2300/9595
2020-02-06T23:27:53.1921428Z .................................................................................................... 2400/9595
2020-02-06T23:27:57.8417384Z .................F.................................................................................. 2500/9595
2020-02-06T23:28:18.3158867Z .................................................................................................... 2600/9595
---
2020-02-06T23:30:55.1414051Z ..............................................................i...............i..................... 4900/9595
2020-02-06T23:31:02.4817761Z .................................................................................................... 5000/9595
2020-02-06T23:31:10.7189504Z .................................................................................................... 5100/9595
2020-02-06T23:31:15.2129519Z .....i.............................................................................................. 5200/9595
2020-02-06T23:31:25.9500287Z ...............................................................................ii.ii........i...i... 5300/9595
2020-02-06T23:31:34.1173757Z .................i.................................................................................. 5500/9595
2020-02-06T23:31:43.1674990Z .................................................................................................... 5600/9595
2020-02-06T23:31:49.6740828Z ..................................................................i................................. 5700/9595
2020-02-06T23:31:56.6963867Z .................................................................................................... 5800/9595
2020-02-06T23:31:56.6963867Z .................................................................................................... 5800/9595
2020-02-06T23:32:03.7534565Z .................................................................................................... 5900/9595
2020-02-06T23:32:13.1257966Z .........................................................ii...i..ii...........i..................... 6000/9595
2020-02-06T23:32:34.3853975Z .................................................................................................... 6200/9595
2020-02-06T23:32:41.9765701Z .................................................................................................... 6300/9595
2020-02-06T23:32:41.9765701Z .................................................................................................... 6300/9595
2020-02-06T23:32:49.4714775Z .....................................................................................i..ii.......... 6400/9595
2020-02-06T23:33:12.8679256Z .................................................................................................... 6600/9595
2020-02-06T23:33:22.2090003Z ........................................................................i........................... 6700/9595
2020-02-06T23:33:24.5852089Z .................................................................................................... 6800/9595
2020-02-06T23:33:26.7444452Z ...............................................................................i.................... 6900/9595
---
2020-02-06T23:35:05.1302082Z .................................................................................................... 7600/9595
2020-02-06T23:35:09.8314824Z .................................................................................................... 7700/9595
2020-02-06T23:35:16.7324815Z .................................................................................................... 7800/9595
2020-02-06T23:35:24.6540206Z .................................................................................................... 7900/9595
2020-02-06T23:35:32.1828725Z ..........................................iiiiiii.i................................................. 8000/9595
2020-02-06T23:35:46.5194627Z .................................................................................................... 8200/9595
2020-02-06T23:35:52.5223795Z .................................................................................................... 8300/9595
2020-02-06T23:36:07.2637189Z .................................................................................................... 8400/9595
2020-02-06T23:36:14.3541717Z .................................................................................................... 8500/9595
2020-02-06T23:36:14.3541717Z .................................................................................................... 8500/9595
2020-02-06T23:36:22.0271940Z .................................................................................................... 8600/9595
2020-02-06T23:36:58.7848635Z .................................................................................................... 8700/9595
2020-02-06T23:37:07.2741734Z .................................................................................................... 8800/9595
2020-02-06T23:37:13.7100111Z .................................................................................................... 8900/9595
2020-02-06T23:37:19.9693060Z .................................................................................................... 9000/9595
2020-02-06T23:37:25.1431966Z .................................................................................................... 9100/9595
2020-02-06T23:37:32.2453134Z .................................................................................................... 9200/9595
2020-02-06T23:37:39.6733494Z ................F..F.FFFFFFFFFF..................................................................... 9300/9595
2020-02-06T23:37:54.6416033Z ...................................................................................................i 9500/9595
2020-02-06T23:38:09.1959759Z ...............................................................................................
2020-02-06T23:38:09.1960738Z failures:
2020-02-06T23:38:09.1995133Z 
---
2020-02-06T23:38:09.1997304Z 41 
2020-02-06T23:38:09.1997424Z 
2020-02-06T23:38:09.1997543Z 
2020-02-06T23:38:09.1997704Z The actual stderr differed from the expected stderr.
2020-02-06T23:38:09.1998188Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-param-elided-lifetime/const-param-elided-lifetime.stderr
2020-02-06T23:38:09.1998675Z To update references, rerun the tests and pass the `--bless` flag
2020-02-06T23:38:09.1999164Z To only update this specific test, also pass `--test-args const-generics/const-param-elided-lifetime.rs`
2020-02-06T23:38:09.1999508Z error: 1 errors occurred comparing output.
2020-02-06T23:38:09.1999649Z status: exit code: 1
2020-02-06T23:38:09.1999649Z status: exit code: 1
2020-02-06T23:38:09.2000648Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const-param-elided-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-param-elided-lifetime" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-param-elided-lifetime/auxiliary" "-A" "unused"
2020-02-06T23:38:09.2001318Z ------------------------------------------
2020-02-06T23:38:09.2001491Z 
2020-02-06T23:38:09.2001971Z ------------------------------------------
2020-02-06T23:38:09.2002291Z stderr:
2020-02-06T23:38:09.2002291Z stderr:
2020-02-06T23:38:09.2002647Z ------------------------------------------
2020-02-06T23:38:09.2002835Z error[E0637]: `&` without an explicit lifetime name cannot be used here
2020-02-06T23:38:09.2003262Z   --> /checkout/src/test/ui/const-generics/const-param-elided-lifetime.rs:9:19
2020-02-06T23:38:09.2003453Z    |
2020-02-06T23:38:09.2003591Z LL | struct A<const N: &u8>;
2020-02-06T23:38:09.2003876Z 
2020-02-06T23:38:09.2004016Z error[E0637]: `&` without an explicit lifetime name cannot be used here
2020-02-06T23:38:09.2004441Z   --> /checkout/src/test/ui/const-generics/const-param-elided-lifetime.rs:13:15
2020-02-06T23:38:09.2004623Z    |
2020-02-06T23:38:09.2004623Z    |
2020-02-06T23:38:09.2004793Z LL | impl<const N: &u8> A<N> { //~ ERROR `&` without an explicit lifetime name cannot be used here
2020-02-06T23:38:09.2005094Z 
2020-02-06T23:38:09.2005252Z error[E0637]: `&` without an explicit lifetime name cannot be used here
2020-02-06T23:38:09.2005686Z   --> /checkout/src/test/ui/const-generics/const-param-elided-lifetime.rs:14:21
2020-02-06T23:38:09.2005871Z    |
2020-02-06T23:38:09.2005871Z    |
2020-02-06T23:38:09.2006043Z LL |     fn foo<const M: &u8>(&self) {}
2020-02-06T23:38:09.2006307Z 
2020-02-06T23:38:09.2006467Z error[E0637]: `&` without an explicit lifetime name cannot be used here
2020-02-06T23:38:09.2006975Z   --> /checkout/src/test/ui/const-generics/const-param-elided-lifetime.rs:18:15
2020-02-06T23:38:09.2007149Z    |
2020-02-06T23:38:09.2007149Z    |
2020-02-06T23:38:09.2007301Z LL | impl<const N: &u8> B for A<N> {}
2020-02-06T23:38:09.2092652Z 
2020-02-06T23:38:09.2092922Z error[E0637]: `&` without an explicit lifetime name cannot be used here
2020-02-06T23:38:09.2093497Z   --> /checkout/src/test/ui/const-generics/const-param-elided-lifetime.rs:21:17
2020-02-06T23:38:09.2093551Z    |
2020-02-06T23:38:09.2093551Z    |
2020-02-06T23:38:09.2093595Z LL | fn bar<const N: &u8>() {}
2020-02-06T23:38:09.2093689Z 
2020-02-06T23:38:09.2093738Z warning: the feature `const_generics` is incomplete and may cause the compiler to crash
2020-02-06T23:38:09.2094021Z   --> /checkout/src/test/ui/const-generics/const-param-elided-lifetime.rs:6:12
2020-02-06T23:38:09.2094085Z    |
---
2020-02-06T23:38:09.2095756Z 21 
2020-02-06T23:38:09.2095782Z 
2020-02-06T23:38:09.2095807Z 
2020-02-06T23:38:09.2095868Z The actual stderr differed from the expected stderr.
2020-02-06T23:38:09.2096470Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0637/E0637.stderr
2020-02-06T23:38:09.2096799Z To update references, rerun the tests and pass the `--bless` flag
2020-02-06T23:38:09.2097126Z To only update this specific test, also pass `--test-args error-codes/E0637.rs`
2020-02-06T23:38:09.2097212Z error: 1 errors occurred comparing output.
2020-02-06T23:38:09.2097258Z status: exit code: 1
2020-02-06T23:38:09.2097258Z status: exit code: 1
2020-02-06T23:38:09.2098090Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0637.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0637" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0637/auxiliary" "-A" "unused"
2020-02-06T23:38:09.2098476Z ------------------------------------------
2020-02-06T23:38:09.2098514Z 
2020-02-06T23:38:09.2098763Z ------------------------------------------
2020-02-06T23:38:09.2098826Z stderr:
2020-02-06T23:38:09.2098826Z stderr:
2020-02-06T23:38:09.2099068Z ------------------------------------------
2020-02-06T23:38:09.2099308Z error[E0637]: `'_` cannot be used here
2020-02-06T23:38:09.2099579Z   --> /checkout/src/test/ui/error-codes/E0637.rs:1:16
2020-02-06T23:38:09.2099629Z    |
2020-02-06T23:38:09.2099888Z LL | struct Foo<'a: '_>(&'a u8); //~ ERROR cannot be used here
2020-02-06T23:38:09.2100164Z    |                ^^ `'_` is a reserved lifetime name
2020-02-06T23:38:09.2100439Z error[E0637]: `'_` cannot be used here
2020-02-06T23:38:09.2100698Z   --> /checkout/src/test/ui/error-codes/E0637.rs:2:12
2020-02-06T23:38:09.2100759Z    |
2020-02-06T23:38:09.2100759Z    |
2020-02-06T23:38:09.2101022Z LL | fn foo<'a: '_>(_: &'a u8) {} //~ ERROR cannot be used here
2020-02-06T23:38:09.2101276Z    |            ^^ `'_` is a reserved lifetime name
2020-02-06T23:38:09.2101825Z error[E0637]: `'_` cannot be used here
2020-02-06T23:38:09.2102349Z   --> /checkout/src/test/ui/error-codes/E0637.rs:5:10
2020-02-06T23:38:09.2102422Z    |
2020-02-06T23:38:09.2102422Z    |
2020-02-06T23:38:09.2102687Z LL | impl<'a: '_> Bar<'a> { //~ ERROR cannot be used here
2020-02-06T23:38:09.2102937Z    |          ^^ `'_` is a reserved lifetime name
2020-02-06T23:38:09.2103035Z error: aborting due to 3 previous errors
2020-02-06T23:38:09.2103066Z 
2020-02-06T23:38:09.2103334Z For more information about this error, try `rustc --explain E0637`.
2020-02-06T23:38:09.2103371Z 
---
2020-02-06T23:38:09.2104942Z 
2020-02-06T23:38:09.2104968Z 
2020-02-06T23:38:09.2105028Z The actual stderr differed from the expected stderr.
2020-02-06T23:38:09.2106706Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/issue-65285-incorrect-explicit-lifetime-name-needed/issue-65285-incorrect-explicit-lifetime-name-needed.stderr
2020-02-06T23:38:09.2107112Z To update references, rerun the tests and pass the `--bless` flag
2020-02-06T23:38:09.2107489Z To only update this specific test, also pass `--test-args generics/issue-65285-incorrect-explicit-lifetime-name-needed.rs`
2020-02-06T23:38:09.2107601Z error: 1 errors occurred comparing output.
2020-02-06T23:38:09.2107663Z status: exit code: 1
2020-02-06T23:38:09.2107663Z status: exit code: 1
2020-02-06T23:38:09.2113123Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generics/issue-65285-incorrect-explicit-lifetime-name-needed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/issue-65285-incorrect-explicit-lifetime-name-needed" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/issue-65285-incorrect-explicit-lifetime-name-needed/auxiliary" "-A" "unused"
2020-02-06T23:38:09.2113657Z ------------------------------------------
2020-02-06T23:38:09.2113715Z 
2020-02-06T23:38:09.2113987Z ------------------------------------------
2020-02-06T23:38:09.2114036Z stderr:
2020-02-06T23:38:09.2114036Z stderr:
2020-02-06T23:38:09.2114259Z ------------------------------------------
2020-02-06T23:38:09.2114328Z error[E0637]: `&` without an explicit lifetime name cannot be used here
2020-02-06T23:38:09.2114620Z   --> /checkout/src/test/ui/generics/issue-65285-incorrect-explicit-lifetime-name-needed.rs:5:37
2020-02-06T23:38:09.2114676Z    |
2020-02-06T23:38:09.2114737Z LL | fn should_error<T>() where T : Into<&u32> {}
2020-02-06T23:38:09.2114822Z 
2020-02-06T23:38:09.2114880Z error[E0106]: missing lifetime specifier
2020-02-06T23:38:09.2115166Z   --> /checkout/src/test/ui/generics/issue-65285-incorrect-explicit-lifetime-name-needed.rs:9:19
2020-02-06T23:38:09.2115217Z    |
2020-02-06T23:38:09.2115217Z    |
2020-02-06T23:38:09.2115454Z LL |     fn foo<'b, L: X<&'b Nested<K>>>();
2020-02-06T23:38:09.2115742Z 
2020-02-06T23:38:09.2115791Z error[E0106]: missing lifetime specifier
2020-02-06T23:38:09.2116135Z   --> /checkout/src/test/ui/generics/issue-65285-incorrect-explicit-lifetime-name-needed.rs:13:15
2020-02-06T23:38:09.2116187Z    |
2020-02-06T23:38:09.2116187Z    |
2020-02-06T23:38:09.2116411Z LL | fn bar<'b, L: X<&'b Nested<i32>>>(){}
2020-02-06T23:38:09.2116510Z 
2020-02-06T23:38:09.2116725Z error: aborting due to 3 previous errors
2020-02-06T23:38:09.2116757Z 
2020-02-06T23:38:09.2116817Z Some errors have detailed explanations: E0106, E0637.
---
2020-02-06T23:38:09.2118230Z 39 
2020-02-06T23:38:09.2118257Z 
2020-02-06T23:38:09.2118282Z 
2020-02-06T23:38:09.2118341Z The actual stderr differed from the expected stderr.
2020-02-06T23:38:09.2118646Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/in-binder/in-binder.stderr
2020-02-06T23:38:09.2118901Z To update references, rerun the tests and pass the `--bless` flag
2020-02-06T23:38:09.2119222Z To only update this specific test, also pass `--test-args underscore-lifetime/in-binder.rs`
2020-02-06T23:38:09.2119309Z error: 1 errors occurred comparing output.
2020-02-06T23:38:09.2119384Z status: exit code: 1
2020-02-06T23:38:09.2119384Z status: exit code: 1
2020-02-06T23:38:09.2120238Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-lifetime/in-binder.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/in-binder" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/in-binder/auxiliary" "-A" "unused"
2020-02-06T23:38:09.2120746Z ------------------------------------------
2020-02-06T23:38:09.2120786Z 
2020-02-06T23:38:09.2121055Z ------------------------------------------
2020-02-06T23:38:09.2121105Z stderr:
2020-02-06T23:38:09.2121105Z stderr:
2020-02-06T23:38:09.2121348Z ------------------------------------------
2020-02-06T23:38:09.2121609Z error[E0637]: `'_` cannot be used here
2020-02-06T23:38:09.2121903Z   --> /checkout/src/test/ui/underscore-lifetime/in-binder.rs:9:6
2020-02-06T23:38:09.2121956Z    |
2020-02-06T23:38:09.2122190Z LL | impl<'_> IceCube<'_> {}
2020-02-06T23:38:09.2122649Z    |      ^^ `'_` is a reserved lifetime name
2020-02-06T23:38:09.2122945Z error[E0637]: `'_` cannot be used here
2020-02-06T23:38:09.2123232Z   --> /checkout/src/test/ui/underscore-lifetime/in-binder.rs:12:15
2020-02-06T23:38:09.2123284Z    |
2020-02-06T23:38:09.2123505Z LL | struct Struct<'_> {
---
2020-02-06T23:38:09.2130805Z 
2020-02-06T23:38:09.2130832Z 
2020-02-06T23:38:09.2130892Z The actual stderr differed from the expected stderr.
2020-02-06T23:38:09.2131261Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/underscore-lifetime-binders/underscore-lifetime-binders.stderr
2020-02-06T23:38:09.2131843Z To update references, rerun the tests and pass the `--bless` flag
2020-02-06T23:38:09.2132197Z To only update this specific test, also pass `--test-args underscore-lifetime/underscore-lifetime-binders.rs`
2020-02-06T23:38:09.2132324Z error: 1 errors occurred comparing output.
2020-02-06T23:38:09.2132387Z status: exit code: 1
2020-02-06T23:38:09.2132387Z status: exit code: 1
2020-02-06T23:38:09.2133444Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-lifetime/underscore-lifetime-binders.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/underscore-lifetime-binders" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/underscore-lifetime-binders/auxiliary" "-A" "unused"
2020-02-06T23:38:09.2133833Z ------------------------------------------
2020-02-06T23:38:09.2133871Z 
2020-02-06T23:38:09.2134136Z ------------------------------------------
2020-02-06T23:38:09.2134186Z stderr:
---
2020-02-06T23:38:09.2135836Z 
2020-02-06T23:38:09.2136099Z error[E0637]: `'_` cannot be used here
2020-02-06T23:38:09.2136408Z   --> /checkout/src/test/ui/underscore-lifetime/underscore-lifetime-binders.rs:10:25
2020-02-06T23:38:09.2136573Z    |
2020-02-06T23:38:09.2136825Z LL | fn meh() -> Box<dyn for<'_> Meh<'_>> //~ ERROR cannot be used here
2020-02-06T23:38:09.2137082Z    |                         ^^ `'_` is a reserved lifetime name
2020-02-06T23:38:09.2137159Z error[E0106]: missing lifetime specifier
2020-02-06T23:38:09.2137426Z   --> /checkout/src/test/ui/underscore-lifetime/underscore-lifetime-binders.rs:2:17
2020-02-06T23:38:09.2137475Z    |
2020-02-06T23:38:09.2137475Z    |
2020-02-06T23:38:09.2137713Z LL | struct Baz<'a>(&'_ &'a u8); //~ ERROR missing lifetime specifier
2020-02-06T23:38:09.2137970Z    |                 ^^ help: consider using the named lifetime: `'a`
2020-02-06T23:38:09.2138066Z error[E0106]: missing lifetime specifier
2020-02-06T23:38:09.2138326Z   --> /checkout/src/test/ui/underscore-lifetime/underscore-lifetime-binders.rs:10:33
2020-02-06T23:38:09.2138386Z    |
2020-02-06T23:38:09.2138386Z    |
2020-02-06T23:38:09.2138623Z LL | fn meh() -> Box<dyn for<'_> Meh<'_>> //~ ERROR cannot be used here
2020-02-06T23:38:09.2138889Z    |                                 ^^ help: consider giving it a 'static lifetime: `'static`
2020-02-06T23:38:09.2139236Z    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
2020-02-06T23:38:09.2139277Z 
2020-02-06T23:38:09.2139337Z error[E0106]: missing lifetime specifier
2020-02-06T23:38:09.2139598Z   --> /checkout/src/test/ui/underscore-lifetime/underscore-lifetime-binders.rs:16:35
2020-02-06T23:38:09.2139598Z   --> /checkout/src/test/ui/underscore-lifetime/underscore-lifetime-binders.rs:16:35
2020-02-06T23:38:09.2139646Z    |
2020-02-06T23:38:09.2139902Z LL | fn foo2(_: &'_ u8, y: &'_ u8) -> &'_ u8 { y } //~ ERROR missing lifetime specifier
2020-02-06T23:38:09.2140236Z    |
2020-02-06T23:38:09.2140547Z    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from argument 1 or `y`
2020-02-06T23:38:09.2140622Z help: consider introducing a named lifetime parameter
2020-02-06T23:38:09.2140783Z    |
2020-02-06T23:38:09.2140783Z    |
2020-02-06T23:38:09.2141053Z LL | fn foo2<'a>(_: &'a u8, y: &'a u8) -> &'a u8 { y } //~ ERROR missing lifetime specifier
2020-02-06T23:38:09.2141153Z 
2020-02-06T23:38:09.2141195Z error: aborting due to 5 previous errors
2020-02-06T23:38:09.2141224Z 
2020-02-06T23:38:09.2141282Z Some errors have detailed explanations: E0106, E0637.
---
2020-02-06T23:38:09.2142685Z 
2020-02-06T23:38:09.2142724Z 
2020-02-06T23:38:09.2142768Z The actual stderr differed from the expected stderr.
2020-02-06T23:38:09.2143334Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/underscore-outlives-bounds/underscore-outlives-bounds.stderr
2020-02-06T23:38:09.2143619Z To update references, rerun the tests and pass the `--bless` flag
2020-02-06T23:38:09.2143918Z To only update this specific test, also pass `--test-args underscore-lifetime/underscore-outlives-bounds.rs`
2020-02-06T23:38:09.2144176Z error: 1 errors occurred comparing output.
2020-02-06T23:38:09.2144227Z status: exit code: 1
2020-02-06T23:38:09.2144227Z status: exit code: 1
2020-02-06T23:38:09.2145130Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-lifetime/underscore-outlives-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/underscore-outlives-bounds" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/underscore-outlives-bounds/auxiliary" "-A" "unused"
2020-02-06T23:38:09.2145473Z ------------------------------------------
2020-02-06T23:38:09.2145533Z 
2020-02-06T23:38:09.2145773Z ------------------------------------------
2020-02-06T23:38:09.2145821Z stderr:
2020-02-06T23:38:09.2145821Z stderr:
2020-02-06T23:38:09.2146042Z ------------------------------------------
2020-02-06T23:38:09.2146280Z error[E0637]: `'_` cannot be used here
2020-02-06T23:38:09.2146544Z   --> /checkout/src/test/ui/underscore-lifetime/underscore-outlives-bounds.rs:7:10
2020-02-06T23:38:09.2146594Z    |
2020-02-06T23:38:09.2146863Z LL | impl<'b: '_> Foo<'b> for i32 {} //~ ERROR `'_` cannot be used here
2020-02-06T23:38:09.2147096Z    |          ^^ `'_` is a reserved lifetime name
2020-02-06T23:38:09.2147188Z error: aborting due to previous error
2020-02-06T23:38:09.2147217Z 
2020-02-06T23:38:09.2147465Z For more information about this error, try `rustc --explain E0637`.
2020-02-06T23:38:09.2147500Z 
---
2020-02-06T23:38:09.2148608Z 9 
2020-02-06T23:38:09.2148635Z 
2020-02-06T23:38:09.2148674Z 
2020-02-06T23:38:09.2148718Z The actual stderr differed from the expected stderr.
2020-02-06T23:38:09.2149098Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clause-inherent-impl-ampersand.rust2015/where-clause-inherent-impl-ampersand.rust2015.stderr
2020-02-06T23:38:09.2149375Z To update references, rerun the tests and pass the `--bless` flag
2020-02-06T23:38:09.2149784Z To only update this specific test, also pass `--test-args underscore-lifetime/where-clause-inherent-impl-ampersand.rs`
2020-02-06T23:38:09.2149898Z error in revision `rust2015`: 1 errors occurred comparing output.
2020-02-06T23:38:09.2149942Z status: exit code: 1
2020-02-06T23:38:09.2149942Z status: exit code: 1
2020-02-06T23:38:09.2150846Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-lifetime/where-clause-inherent-impl-ampersand.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rust2015" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clause-inherent-impl-ampersand.rust2015" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clause-inherent-impl-ampersand.rust2015/auxiliary" "-A" "unused"
2020-02-06T23:38:09.2151307Z ------------------------------------------
2020-02-06T23:38:09.2151357Z 
2020-02-06T23:38:09.2151642Z ------------------------------------------
2020-02-06T23:38:09.2151696Z stderr:
2020-02-06T23:38:09.2151696Z stderr:
2020-02-06T23:38:09.2151952Z ------------------------------------------
2020-02-06T23:38:09.2152004Z error[E0637]: `&` without an explicit lifetime name cannot be used here
2020-02-06T23:38:09.2152276Z   --> /checkout/src/test/ui/underscore-lifetime/where-clause-inherent-impl-ampersand.rs:13:17
2020-02-06T23:38:09.2152348Z    |
2020-02-06T23:38:09.2152391Z LL |     T: WithType<&u32>
2020-02-06T23:38:09.2152483Z 
2020-02-06T23:38:09.2152526Z error: aborting due to previous error
2020-02-06T23:38:09.2152671Z 
2020-02-06T23:38:09.2152921Z For more information about this error, try `rustc --explain E0637`.
---
2020-02-06T23:38:09.2154308Z 9 
2020-02-06T23:38:09.2154348Z 
2020-02-06T23:38:09.2154373Z 
2020-02-06T23:38:09.2154418Z The actual stderr differed from the expected stderr.
2020-02-06T23:38:09.2154796Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clause-inherent-impl-ampersand.rust2018/where-clause-inherent-impl-ampersand.rust2018.stderr
2020-02-06T23:38:09.2155074Z To update references, rerun the tests and pass the `--bless` flag
2020-02-06T23:38:09.2155400Z To only update this specific test, also pass `--test-args underscore-lifetime/where-clause-inherent-impl-ampersand.rs`
2020-02-06T23:38:09.2155524Z error in revision `rust2018`: 1 errors occurred comparing output.
2020-02-06T23:38:09.2155571Z status: exit code: 1
2020-02-06T23:38:09.2155571Z status: exit code: 1
2020-02-06T23:38:09.2156561Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-lifetime/where-clause-inherent-impl-ampersand.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rust2018" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clause-inherent-impl-ampersand.rust2018" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clause-inherent-impl-ampersand.rust2018/auxiliary" "-A" "unused"
2020-02-06T23:38:09.2156948Z ------------------------------------------
2020-02-06T23:38:09.2156984Z 
2020-02-06T23:38:09.2157231Z ------------------------------------------
2020-02-06T23:38:09.2157296Z stderr:
2020-02-06T23:38:09.2157296Z stderr:
2020-02-06T23:38:09.2157535Z ------------------------------------------
2020-02-06T23:38:09.2157590Z error[E0637]: `&` without an explicit lifetime name cannot be used here
2020-02-06T23:38:09.2157907Z   --> /checkout/src/test/ui/underscore-lifetime/where-clause-inherent-impl-ampersand.rs:13:17
2020-02-06T23:38:09.2157964Z    |
2020-02-06T23:38:09.2158009Z LL |     T: WithType<&u32>
2020-02-06T23:38:09.2158105Z 
2020-02-06T23:38:09.2158148Z error: aborting due to previous error
2020-02-06T23:38:09.2158177Z 
2020-02-06T23:38:09.2158463Z For more information about this error, try `rustc --explain E0637`.
---
2020-02-06T23:38:09.2160166Z 9 
2020-02-06T23:38:09.2160194Z 
2020-02-06T23:38:09.2160221Z 
2020-02-06T23:38:09.2160267Z The actual stderr differed from the expected stderr.
2020-02-06T23:38:09.2160687Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clause-inherent-impl-underscore.rust2015/where-clause-inherent-impl-underscore.rust2015.stderr
2020-02-06T23:38:09.2160973Z To update references, rerun the tests and pass the `--bless` flag
2020-02-06T23:38:09.2161344Z To only update this specific test, also pass `--test-args underscore-lifetime/where-clause-inherent-impl-underscore.rs`
2020-02-06T23:38:09.2161434Z error in revision `rust2015`: 1 errors occurred comparing output.
2020-02-06T23:38:09.2161496Z status: exit code: 1
2020-02-06T23:38:09.2161496Z status: exit code: 1
2020-02-06T23:38:09.2162470Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-lifetime/where-clause-inherent-impl-underscore.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rust2015" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clause-inherent-impl-underscore.rust2015" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clause-inherent-impl-underscore.rust2015/auxiliary" "-A" "unused"
2020-02-06T23:38:09.2163064Z ------------------------------------------
2020-02-06T23:38:09.2163102Z 
2020-02-06T23:38:09.2163365Z ------------------------------------------
2020-02-06T23:38:09.2163415Z stderr:
---
2020-02-06T23:38:09.2166384Z 9 
2020-02-06T23:38:09.2166411Z 
2020-02-06T23:38:09.2166437Z 
2020-02-06T23:38:09.2166499Z The actual stderr differed from the expected stderr.
2020-02-06T23:38:09.2166906Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clause-inherent-impl-underscore.rust2018/where-clause-inherent-impl-underscore.rust2018.stderr
2020-02-06T23:38:09.2167187Z To update references, rerun the tests and pass the `--bless` flag
2020-02-06T23:38:09.2167741Z To only update this specific test, also pass `--test-args underscore-lifetime/where-clause-inherent-impl-underscore.rs`
2020-02-06T23:38:09.2167838Z error in revision `rust2018`: 1 errors occurred comparing output.
2020-02-06T23:38:09.2167903Z status: exit code: 1
2020-02-06T23:38:09.2167903Z status: exit code: 1
2020-02-06T23:38:09.2168926Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-lifetime/where-clause-inherent-impl-underscore.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rust2018" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clause-inherent-impl-underscore.rust2018" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clause-inherent-impl-underscore.rust2018/auxiliary" "-A" "unused"
2020-02-06T23:38:09.2169311Z ------------------------------------------
2020-02-06T23:38:09.2169347Z 
2020-02-06T23:38:09.2169825Z ------------------------------------------
2020-02-06T23:38:09.2169877Z stderr:
---
2020-02-06T23:38:09.2173012Z 9 
2020-02-06T23:38:09.2173039Z 
2020-02-06T23:38:09.2173066Z 
2020-02-06T23:38:09.2173127Z The actual stderr differed from the expected stderr.
2020-02-06T23:38:09.2173519Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clause-trait-impl-region.rust2015/where-clause-trait-impl-region.rust2015.stderr
2020-02-06T23:38:09.2173835Z To update references, rerun the tests and pass the `--bless` flag
2020-02-06T23:38:09.2174167Z To only update this specific test, also pass `--test-args underscore-lifetime/where-clause-trait-impl-region.rs`
2020-02-06T23:38:09.2174258Z error in revision `rust2015`: 1 errors occurred comparing output.
2020-02-06T23:38:09.2174321Z status: exit code: 1
2020-02-06T23:38:09.2174321Z status: exit code: 1
2020-02-06T23:38:09.2175467Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-lifetime/where-clause-trait-impl-region.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rust2015" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clause-trait-impl-region.rust2015" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clause-trait-impl-region.rust2015/auxiliary" "-A" "unused"
2020-02-06T23:38:09.2175911Z ------------------------------------------
2020-02-06T23:38:09.2175962Z 
2020-02-06T23:38:09.2176183Z ------------------------------------------
2020-02-06T23:38:09.2176229Z stderr:
2020-02-06T23:38:09.2176229Z stderr:
2020-02-06T23:38:09.2176440Z ------------------------------------------
2020-02-06T23:38:09.2176508Z error[E0637]: `&` without an explicit lifetime name cannot be used here
2020-02-06T23:38:09.2176776Z   --> /checkout/src/test/ui/underscore-lifetime/where-clause-trait-impl-region.rs:11:17
2020-02-06T23:38:09.2176840Z    |
2020-02-06T23:38:09.2176883Z LL |     T: WithType<&u32>
2020-02-06T23:38:09.2176957Z 
2020-02-06T23:38:09.2177012Z error: aborting due to previous error
2020-02-06T23:38:09.2177039Z 
2020-02-06T23:38:09.2177281Z For more information about this error, try `rustc --explain E0637`.
---
2020-02-06T23:38:09.2178388Z 9 
2020-02-06T23:38:09.2178414Z 
2020-02-06T23:38:09.2178453Z 
2020-02-06T23:38:09.2178496Z The actual stderr differed from the expected stderr.
2020-02-06T23:38:09.2178853Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clause-trait-impl-region.rust2018/where-clause-trait-impl-region.rust2018.stderr
2020-02-06T23:38:09.2179143Z To update references, rerun the tests and pass the `--bless` flag
2020-02-06T23:38:09.2179455Z To only update this specific test, also pass `--test-args underscore-lifetime/where-clause-trait-impl-region.rs`
2020-02-06T23:38:09.2179557Z error in revision `rust2018`: 1 errors occurred comparing output.
2020-02-06T23:38:09.2179603Z status: exit code: 1
2020-02-06T23:38:09.2179603Z status: exit code: 1
2020-02-06T23:38:09.2180541Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-lifetime/where-clause-trait-impl-region.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rust2018" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clause-trait-impl-region.rust2018" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clause-trait-impl-region.rust2018/auxiliary" "-A" "unused"
2020-02-06T23:38:09.2181015Z ------------------------------------------
2020-02-06T23:38:09.2181064Z 
2020-02-06T23:38:09.2181288Z ------------------------------------------
2020-02-06T23:38:09.2181506Z stderr:
2020-02-06T23:38:09.2181506Z stderr:
2020-02-06T23:38:09.2181792Z ------------------------------------------
2020-02-06T23:38:09.2181846Z error[E0637]: `&` without an explicit lifetime name cannot be used here
2020-02-06T23:38:09.2182168Z   --> /checkout/src/test/ui/underscore-lifetime/where-clause-trait-impl-region.rs:11:17
2020-02-06T23:38:09.2182240Z    |
2020-02-06T23:38:09.2182282Z LL |     T: WithType<&u32>
2020-02-06T23:38:09.2182359Z 
2020-02-06T23:38:09.2182416Z error: aborting due to previous error
2020-02-06T23:38:09.2182445Z 
2020-02-06T23:38:09.2182841Z For more information about this error, try `rustc --explain E0637`.
---
2020-02-06T23:38:09.2184070Z 9 
2020-02-06T23:38:09.2184111Z 
2020-02-06T23:38:09.2184136Z 
2020-02-06T23:38:09.2184180Z The actual stderr differed from the expected stderr.
2020-02-06T23:38:09.2184663Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clause-trait-impl-underscore.rust2015/where-clause-trait-impl-underscore.rust2015.stderr
2020-02-06T23:38:09.2185080Z To update references, rerun the tests and pass the `--bless` flag
2020-02-06T23:38:09.2185411Z To only update this specific test, also pass `--test-args underscore-lifetime/where-clause-trait-impl-underscore.rs`
2020-02-06T23:38:09.2185516Z error in revision `rust2015`: 1 errors occurred comparing output.
2020-02-06T23:38:09.2185564Z status: exit code: 1
2020-02-06T23:38:09.2185564Z status: exit code: 1
2020-02-06T23:38:09.2186528Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-lifetime/where-clause-trait-impl-underscore.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rust2015" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clause-trait-impl-underscore.rust2015" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clause-trait-impl-underscore.rust2015/auxiliary" "-A" "unused"
2020-02-06T23:38:09.2186918Z ------------------------------------------
2020-02-06T23:38:09.2186956Z 
2020-02-06T23:38:09.2187200Z ------------------------------------------
2020-02-06T23:38:09.2187249Z stderr:
---
2020-02-06T23:38:09.2190199Z 9 
2020-02-06T23:38:09.2190238Z 
2020-02-06T23:38:09.2190262Z 
2020-02-06T23:38:09.2190304Z The actual stderr differed from the expected stderr.
2020-02-06T23:38:09.2190666Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clause-trait-impl-underscore.rust2018/where-clause-trait-impl-underscore.rust2018.stderr
2020-02-06T23:38:09.2191408Z To update references, rerun the tests and pass the `--bless` flag
2020-02-06T23:38:09.2191811Z To only update this specific test, also pass `--test-args underscore-lifetime/where-clause-trait-impl-underscore.rs`
2020-02-06T23:38:09.2191916Z error in revision `rust2018`: 1 errors occurred comparing output.
2020-02-06T23:38:09.2191962Z status: exit code: 1
2020-02-06T23:38:09.2191962Z status: exit code: 1
2020-02-06T23:38:09.2192932Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-lifetime/where-clause-trait-impl-underscore.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rust2018" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clause-trait-impl-underscore.rust2018" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clause-trait-impl-underscore.rust2018/auxiliary" "-A" "unused"
2020-02-06T23:38:09.2193289Z ------------------------------------------
2020-02-06T23:38:09.2193324Z 
2020-02-06T23:38:09.2193543Z ------------------------------------------
2020-02-06T23:38:09.2193605Z stderr:
---
2020-02-06T23:38:09.2197342Z 
2020-02-06T23:38:09.2197367Z 
2020-02-06T23:38:09.2197412Z The actual stderr differed from the expected stderr.
2020-02-06T23:38:09.2197744Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clauses/where-clauses.stderr
2020-02-06T23:38:09.2198034Z To update references, rerun the tests and pass the `--bless` flag
2020-02-06T23:38:09.2198534Z To only update this specific test, also pass `--test-args underscore-lifetime/where-clauses.rs`
2020-02-06T23:38:09.2198638Z error: 1 errors occurred comparing output.
2020-02-06T23:38:09.2198681Z status: exit code: 1
2020-02-06T23:38:09.2198681Z status: exit code: 1
2020-02-06T23:38:09.2199525Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-lifetime/where-clauses.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clauses" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-lifetime/where-clauses/auxiliary" "-A" "unused"
2020-02-06T23:38:09.2200111Z ------------------------------------------
2020-02-06T23:38:09.2200153Z 
2020-02-06T23:38:09.2200408Z ------------------------------------------
2020-02-06T23:38:09.2200455Z stderr:
2020-02-06T23:38:09.2200455Z stderr:
2020-02-06T23:38:09.2200693Z ------------------------------------------
2020-02-06T23:38:09.2200913Z error[E0637]: `'_` cannot be used here
2020-02-06T23:38:09.2201163Z   --> /checkout/src/test/ui/underscore-lifetime/where-clauses.rs:3:10
2020-02-06T23:38:09.2201231Z    |
2020-02-06T23:38:09.2201489Z LL | impl<'b: '_> Foo<'b> for i32 {} //~ ERROR `'_` cannot be used here
2020-02-06T23:38:09.2201722Z    |          ^^ `'_` is a reserved lifetime name
2020-02-06T23:38:09.2201989Z error[E0637]: `'_` cannot be used here
2020-02-06T23:38:09.2202240Z   --> /checkout/src/test/ui/underscore-lifetime/where-clauses.rs:5:9
2020-02-06T23:38:09.2202302Z    |
2020-02-06T23:38:09.2202302Z    |
2020-02-06T23:38:09.2202558Z LL | impl<T: '_> Foo<'static> for Vec<T> {} //~ ERROR `'_` cannot be used here
2020-02-06T23:38:09.2202813Z    |         ^^ `'_` is a reserved lifetime name
2020-02-06T23:38:09.2202905Z error: aborting due to 2 previous errors
2020-02-06T23:38:09.2202933Z 
2020-02-06T23:38:09.2203186Z For more information about this error, try `rustc --explain E0637`.
2020-02-06T23:38:09.2203221Z 
---
2020-02-06T23:38:09.2208520Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-06T23:38:09.2208594Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-06T23:38:09.2208625Z 
2020-02-06T23:38:09.2208649Z 
2020-02-06T23:38:09.2210203Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-06T23:38:09.2210475Z 
2020-02-06T23:38:09.2210503Z 
2020-02-06T23:38:09.2210561Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-06T23:38:09.2210608Z Build completed unsuccessfully in 1:02:48
2020-02-06T23:38:09.2210608Z Build completed unsuccessfully in 1:02:48
2020-02-06T23:38:09.2210659Z == clock drift check ==
2020-02-06T23:38:09.7323228Z   local time: Thu Feb  6 23:38:09 UTC 2020
2020-02-06T23:38:09.7323444Z   network time: Thu, 06 Feb 2020 23:38:09 GMT
2020-02-06T23:38:09.7323539Z == end clock drift check ==
2020-02-06T23:38:09.9605513Z 
2020-02-06T23:38:09.9705152Z ##[error]Bash exited with code '1'.
2020-02-06T23:38:09.9717448Z ##[section]Finishing: Run build
2020-02-06T23:38:09.9741471Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68908/merge to s
2020-02-06T23:38:09.9743440Z Task         : Get sources
2020-02-06T23:38:09.9743505Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-06T23:38:09.9743551Z Version      : 1.0.0
2020-02-06T23:38:09.9743592Z Author       : Microsoft
2020-02-06T23:38:09.9743592Z Author       : Microsoft
2020-02-06T23:38:09.9743655Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-06T23:38:09.9743717Z ==============================================================================
2020-02-06T23:38:10.3856881Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-06T23:38:10.3896185Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68908/merge to s
2020-02-06T23:38:10.4005005Z Cleaning up task key
2020-02-06T23:38:10.4005788Z Start cleaning up orphan processes.
2020-02-06T23:38:10.4112405Z Terminate orphan process: pid (4519) (python)
2020-02-06T23:38:10.4358297Z ##[section]Finishing: Finalize Job
