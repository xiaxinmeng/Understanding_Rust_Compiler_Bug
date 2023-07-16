plain
2019-10-04T03:18:47.3544280Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-04T03:18:47.3782812Z ##[command]git config gc.auto 0
2019-10-04T03:18:47.3850722Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-04T03:18:47.3913824Z ##[command]git config --get-all http.proxy
2019-10-04T03:18:47.4074713Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65083/merge:refs/remotes/pull/65083/merge
---
2019-10-04T04:23:54.3779905Z .................................................................................................... 1500/9100
2019-10-04T04:24:01.6807161Z .................................................................................................... 1600/9100
2019-10-04T04:24:11.2486455Z .................................................................................................... 1700/9100
2019-10-04T04:24:21.1027572Z .......i...............i............................................................................ 1800/9100
2019-10-04T04:24:28.5643376Z ..................................................................................................ii 1900/9100
2019-10-04T04:24:45.9809265Z iii................................................................................................. 2000/9100
2019-10-04T04:24:55.4449557Z .................................................................................................... 2200/9100
2019-10-04T04:24:58.2739578Z .................................................................................................... 2300/9100
2019-10-04T04:25:04.9429032Z .................................................................................................... 2400/9100
2019-10-04T04:25:10.9601797Z .................................................................................................... 2500/9100
---
2019-10-04T04:28:15.1728095Z .....................................................................................i.............. 4700/9100
2019-10-04T04:28:23.7028758Z .i.................................................................................................. 4800/9100
2019-10-04T04:28:34.7052595Z .................................................................................................... 4900/9100
2019-10-04T04:28:40.7874135Z .................................................................................................... 5000/9100
2019-10-04T04:28:53.5029517Z .............................................................................ii.ii.................. 5100/9100
2019-10-04T04:29:03.7087173Z .................................................................................................... 5300/9100
2019-10-04T04:29:14.1003776Z .................................................................................................... 5400/9100
2019-10-04T04:29:21.5440129Z ...........................................i........................................................ 5500/9100
2019-10-04T04:29:28.7397910Z .................................................................................................... 5600/9100
2019-10-04T04:29:28.7397910Z .................................................................................................... 5600/9100
2019-10-04T04:29:40.1002423Z .................................................................................................... 5700/9100
2019-10-04T04:29:48.1979895Z ........................................ii...i..ii............i..................................... 5800/9100
2019-10-04T04:30:15.2084067Z .................................................................................................... 6000/9100
2019-10-04T04:30:25.0017978Z .................................................................................................... 6100/9100
2019-10-04T04:30:25.0017978Z .................................................................................................... 6100/9100
2019-10-04T04:30:41.3540228Z .............................................i..ii.................................................. 6200/9100
2019-10-04T04:31:04.8277124Z .................................................................................................... 6400/9100
2019-10-04T04:31:07.1998069Z .....i.............................................................................................. 6500/9100
2019-10-04T04:31:09.5436008Z .............................................................................i...................... 6600/9100
2019-10-04T04:31:12.5170983Z .................................................................................................... 6700/9100
---
2019-10-04T04:35:34.7824359Z ---- [ui] ui/stability-attribute/stability-attribute-generic.rs stdout ----
2019-10-04T04:35:34.7824747Z 
2019-10-04T04:35:34.7824988Z error: ui test compiled successfully!
2019-10-04T04:35:34.7825131Z status: exit code: 0
2019-10-04T04:35:34.7826567Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/stability-attribute/stability-attribute-generic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/stability-attribute-generic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/stability-attribute-generic/auxiliary" "-A" "unused"
2019-10-04T04:35:34.7828039Z ------------------------------------------
2019-10-04T04:35:34.7828366Z 
2019-10-04T04:35:34.7828765Z ------------------------------------------
2019-10-04T04:35:34.7828955Z stderr:
---
2019-10-04T04:35:34.7830703Z diff of stderr:
2019-10-04T04:35:34.7830823Z 
2019-10-04T04:35:34.7830959Z 107    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-10-04T04:35:34.7831105Z 108 
2019-10-04T04:35:34.7831236Z 109 error: This stability annotation is useless
2019-10-04T04:35:34.7831657Z -   --> /home/host/Public/rust/src/test/ui/stability-attribute/stability-attribute-sanity.rs:76:9
2019-10-04T04:35:34.7832072Z +   --> $DIR/stability-attribute-sanity.rs:76:9
2019-10-04T04:35:34.7832246Z 111    |
2019-10-04T04:35:34.7832611Z - LL |         T> { //~ ERROR This stability annotation is useless
2019-10-04T04:35:34.7832802Z + LL |         T> {
2019-10-04T04:35:34.7833295Z 114 
2019-10-04T04:35:34.7833493Z 115 error: aborting due to 19 previous errors
2019-10-04T04:35:34.7833607Z 
2019-10-04T04:35:34.7833717Z 
2019-10-04T04:35:34.7833717Z 
2019-10-04T04:35:34.7833865Z The actual stderr differed from the expected stderr.
2019-10-04T04:35:34.7834379Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/stability-attribute-sanity/stability-attribute-sanity.stderr
2019-10-04T04:35:34.7834830Z To update references, rerun the tests and pass the `--bless` flag
2019-10-04T04:35:34.7835286Z To only update this specific test, also pass `--test-args stability-attribute/stability-attribute-sanity.rs`
2019-10-04T04:35:34.7835789Z error: 1 errors occurred comparing output.
2019-10-04T04:35:34.7835920Z status: exit code: 1
2019-10-04T04:35:34.7835920Z status: exit code: 1
2019-10-04T04:35:34.7836876Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/stability-attribute-sanity" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/stability-attribute-sanity/auxiliary" "-A" "unused"
2019-10-04T04:35:34.7837786Z ------------------------------------------
2019-10-04T04:35:34.7838031Z 
2019-10-04T04:35:34.7838770Z ------------------------------------------
2019-10-04T04:35:34.7839028Z stderr:
2019-10-04T04:35:34.7839028Z stderr:
2019-10-04T04:35:34.7839386Z ------------------------------------------
2019-10-04T04:35:34.7839783Z error[E0541]: unknown meta item 'reason'
2019-10-04T04:35:34.7840404Z    |
2019-10-04T04:35:34.7840404Z    |
2019-10-04T04:35:34.7840828Z LL |     #[stable(feature = "a", since = "b", reason)] //~ ERROR unknown meta item 'reason' [E0541]
2019-10-04T04:35:34.7841036Z    |                                          ^^^^^^ expected one of `since`, `note`
2019-10-04T04:35:34.7841151Z 
2019-10-04T04:35:34.7841284Z error[E0539]: incorrect meta item
2019-10-04T04:35:34.7841865Z    |
2019-10-04T04:35:34.7841865Z    |
2019-10-04T04:35:34.7842002Z LL |     #[stable(feature = "a", since)] //~ ERROR incorrect meta item [E0539]
2019-10-04T04:35:34.7842276Z 
2019-10-04T04:35:34.7842276Z 
2019-10-04T04:35:34.7842406Z error[E0539]: incorrect meta item
2019-10-04T04:35:34.7842981Z    |
2019-10-04T04:35:34.7842981Z    |
2019-10-04T04:35:34.7843135Z LL |     #[stable(feature, since = "a")] //~ ERROR incorrect meta item [E0539]
2019-10-04T04:35:34.7843386Z 
2019-10-04T04:35:34.7843386Z 
2019-10-04T04:35:34.7843517Z error[E0539]: incorrect meta item
2019-10-04T04:35:34.7844496Z    |
2019-10-04T04:35:34.7844496Z    |
2019-10-04T04:35:34.7844777Z LL |     #[stable(feature = "a", since(b))] //~ ERROR incorrect meta item [E0539]
2019-10-04T04:35:34.7845445Z 
2019-10-04T04:35:34.7845445Z 
2019-10-04T04:35:34.7845668Z error[E0539]: incorrect meta item
2019-10-04T04:35:34.7847225Z    |
2019-10-04T04:35:34.7847225Z    |
2019-10-04T04:35:34.7847544Z LL |     #[stable(feature(b), since = "a")] //~ ERROR incorrect meta item [E0539]
2019-10-04T04:35:34.7847625Z 
2019-10-04T04:35:34.7847979Z error[E0546]: missing 'feature'
2019-10-04T04:35:34.7848415Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:25:5
2019-10-04T04:35:34.7848481Z    |
2019-10-04T04:35:34.7848481Z    |
2019-10-04T04:35:34.7848789Z LL |     #[unstable(issue = "0")] //~ ERROR missing 'feature' [E0546]
2019-10-04T04:35:34.7848870Z 
2019-10-04T04:35:34.7849077Z error[E0547]: missing 'issue'
2019-10-04T04:35:34.7849347Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:28:5
2019-10-04T04:35:34.7849395Z    |
2019-10-04T04:35:34.7849395Z    |
2019-10-04T04:35:34.7849632Z LL |     #[unstable(feature = "b")] //~ ERROR missing 'issue' [E0547]
2019-10-04T04:35:34.7849838Z 
2019-10-04T04:35:34.7850078Z error[E0546]: missing 'feature'
2019-10-04T04:35:34.7850333Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:31:5
2019-10-04T04:35:34.7850399Z    |
2019-10-04T04:35:34.7850399Z    |
2019-10-04T04:35:34.7850636Z LL |     #[stable(since = "a")] //~ ERROR missing 'feature' [E0546]
2019-10-04T04:35:34.7850741Z 
2019-10-04T04:35:34.7850741Z 
2019-10-04T04:35:34.7850948Z error[E0542]: missing 'since'
2019-10-04T04:35:34.7851250Z    |
2019-10-04T04:35:34.7851250Z    |
2019-10-04T04:35:34.7851502Z LL |     #[stable(feature = "a")] //~ ERROR missing 'since' [E0542]
2019-10-04T04:35:34.7851582Z 
2019-10-04T04:35:34.7851582Z 
2019-10-04T04:35:34.7851799Z error[E0542]: missing 'since'
2019-10-04T04:35:34.7852109Z    |
2019-10-04T04:35:34.7852109Z    |
2019-10-04T04:35:34.7852355Z LL |     #[rustc_deprecated(reason = "a")] //~ ERROR missing 'since' [E0542]
2019-10-04T04:35:34.7852455Z 
2019-10-04T04:35:34.7852658Z error[E0543]: missing 'reason'
2019-10-04T04:35:34.7852927Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:44:5
2019-10-04T04:35:34.7852975Z    |
2019-10-04T04:35:34.7852975Z    |
2019-10-04T04:35:34.7853228Z LL |     #[rustc_deprecated(since = "a")] //~ ERROR missing 'reason' [E0543]
2019-10-04T04:35:34.7853327Z 
2019-10-04T04:35:34.7853368Z error[E0544]: multiple stability levels
2019-10-04T04:35:34.7853622Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:49:1
2019-10-04T04:35:34.7853687Z    |
2019-10-04T04:35:34.7853687Z    |
2019-10-04T04:35:34.7853736Z LL | #[stable(feature = "a", since = "b")] //~ ERROR multiple stability levels [E0544]
2019-10-04T04:35:34.7853820Z 
2019-10-04T04:35:34.7853879Z error[E0544]: multiple stability levels
2019-10-04T04:35:34.7854135Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:53:1
2019-10-04T04:35:34.7854182Z    |
2019-10-04T04:35:34.7854182Z    |
2019-10-04T04:35:34.7854246Z LL | #[unstable(feature = "b", issue = "0")] //~ ERROR multiple stability levels [E0544]
2019-10-04T04:35:34.7854335Z 
2019-10-04T04:35:34.7854393Z error[E0544]: multiple stability levels
2019-10-04T04:35:34.7854650Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:57:1
2019-10-04T04:35:34.7854698Z    |
2019-10-04T04:35:34.7854698Z    |
2019-10-04T04:35:34.7854745Z LL | #[stable(feature = "a", since = "b")] //~ ERROR multiple stability levels [E0544]
2019-10-04T04:35:34.7854838Z 
2019-10-04T04:35:34.7854882Z error[E0540]: multiple rustc_deprecated attributes
2019-10-04T04:35:34.7855163Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:65:1
2019-10-04T04:35:34.7855211Z    |
2019-10-04T04:35:34.7855211Z    |
2019-10-04T04:35:34.7855259Z LL | pub const fn multiple4() { } //~ ERROR multiple rustc_deprecated attributes [E0540]
2019-10-04T04:35:34.7855351Z 
2019-10-04T04:35:34.7855394Z error[E0553]: multiple rustc_const_unstable attributes
2019-10-04T04:35:34.7855741Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:65:1
2019-10-04T04:35:34.7855820Z    |
2019-10-04T04:35:34.7855820Z    |
2019-10-04T04:35:34.7855869Z LL | pub const fn multiple4() { } //~ ERROR multiple rustc_deprecated attributes [E0540]
2019-10-04T04:35:34.7855960Z 
2019-10-04T04:35:34.7856005Z error: Invalid stability or deprecation version found
2019-10-04T04:35:34.7856295Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:65:1
2019-10-04T04:35:34.7856343Z    |
2019-10-04T04:35:34.7856343Z    |
2019-10-04T04:35:34.7856505Z LL | pub const fn multiple4() { } //~ ERROR multiple rustc_deprecated attributes [E0540]
2019-10-04T04:35:34.7856579Z 
2019-10-04T04:35:34.7856579Z 
2019-10-04T04:35:34.7856646Z error[E0549]: rustc_deprecated attribute must be paired with either stable or unstable attribute
2019-10-04T04:35:34.7856995Z    |
2019-10-04T04:35:34.7857056Z LL | fn deprecated_without_unstable_or_stable() { }
2019-10-04T04:35:34.7857104Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-10-04T04:35:34.7857136Z 
2019-10-04T04:35:34.7857136Z 
2019-10-04T04:35:34.7857180Z error: This stability annotation is useless
2019-10-04T04:35:34.7857776Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:76:9
2019-10-04T04:35:34.7857834Z    |
2019-10-04T04:35:34.7857880Z LL |         T> { //~ ERROR This stability annotation is useless
2019-10-04T04:35:34.7857982Z 
2019-10-04T04:35:34.7858024Z error: aborting due to 19 previous errors
2019-10-04T04:35:34.7858052Z 
2019-10-04T04:35:34.7858317Z For more information about this error, try `rustc --explain E0541`.
---
2019-10-04T04:35:34.7879667Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-04T04:35:34.7879754Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-04T04:35:34.7898050Z 
2019-10-04T04:35:34.7898169Z 
2019-10-04T04:35:34.7900118Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-04T04:35:34.7900640Z 
2019-10-04T04:35:34.7900693Z 
2019-10-04T04:35:34.7905956Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-04T04:35:34.7906037Z Build completed unsuccessfully in 1:08:57
2019-10-04T04:35:34.7906037Z Build completed unsuccessfully in 1:08:57
2019-10-04T04:35:34.7959398Z == clock drift check ==
2019-10-04T04:35:34.7973590Z   local time: Fri Oct  4 04:35:34 UTC 2019
2019-10-04T04:35:34.9474314Z   network time: Fri, 04 Oct 2019 04:35:34 GMT
2019-10-04T04:35:34.9480570Z == end clock drift check ==
2019-10-04T04:35:35.9597101Z ##[error]Bash exited with code '1'.
2019-10-04T04:35:35.9669372Z ##[section]Starting: Checkout
2019-10-04T04:35:35.9671237Z ==============================================================================
2019-10-04T04:35:35.9671293Z Task         : Get sources
2019-10-04T04:35:35.9671339Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
