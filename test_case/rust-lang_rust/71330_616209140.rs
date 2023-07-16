plain
2020-04-19T18:09:43.0342150Z ========================== Starting Command Output ===========================
2020-04-19T18:09:43.0347595Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/13b0eac0-5cbf-421e-bcfb-74f76ee06700.sh
2020-04-19T18:09:43.0348089Z 
2020-04-19T18:09:43.0354160Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-19T18:09:43.0377337Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71330/merge to s
2020-04-19T18:09:43.0380490Z Task         : Get sources
2020-04-19T18:09:43.0380770Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T18:09:43.0381065Z Version      : 1.0.0
2020-04-19T18:09:43.0381259Z Author       : Microsoft
---
2020-04-19T18:09:44.0354384Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-19T18:09:44.0361967Z ##[command]git config gc.auto 0
2020-04-19T18:09:44.0369975Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-19T18:09:44.0373440Z ##[command]git config --get-all http.proxy
2020-04-19T18:09:44.0380479Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71330/merge:refs/remotes/pull/71330/merge
---
2020-04-19T18:12:55.0862215Z  ---> 318032b5f0e2
2020-04-19T18:12:55.0863217Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-19T18:12:55.0866136Z  ---> Using cache
2020-04-19T18:12:55.0866761Z  ---> d44a858fd1ce
2020-04-19T18:12:55.0867958Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-19T18:12:55.0871260Z  ---> 58b910f50f5a
2020-04-19T18:12:55.0871694Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-19T18:12:55.0874009Z  ---> Using cache
2020-04-19T18:12:55.0874563Z  ---> ee7702aadba1
---
2020-04-19T18:12:55.1241221Z Looks like docker image is the same as before, not uploading
2020-04-19T18:13:03.6466915Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-19T18:13:03.6774130Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-19T18:13:03.6804385Z == clock drift check ==
2020-04-19T18:13:03.6813734Z   local time: Sun Apr 19 18:13:03 UTC 2020
2020-04-19T18:13:04.0036305Z   network time: Sun, 19 Apr 2020 18:13:03 GMT
2020-04-19T18:13:04.0060121Z Starting sccache server...
2020-04-19T18:13:04.0945304Z configure: processing command line
2020-04-19T18:13:04.0945713Z configure: 
2020-04-19T18:13:04.0947110Z configure: rust.dist-src        := False
---
2020-04-19T18:18:07.1248226Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T18:18:08.5275367Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T18:18:10.0149318Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T18:18:11.3785308Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T18:18:19.6923721Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T18:18:21.9249991Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T18:18:25.9962714Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T18:18:29.8981091Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T18:18:38.7673042Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T18:39:30.7075827Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T18:39:32.2028937Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T18:39:33.6958170Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T18:39:33.9448828Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T18:39:43.8874039Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T18:39:46.0185382Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T18:39:50.4900929Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T18:39:54.6915897Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T18:40:04.8622570Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T19:01:33.3267187Z .................................................................................................... 1700/9907
2020-04-19T19:01:37.6812098Z .................................................................................................... 1800/9907
2020-04-19T19:01:46.8196109Z ..................................................................................................i. 1900/9907
2020-04-19T19:01:54.6515920Z .................................................................................................... 2000/9907
2020-04-19T19:02:00.9549328Z ........................................................................................iiiii....... 2100/9907
2020-04-19T19:02:20.4254461Z .................................................................................................... 2300/9907
2020-04-19T19:02:22.6403144Z .................................................................................................... 2400/9907
2020-04-19T19:02:24.9890858Z .................................................................................................... 2500/9907
2020-04-19T19:02:31.1797917Z .................................................................................................... 2600/9907
---
2020-04-19T19:05:11.1075575Z ................................................................i...............i................... 5000/9907
2020-04-19T19:05:17.8721226Z .................................................................................................... 5100/9907
2020-04-19T19:05:24.2768125Z .................................................................................................... 5200/9907
2020-04-19T19:05:29.0086118Z ..........i......................................................................................... 5300/9907
2020-04-19T19:05:37.9001007Z i................................................................................................... 5400/9907
2020-04-19T19:05:42.3561126Z ii.ii........i...i.................................................................................. 5500/9907
2020-04-19T19:05:49.4842325Z ...............................................i.................................................... 5700/9907
2020-04-19T19:05:57.6940628Z ...............................................................................ii................... 5800/9907
2020-04-19T19:06:04.1498707Z ..................i................................................................................. 5900/9907
2020-04-19T19:06:09.0963755Z .................................................................................................... 6000/9907
2020-04-19T19:06:09.0963755Z .................................................................................................... 6000/9907
2020-04-19T19:06:19.3230942Z .................................................................................................... 6100/9907
2020-04-19T19:06:29.1782958Z ............ii...i..ii...........i.................................................................. 6200/9907
2020-04-19T19:06:43.1427173Z .................................................................................................... 6400/9907
2020-04-19T19:06:46.4390878Z .................................................................................................... 6500/9907
2020-04-19T19:06:46.4390878Z .................................................................................................... 6500/9907
2020-04-19T19:06:56.6268582Z ..........................................i..ii..................................................... 6600/9907
2020-04-19T19:07:16.7945033Z .................................................................................................... 6800/9907
2020-04-19T19:07:18.9329154Z ...........................................i........................................................ 6900/9907
2020-04-19T19:07:20.8407623Z .................................................................................................... 7000/9907
2020-04-19T19:07:22.9027648Z ...................................................................................i................ 7100/9907
---
2020-04-19T19:08:47.0929338Z .................................................................................................... 7800/9907
2020-04-19T19:08:51.3591439Z .................................................................................................... 7900/9907
2020-04-19T19:08:57.2008593Z .................................................................................................... 8000/9907
2020-04-19T19:09:02.9098283Z .................................................i.................................................. 8100/9907
2020-04-19T19:09:12.0050757Z ..................................................................................................ii 8200/9907
2020-04-19T19:09:16.9086109Z iiii.iiiii.i........................................................................................ 8300/9907
2020-04-19T19:09:29.1298939Z .................................................................................................... 8500/9907
2020-04-19T19:09:36.4279248Z .................................................................................................... 8600/9907
2020-04-19T19:09:49.2046828Z .................................................................................................... 8700/9907
2020-04-19T19:09:55.3351575Z .................................................................................................... 8800/9907
---
2020-04-19T19:11:33.2160840Z - error[E0391]: cycle detected when const checking `FOO`
2020-04-19T19:11:33.2161476Z -   --> $DIR/issue-17252.rs:1:20
2020-04-19T19:11:33.2161922Z + error[E0391]: cycle detected when normalizing `FOO`
2020-04-19T19:11:33.2162271Z 3    |
2020-04-19T19:11:33.2162829Z + note: ...which requires const-evaluating + checking `FOO`...
2020-04-19T19:11:33.2163773Z +    |
2020-04-19T19:11:33.2164108Z 4 LL | const FOO: usize = FOO;
2020-04-19T19:11:33.2164623Z -    |                    ^^^
2020-04-19T19:11:33.2164981Z +    | ^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T19:11:33.2164981Z +    | ^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T19:11:33.2170326Z + note: ...which requires const-evaluating + checking `FOO`...
2020-04-19T19:11:33.2171329Z 6    |
2020-04-19T19:11:33.2171807Z -    = note: ...which again requires const checking `FOO`, completing the cycle
2020-04-19T19:11:33.2171807Z -    = note: ...which again requires const checking `FOO`, completing the cycle
2020-04-19T19:11:33.2172355Z - note: cycle used when const checking `main::{{constant}}#0`
2020-04-19T19:11:33.2172638Z + LL | const FOO: usize = FOO;
2020-04-19T19:11:33.2173310Z + note: ...which requires const-evaluating `FOO`...
2020-04-19T19:11:33.2173725Z +   --> $DIR/issue-17252.rs:1:1
2020-04-19T19:11:33.2173918Z +    |
2020-04-19T19:11:33.2174091Z + LL | const FOO: usize = FOO;
2020-04-19T19:11:33.2174091Z + LL | const FOO: usize = FOO;
2020-04-19T19:11:33.2174297Z +    | ^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T19:11:33.2174590Z +    = note: ...which again requires normalizing `FOO`, completing the cycle
2020-04-19T19:11:33.2175133Z + note: cycle used when const-evaluating `main::{{constant}}#0`
2020-04-19T19:11:33.2175780Z 10    |
2020-04-19T19:11:33.2175780Z 10    |
2020-04-19T19:11:33.2176016Z 11 LL |     let _x: [u8; FOO]; // caused stack overflow prior to fix
2020-04-19T19:11:33.2176313Z 
2020-04-19T19:11:33.2176516Z The actual stderr differed from the expected stderr.
2020-04-19T19:11:33.2177124Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17252/issue-17252.stderr
2020-04-19T19:11:33.2177124Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17252/issue-17252.stderr
2020-04-19T19:11:33.2177903Z To update references, rerun the tests and pass the `--bless` flag
2020-04-19T19:11:33.2178536Z To only update this specific test, also pass `--test-args issues/issue-17252.rs`
2020-04-19T19:11:33.2178933Z error: 1 errors occurred comparing output.
2020-04-19T19:11:33.2179166Z status: exit code: 1
2020-04-19T19:11:33.2179166Z status: exit code: 1
2020-04-19T19:11:33.2180925Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-17252.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17252" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17252/auxiliary"
2020-04-19T19:11:33.2182378Z ------------------------------------------
2020-04-19T19:11:33.2182526Z 
2020-04-19T19:11:33.2182862Z ------------------------------------------
2020-04-19T19:11:33.2183038Z stderr:
2020-04-19T19:11:33.2183038Z stderr:
2020-04-19T19:11:33.2183450Z ------------------------------------------
2020-04-19T19:11:33.2183672Z error[E0391]: cycle detected when normalizing `FOO`
2020-04-19T19:11:33.2183833Z    |
2020-04-19T19:11:33.2184156Z note: ...which requires const-evaluating + checking `FOO`...
2020-04-19T19:11:33.2184723Z    |
2020-04-19T19:11:33.2184881Z LL | const FOO: usize = FOO; //~ ERROR E0391
2020-04-19T19:11:33.2185084Z    | ^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T19:11:33.2185084Z    | ^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T19:11:33.2185436Z note: ...which requires const-evaluating + checking `FOO`...
2020-04-19T19:11:33.2185987Z    |
2020-04-19T19:11:33.2186158Z LL | const FOO: usize = FOO; //~ ERROR E0391
2020-04-19T19:11:33.2186345Z    | ^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T19:11:33.2186677Z note: ...which requires const-evaluating `FOO`...
2020-04-19T19:11:33.2186677Z note: ...which requires const-evaluating `FOO`...
2020-04-19T19:11:33.2187066Z   --> /checkout/src/test/ui/issues/issue-17252.rs:1:1
2020-04-19T19:11:33.2187233Z    |
2020-04-19T19:11:33.2187386Z LL | const FOO: usize = FOO; //~ ERROR E0391
2020-04-19T19:11:33.2187828Z    | ^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T19:11:33.2188082Z    = note: ...which again requires normalizing `FOO`, completing the cycle
2020-04-19T19:11:33.2188583Z note: cycle used when const-evaluating `main::{{constant}}#0`
2020-04-19T19:11:33.2189252Z    |
2020-04-19T19:11:33.2189252Z    |
2020-04-19T19:11:33.2189461Z LL |     let _x: [u8; FOO]; // caused stack overflow prior to fix
2020-04-19T19:11:33.2189833Z 
2020-04-19T19:11:33.2189990Z error: aborting due to previous error
2020-04-19T19:11:33.2190128Z 
2020-04-19T19:11:33.2190515Z For more information about this error, try `rustc --explain E0391`.
2020-04-19T19:11:33.2190515Z For more information about this error, try `rustc --explain E0391`.
2020-04-19T19:11:33.2190698Z 
2020-04-19T19:11:33.2191022Z ------------------------------------------
2020-04-19T19:11:33.2191166Z 
2020-04-19T19:11:33.2191264Z 
2020-04-19T19:11:33.2191600Z ---- [ui] ui/issues/issue-23302-1.rs stdout ----
2020-04-19T19:11:33.2191799Z diff of stderr:
2020-04-19T19:11:33.2191912Z 
2020-04-19T19:11:33.2192340Z - error[E0391]: cycle detected when const checking `X::A::{{constant}}#0`
2020-04-19T19:11:33.2193310Z + error[E0391]: cycle detected when const-evaluating + checking `X::A::{{constant}}#0`
2020-04-19T19:11:33.2193973Z 3    |
2020-04-19T19:11:33.2193973Z 3    |
2020-04-19T19:11:33.2194133Z 4 LL |     A = X::A as isize,
2020-04-19T19:11:33.2194408Z 5    |         ^^^^^^^^^^^^^
2020-04-19T19:11:33.2194571Z 6    |
2020-04-19T19:11:33.2194571Z 6    |
2020-04-19T19:11:33.2195053Z -    = note: ...which again requires const checking `X::A::{{constant}}#0`, completing the cycle
2020-04-19T19:11:33.2195666Z - note: cycle used when processing `X::A::{{constant}}#0`
2020-04-19T19:11:33.2196261Z + note: ...which requires const-evaluating + checking `X::A::{{constant}}#0`...
2020-04-19T19:11:33.2196892Z 10    |
2020-04-19T19:11:33.2196892Z 10    |
2020-04-19T19:11:33.2197070Z 11 LL |     A = X::A as isize,
2020-04-19T19:11:33.2197347Z 12    |         ^^^^^^^^^^^^^
2020-04-19T19:11:33.2197347Z 12    |         ^^^^^^^^^^^^^
2020-04-19T19:11:33.2197801Z + note: ...which requires const-evaluating `X::A::{{constant}}#0`...
2020-04-19T19:11:33.2198397Z +    |
2020-04-19T19:11:33.2198397Z +    |
2020-04-19T19:11:33.2198569Z + LL |     A = X::A as isize,
2020-04-19T19:11:33.2198759Z +    |         ^^^^^^^^^^^^^
2020-04-19T19:11:33.2199007Z +    = note: ...which requires normalizing `X::A as isize`...
2020-04-19T19:11:33.2199613Z +    = note: ...which again requires const-evaluating + checking `X::A::{{constant}}#0`, completing the cycle
2020-04-19T19:11:33.2200185Z + note: cycle used when collecting item types in top-level module
2020-04-19T19:11:33.2200761Z +    |
2020-04-19T19:11:33.2200909Z + LL | enum X {
2020-04-19T19:11:33.2201052Z +    | ^^^^^^
2020-04-19T19:11:33.2201175Z 13 
2020-04-19T19:11:33.2201175Z 13 
2020-04-19T19:11:33.2201357Z 14 error: aborting due to previous error
2020-04-19T19:11:33.2201521Z 15 
2020-04-19T19:11:33.2201610Z 
2020-04-19T19:11:33.2201693Z 
2020-04-19T19:11:33.2201881Z The actual stderr differed from the expected stderr.
2020-04-19T19:11:33.2202444Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-1/issue-23302-1.stderr
2020-04-19T19:11:33.2202974Z To update references, rerun the tests and pass the `--bless` flag
2020-04-19T19:11:33.2203489Z To only update this specific test, also pass `--test-args issues/issue-23302-1.rs`
2020-04-19T19:11:33.2203850Z error: 1 errors occurred comparing output.
2020-04-19T19:11:33.2204065Z status: exit code: 1
2020-04-19T19:11:33.2204065Z status: exit code: 1
2020-04-19T19:11:33.2205946Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23302-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-1/auxiliary"
2020-04-19T19:11:33.2207290Z ------------------------------------------
2020-04-19T19:11:33.2207437Z 
2020-04-19T19:11:33.2207773Z ------------------------------------------
2020-04-19T19:11:33.2207947Z stderr:
2020-04-19T19:11:33.2207947Z stderr:
2020-04-19T19:11:33.2209367Z ------------------------------------------
2020-04-19T19:11:33.2209895Z error[E0391]: cycle detected when const-evaluating + checking `X::A::{{constant}}#0`
2020-04-19T19:11:33.2210621Z    |
2020-04-19T19:11:33.2210621Z    |
2020-04-19T19:11:33.2210815Z LL |     A = X::A as isize, //~ ERROR E0391
2020-04-19T19:11:33.2211165Z    |
2020-04-19T19:11:33.2211165Z    |
2020-04-19T19:11:33.2211612Z note: ...which requires const-evaluating + checking `X::A::{{constant}}#0`...
2020-04-19T19:11:33.2212305Z    |
2020-04-19T19:11:33.2212305Z    |
2020-04-19T19:11:33.2212495Z LL |     A = X::A as isize, //~ ERROR E0391
2020-04-19T19:11:33.2212701Z    |         ^^^^^^^^^^^^^
2020-04-19T19:11:33.2213131Z note: ...which requires const-evaluating `X::A::{{constant}}#0`...
2020-04-19T19:11:33.2213809Z    |
2020-04-19T19:11:33.2213809Z    |
2020-04-19T19:11:33.2213985Z LL |     A = X::A as isize, //~ ERROR E0391
2020-04-19T19:11:33.2214190Z    |         ^^^^^^^^^^^^^
2020-04-19T19:11:33.2214545Z    = note: ...which requires normalizing `X::A as isize`...
2020-04-19T19:11:33.2215209Z    = note: ...which again requires const-evaluating + checking `X::A::{{constant}}#0`, completing the cycle
2020-04-19T19:11:33.2215765Z note: cycle used when collecting item types in top-level module
2020-04-19T19:11:33.2216430Z    |
2020-04-19T19:11:33.2216556Z LL | enum X {
2020-04-19T19:11:33.2216706Z    | ^^^^^^
2020-04-19T19:11:33.2216806Z 
---
2020-04-19T19:11:33.2218297Z 
2020-04-19T19:11:33.2218588Z ---- [ui] ui/issues/issue-23302-2.rs stdout ----
2020-04-19T19:11:33.2218761Z diff of stderr:
2020-04-19T19:11:33.2218861Z 
2020-04-19T19:11:33.2219237Z - error[E0391]: cycle detected when const checking `Y::A::{{constant}}#0`
2020-04-19T19:11:33.2219739Z + error[E0391]: cycle detected when const-evaluating + checking `Y::A::{{constant}}#0`
2020-04-19T19:11:33.2220305Z 3    |
2020-04-19T19:11:33.2220305Z 3    |
2020-04-19T19:11:33.2220444Z 4 LL |     A = Y::B as isize,
2020-04-19T19:11:33.2220696Z 5    |         ^^^^^^^^^^^^^
2020-04-19T19:11:33.2220825Z 6    |
2020-04-19T19:11:33.2220825Z 6    |
2020-04-19T19:11:33.2221243Z -    = note: ...which again requires const checking `Y::A::{{constant}}#0`, completing the cycle
2020-04-19T19:11:33.2221704Z - note: cycle used when processing `Y::A::{{constant}}#0`
2020-04-19T19:11:33.2222176Z + note: ...which requires const-evaluating + checking `Y::A::{{constant}}#0`...
2020-04-19T19:11:33.2222714Z 10    |
2020-04-19T19:11:33.2222714Z 10    |
2020-04-19T19:11:33.2222869Z 11 LL |     A = Y::B as isize,
2020-04-19T19:11:33.2223113Z 12    |         ^^^^^^^^^^^^^
2020-04-19T19:11:33.2223113Z 12    |         ^^^^^^^^^^^^^
2020-04-19T19:11:33.2223554Z + note: ...which requires const-evaluating `Y::A::{{constant}}#0`...
2020-04-19T19:11:33.2224077Z +    |
2020-04-19T19:11:33.2224077Z +    |
2020-04-19T19:11:33.2224229Z + LL |     A = Y::B as isize,
2020-04-19T19:11:33.2224395Z +    |         ^^^^^^^^^^^^^
2020-04-19T19:11:33.2224609Z +    = note: ...which requires normalizing `Y::B as isize`...
2020-04-19T19:11:33.2225134Z +    = note: ...which again requires const-evaluating + checking `Y::A::{{constant}}#0`, completing the cycle
2020-04-19T19:11:33.2225636Z + note: cycle used when collecting item types in top-level module
2020-04-19T19:11:33.2226150Z +    |
2020-04-19T19:11:33.2226265Z + LL | enum Y {
2020-04-19T19:11:33.2226388Z +    | ^^^^^^
2020-04-19T19:11:33.2226493Z 13 
2020-04-19T19:11:33.2226493Z 13 
2020-04-19T19:11:33.2226654Z 14 error: aborting due to previous error
2020-04-19T19:11:33.2226799Z 15 
2020-04-19T19:11:33.2226876Z 
2020-04-19T19:11:33.2226948Z 
2020-04-19T19:11:33.2227116Z The actual stderr differed from the expected stderr.
2020-04-19T19:11:33.2227612Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-2/issue-23302-2.stderr
2020-04-19T19:11:33.2228070Z To update references, rerun the tests and pass the `--bless` flag
2020-04-19T19:11:33.2228518Z To only update this specific test, also pass `--test-args issues/issue-23302-2.rs`
2020-04-19T19:11:33.2228836Z error: 1 errors occurred comparing output.
2020-04-19T19:11:33.2229024Z status: exit code: 1
2020-04-19T19:11:33.2229024Z status: exit code: 1
2020-04-19T19:11:33.2230879Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23302-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-2/auxiliary"
2020-04-19T19:11:33.2232225Z ------------------------------------------
2020-04-19T19:11:33.2232371Z 
2020-04-19T19:11:33.2232707Z ------------------------------------------
2020-04-19T19:11:33.2232881Z stderr:
2020-04-19T19:11:33.2232881Z stderr:
2020-04-19T19:11:33.2233209Z ------------------------------------------
2020-04-19T19:11:33.2233724Z error[E0391]: cycle detected when const-evaluating + checking `Y::A::{{constant}}#0`
2020-04-19T19:11:33.2234430Z    |
2020-04-19T19:11:33.2234430Z    |
2020-04-19T19:11:33.2234622Z LL |     A = Y::B as isize, //~ ERROR E0391
2020-04-19T19:11:33.2234968Z    |
2020-04-19T19:11:33.2234968Z    |
2020-04-19T19:11:33.2235411Z note: ...which requires const-evaluating + checking `Y::A::{{constant}}#0`...
2020-04-19T19:11:33.2236109Z    |
2020-04-19T19:11:33.2236109Z    |
2020-04-19T19:11:33.2236298Z LL |     A = Y::B as isize, //~ ERROR E0391
2020-04-19T19:11:33.2236504Z    |         ^^^^^^^^^^^^^
2020-04-19T19:11:33.2236934Z note: ...which requires const-evaluating `Y::A::{{constant}}#0`...
2020-04-19T19:11:33.2237616Z    |
2020-04-19T19:11:33.2237616Z    |
2020-04-19T19:11:33.2237790Z LL |     A = Y::B as isize, //~ ERROR E0391
2020-04-19T19:11:33.2237995Z    |         ^^^^^^^^^^^^^
2020-04-19T19:11:33.2238250Z    = note: ...which requires normalizing `Y::B as isize`...
2020-04-19T19:11:33.2238845Z    = note: ...which again requires const-evaluating + checking `Y::A::{{constant}}#0`, completing the cycle
2020-04-19T19:11:33.2239391Z note: cycle used when collecting item types in top-level module
2020-04-19T19:11:33.2240056Z    |
2020-04-19T19:11:33.2240179Z LL | enum Y {
2020-04-19T19:11:33.2240332Z    | ^^^^^^
2020-04-19T19:11:33.2240432Z 
---
2020-04-19T19:11:33.2244960Z +    | ^^^^^^^^^^^^^^^^^
2020-04-19T19:11:33.2245102Z 6    |
2020-04-19T19:11:33.2245460Z - note: ...which requires const checking `B`...
2020-04-19T19:11:33.2248545Z -   --> $DIR/issue-23302-3.rs:3:16
2020-04-19T19:11:33.2248976Z + note: ...which requires const-evaluating + checking `A`...
2020-04-19T19:11:33.2249794Z 9    |
2020-04-19T19:11:33.2249794Z 9    |
2020-04-19T19:11:33.2250091Z - LL | const B: i32 = A;
2020-04-19T19:11:33.2250861Z -    = note: ...which again requires const checking `A`, completing the cycle
2020-04-19T19:11:33.2251287Z - note: cycle used when processing `A`
2020-04-19T19:11:33.2251487Z + LL | const A: i32 = B;
2020-04-19T19:11:33.2251678Z +    | ^^^^^^^^^^^^^^^^^
2020-04-19T19:11:33.2251678Z +    | ^^^^^^^^^^^^^^^^^
2020-04-19T19:11:33.2252052Z + note: ...which requires const-evaluating `A`...
2020-04-19T19:11:33.2252566Z 14   --> $DIR/issue-23302-3.rs:1:1
2020-04-19T19:11:33.2252752Z 15    |
2020-04-19T19:11:33.2252966Z 16 LL | const A: i32 = B;
2020-04-19T19:11:33.2253099Z 
2020-04-19T19:11:33.2253235Z 17    | ^^^^^^^^^^^^^^^^^
2020-04-19T19:11:33.2253460Z +    = note: ...which requires normalizing `B`...
2020-04-19T19:11:33.2253907Z + note: ...which requires const-evaluating + checking `B`...
2020-04-19T19:11:33.2254486Z +    |
2020-04-19T19:11:33.2254486Z +    |
2020-04-19T19:11:33.2254635Z + LL | const B: i32 = A;
2020-04-19T19:11:33.2254812Z +    | ^^^^^^^^^^^^^^^^^
2020-04-19T19:11:33.2255223Z + note: ...which requires const-evaluating + checking `B`...
2020-04-19T19:11:33.2255788Z +    |
2020-04-19T19:11:33.2255788Z +    |
2020-04-19T19:11:33.2255952Z + LL | const B: i32 = A;
2020-04-19T19:11:33.2256127Z +    | ^^^^^^^^^^^^^^^^^
2020-04-19T19:11:33.2256498Z + note: ...which requires const-evaluating `B`...
2020-04-19T19:11:33.2257064Z +    |
2020-04-19T19:11:33.2257064Z +    |
2020-04-19T19:11:33.2257211Z + LL | const B: i32 = A;
2020-04-19T19:11:33.2257612Z +    = note: ...which requires normalizing `A`...
2020-04-19T19:11:33.2257612Z +    = note: ...which requires normalizing `A`...
2020-04-19T19:11:33.2258117Z +    = note: ...which again requires const-evaluating + checking `A`, completing the cycle
2020-04-19T19:11:33.2258450Z +    = note: cycle used when running analysis passes on this crate
2020-04-19T19:11:33.2258831Z 19 error: aborting due to previous error
2020-04-19T19:11:33.2258994Z 20 
2020-04-19T19:11:33.2259097Z 
2020-04-19T19:11:33.2259180Z 
2020-04-19T19:11:33.2259180Z 
2020-04-19T19:11:33.2259354Z The actual stderr differed from the expected stderr.
2020-04-19T19:11:33.2259925Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-3/issue-23302-3.stderr
2020-04-19T19:11:33.2260476Z To update references, rerun the tests and pass the `--bless` flag
2020-04-19T19:11:33.2260979Z To only update this specific test, also pass `--test-args issues/issue-23302-3.rs`
2020-04-19T19:11:33.2261363Z error: 1 errors occurred comparing output.
2020-04-19T19:11:33.2261562Z status: exit code: 1
2020-04-19T19:11:33.2261562Z status: exit code: 1
2020-04-19T19:11:33.2263196Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23302-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-3/auxiliary"
2020-04-19T19:11:33.2264506Z ------------------------------------------
2020-04-19T19:11:33.2264653Z 
2020-04-19T19:11:33.2264975Z ------------------------------------------
2020-04-19T19:11:33.2265149Z stderr:
2020-04-19T19:11:33.2265149Z stderr:
2020-04-19T19:11:33.2265498Z ------------------------------------------
2020-04-19T19:11:33.2265933Z error[E0391]: cycle detected when const-evaluating + checking `A`
2020-04-19T19:11:33.2266393Z   --> /checkout/src/test/ui/issues/issue-23302-3.rs:1:1
2020-04-19T19:11:33.2266604Z    |
2020-04-19T19:11:33.2266961Z LL | const A: i32 = B; //~ ERROR cycle detected
2020-04-19T19:11:33.2267329Z    |
2020-04-19T19:11:33.2267329Z    |
2020-04-19T19:11:33.2267833Z note: ...which requires const-evaluating + checking `A`...
2020-04-19T19:11:33.2268515Z    |
2020-04-19T19:11:33.2268515Z    |
2020-04-19T19:11:33.2268696Z LL | const A: i32 = B; //~ ERROR cycle detected
2020-04-19T19:11:33.2269280Z note: ...which requires const-evaluating `A`...
2020-04-19T19:11:33.2269710Z   --> /checkout/src/test/ui/issues/issue-23302-3.rs:1:1
2020-04-19T19:11:33.2269966Z    |
2020-04-19T19:11:33.2269966Z    |
2020-04-19T19:11:33.2270145Z LL | const A: i32 = B; //~ ERROR cycle detected
2020-04-19T19:11:33.2270606Z    = note: ...which requires normalizing `B`...
2020-04-19T19:11:33.2270606Z    = note: ...which requires normalizing `B`...
2020-04-19T19:11:33.2271041Z note: ...which requires const-evaluating + checking `B`...
2020-04-19T19:11:33.2271692Z    |
2020-04-19T19:11:33.2271835Z LL | const B: i32 = A;
2020-04-19T19:11:33.2272017Z    | ^^^^^^^^^^^^^^^^^
2020-04-19T19:11:33.2272017Z    | ^^^^^^^^^^^^^^^^^
2020-04-19T19:11:33.2272404Z note: ...which requires const-evaluating + checking `B`...
2020-04-19T19:11:33.2273054Z    |
2020-04-19T19:11:33.2273196Z LL | const B: i32 = A;
2020-04-19T19:11:33.2273362Z    | ^^^^^^^^^^^^^^^^^
2020-04-19T19:11:33.2274980Z note: ...which requires const-evaluating `B`...
2020-04-19T19:11:33.2274980Z note: ...which requires const-evaluating `B`...
2020-04-19T19:11:33.2277219Z   --> /checkout/src/test/ui/issues/issue-23302-3.rs:3:1
2020-04-19T19:11:33.2277479Z    |
2020-04-19T19:11:33.2277658Z LL | const B: i32 = A;
2020-04-19T19:11:33.2277846Z    | ^^^^^^^^^^^^^^^^^
2020-04-19T19:11:33.2278063Z    = note: ...which requires normalizing `A`...
2020-04-19T19:11:33.2278677Z    = note: ...which again requires const-evaluating + checking `A`, completing the cycle
2020-04-19T19:11:33.2279056Z    = note: cycle used when running analysis passes on this crate
2020-04-19T19:11:33.2279420Z error: aborting due to previous error
2020-04-19T19:11:33.2279585Z 
2020-04-19T19:11:33.2280179Z For more information about this error, try `rustc --explain E0391`.
2020-04-19T19:11:33.2280377Z 
2020-04-19T19:11:33.2280377Z 
2020-04-19T19:11:33.2280732Z ------------------------------------------
2020-04-19T19:11:33.2280907Z 
2020-04-19T19:11:33.2281000Z 
2020-04-19T19:11:33.2281355Z ---- [ui] ui/issues/issue-36163.rs stdout ----
2020-04-19T19:11:33.2281568Z diff of stderr:
2020-04-19T19:11:33.2281706Z 
2020-04-19T19:11:33.2282166Z - error[E0391]: cycle detected when const checking `Foo::B::{{constant}}#0`
2020-04-19T19:11:33.2283034Z + error[E0391]: cycle detected when const-evaluating + checking `Foo::B::{{constant}}#0`
2020-04-19T19:11:33.2284467Z 3    |
2020-04-19T19:11:33.2284614Z 4 LL |     B = A,
2020-04-19T19:11:33.2284732Z 
2020-04-19T19:11:33.2284881Z 5    |         ^
2020-04-19T19:11:33.2284881Z 5    |         ^
2020-04-19T19:11:33.2285022Z 6    |
2020-04-19T19:11:33.2285406Z - note: ...which requires const checking `A`...
2020-04-19T19:11:33.2286016Z -   --> $DIR/issue-36163.rs:1:18
2020-04-19T19:11:33.2286553Z + note: ...which requires const-evaluating + checking `Foo::B::{{constant}}#0`...
2020-04-19T19:11:33.2287226Z 9    |
2020-04-19T19:11:33.2287226Z 9    |
2020-04-19T19:11:33.2287772Z - LL | const A: isize = Foo::B as isize;
2020-04-19T19:11:33.2288185Z -    |                  ^^^^^^^^^^^^^^^
2020-04-19T19:11:33.2288766Z -    = note: ...which again requires const checking `Foo::B::{{constant}}#0`, completing the cycle
2020-04-19T19:11:33.2289361Z - note: cycle used when processing `Foo::B::{{constant}}#0`
2020-04-19T19:11:33.2289615Z + LL |     B = A,
2020-04-19T19:11:33.2289791Z +    |         ^
2020-04-19T19:11:33.2290258Z + note: ...which requires const-evaluating `Foo::B::{{constant}}#0`...
2020-04-19T19:11:33.2290900Z 15    |
2020-04-19T19:11:33.2291064Z 16 LL |     B = A,
2020-04-19T19:11:33.2291182Z 
2020-04-19T19:11:33.2291317Z 17    |         ^
2020-04-19T19:11:33.2291317Z 17    |         ^
2020-04-19T19:11:33.2291546Z +    = note: ...which requires normalizing `A`...
2020-04-19T19:11:33.2293068Z + note: ...which requires const-evaluating + checking `A`...
2020-04-19T19:11:33.2293737Z +    |
2020-04-19T19:11:33.2293737Z +    |
2020-04-19T19:11:33.2293931Z + LL | const A: isize = Foo::B as isize;
2020-04-19T19:11:33.2294177Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T19:11:33.2294628Z + note: ...which requires const-evaluating + checking `A`...
2020-04-19T19:11:33.2295400Z +    |
2020-04-19T19:11:33.2295400Z +    |
2020-04-19T19:11:33.2295655Z + LL | const A: isize = Foo::B as isize;
2020-04-19T19:11:33.2296504Z + note: ...which requires const-evaluating `A`...
2020-04-19T19:11:33.2296918Z +   --> $DIR/issue-36163.rs:1:1
2020-04-19T19:11:33.2297113Z +    |
2020-04-19T19:11:33.2297113Z +    |
2020-04-19T19:11:33.2303463Z + LL | const A: isize = Foo::B as isize;
2020-04-19T19:11:33.2304000Z +    = note: ...which requires normalizing `A`...
2020-04-19T19:11:33.2304000Z +    = note: ...which requires normalizing `A`...
2020-04-19T19:11:33.2304809Z +    = note: ...which again requires const-evaluating + checking `Foo::B::{{constant}}#0`, completing the cycle
2020-04-19T19:11:33.2305418Z + note: cycle used when collecting item types in top-level module
2020-04-19T19:11:33.2306050Z +    |
2020-04-19T19:11:33.2306050Z +    |
2020-04-19T19:11:33.2306245Z + LL | / const A: isize = Foo::B as isize;
2020-04-19T19:11:33.2306472Z + LL | |
2020-04-19T19:11:33.2306624Z + LL | | enum Foo {
2020-04-19T19:11:33.2306799Z + LL | |     B = A,
2020-04-19T19:11:33.2306966Z + LL | | }
2020-04-19T19:11:33.2307102Z + LL | |
2020-04-19T19:11:33.2307256Z + LL | | fn main() {}
2020-04-19T19:11:33.2307587Z 18 
2020-04-19T19:11:33.2307767Z 19 error: aborting due to previous error
2020-04-19T19:11:33.2307943Z 20 
2020-04-19T19:11:33.2308054Z 
2020-04-19T19:11:33.2308054Z 
2020-04-19T19:11:33.2308145Z 
2020-04-19T19:11:33.2308333Z The actual stderr differed from the expected stderr.
2020-04-19T19:11:33.2309730Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36163/issue-36163.stderr
2020-04-19T19:11:33.2310411Z To update references, rerun the tests and pass the `--bless` flag
2020-04-19T19:11:33.2310953Z To only update this specific test, also pass `--test-args issues/issue-36163.rs`
2020-04-19T19:11:33.2311459Z error: 1 errors occurred comparing output.
2020-04-19T19:11:33.2311684Z status: exit code: 1
2020-04-19T19:11:33.2311684Z status: exit code: 1
2020-04-19T19:11:33.2313433Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-36163.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36163" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36163/auxiliary"
2020-04-19T19:11:33.2316642Z ------------------------------------------
2020-04-19T19:11:33.2316840Z 
2020-04-19T19:11:33.2317214Z ------------------------------------------
2020-04-19T19:11:33.2317403Z stderr:
2020-04-19T19:11:33.2317403Z stderr:
2020-04-19T19:11:33.2317782Z ------------------------------------------
2020-04-19T19:11:33.2318354Z error[E0391]: cycle detected when const-evaluating + checking `Foo::B::{{constant}}#0`
2020-04-19T19:11:33.2319397Z    |
2020-04-19T19:11:33.2319397Z    |
2020-04-19T19:11:33.2319564Z LL |     B = A, //~ ERROR E0391
2020-04-19T19:11:33.2319893Z    |
2020-04-19T19:11:33.2319893Z    |
2020-04-19T19:11:33.2320453Z note: ...which requires const-evaluating + checking `Foo::B::{{constant}}#0`...
2020-04-19T19:11:33.2321207Z    |
2020-04-19T19:11:33.2321207Z    |
2020-04-19T19:11:33.2321369Z LL |     B = A, //~ ERROR E0391
2020-04-19T19:11:33.2321546Z    |         ^
2020-04-19T19:11:33.2322003Z note: ...which requires const-evaluating `Foo::B::{{constant}}#0`...
2020-04-19T19:11:33.2322774Z    |
2020-04-19T19:11:33.2322774Z    |
2020-04-19T19:11:33.2322937Z LL |     B = A, //~ ERROR E0391
2020-04-19T19:11:33.2323480Z    = note: ...which requires normalizing `A`...
2020-04-19T19:11:33.2323480Z    = note: ...which requires normalizing `A`...
2020-04-19T19:11:33.2324015Z note: ...which requires const-evaluating + checking `A`...
2020-04-19T19:11:33.2327519Z    |
2020-04-19T19:11:33.2327712Z LL | const A: isize = Foo::B as isize;
2020-04-19T19:11:33.2327973Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T19:11:33.2327973Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T19:11:33.2329154Z note: ...which requires const-evaluating + checking `A`...
2020-04-19T19:11:33.2329861Z    |
2020-04-19T19:11:33.2330047Z LL | const A: isize = Foo::B as isize;
2020-04-19T19:11:33.2330283Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T19:11:33.2330950Z note: ...which requires const-evaluating `A`...
2020-04-19T19:11:33.2330950Z note: ...which requires const-evaluating `A`...
2020-04-19T19:11:33.2331410Z   --> /checkout/src/test/ui/issues/issue-36163.rs:1:1
2020-04-19T19:11:33.2331615Z    |
2020-04-19T19:11:33.2331815Z LL | const A: isize = Foo::B as isize;
2020-04-19T19:11:33.2332066Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-19T19:11:33.2332312Z    = note: ...which requires normalizing `A`...
2020-04-19T19:11:33.2332931Z    = note: ...which again requires const-evaluating + checking `Foo::B::{{constant}}#0`, completing the cycle
2020-04-19T19:11:33.2333549Z note: cycle used when collecting item types in top-level module
2020-04-19T19:11:33.2334238Z    |
2020-04-19T19:11:33.2334442Z LL | / const A: isize = Foo::B as isize;
2020-04-19T19:11:33.2335215Z LL | |
2020-04-19T19:11:33.2335374Z LL | | enum Foo {
2020-04-19T19:11:33.2335374Z LL | | enum Foo {
2020-04-19T19:11:33.2335582Z LL | |     B = A, //~ ERROR E0391
2020-04-19T19:11:33.2336077Z LL | |
2020-04-19T19:11:33.2336238Z LL | | fn main() {}
2020-04-19T19:11:33.2336404Z    | |____________^
2020-04-19T19:11:33.2336525Z 
---
2020-04-19T19:11:33.2341638Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-19T19:11:33.2342008Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-19T19:11:33.2342236Z 
2020-04-19T19:11:33.2342326Z 
2020-04-19T19:11:33.2346877Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-19T19:11:33.2349708Z 
2020-04-19T19:11:33.2349805Z 
2020-04-19T19:11:33.2350369Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-19T19:11:33.2350721Z Build completed unsuccessfully in 0:56:56
2020-04-19T19:11:33.2350721Z Build completed unsuccessfully in 0:56:56
2020-04-19T19:11:33.2350949Z == clock drift check ==
2020-04-19T19:11:33.2351175Z   local time: Sun Apr 19 19:11:33 UTC 2020
2020-04-19T19:11:33.5717780Z   network time: Sun, 19 Apr 2020 19:11:33 GMT
2020-04-19T19:11:34.2261754Z 
2020-04-19T19:11:34.2261754Z 
2020-04-19T19:11:34.2327307Z ##[error]Bash exited with code '1'.
2020-04-19T19:11:34.2352563Z ##[section]Finishing: Run build
2020-04-19T19:11:34.2397982Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71330/merge to s
2020-04-19T19:11:34.2402130Z Task         : Get sources
2020-04-19T19:11:34.2402393Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T19:11:34.2402624Z Version      : 1.0.0
2020-04-19T19:11:34.2402795Z Author       : Microsoft
2020-04-19T19:11:34.2402795Z Author       : Microsoft
2020-04-19T19:11:34.2403071Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-19T19:11:34.2403362Z ==============================================================================
2020-04-19T19:11:34.5500726Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-19T19:11:34.5553668Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71330/merge to s
2020-04-19T19:11:34.5636837Z Cleaning up task key
2020-04-19T19:11:34.5638013Z Start cleaning up orphan processes.
2020-04-19T19:11:34.5819486Z Terminate orphan process: pid (4373) (python)
2020-04-19T19:11:34.5982581Z ##[section]Finishing: Finalize Job
