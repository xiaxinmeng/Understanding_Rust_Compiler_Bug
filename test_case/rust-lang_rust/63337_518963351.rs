plain
2019-08-07T05:29:58.5730028Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-07T05:29:58.5937508Z ##[command]git config gc.auto 0
2019-08-07T05:29:58.5982448Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-07T05:29:58.6028949Z ##[command]git config --get-all http.proxy
2019-08-07T05:29:58.6156619Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63337/merge:refs/remotes/pull/63337/merge
---
2019-08-07T05:30:33.9684282Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-07T05:30:33.9684310Z 
2019-08-07T05:30:33.9684495Z   git checkout -b <new-branch-name>
2019-08-07T05:30:33.9684519Z 
2019-08-07T05:30:33.9684560Z HEAD is now at 16e0fdfe4 Merge 92d4d58c4e91567743bf7f405bcfb362523959be into 615c46086a994f088c9ed569fc36df229ae115b6
2019-08-07T05:30:33.9869075Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-07T05:30:33.9871862Z ==============================================================================
2019-08-07T05:30:33.9871919Z Task         : Bash
2019-08-07T05:30:33.9871966Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-07T06:33:22.0107675Z .................................................................................................... 1400/8841
2019-08-07T06:33:28.0613008Z .................................................................................................... 1500/8841
2019-08-07T06:33:39.0069965Z ...............................................................................i...............i.... 1600/8841
2019-08-07T06:33:47.1683762Z .................................................................................................... 1700/8841
2019-08-07T06:33:55.4905092Z .................................................................iiiii.............................. 1800/8841
2019-08-07T06:34:15.6852988Z .................................................................................................... 2000/8841
2019-08-07T06:34:18.2459089Z .................................................................................................... 2100/8841
2019-08-07T06:34:21.4025515Z .................................................................................................... 2200/8841
2019-08-07T06:34:29.2559888Z .................................................................................................... 2300/8841
---
2019-08-07T06:38:18.2737074Z ................................................................................F.F..F.............. 5200/8841
2019-08-07T06:38:28.7617713Z ..................................................................................i................. 5300/8841
2019-08-07T06:38:36.7588476Z .................................................................................................... 5400/8841
2019-08-07T06:38:42.4605779Z .................................................................................................... 5500/8841
2019-08-07T06:38:53.8834175Z .........................................................................F...ii...i..ii...........i. 5600/8841
2019-08-07T06:39:19.7064263Z .................................................................................................... 5800/8841
2019-08-07T06:39:24.9213596Z .................................................................................................... 5900/8841
2019-08-07T06:39:24.9213596Z .................................................................................................... 5900/8841
2019-08-07T06:39:30.1966227Z ..............................................................................i..ii................. 6000/8841
2019-08-07T06:40:00.3676709Z .................................................................................................... 6200/8841
2019-08-07T06:40:02.5533139Z .....................i.............................................................................. 6300/8841
2019-08-07T06:40:04.7222430Z ............F..F.............................................................................i...... 6400/8841
2019-08-07T06:40:07.5229834Z .................................................................................................... 6500/8841
---
2019-08-07T06:44:11.4180607Z 
2019-08-07T06:44:11.4223756Z ---- [ui] ui/block-result/consider-removing-last-semi.rs stdout ----
2019-08-07T06:44:11.4223830Z diff of stderr:
2019-08-07T06:44:11.4223881Z 
2019-08-07T06:44:11.4224090Z 4 LL | fn f() -> String {
2019-08-07T06:44:11.4224336Z 5    |    -      ^^^^^^ expected struct `std::string::String`, found ()
2019-08-07T06:44:11.4224634Z 6    |    |
2019-08-07T06:44:11.4224894Z -    |    this function's body doesn't return
2019-08-07T06:44:11.4225096Z +    |    this function's body doesn't return a value
2019-08-07T06:44:11.4225155Z 8 LL |     0u8;
2019-08-07T06:44:11.4225193Z 9 LL |     "bla".to_string();
2019-08-07T06:44:11.4225405Z 10    |                      - help: consider removing this semicolon
2019-08-07T06:44:11.4225436Z 
2019-08-07T06:44:11.4225674Z 18 LL | fn g() -> String {
2019-08-07T06:44:11.4225914Z 19    |    -      ^^^^^^ expected struct `std::string::String`, found ()
2019-08-07T06:44:11.4225958Z 20    |    |
2019-08-07T06:44:11.4226184Z -    |    this function's body doesn't return
2019-08-07T06:44:11.4226403Z +    |    this function's body doesn't return a value
2019-08-07T06:44:11.4226610Z 22 LL |     "this won't work".to_string();
2019-08-07T06:44:11.4226671Z 23 LL |     "removeme".to_string();
2019-08-07T06:44:11.4226918Z 24    |                           - help: consider removing this semicolon
2019-08-07T06:44:11.4226974Z 
2019-08-07T06:44:11.4227149Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4227149Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4227580Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/consider-removing-last-semi/consider-removing-last-semi.stderr
2019-08-07T06:44:11.4228315Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4228838Z To only update this specific test, also pass `--test-args block-result/consider-removing-last-semi.rs`
2019-08-07T06:44:11.4228935Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4228998Z status: exit code: 1
2019-08-07T06:44:11.4228998Z status: exit code: 1
2019-08-07T06:44:11.4229817Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/block-result/consider-removing-last-semi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/consider-removing-last-semi" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/consider-removing-last-semi/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4230313Z ------------------------------------------
2019-08-07T06:44:11.4230358Z 
2019-08-07T06:44:11.4230575Z ------------------------------------------
2019-08-07T06:44:11.4230638Z stderr:
2019-08-07T06:44:11.4230638Z stderr:
2019-08-07T06:44:11.4230847Z ------------------------------------------
2019-08-07T06:44:11.4230894Z error[E0308]: mismatched types
2019-08-07T06:44:11.4231164Z   --> /checkout/src/test/ui/block-result/consider-removing-last-semi.rs:1:11
2019-08-07T06:44:11.4231214Z    |
2019-08-07T06:44:11.4231437Z LL | fn f() -> String {  //~ ERROR mismatched types
2019-08-07T06:44:11.4231710Z    |    -      ^^^^^^ expected struct `std::string::String`, found ()
2019-08-07T06:44:11.4231757Z    |    |
2019-08-07T06:44:11.4231975Z    |    this function's body doesn't return a value
2019-08-07T06:44:11.4232039Z LL |     0u8;
2019-08-07T06:44:11.4232082Z LL |     "bla".to_string();
2019-08-07T06:44:11.4232316Z    |                      - help: consider removing this semicolon
2019-08-07T06:44:11.4232425Z    = note: expected type `std::string::String`
2019-08-07T06:44:11.4232479Z               found type `()`
2019-08-07T06:44:11.4232508Z 
2019-08-07T06:44:11.4232564Z error[E0308]: mismatched types
2019-08-07T06:44:11.4232564Z error[E0308]: mismatched types
2019-08-07T06:44:11.4232811Z   --> /checkout/src/test/ui/block-result/consider-removing-last-semi.rs:6:11
2019-08-07T06:44:11.4232859Z    |
2019-08-07T06:44:11.4233094Z LL | fn g() -> String {  //~ ERROR mismatched types
2019-08-07T06:44:11.4233336Z    |    -      ^^^^^^ expected struct `std::string::String`, found ()
2019-08-07T06:44:11.4233392Z    |    |
2019-08-07T06:44:11.4233613Z    |    this function's body doesn't return a value
2019-08-07T06:44:11.4233845Z LL |     "this won't work".to_string();
2019-08-07T06:44:11.4233891Z LL |     "removeme".to_string();
2019-08-07T06:44:11.4234129Z    |                           - help: consider removing this semicolon
2019-08-07T06:44:11.4234234Z    = note: expected type `std::string::String`
2019-08-07T06:44:11.4234279Z               found type `()`
2019-08-07T06:44:11.4234323Z 
2019-08-07T06:44:11.4234374Z error: aborting due to 2 previous errors
---
2019-08-07T06:44:11.4234964Z 
2019-08-07T06:44:11.4235184Z ---- [ui] ui/block-result/issue-11714.rs stdout ----
2019-08-07T06:44:11.4235248Z diff of stderr:
2019-08-07T06:44:11.4235284Z 
2019-08-07T06:44:11.4235480Z 4 LL | fn blah() -> i32 {
2019-08-07T06:44:11.4235711Z 5    |    ----      ^^^ expected i32, found ()
2019-08-07T06:44:11.4235758Z 6    |    |
2019-08-07T06:44:11.4235972Z -    |    this function's body doesn't return
2019-08-07T06:44:11.4236420Z +    |    this function's body doesn't return a value
2019-08-07T06:44:11.4236631Z 9 LL |     ;
2019-08-07T06:44:11.4236827Z 10    |     - help: consider removing this semicolon
2019-08-07T06:44:11.4236870Z 
2019-08-07T06:44:11.4236968Z 
2019-08-07T06:44:11.4236968Z 
2019-08-07T06:44:11.4237016Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4237307Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/issue-11714/issue-11714.stderr
2019-08-07T06:44:11.4238133Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4238513Z To only update this specific test, also pass `--test-args block-result/issue-11714.rs`
2019-08-07T06:44:11.4238742Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4238789Z status: exit code: 1
2019-08-07T06:44:11.4238789Z status: exit code: 1
2019-08-07T06:44:11.4239581Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/block-result/issue-11714.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/issue-11714" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/issue-11714/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4239932Z ------------------------------------------
2019-08-07T06:44:11.4239984Z 
2019-08-07T06:44:11.4240220Z ------------------------------------------
2019-08-07T06:44:11.4240268Z stderr:
2019-08-07T06:44:11.4240268Z stderr:
2019-08-07T06:44:11.4240500Z ------------------------------------------
2019-08-07T06:44:11.4240575Z error[E0308]: mismatched types
2019-08-07T06:44:11.4240837Z   --> /checkout/src/test/ui/block-result/issue-11714.rs:1:14
2019-08-07T06:44:11.4240888Z    |
2019-08-07T06:44:11.4241256Z LL | fn blah() -> i32 { //~ ERROR mismatched types
2019-08-07T06:44:11.4241594Z    |    ----      ^^^ expected i32, found ()
2019-08-07T06:44:11.4241637Z    |    |
2019-08-07T06:44:11.4241859Z    |    this function's body doesn't return a value
2019-08-07T06:44:11.4241949Z LL |     ;
2019-08-07T06:44:11.4242172Z    |     - help: consider removing this semicolon
2019-08-07T06:44:11.4242214Z    |
2019-08-07T06:44:11.4242253Z    = note: expected type `i32`
---
2019-08-07T06:44:11.4243366Z 
2019-08-07T06:44:11.4243555Z 4 LL | fn foo() -> String {
2019-08-07T06:44:11.4243806Z 5    |    ---      ^^^^^^ expected struct `std::string::String`, found ()
2019-08-07T06:44:11.4243853Z 6    |    |
2019-08-07T06:44:11.4244068Z -    |    this function's body doesn't return
2019-08-07T06:44:11.4244305Z +    |    this function's body doesn't return a value
2019-08-07T06:44:11.4244389Z 9 LL |     ;
2019-08-07T06:44:11.4246142Z 10    |     - help: consider removing this semicolon
2019-08-07T06:44:11.4246201Z 
2019-08-07T06:44:11.4246201Z 
2019-08-07T06:44:11.4246389Z 18 LL | fn bar() -> String {
2019-08-07T06:44:11.4246602Z 19    |    ---      ^^^^^^ expected struct `std::string::String`, found ()
2019-08-07T06:44:11.4246673Z 20    |    |
2019-08-07T06:44:11.4246865Z -    |    this function's body doesn't return
2019-08-07T06:44:11.4247061Z +    |    this function's body doesn't return a value
2019-08-07T06:44:11.4247120Z 22 LL |     "foobar".to_string()
2019-08-07T06:44:11.4247158Z 23 LL |     ;
2019-08-07T06:44:11.4247351Z 24    |     - help: consider removing this semicolon
2019-08-07T06:44:11.4247519Z 
2019-08-07T06:44:11.4247575Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4248358Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/issue-13428/issue-13428.stderr
2019-08-07T06:44:11.4248358Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/issue-13428/issue-13428.stderr
2019-08-07T06:44:11.4248666Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4248957Z To only update this specific test, also pass `--test-args block-result/issue-13428.rs`
2019-08-07T06:44:11.4249037Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4249099Z status: exit code: 1
2019-08-07T06:44:11.4249099Z status: exit code: 1
2019-08-07T06:44:11.4249952Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/block-result/issue-13428.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/issue-13428" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/issue-13428/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4250278Z ------------------------------------------
2019-08-07T06:44:11.4250312Z 
2019-08-07T06:44:11.4250543Z ------------------------------------------
2019-08-07T06:44:11.4250587Z stderr:
2019-08-07T06:44:11.4250587Z stderr:
2019-08-07T06:44:11.4250797Z ------------------------------------------
2019-08-07T06:44:11.4250859Z error[E0308]: mismatched types
2019-08-07T06:44:11.4251105Z   --> /checkout/src/test/ui/block-result/issue-13428.rs:3:13
2019-08-07T06:44:11.4251274Z    |
2019-08-07T06:44:11.4251470Z LL | fn foo() -> String {  //~ ERROR mismatched types
2019-08-07T06:44:11.4251700Z    |    ---      ^^^^^^ expected struct `std::string::String`, found ()
2019-08-07T06:44:11.4251741Z    |    |
2019-08-07T06:44:11.4251931Z    |    this function's body doesn't return a value
2019-08-07T06:44:11.4252019Z LL |     ;
2019-08-07T06:44:11.4252217Z    |     - help: consider removing this semicolon
2019-08-07T06:44:11.4252273Z    |
2019-08-07T06:44:11.4252311Z    = note: expected type `std::string::String`
2019-08-07T06:44:11.4252311Z    = note: expected type `std::string::String`
2019-08-07T06:44:11.4252350Z               found type `()`
2019-08-07T06:44:11.4252375Z 
2019-08-07T06:44:11.4252428Z error[E0308]: mismatched types
2019-08-07T06:44:11.4252760Z   --> /checkout/src/test/ui/block-result/issue-13428.rs:11:13
2019-08-07T06:44:11.4252804Z    |
2019-08-07T06:44:11.4253026Z LL | fn bar() -> String {  //~ ERROR mismatched types
2019-08-07T06:44:11.4253262Z    |    ---      ^^^^^^ expected struct `std::string::String`, found ()
2019-08-07T06:44:11.4253305Z    |    |
2019-08-07T06:44:11.4253524Z    |    this function's body doesn't return a value
2019-08-07T06:44:11.4253569Z LL |     "foobar".to_string()
2019-08-07T06:44:11.4253608Z LL |     ;
2019-08-07T06:44:11.4253811Z    |     - help: consider removing this semicolon
2019-08-07T06:44:11.4253909Z    = note: expected type `std::string::String`
2019-08-07T06:44:11.4253959Z               found type `()`
2019-08-07T06:44:11.4254001Z 
2019-08-07T06:44:11.4254041Z error: aborting due to 2 previous errors
---
2019-08-07T06:44:11.4255020Z 
2019-08-07T06:44:11.4255059Z 25 error[E0308]: mismatched types
2019-08-07T06:44:11.4255285Z 26   --> $DIR/variadic-ffi-1.rs:19:56
2019-08-07T06:44:11.4255328Z 27    |
2019-08-07T06:44:11.4255369Z + LL |     fn foo(f: isize, x: u8, ...);
2019-08-07T06:44:11.4255658Z +    |     ----------------------------- for<'r> unsafe extern "C" fn(isize, u8, std::ffi::VaListImpl<'r>, ...) {foo} defined here
2019-08-07T06:44:11.4255727Z + ...
2019-08-07T06:44:11.4255850Z 28 LL |         let x: unsafe extern "C" fn(f: isize, x: u8) = foo;
2019-08-07T06:44:11.4256184Z 29    |                                                        ^^^ expected non-variadic fn, found variadic function
2019-08-07T06:44:11.4256260Z 
2019-08-07T06:44:11.4256300Z 34 error[E0308]: mismatched types
2019-08-07T06:44:11.4256523Z 35   --> $DIR/variadic-ffi-1.rs:20:54
2019-08-07T06:44:11.4256565Z 36    |
2019-08-07T06:44:11.4256565Z 36    |
2019-08-07T06:44:11.4256686Z + LL | extern "C" fn bar(f: isize, x: u8) {}
2019-08-07T06:44:11.4256978Z +    | ------------------------------------- extern "C" fn(isize, u8) {bar} defined here
2019-08-07T06:44:11.4257026Z + ...
2019-08-07T06:44:11.4257070Z 37 LL |         let y: extern "C" fn(f: isize, x: u8, ...) = bar;
2019-08-07T06:44:11.4259569Z 38    |                                                      ^^^ expected variadic fn, found non-variadic function
2019-08-07T06:44:11.4259669Z 
2019-08-07T06:44:11.4259694Z 
2019-08-07T06:44:11.4259768Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4259768Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4260080Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-1/variadic-ffi-1.stderr
2019-08-07T06:44:11.4260324Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4260626Z To only update this specific test, also pass `--test-args c-variadic/variadic-ffi-1.rs`
2019-08-07T06:44:11.4260717Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4260765Z status: exit code: 1
2019-08-07T06:44:11.4260765Z status: exit code: 1
2019-08-07T06:44:11.4261704Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-variadic/variadic-ffi-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-1/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4262129Z ------------------------------------------
2019-08-07T06:44:11.4262159Z 
2019-08-07T06:44:11.4262361Z ------------------------------------------
2019-08-07T06:44:11.4262538Z stderr:
2019-08-07T06:44:11.4262538Z stderr:
2019-08-07T06:44:11.4262744Z ------------------------------------------
2019-08-07T06:44:11.4262992Z error[E0045]: C-variadic function must have C or cdecl calling convention
2019-08-07T06:44:11.4263240Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:5:5
2019-08-07T06:44:11.4263287Z    |
2019-08-07T06:44:11.4263336Z LL |     fn printf(_: *const u8, ...); //~ ERROR: variadic function must have C or cdecl calling
2019-08-07T06:44:11.4263605Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ C-variadics require C or cdecl calling convention
2019-08-07T06:44:11.4263641Z 
2019-08-07T06:44:11.4263695Z error[E0060]: this function takes at least 2 parameters but 0 parameters were supplied
2019-08-07T06:44:11.4263942Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:16:9
2019-08-07T06:44:11.4263988Z    |
2019-08-07T06:44:11.4264027Z LL |     fn foo(f: isize, x: u8, ...);
2019-08-07T06:44:11.4264243Z    |     ----------------------------- defined here
2019-08-07T06:44:11.4264302Z ...
2019-08-07T06:44:11.4264352Z LL |         foo();  //~ ERROR this function takes at least 2 parameters but 0 parameters were supplied
2019-08-07T06:44:11.4264593Z    |         ^^^^^ expected at least 2 parameters
2019-08-07T06:44:11.4264649Z 
2019-08-07T06:44:11.4264696Z error[E0060]: this function takes at least 2 parameters but 1 parameter was supplied
2019-08-07T06:44:11.4264971Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:17:9
2019-08-07T06:44:11.4265033Z    |
2019-08-07T06:44:11.4265074Z LL |     fn foo(f: isize, x: u8, ...);
2019-08-07T06:44:11.4265290Z    |     ----------------------------- defined here
2019-08-07T06:44:11.4265584Z ...
2019-08-07T06:44:11.4265668Z LL |         foo(1); //~ ERROR this function takes at least 2 parameters but 1 parameter was supplied
2019-08-07T06:44:11.4265719Z    |         ^^^^^^ expected at least 2 parameters
2019-08-07T06:44:11.4265804Z error[E0308]: mismatched types
2019-08-07T06:44:11.4266088Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:19:56
2019-08-07T06:44:11.4266186Z    |
2019-08-07T06:44:11.4266186Z    |
2019-08-07T06:44:11.4266341Z LL |     fn foo(f: isize, x: u8, ...);
2019-08-07T06:44:11.4266676Z    |     ----------------------------- for<'r> unsafe extern "C" fn(isize, u8, std::ffi::VaListImpl<'r>, ...) {foo} defined here
2019-08-07T06:44:11.4266732Z ...
2019-08-07T06:44:11.4266803Z LL |         let x: unsafe extern "C" fn(f: isize, x: u8) = foo; //~ ERROR mismatched types
2019-08-07T06:44:11.4267090Z    |                                                        ^^^ expected non-variadic fn, found variadic function
2019-08-07T06:44:11.4267139Z    |
2019-08-07T06:44:11.4267209Z    = note: expected type `unsafe extern "C" fn(isize, u8)`
2019-08-07T06:44:11.4267484Z               found type `for<'r> unsafe extern "C" fn(isize, u8, std::ffi::VaListImpl<'r>, ...) {foo}`
2019-08-07T06:44:11.4267561Z error[E0308]: mismatched types
2019-08-07T06:44:11.4268197Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:20:54
2019-08-07T06:44:11.4268252Z    |
2019-08-07T06:44:11.4268252Z    |
2019-08-07T06:44:11.4268295Z LL | extern "C" fn bar(f: isize, x: u8) {}
2019-08-07T06:44:11.4268589Z    | ------------------------------------- extern "C" fn(isize, u8) {bar} defined here
2019-08-07T06:44:11.4268637Z ...
2019-08-07T06:44:11.4268686Z LL |         let y: extern "C" fn(f: isize, x: u8, ...) = bar; //~ ERROR mismatched types
2019-08-07T06:44:11.4268982Z    |                                                      ^^^ expected variadic fn, found non-variadic function
2019-08-07T06:44:11.4269031Z    |
2019-08-07T06:44:11.4269294Z    = note: expected type `for<'r> extern "C" fn(isize, u8, std::ffi::VaListImpl<'r>, ...)`
2019-08-07T06:44:11.4269365Z               found type `extern "C" fn(isize, u8) {bar}`
2019-08-07T06:44:11.4269396Z 
2019-08-07T06:44:11.4269621Z error[E0617]: can't pass `f32` to variadic function
2019-08-07T06:44:11.4269857Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:22:19
2019-08-07T06:44:11.4269918Z    |
2019-08-07T06:44:11.4270138Z LL |         foo(1, 2, 3f32); //~ ERROR can't pass
2019-08-07T06:44:11.4270192Z    |                   ^^^^ help: cast the value to `c_double`: `3f32 as c_double`
2019-08-07T06:44:11.4270249Z 
2019-08-07T06:44:11.4270473Z error[E0617]: can't pass `bool` to variadic function
2019-08-07T06:44:11.4270707Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:23:19
2019-08-07T06:44:11.4270753Z    |
2019-08-07T06:44:11.4270988Z LL |         foo(1, 2, true); //~ ERROR can't pass
2019-08-07T06:44:11.4271042Z    |                   ^^^^ help: cast the value to `c_int`: `true as c_int`
2019-08-07T06:44:11.4271074Z 
2019-08-07T06:44:11.4271536Z error[E0617]: can't pass `i8` to variadic function
2019-08-07T06:44:11.4271747Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:24:19
2019-08-07T06:44:11.4271786Z    |
2019-08-07T06:44:11.4271991Z LL |         foo(1, 2, 1i8);  //~ ERROR can't pass
2019-08-07T06:44:11.4272037Z    |                   ^^^ help: cast the value to `c_int`: `1i8 as c_int`
2019-08-07T06:44:11.4272065Z 
2019-08-07T06:44:11.4272259Z error[E0617]: can't pass `u8` to variadic function
2019-08-07T06:44:11.4272537Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:25:19
2019-08-07T06:44:11.4272578Z    |
2019-08-07T06:44:11.4272770Z LL |         foo(1, 2, 1u8);  //~ ERROR can't pass
2019-08-07T06:44:11.4272831Z    |                   ^^^ help: cast the value to `c_uint`: `1u8 as c_uint`
2019-08-07T06:44:11.4272859Z 
2019-08-07T06:44:11.4273052Z error[E0617]: can't pass `i16` to variadic function
2019-08-07T06:44:11.4273255Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:26:19
2019-08-07T06:44:11.4273309Z    |
2019-08-07T06:44:11.4273611Z LL |         foo(1, 2, 1i16); //~ ERROR can't pass
2019-08-07T06:44:11.4273667Z    |                   ^^^^ help: cast the value to `c_int`: `1i16 as c_int`
2019-08-07T06:44:11.4273711Z 
2019-08-07T06:44:11.4273941Z error[E0617]: can't pass `u16` to variadic function
2019-08-07T06:44:11.4274149Z   --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:27:19
2019-08-07T06:44:11.4274187Z    |
2019-08-07T06:44:11.4274394Z LL |         foo(1, 2, 1u16); //~ ERROR can't pass
2019-08-07T06:44:11.4274516Z    |                   ^^^^ help: cast the value to `c_uint`: `1u16 as c_uint`
2019-08-07T06:44:11.4274597Z error: aborting due to 11 previous errors
2019-08-07T06:44:11.4274621Z 
2019-08-07T06:44:11.4274661Z Some errors have detailed explanations: E0045, E0060, E0308, E0617.
2019-08-07T06:44:11.4274901Z For more information about an error, try `rustc --explain E0045`.
2019-08-07T06:44:11.4274901Z For more information about an error, try `rustc --explain E0045`.
2019-08-07T06:44:11.4274945Z 
2019-08-07T06:44:11.4275136Z ------------------------------------------
2019-08-07T06:44:11.4275164Z 
2019-08-07T06:44:11.4275186Z 
2019-08-07T06:44:11.4275412Z ---- [ui] ui/coercion/coercion-missing-tail-expected-type.rs stdout ----
2019-08-07T06:44:11.4275454Z diff of stderr:
2019-08-07T06:44:11.4275479Z 
2019-08-07T06:44:11.4275661Z 4 LL | fn plus_one(x: i32) -> i32 {
2019-08-07T06:44:11.4275877Z 5    |    --------            ^^^ expected i32, found ()
2019-08-07T06:44:11.4275917Z 6    |    |
2019-08-07T06:44:11.4276104Z -    |    this function's body doesn't return
2019-08-07T06:44:11.4276348Z +    |    this function's body doesn't return a value
2019-08-07T06:44:11.4276391Z 8 LL |     x + 1;
2019-08-07T06:44:11.4276611Z 9    |          - help: consider removing this semicolon
2019-08-07T06:44:11.4276694Z 
2019-08-07T06:44:11.4276694Z 
2019-08-07T06:44:11.4276896Z 17 LL | fn foo() -> Result<u8, u64> {
2019-08-07T06:44:11.4277135Z 18    |    ---      ^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-08-07T06:44:11.4277195Z 19    |    |
2019-08-07T06:44:11.4277409Z -    |    this function's body doesn't return
2019-08-07T06:44:11.4277627Z +    |    this function's body doesn't return a value
2019-08-07T06:44:11.4278019Z 21 LL |     Ok(1);
2019-08-07T06:44:11.4278309Z 22    |          - help: consider removing this semicolon
2019-08-07T06:44:11.4278381Z 
2019-08-07T06:44:11.4278408Z 
2019-08-07T06:44:11.4278468Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4278817Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coercion-missing-tail-expected-type/coercion-missing-tail-expected-type.stderr
2019-08-07T06:44:11.4278817Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coercion-missing-tail-expected-type/coercion-missing-tail-expected-type.stderr
2019-08-07T06:44:11.4279082Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4279363Z To only update this specific test, also pass `--test-args coercion/coercion-missing-tail-expected-type.rs`
2019-08-07T06:44:11.4279464Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4279516Z status: exit code: 1
2019-08-07T06:44:11.4279516Z status: exit code: 1
2019-08-07T06:44:11.4280286Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coercion/coercion-missing-tail-expected-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coercion-missing-tail-expected-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coercion-missing-tail-expected-type/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4280629Z ------------------------------------------
2019-08-07T06:44:11.4280677Z 
2019-08-07T06:44:11.4280890Z ------------------------------------------
2019-08-07T06:44:11.4280933Z stderr:
2019-08-07T06:44:11.4280933Z stderr:
2019-08-07T06:44:11.4281259Z ------------------------------------------
2019-08-07T06:44:11.4284939Z error[E0308]: mismatched types
2019-08-07T06:44:11.4285409Z   --> /checkout/src/test/ui/coercion/coercion-missing-tail-expected-type.rs:3:24
2019-08-07T06:44:11.4285460Z    |
2019-08-07T06:44:11.4285723Z LL | fn plus_one(x: i32) -> i32 { //~ ERROR mismatched types
2019-08-07T06:44:11.4285951Z    |    --------            ^^^ expected i32, found ()
2019-08-07T06:44:11.4285997Z    |    |
2019-08-07T06:44:11.4286233Z    |    this function's body doesn't return a value
2019-08-07T06:44:11.4286380Z LL |     x + 1;
2019-08-07T06:44:11.4286630Z    |          - help: consider removing this semicolon
2019-08-07T06:44:11.4286736Z    = note: expected type `i32`
2019-08-07T06:44:11.4286779Z               found type `()`
2019-08-07T06:44:11.4286808Z 
2019-08-07T06:44:11.4286864Z error[E0308]: mismatched types
2019-08-07T06:44:11.4286864Z error[E0308]: mismatched types
2019-08-07T06:44:11.4287115Z   --> /checkout/src/test/ui/coercion/coercion-missing-tail-expected-type.rs:7:13
2019-08-07T06:44:11.4287162Z    |
2019-08-07T06:44:11.4287420Z LL | fn foo() -> Result<u8, u64> { //~ ERROR mismatched types
2019-08-07T06:44:11.4288244Z    |    ---      ^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-08-07T06:44:11.4288297Z    |    |
2019-08-07T06:44:11.4288538Z    |    this function's body doesn't return a value
2019-08-07T06:44:11.4288584Z LL |     Ok(1);
2019-08-07T06:44:11.4288807Z    |          - help: consider removing this semicolon
2019-08-07T06:44:11.4288852Z    |
2019-08-07T06:44:11.4288924Z    = note: expected type `std::result::Result<u8, u64>`
2019-08-07T06:44:11.4288997Z 
2019-08-07T06:44:11.4289055Z error: aborting due to 2 previous errors
2019-08-07T06:44:11.4289083Z 
2019-08-07T06:44:11.4289323Z For more information about this error, try `rustc --explain E0308`.
---
2019-08-07T06:44:11.4289946Z 
2019-08-07T06:44:11.4289987Z 1 error[E0308]: mismatched types
2019-08-07T06:44:11.4290198Z 2   --> $DIR/fn-item-type.rs:13:19
2019-08-07T06:44:11.4290258Z 3    |
2019-08-07T06:44:11.4290473Z + LL | fn bar<T>(x: isize) -> isize { x * 4 }
2019-08-07T06:44:11.4290837Z +    | -------------------------------------- fn(isize) -> isize {bar::<u8>} defined here
2019-08-07T06:44:11.4291015Z + ...
2019-08-07T06:44:11.4291056Z 4 LL |     eq(foo::<u8>, bar::<u8>);
2019-08-07T06:44:11.4291222Z 5    |                   ^^^^^^^^^ expected fn item, found a different fn item
2019-08-07T06:44:11.4291295Z 
2019-08-07T06:44:11.4291331Z 10 error[E0308]: mismatched types
2019-08-07T06:44:11.4291519Z 11   --> $DIR/fn-item-type.rs:19:19
2019-08-07T06:44:11.4291557Z 12    |
2019-08-07T06:44:11.4291557Z 12    |
2019-08-07T06:44:11.4291759Z + LL | fn foo<T>(x: isize) -> isize { x * 2 }
2019-08-07T06:44:11.4291990Z +    | -------------------------------------- fn(isize) -> isize {foo::<i8>} defined here
2019-08-07T06:44:11.4292032Z + ...
2019-08-07T06:44:11.4292085Z 13 LL |     eq(foo::<u8>, foo::<i8>);
2019-08-07T06:44:11.4292126Z 14    |                   ^^^^^^^^^ expected u8, found i8
2019-08-07T06:44:11.4292202Z 
2019-08-07T06:44:11.4292238Z 19 error[E0308]: mismatched types
2019-08-07T06:44:11.4292424Z 20   --> $DIR/fn-item-type.rs:23:23
2019-08-07T06:44:11.4292468Z 21    |
2019-08-07T06:44:11.4292468Z 21    |
2019-08-07T06:44:11.4292671Z + LL | fn bar<T>(x: isize) -> isize { x * 4 }
2019-08-07T06:44:11.4292907Z +    | -------------------------------------- fn(isize) -> isize {bar::<std::vec::Vec<u8>>} defined here
2019-08-07T06:44:11.4292952Z + ...
2019-08-07T06:44:11.4293038Z 22 LL |     eq(bar::<String>, bar::<Vec<u8>>);
2019-08-07T06:44:11.4293086Z 23    |                       ^^^^^^^^^^^^^^ expected struct `std::string::String`, found struct `std::vec::Vec`
2019-08-07T06:44:11.4293166Z 
2019-08-07T06:44:11.4293295Z 28 error[E0308]: mismatched types
2019-08-07T06:44:11.4293521Z 29   --> $DIR/fn-item-type.rs:30:26
2019-08-07T06:44:11.4293675Z 30    |
2019-08-07T06:44:11.4293675Z 30    |
2019-08-07T06:44:11.4293736Z + LL | trait Foo { fn foo() { /* this is a default fn */ } }
2019-08-07T06:44:11.4293995Z +    |             --------------------------------------- fn() {<u16 as Foo>::foo} defined here
2019-08-07T06:44:11.4294040Z + ...
2019-08-07T06:44:11.4294097Z 31 LL |     eq(<u8 as Foo>::foo, <u16 as Foo>::foo);
2019-08-07T06:44:11.4294219Z 32    |                          ^^^^^^^^^^^^^^^^^ expected u8, found u16
2019-08-07T06:44:11.4294301Z 
2019-08-07T06:44:11.4294325Z 
2019-08-07T06:44:11.4294366Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4294658Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-item-type/fn-item-type.stderr
2019-08-07T06:44:11.4294658Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-item-type/fn-item-type.stderr
2019-08-07T06:44:11.4294902Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4295142Z To only update this specific test, also pass `--test-args fn/fn-item-type.rs`
2019-08-07T06:44:11.4295229Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4295270Z status: exit code: 1
2019-08-07T06:44:11.4295270Z status: exit code: 1
2019-08-07T06:44:11.4296025Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fn/fn-item-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-item-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-item-type/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4296320Z ------------------------------------------
2019-08-07T06:44:11.4296349Z 
2019-08-07T06:44:11.4296562Z ------------------------------------------
2019-08-07T06:44:11.4296600Z stderr:
2019-08-07T06:44:11.4296600Z stderr:
2019-08-07T06:44:11.4296783Z ------------------------------------------
2019-08-07T06:44:11.4296839Z error[E0308]: mismatched types
2019-08-07T06:44:11.4297035Z   --> /checkout/src/test/ui/fn/fn-item-type.rs:13:19
2019-08-07T06:44:11.4297074Z    |
2019-08-07T06:44:11.4297273Z LL | fn bar<T>(x: isize) -> isize { x * 4 }
2019-08-07T06:44:11.4297497Z    | -------------------------------------- fn(isize) -> isize {bar::<u8>} defined here
2019-08-07T06:44:11.4297549Z ...
2019-08-07T06:44:11.4297603Z LL |     eq(foo::<u8>, bar::<u8>);
2019-08-07T06:44:11.4297948Z    |                   ^^^^^^^^^ expected fn item, found a different fn item
2019-08-07T06:44:11.4298003Z    |
2019-08-07T06:44:11.4298286Z    = note: expected type `fn(isize) -> isize {foo::<u8>}`
2019-08-07T06:44:11.4298537Z               found type `fn(isize) -> isize {bar::<u8>}`
2019-08-07T06:44:11.4298611Z error[E0308]: mismatched types
2019-08-07T06:44:11.4298861Z   --> /checkout/src/test/ui/fn/fn-item-type.rs:19:19
2019-08-07T06:44:11.4298907Z    |
2019-08-07T06:44:11.4298907Z    |
2019-08-07T06:44:11.4299121Z LL | fn foo<T>(x: isize) -> isize { x * 2 }
2019-08-07T06:44:11.4299392Z    | -------------------------------------- fn(isize) -> isize {foo::<i8>} defined here
2019-08-07T06:44:11.4299441Z ...
2019-08-07T06:44:11.4299482Z LL |     eq(foo::<u8>, foo::<i8>);
2019-08-07T06:44:11.4299528Z    |                   ^^^^^^^^^ expected u8, found i8
2019-08-07T06:44:11.4299584Z    |
2019-08-07T06:44:11.4299821Z    = note: expected type `fn(isize) -> isize {foo::<u8>}`
2019-08-07T06:44:11.4300049Z               found type `fn(isize) -> isize {foo::<i8>}`
2019-08-07T06:44:11.4300138Z error[E0308]: mismatched types
2019-08-07T06:44:11.4300362Z   --> /checkout/src/test/ui/fn/fn-item-type.rs:23:23
2019-08-07T06:44:11.4300408Z    |
2019-08-07T06:44:11.4300408Z    |
2019-08-07T06:44:11.4300636Z LL | fn bar<T>(x: isize) -> isize { x * 4 }
2019-08-07T06:44:11.4301006Z    | -------------------------------------- fn(isize) -> isize {bar::<std::vec::Vec<u8>>} defined here
2019-08-07T06:44:11.4301170Z ...
2019-08-07T06:44:11.4301228Z LL |     eq(bar::<String>, bar::<Vec<u8>>);
2019-08-07T06:44:11.4301400Z    |                       ^^^^^^^^^^^^^^ expected struct `std::string::String`, found struct `std::vec::Vec`
2019-08-07T06:44:11.4301440Z    |
2019-08-07T06:44:11.4301697Z    = note: expected type `fn(isize) -> isize {bar::<std::string::String>}`
2019-08-07T06:44:11.4301911Z               found type `fn(isize) -> isize {bar::<std::vec::Vec<u8>>}`
2019-08-07T06:44:11.4302067Z error[E0308]: mismatched types
2019-08-07T06:44:11.4302291Z   --> /checkout/src/test/ui/fn/fn-item-type.rs:30:26
2019-08-07T06:44:11.4302331Z    |
2019-08-07T06:44:11.4302331Z    |
2019-08-07T06:44:11.4302369Z LL | trait Foo { fn foo() { /* this is a default fn */ } }
2019-08-07T06:44:11.4302616Z    |             --------------------------------------- fn() {<u16 as Foo>::foo} defined here
2019-08-07T06:44:11.4302657Z ...
2019-08-07T06:44:11.4302704Z LL |     eq(<u8 as Foo>::foo, <u16 as Foo>::foo);
2019-08-07T06:44:11.4302763Z    |                          ^^^^^^^^^^^^^^^^^ expected u8, found u16
2019-08-07T06:44:11.4302800Z    |
2019-08-07T06:44:11.4302838Z    = note: expected type `fn() {<u8 as Foo>::foo}`
2019-08-07T06:44:11.4302894Z               found type `fn() {<u16 as Foo>::foo}`
2019-08-07T06:44:11.4302957Z error: aborting due to 4 previous errors
2019-08-07T06:44:11.4302981Z 
2019-08-07T06:44:11.4303216Z For more information about this error, try `rustc --explain E0308`.
2019-08-07T06:44:11.4303245Z 
---
2019-08-07T06:44:11.4303743Z 
2019-08-07T06:44:11.4303795Z 1 error[E0308]: mismatched types
2019-08-07T06:44:11.4303977Z 2   --> $DIR/issue-10764.rs:4:15
2019-08-07T06:44:11.4304024Z 3    |
2019-08-07T06:44:11.4304060Z + LL | extern fn bar() {}
2019-08-07T06:44:11.4304278Z +    | ------------------ extern "C" fn() {bar} defined here
2019-08-07T06:44:11.4304318Z + LL | 
2019-08-07T06:44:11.4304353Z 4 LL | fn main() { f(bar) }
2019-08-07T06:44:11.4304602Z 5    |               ^^^ expected "Rust" fn, found "C" fn
2019-08-07T06:44:11.4304669Z 
2019-08-07T06:44:11.4304691Z 
2019-08-07T06:44:11.4304744Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4305057Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10764/issue-10764.stderr
2019-08-07T06:44:11.4305057Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10764/issue-10764.stderr
2019-08-07T06:44:11.4305273Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4305540Z To only update this specific test, also pass `--test-args issues/issue-10764.rs`
2019-08-07T06:44:11.4305612Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4305653Z status: exit code: 1
2019-08-07T06:44:11.4305653Z status: exit code: 1
2019-08-07T06:44:11.4306306Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-10764.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10764" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10764/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4306615Z ------------------------------------------
2019-08-07T06:44:11.4306644Z 
2019-08-07T06:44:11.4306848Z ------------------------------------------
2019-08-07T06:44:11.4306905Z stderr:
2019-08-07T06:44:11.4306905Z stderr:
2019-08-07T06:44:11.4307107Z ------------------------------------------
2019-08-07T06:44:11.4307150Z error[E0308]: mismatched types
2019-08-07T06:44:11.4308074Z   --> /checkout/src/test/ui/issues/issue-10764.rs:4:15
2019-08-07T06:44:11.4308152Z    |
2019-08-07T06:44:11.4308194Z LL | extern fn bar() {}
2019-08-07T06:44:11.4308527Z    | ------------------ extern "C" fn() {bar} defined here
2019-08-07T06:44:11.4308579Z LL | 
2019-08-07T06:44:11.4308626Z LL | fn main() { f(bar) }
2019-08-07T06:44:11.4308672Z    |               ^^^ expected "Rust" fn, found "C" fn
2019-08-07T06:44:11.4308771Z    = note: expected type `fn()`
2019-08-07T06:44:11.4308771Z    = note: expected type `fn()`
2019-08-07T06:44:11.4308924Z               found type `extern "C" fn() {bar}`
2019-08-07T06:44:11.4309012Z error: aborting due to previous error
2019-08-07T06:44:11.4309040Z 
2019-08-07T06:44:11.4309315Z For more information about this error, try `rustc --explain E0308`.
2019-08-07T06:44:11.4309365Z 
2019-08-07T06:44:11.4309365Z 
2019-08-07T06:44:11.4309578Z ------------------------------------------
2019-08-07T06:44:11.4309608Z 
2019-08-07T06:44:11.4309634Z 
2019-08-07T06:44:11.4309864Z ---- [ui] ui/issues/issue-32323.rs stdout ----
2019-08-07T06:44:11.4309920Z diff of stderr:
2019-08-07T06:44:11.4309949Z 
2019-08-07T06:44:11.4310175Z 4 LL | pub fn f<'a, T: Tr<'a>>() -> <T as Tr<'a>>::Out {}
2019-08-07T06:44:11.4310448Z 5    |        -                     ^^^^^^^^^^^^^^^^^^ expected associated type, found ()
2019-08-07T06:44:11.4310500Z 6    |        |
2019-08-07T06:44:11.4310723Z -    |        this function's body doesn't return
2019-08-07T06:44:11.4310969Z +    |        this function's body doesn't return a value
2019-08-07T06:44:11.4311024Z 8    |
2019-08-07T06:44:11.4311353Z 9    = note: expected type `<T as Tr<'a>>::Out`
2019-08-07T06:44:11.4311435Z 
2019-08-07T06:44:11.4311457Z 
2019-08-07T06:44:11.4311495Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4311763Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32323/issue-32323.stderr
2019-08-07T06:44:11.4311763Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32323/issue-32323.stderr
2019-08-07T06:44:11.4311977Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4312226Z To only update this specific test, also pass `--test-args issues/issue-32323.rs`
2019-08-07T06:44:11.4312312Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4312353Z status: exit code: 1
2019-08-07T06:44:11.4312353Z status: exit code: 1
2019-08-07T06:44:11.4312986Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-32323.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32323" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32323/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4313296Z ------------------------------------------
2019-08-07T06:44:11.4313341Z 
2019-08-07T06:44:11.4313554Z ------------------------------------------
2019-08-07T06:44:11.4313595Z stderr:
2019-08-07T06:44:11.4313595Z stderr:
2019-08-07T06:44:11.4313811Z ------------------------------------------
2019-08-07T06:44:11.4313855Z error[E0308]: mismatched types
2019-08-07T06:44:11.4314074Z   --> /checkout/src/test/ui/issues/issue-32323.rs:5:30
2019-08-07T06:44:11.4314116Z    |
2019-08-07T06:44:11.4314345Z LL | pub fn f<'a, T: Tr<'a>>() -> <T as Tr<'a>>::Out {}
2019-08-07T06:44:11.4314586Z    |        -                     ^^^^^^^^^^^^^^^^^^ expected associated type, found ()
2019-08-07T06:44:11.4314658Z    |        |
2019-08-07T06:44:11.4314877Z    |        this function's body doesn't return a value
2019-08-07T06:44:11.4314920Z    |
2019-08-07T06:44:11.4315121Z    = note: expected type `<T as Tr<'a>>::Out`
2019-08-07T06:44:11.4315206Z 
2019-08-07T06:44:11.4315245Z error: aborting due to previous error
2019-08-07T06:44:11.4315272Z 
2019-08-07T06:44:11.4315589Z For more information about this error, try `rustc --explain E0308`.
---
2019-08-07T06:44:11.4316332Z 
2019-08-07T06:44:11.4316388Z 1 error[E0308]: mismatched types
2019-08-07T06:44:11.4316583Z 2   --> $DIR/issue-35241.rs:3:20
2019-08-07T06:44:11.4316624Z 3    |
2019-08-07T06:44:11.4316756Z + LL | struct Foo(u32);
2019-08-07T06:44:11.4316998Z +    | ---------------- fn(u32) -> Foo {Foo} defined here
2019-08-07T06:44:11.4317042Z + LL | 
2019-08-07T06:44:11.4317351Z 4 LL | fn test() -> Foo { Foo }
2019-08-07T06:44:11.4317669Z 5    |              ---   ^^^
2019-08-07T06:44:11.4318102Z 
2019-08-07T06:44:11.4318129Z 
2019-08-07T06:44:11.4318195Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4318547Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35241/issue-35241.stderr
2019-08-07T06:44:11.4318547Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35241/issue-35241.stderr
2019-08-07T06:44:11.4318799Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4319079Z To only update this specific test, also pass `--test-args issues/issue-35241.rs`
2019-08-07T06:44:11.4319155Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4319215Z status: exit code: 1
2019-08-07T06:44:11.4319215Z status: exit code: 1
2019-08-07T06:44:11.4319941Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-35241.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35241" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35241/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4320265Z ------------------------------------------
2019-08-07T06:44:11.4320298Z 
2019-08-07T06:44:11.4320524Z ------------------------------------------
2019-08-07T06:44:11.4320568Z stderr:
2019-08-07T06:44:11.4320568Z stderr:
2019-08-07T06:44:11.4320779Z ------------------------------------------
2019-08-07T06:44:11.4320826Z error[E0308]: mismatched types
2019-08-07T06:44:11.4321072Z   --> /checkout/src/test/ui/issues/issue-35241.rs:3:20
2019-08-07T06:44:11.4321234Z    |
2019-08-07T06:44:11.4321272Z LL | struct Foo(u32);
2019-08-07T06:44:11.4321609Z    | ---------------- fn(u32) -> Foo {Foo} defined here
2019-08-07T06:44:11.4321651Z LL | 
2019-08-07T06:44:11.4321847Z LL | fn test() -> Foo { Foo } //~ ERROR mismatched types
2019-08-07T06:44:11.4322038Z    |              ---   ^^^
2019-08-07T06:44:11.4322079Z    |              |     |
2019-08-07T06:44:11.4322120Z    |              |     expected struct `Foo`, found fn item
2019-08-07T06:44:11.4322189Z    |              |     help: use parentheses to call this function: `Foo(...)`
2019-08-07T06:44:11.4322235Z    |              expected `Foo` because of return type
2019-08-07T06:44:11.4322325Z    = note: expected type `Foo`
2019-08-07T06:44:11.4322325Z    = note: expected type `Foo`
2019-08-07T06:44:11.4322523Z               found type `fn(u32) -> Foo {Foo}`
2019-08-07T06:44:11.4322588Z error: aborting due to previous error
2019-08-07T06:44:11.4322629Z 
2019-08-07T06:44:11.4322837Z For more information about this error, try `rustc --explain E0308`.
2019-08-07T06:44:11.4322873Z 
---
2019-08-07T06:44:11.4323375Z 
2019-08-07T06:44:11.4323564Z 16 LL | fn foo() -> bool {
2019-08-07T06:44:11.4323754Z 17    |    ---      ^^^^ expected bool, found ()
2019-08-07T06:44:11.4323794Z 18    |    |
2019-08-07T06:44:11.4324088Z -    |    this function's body doesn't return
2019-08-07T06:44:11.4324346Z +    |    this function's body doesn't return a value
2019-08-07T06:44:11.4324445Z 21 LL |     break true;
2019-08-07T06:44:11.4324669Z 22    |               - help: consider removing this semicolon
2019-08-07T06:44:11.4324700Z 
2019-08-07T06:44:11.4324724Z 
2019-08-07T06:44:11.4324724Z 
2019-08-07T06:44:11.4324780Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4325156Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43162/issue-43162.stderr
2019-08-07T06:44:11.4325392Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4325656Z To only update this specific test, also pass `--test-args issues/issue-43162.rs`
2019-08-07T06:44:11.4325726Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4325766Z status: exit code: 1
2019-08-07T06:44:11.4325766Z status: exit code: 1
2019-08-07T06:44:11.4326420Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-43162.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43162" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43162/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4326728Z ------------------------------------------
2019-08-07T06:44:11.4326758Z 
2019-08-07T06:44:11.4326960Z ------------------------------------------
2019-08-07T06:44:11.4327018Z stderr:
2019-08-07T06:44:11.4327018Z stderr:
2019-08-07T06:44:11.4327220Z ------------------------------------------
2019-08-07T06:44:11.4327263Z error[E0268]: `break` outside of loop
2019-08-07T06:44:11.4327615Z   --> /checkout/src/test/ui/issues/issue-43162.rs:3:5
2019-08-07T06:44:11.4328037Z LL |     break true; //~ ERROR E0268
2019-08-07T06:44:11.4328102Z    |     ^^^^^^^^^^ cannot break outside of a loop
2019-08-07T06:44:11.4328132Z 
2019-08-07T06:44:11.4328132Z 
2019-08-07T06:44:11.4328174Z error[E0268]: `break` outside of loop
2019-08-07T06:44:11.4328466Z   --> /checkout/src/test/ui/issues/issue-43162.rs:7:5
2019-08-07T06:44:11.4328531Z    |
2019-08-07T06:44:11.4328574Z LL |     break {}; //~ ERROR E0268
2019-08-07T06:44:11.4328620Z    |     ^^^^^^^^ cannot break outside of a loop
2019-08-07T06:44:11.4328718Z error[E0308]: mismatched types
2019-08-07T06:44:11.4328955Z   --> /checkout/src/test/ui/issues/issue-43162.rs:1:13
2019-08-07T06:44:11.4329000Z    |
2019-08-07T06:44:11.4329206Z LL | fn foo() -> bool {
2019-08-07T06:44:11.4329206Z LL | fn foo() -> bool {
2019-08-07T06:44:11.4329423Z    |    ---      ^^^^ expected bool, found ()
2019-08-07T06:44:11.4329468Z    |    |
2019-08-07T06:44:11.4329702Z    |    this function's body doesn't return a value
2019-08-07T06:44:11.4329751Z LL |     //~^ ERROR E0308
2019-08-07T06:44:11.4329804Z LL |     break true; //~ ERROR E0268
2019-08-07T06:44:11.4330037Z    |               - help: consider removing this semicolon
2019-08-07T06:44:11.4330149Z    = note: expected type `bool`
2019-08-07T06:44:11.4330193Z               found type `()`
2019-08-07T06:44:11.4330236Z 
2019-08-07T06:44:11.4330278Z error: aborting due to 3 previous errors
---
2019-08-07T06:44:11.4330934Z 
2019-08-07T06:44:11.4331359Z ---- [ui] ui/issues/issue-44023.rs stdout ----
2019-08-07T06:44:11.4331523Z diff of stderr:
2019-08-07T06:44:11.4331547Z 
2019-08-07T06:44:11.4331761Z 4 LL | fn საჭმელად_გემრიელი_სადილი ( ) -> isize {
2019-08-07T06:44:11.4332085Z 5    |    ------------------------        ^^^^^ expected isize, found ()
2019-08-07T06:44:11.4332135Z 6    |    |
2019-08-07T06:44:11.4355028Z -    |    this function's body doesn't return
2019-08-07T06:44:11.4355496Z +    |    this function's body doesn't return a value
2019-08-07T06:44:11.4355608Z 9    = note: expected type `isize`
2019-08-07T06:44:11.4355655Z 10               found type `()`
2019-08-07T06:44:11.4355847Z 
2019-08-07T06:44:11.4355873Z 
2019-08-07T06:44:11.4355873Z 
2019-08-07T06:44:11.4355919Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4356317Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44023/issue-44023.stderr
2019-08-07T06:44:11.4356762Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4357230Z To only update this specific test, also pass `--test-args issues/issue-44023.rs`
2019-08-07T06:44:11.4357325Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4357484Z status: exit code: 1
2019-08-07T06:44:11.4357484Z status: exit code: 1
2019-08-07T06:44:11.4358784Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-44023.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44023" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44023/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4359146Z ------------------------------------------
2019-08-07T06:44:11.4359181Z 
2019-08-07T06:44:11.4359397Z ------------------------------------------
2019-08-07T06:44:11.4359453Z stderr:
2019-08-07T06:44:11.4359453Z stderr:
2019-08-07T06:44:11.4359663Z ------------------------------------------
2019-08-07T06:44:11.4359720Z error[E0308]: mismatched types
2019-08-07T06:44:11.4359960Z   --> /checkout/src/test/ui/issues/issue-44023.rs:5:36
2019-08-07T06:44:11.4360007Z    |
2019-08-07T06:44:11.4360256Z LL | fn საჭმელად_გემრიელი_სადილი ( ) -> isize { //~ ERROR mismatched types
2019-08-07T06:44:11.4360508Z    |    ------------------------        ^^^^^ expected isize, found ()
2019-08-07T06:44:11.4360559Z    |    |
2019-08-07T06:44:11.4360782Z    |    this function's body doesn't return a value
2019-08-07T06:44:11.4360888Z    = note: expected type `isize`
2019-08-07T06:44:11.4360933Z               found type `()`
2019-08-07T06:44:11.4360961Z 
2019-08-07T06:44:11.4361009Z error: aborting due to previous error
---
2019-08-07T06:44:11.4361980Z 
2019-08-07T06:44:11.4362018Z 1 error[E0308]: mismatched types
2019-08-07T06:44:11.4362215Z 2   --> $DIR/issue-5216.rs:3:21
2019-08-07T06:44:11.4362265Z 3    |
2019-08-07T06:44:11.4362303Z + LL | fn f() { }
2019-08-07T06:44:11.4362539Z +    | ---------- fn() {f} defined here
2019-08-07T06:44:11.4362583Z + LL | struct S(Box<dyn FnMut()>);
2019-08-07T06:44:11.4362637Z 4 LL | pub static C: S = S(f);
2019-08-07T06:44:11.4362683Z 5    |                     ^ expected struct `std::boxed::Box`, found fn item
2019-08-07T06:44:11.4362753Z 
2019-08-07T06:44:11.4362792Z 10 error[E0308]: mismatched types
2019-08-07T06:44:11.4363115Z 11   --> $DIR/issue-5216.rs:8:19
2019-08-07T06:44:11.4363188Z 12    |
2019-08-07T06:44:11.4363188Z 12    |
2019-08-07T06:44:11.4363236Z + LL | fn g() { }
2019-08-07T06:44:11.4363426Z +    | ---------- fn() {g} defined here
2019-08-07T06:44:11.4363467Z + LL | type T = Box<dyn FnMut()>;
2019-08-07T06:44:11.4363627Z 13 LL | pub static D: T = g;
2019-08-07T06:44:11.4363677Z 14    |                   ^ expected struct `std::boxed::Box`, found fn item
2019-08-07T06:44:11.4363739Z 
2019-08-07T06:44:11.4363768Z 
2019-08-07T06:44:11.4363806Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4364097Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5216/issue-5216.stderr
2019-08-07T06:44:11.4364097Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5216/issue-5216.stderr
2019-08-07T06:44:11.4364439Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4364691Z To only update this specific test, also pass `--test-args issues/issue-5216.rs`
2019-08-07T06:44:11.4364887Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4364929Z status: exit code: 1
2019-08-07T06:44:11.4364929Z status: exit code: 1
2019-08-07T06:44:11.4365593Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-5216.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5216" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5216/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4365896Z ------------------------------------------
2019-08-07T06:44:11.4365934Z 
2019-08-07T06:44:11.4366148Z ------------------------------------------
2019-08-07T06:44:11.4366188Z stderr:
2019-08-07T06:44:11.4366188Z stderr:
2019-08-07T06:44:11.4366382Z ------------------------------------------
2019-08-07T06:44:11.4366431Z error[E0308]: mismatched types
2019-08-07T06:44:11.4366645Z   --> /checkout/src/test/ui/issues/issue-5216.rs:3:21
2019-08-07T06:44:11.4366687Z    |
2019-08-07T06:44:11.4366731Z LL | fn f() { }
2019-08-07T06:44:11.4366926Z    | ---------- fn() {f} defined here
2019-08-07T06:44:11.4366977Z LL | struct S(Box<dyn FnMut()>);
2019-08-07T06:44:11.4367021Z LL | pub static C: S = S(f); //~ ERROR mismatched types
2019-08-07T06:44:11.4367199Z    |                     ^ expected struct `std::boxed::Box`, found fn item
2019-08-07T06:44:11.4367241Z    |
2019-08-07T06:44:11.4367596Z    = note: expected type `std::boxed::Box<(dyn std::ops::FnMut() + 'static)>`
2019-08-07T06:44:11.4367643Z               found type `fn() {f}`
2019-08-07T06:44:11.4368040Z error[E0308]: mismatched types
2019-08-07T06:44:11.4368343Z   --> /checkout/src/test/ui/issues/issue-5216.rs:8:19
2019-08-07T06:44:11.4368387Z    |
2019-08-07T06:44:11.4368387Z    |
2019-08-07T06:44:11.4368428Z LL | fn g() { }
2019-08-07T06:44:11.4368646Z    | ---------- fn() {g} defined here
2019-08-07T06:44:11.4368692Z LL | type T = Box<dyn FnMut()>;
2019-08-07T06:44:11.4368739Z LL | pub static D: T = g; //~ ERROR mismatched types
2019-08-07T06:44:11.4368795Z    |                   ^ expected struct `std::boxed::Box`, found fn item
2019-08-07T06:44:11.4368850Z    |
2019-08-07T06:44:11.4369098Z    = note: expected type `std::boxed::Box<(dyn std::ops::FnMut() + 'static)>`
2019-08-07T06:44:11.4369148Z               found type `fn() {g}`
2019-08-07T06:44:11.4369226Z error: aborting due to 2 previous errors
2019-08-07T06:44:11.4369253Z 
2019-08-07T06:44:11.4369491Z For more information about this error, try `rustc --explain E0308`.
2019-08-07T06:44:11.4369530Z 
2019-08-07T06:44:11.4369530Z 
2019-08-07T06:44:11.4369741Z ------------------------------------------
2019-08-07T06:44:11.4369781Z 
2019-08-07T06:44:11.4369806Z 
2019-08-07T06:44:11.4370033Z ---- [ui] ui/issues/issue-6458-4.rs stdout ----
2019-08-07T06:44:11.4370078Z diff of stderr:
2019-08-07T06:44:11.4370105Z 
2019-08-07T06:44:11.4370321Z 4 LL | fn foo(b: bool) -> Result<bool,String> {
2019-08-07T06:44:11.4370596Z 5    |    ---             ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-08-07T06:44:11.4370646Z 6    |    |
2019-08-07T06:44:11.4370967Z -    |    this function's body doesn't return
2019-08-07T06:44:11.4371546Z +    |    this function's body doesn't return a value
2019-08-07T06:44:11.4371589Z 8 LL |     Err("bar".to_string());
2019-08-07T06:44:11.4371806Z 9    |                           - help: consider removing this semicolon
2019-08-07T06:44:11.4371881Z 
2019-08-07T06:44:11.4371903Z 
2019-08-07T06:44:11.4371941Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4372203Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-6458-4/issue-6458-4.stderr
2019-08-07T06:44:11.4372203Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-6458-4/issue-6458-4.stderr
2019-08-07T06:44:11.4372519Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4372750Z To only update this specific test, also pass `--test-args issues/issue-6458-4.rs`
2019-08-07T06:44:11.4372830Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4372868Z status: exit code: 1
2019-08-07T06:44:11.4372868Z status: exit code: 1
2019-08-07T06:44:11.4373506Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-6458-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-6458-4" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-6458-4/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4373784Z ------------------------------------------
2019-08-07T06:44:11.4373813Z 
2019-08-07T06:44:11.4374001Z ------------------------------------------
2019-08-07T06:44:11.4374038Z stderr:
2019-08-07T06:44:11.4374038Z stderr:
2019-08-07T06:44:11.4374227Z ------------------------------------------
2019-08-07T06:44:11.4374268Z error[E0308]: mismatched types
2019-08-07T06:44:11.4374469Z   --> /checkout/src/test/ui/issues/issue-6458-4.rs:1:20
2019-08-07T06:44:11.4374515Z    |
2019-08-07T06:44:11.4374734Z LL | fn foo(b: bool) -> Result<bool,String> { //~ ERROR mismatched types
2019-08-07T06:44:11.4374964Z    |    ---             ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-08-07T06:44:11.4375014Z    |    |
2019-08-07T06:44:11.4375211Z    |    this function's body doesn't return a value
2019-08-07T06:44:11.4375253Z LL |     Err("bar".to_string());
2019-08-07T06:44:11.4375468Z    |                           - help: consider removing this semicolon
2019-08-07T06:44:11.4375557Z    = note: expected type `std::result::Result<bool, std::string::String>`
2019-08-07T06:44:11.4375602Z               found type `()`
2019-08-07T06:44:11.4375626Z 
2019-08-07T06:44:11.4375663Z error: aborting due to previous error
---
2019-08-07T06:44:11.4376175Z 
2019-08-07T06:44:11.4376386Z ---- [ui] ui/liveness/liveness-forgot-ret.rs stdout ----
2019-08-07T06:44:11.4376428Z diff of stderr:
2019-08-07T06:44:11.4376451Z 
2019-08-07T06:44:11.4376656Z 4 LL | fn f(a: isize) -> isize { if god_exists(a) { return 5; }; }
2019-08-07T06:44:11.4376867Z 5    |    -              ^^^^^ expected isize, found ()
2019-08-07T06:44:11.4376906Z 6    |    |
2019-08-07T06:44:11.4377092Z -    |    this function's body doesn't return
2019-08-07T06:44:11.4377307Z +    |    this function's body doesn't return a value
2019-08-07T06:44:11.4377384Z 9    = note: expected type `isize`
2019-08-07T06:44:11.4377428Z 10               found type `()`
2019-08-07T06:44:11.4377452Z 
2019-08-07T06:44:11.4377474Z 
2019-08-07T06:44:11.4377474Z 
2019-08-07T06:44:11.4377512Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4378387Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-forgot-ret/liveness-forgot-ret.stderr
2019-08-07T06:44:11.4379043Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4379355Z To only update this specific test, also pass `--test-args liveness/liveness-forgot-ret.rs`
2019-08-07T06:44:11.4379442Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4379485Z status: exit code: 1
2019-08-07T06:44:11.4379485Z status: exit code: 1
2019-08-07T06:44:11.4380225Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/liveness/liveness-forgot-ret.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-forgot-ret" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-forgot-ret/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4380699Z ------------------------------------------
2019-08-07T06:44:11.4380733Z 
2019-08-07T06:44:11.4380949Z ------------------------------------------
2019-08-07T06:44:11.4380992Z stderr:
2019-08-07T06:44:11.4380992Z stderr:
2019-08-07T06:44:11.4381215Z ------------------------------------------
2019-08-07T06:44:11.4381261Z error[E0308]: mismatched types
2019-08-07T06:44:11.4381496Z   --> /checkout/src/test/ui/liveness/liveness-forgot-ret.rs:3:19
2019-08-07T06:44:11.4381562Z    |
2019-08-07T06:44:11.4381800Z LL | fn f(a: isize) -> isize { if god_exists(a) { return 5; }; }
2019-08-07T06:44:11.4382026Z    |    -              ^^^^^ expected isize, found ()
2019-08-07T06:44:11.4382084Z    |    |
2019-08-07T06:44:11.4382305Z    |    this function's body doesn't return a value
2019-08-07T06:44:11.4382399Z    = note: expected type `isize`
2019-08-07T06:44:11.4382443Z               found type `()`
2019-08-07T06:44:11.4382470Z 
2019-08-07T06:44:11.4382521Z error: aborting due to previous error
---
2019-08-07T06:44:11.4383099Z 
2019-08-07T06:44:11.4383325Z ---- [ui] ui/liveness/liveness-missing-ret2.rs stdout ----
2019-08-07T06:44:11.4383371Z diff of stderr:
2019-08-07T06:44:11.4383398Z 
2019-08-07T06:44:11.4383610Z 4 LL | fn f() -> isize {
2019-08-07T06:44:11.4383826Z 5    |    -      ^^^^^ expected isize, found ()
2019-08-07T06:44:11.4383871Z 6    |    |
2019-08-07T06:44:11.4384091Z -    |    this function's body doesn't return
2019-08-07T06:44:11.4384340Z +    |    this function's body doesn't return a value
2019-08-07T06:44:11.4384653Z 9    = note: expected type `isize`
2019-08-07T06:44:11.4384700Z 10               found type `()`
2019-08-07T06:44:11.4384731Z 
2019-08-07T06:44:11.4384757Z 
2019-08-07T06:44:11.4384757Z 
2019-08-07T06:44:11.4384815Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4385202Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-missing-ret2/liveness-missing-ret2.stderr
2019-08-07T06:44:11.4385475Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4385777Z To only update this specific test, also pass `--test-args liveness/liveness-missing-ret2.rs`
2019-08-07T06:44:11.4385869Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4385916Z status: exit code: 1
2019-08-07T06:44:11.4385916Z status: exit code: 1
2019-08-07T06:44:11.4387247Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/liveness/liveness-missing-ret2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-missing-ret2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-missing-ret2/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4388475Z ------------------------------------------
2019-08-07T06:44:11.4388518Z 
2019-08-07T06:44:11.4388741Z ------------------------------------------
2019-08-07T06:44:11.4388798Z stderr:
2019-08-07T06:44:11.4388798Z stderr:
2019-08-07T06:44:11.4389181Z ------------------------------------------
2019-08-07T06:44:11.4389228Z error[E0308]: mismatched types
2019-08-07T06:44:11.4389480Z   --> /checkout/src/test/ui/liveness/liveness-missing-ret2.rs:1:11
2019-08-07T06:44:11.4389531Z    |
2019-08-07T06:44:11.4389759Z LL | fn f() -> isize { //~ ERROR mismatched types
2019-08-07T06:44:11.4389986Z    |    -      ^^^^^ expected isize, found ()
2019-08-07T06:44:11.4390033Z    |    |
2019-08-07T06:44:11.4390254Z    |    this function's body doesn't return a value
2019-08-07T06:44:11.4390362Z    = note: expected type `isize`
2019-08-07T06:44:11.4390405Z               found type `()`
2019-08-07T06:44:11.4390433Z 
2019-08-07T06:44:11.4390487Z error: aborting due to previous error
---
2019-08-07T06:44:11.4391080Z 
2019-08-07T06:44:11.4391785Z ---- [ui] ui/liveness/liveness-return-last-stmt-semi.rs stdout ----
2019-08-07T06:44:11.4391863Z diff of stderr:
2019-08-07T06:44:11.4391891Z 
2019-08-07T06:44:11.4392577Z 5    |                                ---      ^^^    - help: consider removing this semicolon
2019-08-07T06:44:11.4392642Z 6    |                                |        |
2019-08-07T06:44:11.4392705Z 7    |                                |        expected i32, found ()
2019-08-07T06:44:11.4393564Z -    |                                this function's body doesn't return
2019-08-07T06:44:11.4393896Z +    |                                this function's body doesn't return a value
2019-08-07T06:44:11.4394004Z 10 LL |     test!();
2019-08-07T06:44:11.4394228Z 11    |     -------- in this macro invocation
2019-08-07T06:44:11.4394270Z 
2019-08-07T06:44:11.4394270Z 
2019-08-07T06:44:11.4394477Z 19 LL | fn no_return() -> i32 {}
2019-08-07T06:44:11.4394703Z 20    |    ---------      ^^^ expected i32, found ()
2019-08-07T06:44:11.4394760Z 21    |    |
2019-08-07T06:44:11.4394985Z -    |    this function's body doesn't return
2019-08-07T06:44:11.4395208Z +    |    this function's body doesn't return a value
2019-08-07T06:44:11.4395303Z 24    = note: expected type `i32`
2019-08-07T06:44:11.4395347Z 25               found type `()`
2019-08-07T06:44:11.4395375Z 
2019-08-07T06:44:11.4395375Z 
2019-08-07T06:44:11.4395579Z 30 LL | fn bar(x: u32) -> u32 {
2019-08-07T06:44:11.4395821Z 31    |    ---            ^^^ expected u32, found ()
2019-08-07T06:44:11.4395868Z 32    |    |
2019-08-07T06:44:11.4396083Z -    |    this function's body doesn't return
2019-08-07T06:44:11.4396318Z +    |    this function's body doesn't return a value
2019-08-07T06:44:11.4396365Z 34 LL |     x * 2;
2019-08-07T06:44:11.4396593Z 35    |          - help: consider removing this semicolon
2019-08-07T06:44:11.4396678Z 
2019-08-07T06:44:11.4396678Z 
2019-08-07T06:44:11.4396980Z 43 LL | fn baz(x: u64) -> u32 {
2019-08-07T06:44:11.4397207Z 44    |    ---            ^^^ expected u32, found ()
2019-08-07T06:44:11.4397262Z 45    |    |
2019-08-07T06:44:11.4397595Z -    |    this function's body doesn't return
2019-08-07T06:44:11.4398053Z +    |    this function's body doesn't return a value
2019-08-07T06:44:11.4398156Z 48    = note: expected type `u32`
2019-08-07T06:44:11.4398199Z 49               found type `()`
2019-08-07T06:44:11.4398227Z 
2019-08-07T06:44:11.4398257Z 
2019-08-07T06:44:11.4398257Z 
2019-08-07T06:44:11.4398429Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4398815Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-return-last-stmt-semi/liveness-return-last-stmt-semi.stderr
2019-08-07T06:44:11.4399075Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4399356Z To only update this specific test, also pass `--test-args liveness/liveness-return-last-stmt-semi.rs`
2019-08-07T06:44:11.4399524Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4399576Z status: exit code: 1
2019-08-07T06:44:11.4399576Z status: exit code: 1
2019-08-07T06:44:11.4400365Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/liveness/liveness-return-last-stmt-semi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-return-last-stmt-semi" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-return-last-stmt-semi/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4400692Z ------------------------------------------
2019-08-07T06:44:11.4400724Z 
2019-08-07T06:44:11.4401059Z ------------------------------------------
2019-08-07T06:44:11.4401100Z stderr:
2019-08-07T06:44:11.4401100Z stderr:
2019-08-07T06:44:11.4401307Z ------------------------------------------
2019-08-07T06:44:11.4401361Z error[E0308]: mismatched types
2019-08-07T06:44:11.4401592Z   --> /checkout/src/test/ui/liveness/liveness-return-last-stmt-semi.rs:4:41
2019-08-07T06:44:11.4401639Z    |
2019-08-07T06:44:11.4401973Z LL | macro_rules! test { () => { fn foo() -> i32 { 1; } } }
2019-08-07T06:44:11.4402204Z    |                                ---      ^^^    - help: consider removing this semicolon
2019-08-07T06:44:11.4402250Z    |                                |        |
2019-08-07T06:44:11.4402313Z    |                                |        expected i32, found ()
2019-08-07T06:44:11.4402534Z    |                                this function's body doesn't return a value
2019-08-07T06:44:11.4402620Z LL |     test!();
2019-08-07T06:44:11.4402806Z    |     -------- in this macro invocation
2019-08-07T06:44:11.4402845Z    |
2019-08-07T06:44:11.4402881Z    = note: expected type `i32`
2019-08-07T06:44:11.4402881Z    = note: expected type `i32`
2019-08-07T06:44:11.4402932Z               found type `()`
2019-08-07T06:44:11.4402956Z 
2019-08-07T06:44:11.4402991Z error[E0308]: mismatched types
2019-08-07T06:44:11.4403216Z   --> /checkout/src/test/ui/liveness/liveness-return-last-stmt-semi.rs:7:19
2019-08-07T06:44:11.4403257Z    |
2019-08-07T06:44:11.4403455Z LL | fn no_return() -> i32 {} //~ ERROR mismatched types
2019-08-07T06:44:11.4403652Z    |    ---------      ^^^ expected i32, found ()
2019-08-07T06:44:11.4403703Z    |    |
2019-08-07T06:44:11.4403898Z    |    this function's body doesn't return a value
2019-08-07T06:44:11.4403990Z    = note: expected type `i32`
2019-08-07T06:44:11.4404028Z               found type `()`
2019-08-07T06:44:11.4404053Z 
2019-08-07T06:44:11.4404087Z error[E0308]: mismatched types
2019-08-07T06:44:11.4404087Z error[E0308]: mismatched types
2019-08-07T06:44:11.4404317Z   --> /checkout/src/test/ui/liveness/liveness-return-last-stmt-semi.rs:9:19
2019-08-07T06:44:11.4404357Z    |
2019-08-07T06:44:11.4404554Z LL | fn bar(x: u32) -> u32 { //~ ERROR mismatched types
2019-08-07T06:44:11.4404773Z    |    ---            ^^^ expected u32, found ()
2019-08-07T06:44:11.4404812Z    |    |
2019-08-07T06:44:11.4405002Z    |    this function's body doesn't return a value
2019-08-07T06:44:11.4405052Z LL |     x * 2;
2019-08-07T06:44:11.4405247Z    |          - help: consider removing this semicolon
2019-08-07T06:44:11.4405321Z    = note: expected type `u32`
2019-08-07T06:44:11.4405364Z               found type `()`
2019-08-07T06:44:11.4405388Z 
2019-08-07T06:44:11.4405495Z error[E0308]: mismatched types
2019-08-07T06:44:11.4405495Z error[E0308]: mismatched types
2019-08-07T06:44:11.4405756Z   --> /checkout/src/test/ui/liveness/liveness-return-last-stmt-semi.rs:13:19
2019-08-07T06:44:11.4405798Z    |
2019-08-07T06:44:11.4405995Z LL | fn baz(x: u64) -> u32 { //~ ERROR mismatched types
2019-08-07T06:44:11.4406195Z    |    ---            ^^^ expected u32, found ()
2019-08-07T06:44:11.4406235Z    |    |
2019-08-07T06:44:11.4406427Z    |    this function's body doesn't return a value
2019-08-07T06:44:11.4406585Z    = note: expected type `u32`
2019-08-07T06:44:11.4406622Z               found type `()`
2019-08-07T06:44:11.4406646Z 
2019-08-07T06:44:11.4406682Z error: aborting due to 4 previous errors
---
2019-08-07T06:44:11.4407227Z 
2019-08-07T06:44:11.4407724Z ---- [ui] ui/missing/missing-return.rs stdout ----
2019-08-07T06:44:11.4407788Z diff of stderr:
2019-08-07T06:44:11.4407826Z 
2019-08-07T06:44:11.4408080Z 4 LL | fn f() -> isize { }
2019-08-07T06:44:11.4408302Z 5    |    -      ^^^^^ expected isize, found ()
2019-08-07T06:44:11.4408358Z 6    |    |
2019-08-07T06:44:11.4408576Z -    |    this function's body doesn't return
2019-08-07T06:44:11.4408800Z +    |    this function's body doesn't return a value
2019-08-07T06:44:11.4408909Z 9    = note: expected type `isize`
2019-08-07T06:44:11.4408953Z 10               found type `()`
2019-08-07T06:44:11.4408981Z 
2019-08-07T06:44:11.4409006Z 
2019-08-07T06:44:11.4409006Z 
2019-08-07T06:44:11.4409059Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4409355Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-return/missing-return.stderr
2019-08-07T06:44:11.4409601Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4409907Z To only update this specific test, also pass `--test-args missing/missing-return.rs`
2019-08-07T06:44:11.4409989Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4410042Z status: exit code: 1
2019-08-07T06:44:11.4410042Z status: exit code: 1
2019-08-07T06:44:11.4410773Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/missing/missing-return.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-return" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-return/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4411121Z ------------------------------------------
2019-08-07T06:44:11.4411156Z 
2019-08-07T06:44:11.4411412Z ------------------------------------------
2019-08-07T06:44:11.4411460Z stderr:
2019-08-07T06:44:11.4411460Z stderr:
2019-08-07T06:44:11.4411696Z ------------------------------------------
2019-08-07T06:44:11.4411744Z error[E0308]: mismatched types
2019-08-07T06:44:11.4412220Z   --> /checkout/src/test/ui/missing/missing-return.rs:3:11
2019-08-07T06:44:11.4412279Z    |
2019-08-07T06:44:11.4412501Z LL | fn f() -> isize { }
2019-08-07T06:44:11.4413142Z    |    -      ^^^^^ expected isize, found ()
2019-08-07T06:44:11.4413200Z    |    |
2019-08-07T06:44:11.4413490Z    |    this function's body doesn't return a value
2019-08-07T06:44:11.4413590Z    = note: expected type `isize`
2019-08-07T06:44:11.4413634Z               found type `()`
2019-08-07T06:44:11.4413664Z 
2019-08-07T06:44:11.4413712Z error: aborting due to previous error
---
2019-08-07T06:44:11.4414444Z 
2019-08-07T06:44:11.4414701Z ---- [ui] ui/parser/issue-62881.rs stdout ----
2019-08-07T06:44:11.4414760Z diff of stderr:
2019-08-07T06:44:11.4414788Z 
2019-08-07T06:44:11.4415455Z 19 LL | fn f() -> isize { fn f() -> isize {} pub f<
2019-08-07T06:44:11.4416325Z 20    |                      -      ^^^^^ expected isize, found ()
2019-08-07T06:44:11.4416423Z 21    |                      |
2019-08-07T06:44:11.4416894Z -    |                      this function's body doesn't return
2019-08-07T06:44:11.4417166Z +    |                      this function's body doesn't return a value
2019-08-07T06:44:11.4417275Z 24    = note: expected type `isize`
2019-08-07T06:44:11.4417321Z 25               found type `()`
2019-08-07T06:44:11.4417351Z 
2019-08-07T06:44:11.4417391Z 
2019-08-07T06:44:11.4417391Z 
2019-08-07T06:44:11.4417437Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4418107Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-62881/issue-62881.stderr
2019-08-07T06:44:11.4418931Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4419232Z To only update this specific test, also pass `--test-args parser/issue-62881.rs`
2019-08-07T06:44:11.4419328Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4419375Z status: exit code: 1
2019-08-07T06:44:11.4419375Z status: exit code: 1
2019-08-07T06:44:11.4420638Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-62881.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-62881" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-62881/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4421011Z ------------------------------------------
2019-08-07T06:44:11.4421045Z 
2019-08-07T06:44:11.4421274Z ------------------------------------------
2019-08-07T06:44:11.4421318Z stderr:
2019-08-07T06:44:11.4421318Z stderr:
2019-08-07T06:44:11.4421528Z ------------------------------------------
2019-08-07T06:44:11.4421758Z error: this file contains an un-closed delimiter
2019-08-07T06:44:11.4421993Z   --> /checkout/src/test/ui/parser/issue-62881.rs:6:53
2019-08-07T06:44:11.4422049Z    |
2019-08-07T06:44:11.4422499Z LL | fn f() -> isize { fn f() -> isize {} pub f<
2019-08-07T06:44:11.4422821Z    |                 - un-closed delimiter
2019-08-07T06:44:11.4423064Z LL | //~ ERROR this file contains an un-closed delimiter
2019-08-07T06:44:11.4423112Z    |                                                     ^
2019-08-07T06:44:11.4423140Z 
2019-08-07T06:44:11.4423140Z 
2019-08-07T06:44:11.4423181Z error: missing `fn` or `struct` for function or struct definition
2019-08-07T06:44:11.4423407Z   --> /checkout/src/test/ui/parser/issue-62881.rs:3:41
2019-08-07T06:44:11.4423447Z    |
2019-08-07T06:44:11.4423639Z LL | fn f() -> isize { fn f() -> isize {} pub f<
2019-08-07T06:44:11.4423718Z 
2019-08-07T06:44:11.4423753Z error[E0308]: mismatched types
2019-08-07T06:44:11.4423953Z   --> /checkout/src/test/ui/parser/issue-62881.rs:3:29
2019-08-07T06:44:11.4424000Z    |
2019-08-07T06:44:11.4424000Z    |
2019-08-07T06:44:11.4424199Z LL | fn f() -> isize { fn f() -> isize {} pub f<
2019-08-07T06:44:11.4424777Z    |                      -      ^^^^^ expected isize, found ()
2019-08-07T06:44:11.4424840Z    |                      |
2019-08-07T06:44:11.4425098Z    |                      this function's body doesn't return a value
2019-08-07T06:44:11.4425187Z    = note: expected type `isize`
2019-08-07T06:44:11.4425228Z               found type `()`
2019-08-07T06:44:11.4425253Z 
2019-08-07T06:44:11.4425419Z error: aborting due to 3 previous errors
---
2019-08-07T06:44:11.4426100Z 
2019-08-07T06:44:11.4426291Z ---- [ui] ui/parser/issue-62895.rs stdout ----
2019-08-07T06:44:11.4426331Z diff of stderr:
2019-08-07T06:44:11.4426355Z 
2019-08-07T06:44:11.4426537Z 38 LL | fn v() -> isize {
2019-08-07T06:44:11.4426844Z 39    |    -      ^^^^^ expected isize, found ()
2019-08-07T06:44:11.4426885Z 40    |    |
2019-08-07T06:44:11.4427089Z -    |    this function's body doesn't return
2019-08-07T06:44:11.4427304Z +    |    this function's body doesn't return a value
2019-08-07T06:44:11.4427514Z 43    = note: expected type `isize`
2019-08-07T06:44:11.4427557Z 44               found type `()`
2019-08-07T06:44:11.4427583Z 
2019-08-07T06:44:11.4427606Z 
2019-08-07T06:44:11.4427606Z 
2019-08-07T06:44:11.4427667Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4428327Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-62895/issue-62895.stderr
2019-08-07T06:44:11.4428586Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4428860Z To only update this specific test, also pass `--test-args parser/issue-62895.rs`
2019-08-07T06:44:11.4428938Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4428993Z status: exit code: 1
2019-08-07T06:44:11.4428993Z status: exit code: 1
2019-08-07T06:44:11.4429712Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-62895.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-62895" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-62895/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4430025Z ------------------------------------------
2019-08-07T06:44:11.4430057Z 
2019-08-07T06:44:11.4430270Z ------------------------------------------
2019-08-07T06:44:11.4430321Z stderr:
2019-08-07T06:44:11.4430321Z stderr:
2019-08-07T06:44:11.4430532Z ------------------------------------------
2019-08-07T06:44:11.4430582Z error: expected identifier, found reserved identifier `_`
2019-08-07T06:44:11.4430831Z   --> /checkout/src/test/ui/parser/issue-62895.rs:4:5
2019-08-07T06:44:11.4430879Z    |
2019-08-07T06:44:11.4430921Z LL | mod _ { //~ ERROR expected identifier
2019-08-07T06:44:11.4431006Z 
2019-08-07T06:44:11.4431049Z error: expected identifier, found reserved identifier `_`
2019-08-07T06:44:11.4431377Z   --> /checkout/src/test/ui/parser/issue-62895.rs:6:5
2019-08-07T06:44:11.4431435Z    |
2019-08-07T06:44:11.4431435Z    |
2019-08-07T06:44:11.4431473Z LL | mod _ { //~ ERROR expected identifier
2019-08-07T06:44:11.4431539Z 
2019-08-07T06:44:11.4431588Z error: missing `fn` for function definition
2019-08-07T06:44:11.4431792Z   --> /checkout/src/test/ui/parser/issue-62895.rs:7:4
2019-08-07T06:44:11.4431830Z    |
2019-08-07T06:44:11.4431830Z    |
2019-08-07T06:44:11.4432045Z LL | pub    g() -> is //~ ERROR missing `fn` for function definition
2019-08-07T06:44:11.4432093Z    |    ^^^^
2019-08-07T06:44:11.4432133Z help: add `fn` here to parse `g` as a public function
2019-08-07T06:44:11.4432178Z    |
2019-08-07T06:44:11.4432388Z LL | pub fn g() -> is //~ ERROR missing `fn` for function definition
2019-08-07T06:44:11.4432453Z 
2019-08-07T06:44:11.4432495Z error: expected item, found `;`
2019-08-07T06:44:11.4432829Z   --> /checkout/src/test/ui/parser/issue-62895.rs:10:9
2019-08-07T06:44:11.4432872Z    |
2019-08-07T06:44:11.4432872Z    |
2019-08-07T06:44:11.4433015Z LL | (), w20); //~ ERROR expected item, found `;`
2019-08-07T06:44:11.4433066Z    |         ^ help: remove this semicolon
2019-08-07T06:44:11.4433093Z 
2019-08-07T06:44:11.4433132Z error[E0412]: cannot find type `isizee` in this scope
2019-08-07T06:44:11.4433395Z   --> /checkout/src/test/ui/parser/issue-62895.rs:5:15
2019-08-07T06:44:11.4433437Z    |
2019-08-07T06:44:11.4433665Z LL | pub fn g() -> isizee { //~ ERROR cannot find type `isizee` in this scope
2019-08-07T06:44:11.4433814Z    |               ^^^^^^ help: a builtin type with a similar name exists: `isize`
2019-08-07T06:44:11.4433885Z error[E0308]: mismatched types
2019-08-07T06:44:11.4434145Z   --> /checkout/src/test/ui/parser/issue-62895.rs:3:11
2019-08-07T06:44:11.4434188Z    |
2019-08-07T06:44:11.4434188Z    |
2019-08-07T06:44:11.4434392Z LL | fn v() -> isize { //~ ERROR mismatched types
2019-08-07T06:44:11.4434596Z    |    -      ^^^^^ expected isize, found ()
2019-08-07T06:44:11.4434650Z    |    |
2019-08-07T06:44:11.4434865Z    |    this function's body doesn't return a value
2019-08-07T06:44:11.4434953Z    = note: expected type `isize`
2019-08-07T06:44:11.4434994Z               found type `()`
2019-08-07T06:44:11.4435020Z 
2019-08-07T06:44:11.4435058Z error: aborting due to 6 previous errors
---
2019-08-07T06:44:11.4436012Z 
2019-08-07T06:44:11.4436046Z 5 LL | |
2019-08-07T06:44:11.4436082Z 6 LL | |     loop {}
2019-08-07T06:44:11.4436128Z 7 LL | | }
2019-08-07T06:44:11.4436317Z -    | |_^ expected normal fn, found unsafe fn
2019-08-07T06:44:11.4436364Z +    | | ^
2019-08-07T06:44:11.4436407Z +    | | |
2019-08-07T06:44:11.4436445Z +    | |_expected normal fn, found unsafe fn
2019-08-07T06:44:11.4436657Z +    |   unsafe extern "C" fn(i32, u32) -> u32 {foo} defined here
2019-08-07T06:44:11.4436705Z 9    |
2019-08-07T06:44:11.4436925Z 10    = note: expected type `fn(proc_macro::TokenStream) -> proc_macro::TokenStream`
2019-08-07T06:44:11.4437144Z 11               found type `unsafe extern "C" fn(i32, u32) -> u32 {foo}`
2019-08-07T06:44:11.4437354Z 
2019-08-07T06:44:11.4437397Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4438444Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/signature/signature.stderr
2019-08-07T06:44:11.4438444Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/signature/signature.stderr
2019-08-07T06:44:11.4439456Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4439727Z To only update this specific test, also pass `--test-args proc-macro/signature.rs`
2019-08-07T06:44:11.4439818Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4439878Z status: exit code: 1
2019-08-07T06:44:11.4439878Z status: exit code: 1
2019-08-07T06:44:11.4440563Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/signature.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/signature" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/signature/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4440892Z ------------------------------------------
2019-08-07T06:44:11.4440924Z 
2019-08-07T06:44:11.4441145Z ------------------------------------------
2019-08-07T06:44:11.4441188Z stderr:
2019-08-07T06:44:11.4441188Z stderr:
2019-08-07T06:44:11.4441398Z ------------------------------------------
2019-08-07T06:44:11.4441575Z error[E0308]: mismatched types
2019-08-07T06:44:11.4441853Z   --> /checkout/src/test/ui/proc-macro/signature.rs:10:1
2019-08-07T06:44:11.4441903Z    |
2019-08-07T06:44:11.4442146Z LL | / pub unsafe extern fn foo(a: i32, b: u32) -> u32 {
2019-08-07T06:44:11.4442195Z LL | |     //~^ ERROR: mismatched types
2019-08-07T06:44:11.4442239Z LL | |     loop {}
2019-08-07T06:44:11.4442328Z    | | ^
2019-08-07T06:44:11.4442366Z    | | |
2019-08-07T06:44:11.4442366Z    | | |
2019-08-07T06:44:11.4442493Z    | |_expected normal fn, found unsafe fn
2019-08-07T06:44:11.4442776Z    |   unsafe extern "C" fn(i32, u32) -> u32 {foo} defined here
2019-08-07T06:44:11.4442822Z    |
2019-08-07T06:44:11.4443170Z    = note: expected type `fn(proc_macro::TokenStream) -> proc_macro::TokenStream`
2019-08-07T06:44:11.4443533Z               found type `unsafe extern "C" fn(i32, u32) -> u32 {foo}`
2019-08-07T06:44:11.4443606Z error: aborting due to previous error
2019-08-07T06:44:11.4443633Z 
2019-08-07T06:44:11.4443877Z For more information about this error, try `rustc --explain E0308`.
2019-08-07T06:44:11.4443909Z 
---
2019-08-07T06:44:11.4444451Z 
2019-08-07T06:44:11.4444502Z 1 error[E0308]: mismatched types
2019-08-07T06:44:11.4444730Z 2   --> $DIR/regions-fn-subtyping-return-static-fail.rs:40:12
2019-08-07T06:44:11.4444772Z 3    |
2019-08-07T06:44:11.4444960Z - LL |     want_F(bar);
2019-08-07T06:44:11.4445195Z -    |            ^^^ expected concrete lifetime, found bound lifetime parameter 'cx
2019-08-07T06:44:11.4445401Z + LL | / fn bar<'a,'b>(x: &'a S) -> &'b S {
2019-08-07T06:44:11.4445444Z + LL | |     panic!()
2019-08-07T06:44:11.4445489Z + LL | | }
2019-08-07T06:44:11.4445701Z +    | |_- for<'a> fn(&'a S) -> &S {bar::<'_>} defined here
2019-08-07T06:44:11.4445752Z + ...
2019-08-07T06:44:11.4445798Z + LL |       want_F(bar);
2019-08-07T06:44:11.4446035Z +    |              ^^^ expected concrete lifetime, found bound lifetime parameter 'cx
2019-08-07T06:44:11.4446080Z 6    |
2019-08-07T06:44:11.4446296Z 7    = note: expected type `for<'cx> fn(&'cx S) -> &'cx S`
2019-08-07T06:44:11.4446637Z 8               found type `for<'a> fn(&'a S) -> &S {bar::<'_>}`
2019-08-07T06:44:11.4446710Z 10 error[E0308]: mismatched types
2019-08-07T06:44:11.4446924Z 11   --> $DIR/regions-fn-subtyping-return-static-fail.rs:48:12
2019-08-07T06:44:11.4446963Z 12    |
2019-08-07T06:44:11.4446963Z 12    |
2019-08-07T06:44:11.4447130Z - LL |     want_G(baz);
2019-08-07T06:44:11.4447358Z -    |            ^^^ expected concrete lifetime, found bound lifetime parameter 'cx
2019-08-07T06:44:11.4447539Z + LL | / fn baz(x: &S) -> &S {
2019-08-07T06:44:11.4447578Z + LL | |     panic!()
2019-08-07T06:44:11.4447625Z + LL | | }
2019-08-07T06:44:11.4448308Z +    | |_- for<'r> fn(&'r S) -> &'r S {baz} defined here
2019-08-07T06:44:11.4448359Z + ...
2019-08-07T06:44:11.4448399Z + LL |       want_G(baz);
2019-08-07T06:44:11.4448671Z +    |              ^^^ expected concrete lifetime, found bound lifetime parameter 'cx
2019-08-07T06:44:11.4448719Z 15    |
2019-08-07T06:44:11.4448950Z 16    = note: expected type `for<'cx> fn(&'cx S) -> &'static S`
2019-08-07T06:44:11.4449193Z 17               found type `for<'r> fn(&'r S) -> &'r S {baz}`
2019-08-07T06:44:11.4449259Z 
2019-08-07T06:44:11.4449303Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4449643Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions-fn-subtyping-return-static-fail/regions-fn-subtyping-return-static-fail.stderr
2019-08-07T06:44:11.4449643Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions-fn-subtyping-return-static-fail/regions-fn-subtyping-return-static-fail.stderr
2019-08-07T06:44:11.4449892Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4450196Z To only update this specific test, also pass `--test-args regions-fn-subtyping-return-static-fail.rs`
2019-08-07T06:44:11.4450395Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4450443Z status: exit code: 1
2019-08-07T06:44:11.4450443Z status: exit code: 1
2019-08-07T06:44:11.4451356Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions-fn-subtyping-return-static-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions-fn-subtyping-return-static-fail" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions-fn-subtyping-return-static-fail/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4451746Z ------------------------------------------
2019-08-07T06:44:11.4451775Z 
2019-08-07T06:44:11.4451962Z ------------------------------------------
2019-08-07T06:44:11.4452020Z stderr:
2019-08-07T06:44:11.4452020Z stderr:
2019-08-07T06:44:11.4452324Z ------------------------------------------
2019-08-07T06:44:11.4452367Z error[E0308]: mismatched types
2019-08-07T06:44:11.4452608Z   --> /checkout/src/test/ui/regions-fn-subtyping-return-static-fail.rs:40:12
2019-08-07T06:44:11.4452655Z    |
2019-08-07T06:44:11.4452856Z LL | / fn bar<'a,'b>(x: &'a S) -> &'b S {
2019-08-07T06:44:11.4452905Z LL | |     panic!()
2019-08-07T06:44:11.4452943Z LL | | }
2019-08-07T06:44:11.4453271Z    | |_- for<'a> fn(&'a S) -> &S {bar::<'_>} defined here
2019-08-07T06:44:11.4453323Z ...
2019-08-07T06:44:11.4453380Z LL |       want_F(bar); //~ ERROR mismatched types
2019-08-07T06:44:11.4453727Z    |              ^^^ expected concrete lifetime, found bound lifetime parameter 'cx
2019-08-07T06:44:11.4453771Z    |
2019-08-07T06:44:11.4453990Z    = note: expected type `for<'cx> fn(&'cx S) -> &'cx S`
2019-08-07T06:44:11.4454204Z               found type `for<'a> fn(&'a S) -> &S {bar::<'_>}`
2019-08-07T06:44:11.4454280Z error[E0308]: mismatched types
2019-08-07T06:44:11.4454527Z   --> /checkout/src/test/ui/regions-fn-subtyping-return-static-fail.rs:48:12
2019-08-07T06:44:11.4454571Z    |
2019-08-07T06:44:11.4454571Z    |
2019-08-07T06:44:11.4454754Z LL | / fn baz(x: &S) -> &S {
2019-08-07T06:44:11.4454803Z LL | |     panic!()
2019-08-07T06:44:11.4454841Z LL | | }
2019-08-07T06:44:11.4455046Z    | |_- for<'r> fn(&'r S) -> &'r S {baz} defined here
2019-08-07T06:44:11.4455095Z ...
2019-08-07T06:44:11.4455137Z LL |       want_G(baz); //~ ERROR mismatched types
2019-08-07T06:44:11.4455382Z    |              ^^^ expected concrete lifetime, found bound lifetime parameter 'cx
2019-08-07T06:44:11.4455436Z    |
2019-08-07T06:44:11.4455768Z    = note: expected type `for<'cx> fn(&'cx S) -> &'static S`
2019-08-07T06:44:11.4455969Z               found type `for<'r> fn(&'r S) -> &'r S {baz}`
2019-08-07T06:44:11.4456048Z error: aborting due to 2 previous errors
2019-08-07T06:44:11.4456073Z 
2019-08-07T06:44:11.4456287Z For more information about this error, try `rustc --explain E0308`.
2019-08-07T06:44:11.4456316Z 
---
2019-08-07T06:44:11.4456897Z 
2019-08-07T06:44:11.4456932Z 19 error[E0308]: mismatched types
2019-08-07T06:44:11.4457150Z 20   --> $DIR/region-lifetime-bounds-on-fns-where-clause.rs:20:43
2019-08-07T06:44:11.4457198Z 21    |
2019-08-07T06:44:11.4457390Z - LL |     let _: fn(&mut &isize, &mut &isize) = a;
2019-08-07T06:44:11.4457632Z -    |                                           ^ expected concrete lifetime, found bound lifetime parameter
2019-08-07T06:44:11.4458339Z + LL | / fn a<'a, 'b>(x: &mut &'a isize, y: &mut &'b isize) where 'b: 'a {
2019-08-07T06:44:11.4458594Z + LL | |     // Note: this is legal because of the `'b:'a` declaration.
2019-08-07T06:44:11.4458744Z + LL | |     *x = *y;
2019-08-07T06:44:11.4458805Z + LL | | }
2019-08-07T06:44:11.4459093Z +    | |_- for<'r, 's> fn(&'r mut &isize, &'s mut &isize) {a::<'_, '_>} defined here
2019-08-07T06:44:11.4459142Z + ...
2019-08-07T06:44:11.4459197Z + LL |       let _: fn(&mut &isize, &mut &isize) = a;
2019-08-07T06:44:11.4459253Z +    |                                             ^ expected concrete lifetime, found bound lifetime parameter
2019-08-07T06:44:11.4459300Z 24    |
2019-08-07T06:44:11.4459687Z 25    = note: expected type `for<'r, 's, 't0, 't1> fn(&'r mut &'s isize, &'t0 mut &'t1 isize)`
2019-08-07T06:44:11.4459958Z 26               found type `for<'r, 's> fn(&'r mut &isize, &'s mut &isize) {a::<'_, '_>}`
2019-08-07T06:44:11.4460017Z 
2019-08-07T06:44:11.4460074Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4460431Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-lifetime-bounds-on-fns-where-clause/region-lifetime-bounds-on-fns-where-clause.stderr
2019-08-07T06:44:11.4460431Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-lifetime-bounds-on-fns-where-clause/region-lifetime-bounds-on-fns-where-clause.stderr
2019-08-07T06:44:11.4460685Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4460990Z To only update this specific test, also pass `--test-args regions/region-lifetime-bounds-on-fns-where-clause.rs`
2019-08-07T06:44:11.4461068Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4461250Z status: exit code: 1
2019-08-07T06:44:11.4461250Z status: exit code: 1
2019-08-07T06:44:11.4461942Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/region-lifetime-bounds-on-fns-where-clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-lifetime-bounds-on-fns-where-clause" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-lifetime-bounds-on-fns-where-clause/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4462232Z ------------------------------------------
2019-08-07T06:44:11.4462260Z 
2019-08-07T06:44:11.4462456Z ------------------------------------------
2019-08-07T06:44:11.4462493Z stderr:
2019-08-07T06:44:11.4462493Z stderr:
2019-08-07T06:44:11.4462677Z ------------------------------------------
2019-08-07T06:44:11.4462725Z error[E0623]: lifetime mismatch
2019-08-07T06:44:11.4462958Z   --> /checkout/src/test/ui/regions/region-lifetime-bounds-on-fns-where-clause.rs:8:10
2019-08-07T06:44:11.4463002Z    |
2019-08-07T06:44:11.4463204Z LL | fn b<'a, 'b>(x: &mut &'a isize, y: &mut &'b isize) {
2019-08-07T06:44:11.4463449Z    |                      ---------          --------- these two types are declared with different lifetimes...
2019-08-07T06:44:11.4463661Z LL |     // Illegal now because there is no `'b:'a` declaration.
2019-08-07T06:44:11.4463713Z LL |     *x = *y; //~ ERROR E0623
2019-08-07T06:44:11.4463762Z    |          ^^ ...but data from `y` flows into `x` here
2019-08-07T06:44:11.4463823Z error[E0623]: lifetime mismatch
2019-08-07T06:44:11.4464057Z   --> /checkout/src/test/ui/regions/region-lifetime-bounds-on-fns-where-clause.rs:14:7
2019-08-07T06:44:11.4464099Z    |
2019-08-07T06:44:11.4464099Z    |
2019-08-07T06:44:11.4464292Z LL | fn c<'a,'b>(x: &mut &'a isize, y: &mut &'b isize) {
2019-08-07T06:44:11.4464770Z    |                     ---------          --------- these two types are declared with different lifetimes...
2019-08-07T06:44:11.4464831Z ...
2019-08-07T06:44:11.4464870Z LL |     a(x, y); //~ ERROR lifetime mismatch [E0623]
2019-08-07T06:44:11.4464927Z    |       ^ ...but data from `y` flows into `x` here
2019-08-07T06:44:11.4464988Z error[E0308]: mismatched types
2019-08-07T06:44:11.4465221Z   --> /checkout/src/test/ui/regions/region-lifetime-bounds-on-fns-where-clause.rs:20:43
2019-08-07T06:44:11.4465278Z    |
2019-08-07T06:44:11.4465278Z    |
2019-08-07T06:44:11.4465583Z LL | / fn a<'a, 'b>(x: &mut &'a isize, y: &mut &'b isize) where 'b: 'a {
2019-08-07T06:44:11.4465835Z LL | |     // Note: this is legal because of the `'b:'a` declaration.
2019-08-07T06:44:11.4465892Z LL | |     *x = *y;
2019-08-07T06:44:11.4465927Z LL | | }
2019-08-07T06:44:11.4466146Z    | |_- for<'r, 's> fn(&'r mut &isize, &'s mut &isize) {a::<'_, '_>} defined here
2019-08-07T06:44:11.4466195Z ...
2019-08-07T06:44:11.4466237Z LL |       let _: fn(&mut &isize, &mut &isize) = a; //~ ERROR mismatched types
2019-08-07T06:44:11.4466365Z    |                                             ^ expected concrete lifetime, found bound lifetime parameter
2019-08-07T06:44:11.4466411Z    |
2019-08-07T06:44:11.4466668Z    = note: expected type `for<'r, 's, 't0, 't1> fn(&'r mut &'s isize, &'t0 mut &'t1 isize)`
2019-08-07T06:44:11.4466895Z               found type `for<'r, 's> fn(&'r mut &isize, &'s mut &isize) {a::<'_, '_>}`
2019-08-07T06:44:11.4466971Z error: aborting due to 3 previous errors
2019-08-07T06:44:11.4467003Z 
2019-08-07T06:44:11.4467212Z For more information about this error, try `rustc --explain E0308`.
2019-08-07T06:44:11.4467240Z 
---
2019-08-07T06:44:11.4468466Z 
2019-08-07T06:44:11.4468519Z 30 error[E0308]: mismatched types
2019-08-07T06:44:11.4468784Z 31   --> $DIR/region-multiple-lifetime-bounds-on-fns-where-clause.rs:22:56
2019-08-07T06:44:11.4468830Z 32    |
2019-08-07T06:44:11.4469061Z - LL |     let _: fn(&mut &isize, &mut &isize, &mut &isize) = a;
2019-08-07T06:44:11.4469363Z -    |                                                        ^ expected concrete lifetime, found bound lifetime parameter
2019-08-07T06:44:11.4469650Z + LL | / fn a<'a, 'b, 'c>(x: &mut &'a isize, y: &mut &'b isize, z: &mut &'c isize) where 'b: 'a + 'c {
2019-08-07T06:44:11.4469897Z + LL | |     // Note: this is legal because of the `'b:'a` declaration.
2019-08-07T06:44:11.4469955Z + LL | |     *x = *y;
2019-08-07T06:44:11.4469997Z + LL | |     *z = *y;
2019-08-07T06:44:11.4470038Z + LL | | }
2019-08-07T06:44:11.4470313Z +    | |_- for<'r, 's, 't0> fn(&'r mut &isize, &'s mut &isize, &'t0 mut &isize) {a::<'_, '_, '_>} defined here
2019-08-07T06:44:11.4470376Z + ...
2019-08-07T06:44:11.4470421Z + LL |       let _: fn(&mut &isize, &mut &isize, &mut &isize) = a;
2019-08-07T06:44:11.4470486Z +    |                                                          ^ expected concrete lifetime, found bound lifetime parameter
2019-08-07T06:44:11.4470548Z 35    |
2019-08-07T06:44:11.4470842Z 36    = note: expected type `for<'r, 's, 't0, 't1, 't2, 't3> fn(&'r mut &'s isize, &'t0 mut &'t1 isize, &'t2 mut &'t3 isize)`
2019-08-07T06:44:11.4471262Z 37               found type `for<'r, 's, 't0> fn(&'r mut &isize, &'s mut &isize, &'t0 mut &isize) {a::<'_, '_, '_>}`
2019-08-07T06:44:11.4471332Z 
2019-08-07T06:44:11.4471494Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4471812Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause/region-multiple-lifetime-bounds-on-fns-where-clause.stderr
2019-08-07T06:44:11.4471812Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause/region-multiple-lifetime-bounds-on-fns-where-clause.stderr
2019-08-07T06:44:11.4472042Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4472307Z To only update this specific test, also pass `--test-args regions/region-multiple-lifetime-bounds-on-fns-where-clause.rs`
2019-08-07T06:44:11.4472382Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4472420Z status: exit code: 1
2019-08-07T06:44:11.4472420Z status: exit code: 1
2019-08-07T06:44:11.4473233Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4473631Z ------------------------------------------
2019-08-07T06:44:11.4473663Z 
2019-08-07T06:44:11.4473870Z ------------------------------------------
2019-08-07T06:44:11.4473911Z stderr:
2019-08-07T06:44:11.4473911Z stderr:
2019-08-07T06:44:11.4474131Z ------------------------------------------
2019-08-07T06:44:11.4474174Z error[E0623]: lifetime mismatch
2019-08-07T06:44:11.4474422Z   --> /checkout/src/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.rs:9:10
2019-08-07T06:44:11.4474476Z    |
2019-08-07T06:44:11.4474722Z LL | fn b<'a, 'b, 'c>(x: &mut &'a isize, y: &mut &'b isize, z: &mut &'c isize) {
2019-08-07T06:44:11.4474990Z    |                          ---------          --------- these two types are declared with different lifetimes...
2019-08-07T06:44:11.4475235Z LL |     // Illegal now because there is no `'b:'a` declaration.
2019-08-07T06:44:11.4475281Z LL |     *x = *y; //~ ERROR E0623
2019-08-07T06:44:11.4475324Z    |          ^^ ...but data from `y` flows into `x` here
2019-08-07T06:44:11.4475411Z error[E0623]: lifetime mismatch
2019-08-07T06:44:11.4475667Z   --> /checkout/src/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.rs:10:10
2019-08-07T06:44:11.4475723Z    |
2019-08-07T06:44:11.4475723Z    |
2019-08-07T06:44:11.4475957Z LL | fn b<'a, 'b, 'c>(x: &mut &'a isize, y: &mut &'b isize, z: &mut &'c isize) {
2019-08-07T06:44:11.4476193Z    |                                             ---------          ---------
2019-08-07T06:44:11.4476247Z    |                                             |
2019-08-07T06:44:11.4476304Z    |                                             these two types are declared with different lifetimes...
2019-08-07T06:44:11.4476349Z ...
2019-08-07T06:44:11.4476388Z LL |     *z = *y; //~ ERROR E0623
2019-08-07T06:44:11.4476440Z    |          ^^ ...but data from `y` flows into `z` here
2019-08-07T06:44:11.4476505Z error[E0623]: lifetime mismatch
2019-08-07T06:44:11.4476767Z   --> /checkout/src/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.rs:16:7
2019-08-07T06:44:11.4476820Z    |
2019-08-07T06:44:11.4476820Z    |
2019-08-07T06:44:11.4477052Z LL | fn c<'a,'b, 'c>(x: &mut &'a isize, y: &mut &'b isize, z: &mut &'c isize) {
2019-08-07T06:44:11.4477326Z    |                         ---------          --------- these two types are declared with different lifetimes...
2019-08-07T06:44:11.4477373Z ...
2019-08-07T06:44:11.4477415Z LL |     a(x, y, z); //~ ERROR lifetime mismatch [E0623]
2019-08-07T06:44:11.4477472Z    |       ^ ...but data from `y` flows into `x` here
2019-08-07T06:44:11.4477545Z error[E0308]: mismatched types
2019-08-07T06:44:11.4478157Z   --> /checkout/src/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.rs:22:56
2019-08-07T06:44:11.4478233Z    |
2019-08-07T06:44:11.4478233Z    |
2019-08-07T06:44:11.4478505Z LL | / fn a<'a, 'b, 'c>(x: &mut &'a isize, y: &mut &'b isize, z: &mut &'c isize) where 'b: 'a + 'c {
2019-08-07T06:44:11.4478750Z LL | |     // Note: this is legal because of the `'b:'a` declaration.
2019-08-07T06:44:11.4478822Z LL | |     *x = *y;
2019-08-07T06:44:11.4478867Z LL | |     *z = *y;
2019-08-07T06:44:11.4478910Z LL | | }
2019-08-07T06:44:11.4479219Z    | |_- for<'r, 's, 't0> fn(&'r mut &isize, &'s mut &isize, &'t0 mut &isize) {a::<'_, '_, '_>} defined here
2019-08-07T06:44:11.4479272Z ...
2019-08-07T06:44:11.4479322Z LL |       let _: fn(&mut &isize, &mut &isize, &mut &isize) = a; //~ ERROR E0308
2019-08-07T06:44:11.4479701Z    |                                                          ^ expected concrete lifetime, found bound lifetime parameter
2019-08-07T06:44:11.4479775Z    |
2019-08-07T06:44:11.4480145Z    = note: expected type `for<'r, 's, 't0, 't1, 't2, 't3> fn(&'r mut &'s isize, &'t0 mut &'t1 isize, &'t2 mut &'t3 isize)`
2019-08-07T06:44:11.4480445Z               found type `for<'r, 's, 't0> fn(&'r mut &isize, &'s mut &isize, &'t0 mut &isize) {a::<'_, '_, '_>}`
2019-08-07T06:44:11.4480527Z error: aborting due to 4 previous errors
2019-08-07T06:44:11.4480555Z 
2019-08-07T06:44:11.4480920Z For more information about this error, try `rustc --explain E0308`.
2019-08-07T06:44:11.4480954Z 
---
2019-08-07T06:44:11.4481549Z 
2019-08-07T06:44:11.4481596Z 1 error[E0308]: mismatched types
2019-08-07T06:44:11.4481839Z 2   --> $DIR/regions-fn-subtyping-return-static.rs:41:12
2019-08-07T06:44:11.4481885Z 3    |
2019-08-07T06:44:11.4482079Z - LL |     want_F(bar);
2019-08-07T06:44:11.4482607Z -    |            ^^^ expected concrete lifetime, found bound lifetime parameter 'cx
2019-08-07T06:44:11.4482800Z + LL | / fn bar<'a,'b>(x: &'a S) -> &'b S {
2019-08-07T06:44:11.4482841Z + LL | |     panic!()
2019-08-07T06:44:11.4482885Z + LL | | }
2019-08-07T06:44:11.4483085Z +    | |_- for<'a> fn(&'a S) -> &S {bar::<'_>} defined here
2019-08-07T06:44:11.4483132Z + ...
2019-08-07T06:44:11.4483181Z + LL |       want_F(bar);
2019-08-07T06:44:11.4483406Z +    |              ^^^ expected concrete lifetime, found bound lifetime parameter 'cx
2019-08-07T06:44:11.4483447Z 6    |
2019-08-07T06:44:11.4483658Z 7    = note: expected type `for<'cx> fn(&'cx S) -> &'cx S`
2019-08-07T06:44:11.4483866Z 8               found type `for<'a> fn(&'a S) -> &S {bar::<'_>}`
2019-08-07T06:44:11.4483917Z 
2019-08-07T06:44:11.4483969Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4484259Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-fn-subtyping-return-static/regions-fn-subtyping-return-static.stderr
2019-08-07T06:44:11.4484259Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-fn-subtyping-return-static/regions-fn-subtyping-return-static.stderr
2019-08-07T06:44:11.4484476Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4484749Z To only update this specific test, also pass `--test-args regions/regions-fn-subtyping-return-static.rs`
2019-08-07T06:44:11.4484834Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4484883Z status: exit code: 1
2019-08-07T06:44:11.4484883Z status: exit code: 1
2019-08-07T06:44:11.4485576Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-fn-subtyping-return-static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-fn-subtyping-return-static" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-fn-subtyping-return-static/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4485874Z ------------------------------------------
2019-08-07T06:44:11.4485904Z 
2019-08-07T06:44:11.4486121Z ------------------------------------------
2019-08-07T06:44:11.4486162Z stderr:
2019-08-07T06:44:11.4486162Z stderr:
2019-08-07T06:44:11.4486374Z ------------------------------------------
2019-08-07T06:44:11.4486431Z error[E0308]: mismatched types
2019-08-07T06:44:11.4486672Z   --> /checkout/src/test/ui/regions/regions-fn-subtyping-return-static.rs:41:12
2019-08-07T06:44:11.4486718Z    |
2019-08-07T06:44:11.4486935Z LL | / fn bar<'a,'b>(x: &'a S) -> &'b S {
2019-08-07T06:44:11.4486978Z LL | |     panic!()
2019-08-07T06:44:11.4487015Z LL | | }
2019-08-07T06:44:11.4487234Z    | |_- for<'a> fn(&'a S) -> &S {bar::<'_>} defined here
2019-08-07T06:44:11.4487283Z ...
2019-08-07T06:44:11.4487511Z LL |       want_F(bar); //~ ERROR mismatched types
2019-08-07T06:44:11.4488145Z    |              ^^^ expected concrete lifetime, found bound lifetime parameter 'cx
2019-08-07T06:44:11.4488220Z    |
2019-08-07T06:44:11.4488453Z    = note: expected type `for<'cx> fn(&'cx S) -> &'cx S`
2019-08-07T06:44:11.4488687Z               found type `for<'a> fn(&'a S) -> &S {bar::<'_>}`
2019-08-07T06:44:11.4488776Z error: aborting due to previous error
2019-08-07T06:44:11.4488911Z 
2019-08-07T06:44:11.4489183Z For more information about this error, try `rustc --explain E0308`.
2019-08-07T06:44:11.4489217Z 
---
2019-08-07T06:44:11.4489814Z 
2019-08-07T06:44:11.4489865Z 19 error[E0308]: mismatched types
2019-08-07T06:44:11.4490098Z 20   --> $DIR/regions-lifetime-bounds-on-fns.rs:20:43
2019-08-07T06:44:11.4490151Z 21    |
2019-08-07T06:44:11.4490374Z - LL |     let _: fn(&mut &isize, &mut &isize) = a;
2019-08-07T06:44:11.4490651Z -    |                                           ^ expected concrete lifetime, found bound lifetime parameter
2019-08-07T06:44:11.4490913Z + LL | / fn a<'a, 'b:'a>(x: &mut &'a isize, y: &mut &'b isize) {
2019-08-07T06:44:11.4491275Z + LL | |     // Note: this is legal because of the `'b:'a` declaration.
2019-08-07T06:44:11.4491334Z + LL | |     *x = *y;
2019-08-07T06:44:11.4491373Z + LL | | }
2019-08-07T06:44:11.4491614Z +    | |_- for<'r, 's> fn(&'r mut &isize, &'s mut &isize) {a::<'_, '_>} defined here
2019-08-07T06:44:11.4491658Z + ...
2019-08-07T06:44:11.4491704Z + LL |       let _: fn(&mut &isize, &mut &isize) = a;
2019-08-07T06:44:11.4491754Z +    |                                             ^ expected concrete lifetime, found bound lifetime parameter
2019-08-07T06:44:11.4491808Z 24    |
2019-08-07T06:44:11.4492065Z 25    = note: expected type `for<'r, 's, 't0, 't1> fn(&'r mut &'s isize, &'t0 mut &'t1 isize)`
2019-08-07T06:44:11.4492314Z 26               found type `for<'r, 's> fn(&'r mut &isize, &'s mut &isize) {a::<'_, '_>}`
2019-08-07T06:44:11.4492369Z 
2019-08-07T06:44:11.4492416Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4492716Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-lifetime-bounds-on-fns/regions-lifetime-bounds-on-fns.stderr
2019-08-07T06:44:11.4492716Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-lifetime-bounds-on-fns/regions-lifetime-bounds-on-fns.stderr
2019-08-07T06:44:11.4492964Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4493235Z To only update this specific test, also pass `--test-args regions/regions-lifetime-bounds-on-fns.rs`
2019-08-07T06:44:11.4493306Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4493354Z status: exit code: 1
2019-08-07T06:44:11.4493354Z status: exit code: 1
2019-08-07T06:44:11.4494034Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-lifetime-bounds-on-fns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-lifetime-bounds-on-fns" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-lifetime-bounds-on-fns/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4494334Z ------------------------------------------
2019-08-07T06:44:11.4494365Z 
2019-08-07T06:44:11.4494583Z ------------------------------------------
2019-08-07T06:44:11.4494624Z stderr:
2019-08-07T06:44:11.4494624Z stderr:
2019-08-07T06:44:11.4494824Z ------------------------------------------
2019-08-07T06:44:11.4494879Z error[E0623]: lifetime mismatch
2019-08-07T06:44:11.4495186Z   --> /checkout/src/test/ui/regions/regions-lifetime-bounds-on-fns.rs:8:10
2019-08-07T06:44:11.4495241Z    |
2019-08-07T06:44:11.4495505Z LL | fn b<'a, 'b>(x: &mut &'a isize, y: &mut &'b isize) {
2019-08-07T06:44:11.4495770Z    |                      ---------          --------- these two types are declared with different lifetimes...
2019-08-07T06:44:11.4496000Z LL |     // Illegal now because there is no `'b:'a` declaration.
2019-08-07T06:44:11.4496056Z LL |     *x = *y; //~ ERROR E0623
2019-08-07T06:44:11.4496180Z    |          ^^ ...but data from `y` flows into `x` here
2019-08-07T06:44:11.4496246Z error[E0623]: lifetime mismatch
2019-08-07T06:44:11.4496517Z   --> /checkout/src/test/ui/regions/regions-lifetime-bounds-on-fns.rs:14:7
2019-08-07T06:44:11.4496560Z    |
2019-08-07T06:44:11.4496560Z    |
2019-08-07T06:44:11.4496775Z LL | fn c<'a,'b>(x: &mut &'a isize, y: &mut &'b isize) {
2019-08-07T06:44:11.4497041Z    |                     ---------          --------- these two types are declared with different lifetimes...
2019-08-07T06:44:11.4497095Z ...
2019-08-07T06:44:11.4497138Z LL |     a(x, y); //~ ERROR lifetime mismatch [E0623]
2019-08-07T06:44:11.4497188Z    |       ^ ...but data from `y` flows into `x` here
2019-08-07T06:44:11.4497253Z error[E0308]: mismatched types
2019-08-07T06:44:11.4497839Z   --> /checkout/src/test/ui/regions/regions-lifetime-bounds-on-fns.rs:20:43
2019-08-07T06:44:11.4498195Z    |
2019-08-07T06:44:11.4498195Z    |
2019-08-07T06:44:11.4498490Z LL | / fn a<'a, 'b:'a>(x: &mut &'a isize, y: &mut &'b isize) {
2019-08-07T06:44:11.4498747Z LL | |     // Note: this is legal because of the `'b:'a` declaration.
2019-08-07T06:44:11.4498811Z LL | |     *x = *y;
2019-08-07T06:44:11.4498852Z LL | | }
2019-08-07T06:44:11.4499528Z    | |_- for<'r, 's> fn(&'r mut &isize, &'s mut &isize) {a::<'_, '_>} defined here
2019-08-07T06:44:11.4499600Z ...
2019-08-07T06:44:11.4499647Z LL |       let _: fn(&mut &isize, &mut &isize) = a; //~ ERROR E0308
2019-08-07T06:44:11.4499714Z    |                                             ^ expected concrete lifetime, found bound lifetime parameter
2019-08-07T06:44:11.4499768Z    |
2019-08-07T06:44:11.4500037Z    = note: expected type `for<'r, 's, 't0, 't1> fn(&'r mut &'s isize, &'t0 mut &'t1 isize)`
2019-08-07T06:44:11.4500296Z               found type `for<'r, 's> fn(&'r mut &isize, &'s mut &isize) {a::<'_, '_>}`
2019-08-07T06:44:11.4500387Z error: aborting due to 3 previous errors
2019-08-07T06:44:11.4500416Z 
2019-08-07T06:44:11.4500651Z For more information about this error, try `rustc --explain E0308`.
2019-08-07T06:44:11.4500692Z 
---
2019-08-07T06:44:11.4501276Z 
2019-08-07T06:44:11.4501318Z 195 error[E0308]: mismatched types
2019-08-07T06:44:11.4501536Z 196   --> $DIR/privacy-enum-ctor.rs:27:20
2019-08-07T06:44:11.4501597Z 197    |
2019-08-07T06:44:11.4501639Z + LL |             Fn(u8),
2019-08-07T06:44:11.4501882Z +    |             ------ fn(u8) -> m::n::Z {m::n::Z::Fn} defined here
2019-08-07T06:44:11.4501935Z + ...
2019-08-07T06:44:11.4501977Z 198 LL |         let _: Z = Z::Fn;
2019-08-07T06:44:11.4502071Z 200    |                    |
2019-08-07T06:44:11.4502099Z 
2019-08-07T06:44:11.4502139Z 222 error[E0308]: mismatched types
2019-08-07T06:44:11.4502368Z 223   --> $DIR/privacy-enum-ctor.rs:43:16
2019-08-07T06:44:11.4502368Z 223   --> $DIR/privacy-enum-ctor.rs:43:16
2019-08-07T06:44:11.4502423Z 224    |
2019-08-07T06:44:11.4502464Z + LL |         Fn(u8),
2019-08-07T06:44:11.4502988Z +    |         ------ fn(u8) -> m::E {m::E::Fn} defined here
2019-08-07T06:44:11.4503057Z + ...
2019-08-07T06:44:11.4503099Z 225 LL |     let _: E = m::E::Fn;
2019-08-07T06:44:11.4503185Z 227    |                |
2019-08-07T06:44:11.4503225Z 
2019-08-07T06:44:11.4503387Z 249 error[E0308]: mismatched types
2019-08-07T06:44:11.4504723Z 250   --> $DIR/privacy-enum-ctor.rs:51:16
2019-08-07T06:44:11.4504723Z 250   --> $DIR/privacy-enum-ctor.rs:51:16
2019-08-07T06:44:11.4505126Z 251    |
2019-08-07T06:44:11.4505172Z + LL |         Fn(u8),
2019-08-07T06:44:11.4505473Z +    |         ------ fn(u8) -> m::E {m::E::Fn} defined here
2019-08-07T06:44:11.4505519Z + ...
2019-08-07T06:44:11.4505569Z 252 LL |     let _: E = E::Fn;
2019-08-07T06:44:11.4505655Z 254    |                |
2019-08-07T06:44:11.4505830Z 
2019-08-07T06:44:11.4505855Z 
2019-08-07T06:44:11.4505900Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4505900Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4506241Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/privacy-enum-ctor/privacy-enum-ctor.stderr
2019-08-07T06:44:11.4506498Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4506757Z To only update this specific test, also pass `--test-args resolve/privacy-enum-ctor.rs`
2019-08-07T06:44:11.4506856Z error: 1 errors occurred comparing output.
2019-08-07T06:44:11.4506901Z status: exit code: 1
2019-08-07T06:44:11.4506901Z status: exit code: 1
2019-08-07T06:44:11.4507620Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/privacy-enum-ctor.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/privacy-enum-ctor" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/privacy-enum-ctor/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4508233Z ------------------------------------------
2019-08-07T06:44:11.4508266Z 
2019-08-07T06:44:11.4508501Z ------------------------------------------
2019-08-07T06:44:11.4508545Z stderr:
2019-08-07T06:44:11.4508545Z stderr:
2019-08-07T06:44:11.4508766Z ------------------------------------------
2019-08-07T06:44:11.4508826Z error[E0423]: expected value, found enum `n::Z`
2019-08-07T06:44:11.4509115Z    |
2019-08-07T06:44:11.4509173Z LL |         n::Z;
2019-08-07T06:44:11.4509216Z    |         ^^^^
2019-08-07T06:44:11.4509216Z    |         ^^^^
2019-08-07T06:44:11.4509432Z help: try using one of the enum's variants
2019-08-07T06:44:11.4509486Z    |
2019-08-07T06:44:11.4509527Z LL |         m::Z::Fn;
2019-08-07T06:44:11.4509579Z    |         ^^^^^^^^
2019-08-07T06:44:11.4509621Z LL |         m::Z::Struct;
2019-08-07T06:44:11.4509670Z    |         ^^^^^^^^^^^^
2019-08-07T06:44:11.4509712Z LL |         m::Z::Unit;
2019-08-07T06:44:11.4509786Z 
2019-08-07T06:44:11.4509786Z 
2019-08-07T06:44:11.4509829Z error[E0423]: expected value, found enum `Z`
2019-08-07T06:44:11.4510116Z    |
2019-08-07T06:44:11.4510163Z LL |         Z;
2019-08-07T06:44:11.4510215Z    |         ^
2019-08-07T06:44:11.4510261Z help: a function with a similar name exists
2019-08-07T06:44:11.4510261Z help: a function with a similar name exists
2019-08-07T06:44:11.4510309Z    |
2019-08-07T06:44:11.4510349Z LL |         f;
2019-08-07T06:44:11.4510391Z    |         ^
2019-08-07T06:44:11.4510609Z help: try using one of the enum's variants
2019-08-07T06:44:11.4510664Z    |
2019-08-07T06:44:11.4510704Z LL |         m::Z::Fn;
2019-08-07T06:44:11.4510746Z    |         ^^^^^^^^
2019-08-07T06:44:11.4510802Z LL |         m::Z::Struct;
2019-08-07T06:44:11.4510853Z    |         ^^^^^^^^^^^^
2019-08-07T06:44:11.4510895Z LL |         m::Z::Unit;
2019-08-07T06:44:11.4510973Z 
2019-08-07T06:44:11.4510973Z 
2019-08-07T06:44:11.4511017Z error[E0423]: expected value, found struct variant `Z::Struct`
2019-08-07T06:44:11.4511316Z    |
2019-08-07T06:44:11.4511316Z    |
2019-08-07T06:44:11.4511358Z LL |         let _: Z = Z::Struct;
2019-08-07T06:44:11.4511519Z    |                    ^^^^^^^^^ did you mean `Z::Struct { /* fields */ }`?
2019-08-07T06:44:11.4511559Z 
2019-08-07T06:44:11.4511611Z error[E0423]: expected value, found enum `m::E`
2019-08-07T06:44:11.4512034Z    |
2019-08-07T06:44:11.4512083Z LL |     let _: E = m::E;
2019-08-07T06:44:11.4512123Z    |                ^^^^
2019-08-07T06:44:11.4512166Z help: a function with a similar name exists
2019-08-07T06:44:11.4512166Z help: a function with a similar name exists
2019-08-07T06:44:11.4512403Z    |
2019-08-07T06:44:11.4512441Z LL |     let _: E = m::f;
2019-08-07T06:44:11.4512479Z    |                   ^
2019-08-07T06:44:11.4512708Z help: try using one of the enum's variants
2019-08-07T06:44:11.4512763Z    |
2019-08-07T06:44:11.4512800Z LL |     let _: E = E::Fn;
2019-08-07T06:44:11.4512891Z LL |     let _: E = E::Struct;
2019-08-07T06:44:11.4512931Z    |                ^^^^^^^^^
2019-08-07T06:44:11.4512931Z    |                ^^^^^^^^^
2019-08-07T06:44:11.4512971Z LL |     let _: E = E::Unit;
2019-08-07T06:44:11.4513077Z help: possible better candidates are found in other modules, you can import them into scope
2019-08-07T06:44:11.4513122Z    |
2019-08-07T06:44:11.4513170Z LL | use std::f32::consts::E;
2019-08-07T06:44:11.4513209Z    |
2019-08-07T06:44:11.4513209Z    |
2019-08-07T06:44:11.4513312Z LL | use std::f64::consts::E;
2019-08-07T06:44:11.4513355Z    |
2019-08-07T06:44:11.4513380Z 
2019-08-07T06:44:11.4513422Z error[E0423]: expected value, found struct variant `m::E::Struct`
2019-08-07T06:44:11.4513730Z    |
2019-08-07T06:44:11.4513730Z    |
2019-08-07T06:44:11.4513769Z LL |     let _: E = m::E::Struct;
2019-08-07T06:44:11.4513814Z    |                ^^^^^^^^^^^^ did you mean `m::E::Struct { /* fields */ }`?
2019-08-07T06:44:11.4513844Z 
2019-08-07T06:44:11.4513896Z error[E0423]: expected value, found enum `E`
2019-08-07T06:44:11.4514274Z    |
2019-08-07T06:44:11.4514330Z LL |     let _: E = E;
2019-08-07T06:44:11.4514367Z    |                ^
2019-08-07T06:44:11.4514367Z    |                ^
2019-08-07T06:44:11.4514557Z help: try using one of the enum's variants
2019-08-07T06:44:11.4514604Z    |
2019-08-07T06:44:11.4514639Z LL |     let _: E = E::Fn;
2019-08-07T06:44:11.4514712Z LL |     let _: E = E::Struct;
2019-08-07T06:44:11.4514756Z    |                ^^^^^^^^^
2019-08-07T06:44:11.4514756Z    |                ^^^^^^^^^
2019-08-07T06:44:11.4514793Z LL |     let _: E = E::Unit;
2019-08-07T06:44:11.4514889Z help: possible better candidates are found in other modules, you can import them into scope
2019-08-07T06:44:11.4514930Z    |
2019-08-07T06:44:11.4514965Z LL | use std::f32::consts::E;
2019-08-07T06:44:11.4515123Z    |
2019-08-07T06:44:11.4515123Z    |
2019-08-07T06:44:11.4515161Z LL | use std::f64::consts::E;
2019-08-07T06:44:11.4515198Z    |
2019-08-07T06:44:11.4515222Z 
2019-08-07T06:44:11.4515274Z error[E0423]: expected value, found struct variant `E::Struct`
2019-08-07T06:44:11.4515555Z    |
2019-08-07T06:44:11.4515604Z LL |     let _: E = E::Struct;
2019-08-07T06:44:11.4515604Z LL |     let _: E = E::Struct;
2019-08-07T06:44:11.4515649Z    |                ^^^^^^^^^ did you mean `E::Struct { /* fields */ }`?
2019-08-07T06:44:11.4515677Z 
2019-08-07T06:44:11.4515717Z error[E0412]: cannot find type `Z` in this scope
2019-08-07T06:44:11.4516005Z    |
2019-08-07T06:44:11.4516005Z    |
2019-08-07T06:44:11.4516043Z LL |     let _: Z = m::n::Z;
2019-08-07T06:44:11.4516131Z help: an enum with a similar name exists
2019-08-07T06:44:11.4516169Z    |
2019-08-07T06:44:11.4516169Z    |
2019-08-07T06:44:11.4516207Z LL |     let _: E = m::n::Z;
2019-08-07T06:44:11.4516299Z help: possible candidate is found in another module, you can import it into scope
2019-08-07T06:44:11.4516342Z    |
2019-08-07T06:44:11.4516342Z    |
2019-08-07T06:44:11.4516387Z LL | use m::n::Z;
2019-08-07T06:44:11.4516532Z 
2019-08-07T06:44:11.4516532Z 
2019-08-07T06:44:11.4516572Z error[E0423]: expected value, found enum `m::n::Z`
2019-08-07T06:44:11.4516881Z    |
2019-08-07T06:44:11.4516881Z    |
2019-08-07T06:44:11.4516918Z LL |     let _: Z = m::n::Z;
2019-08-07T06:44:11.4516964Z    |                ^^^^^^^
2019-08-07T06:44:11.4517168Z help: try using one of the enum's variants
2019-08-07T06:44:11.4517405Z    |
2019-08-07T06:44:11.4517450Z LL |     let _: Z = m::Z::Fn;
2019-08-07T06:44:11.4517602Z    |                ^^^^^^^^
2019-08-07T06:44:11.4517643Z LL |     let _: Z = m::Z::Struct;
2019-08-07T06:44:11.4517684Z    |                ^^^^^^^^^^^^
2019-08-07T06:44:11.4517731Z LL |     let _: Z = m::Z::Unit;
2019-08-07T06:44:11.4518088Z 
2019-08-07T06:44:11.4518088Z 
2019-08-07T06:44:11.4518161Z error[E0412]: cannot find type `Z` in this scope
2019-08-07T06:44:11.4518534Z    |
2019-08-07T06:44:11.4518534Z    |
2019-08-07T06:44:11.4518575Z LL |     let _: Z = m::n::Z::Fn;
2019-08-07T06:44:11.4518674Z help: an enum with a similar name exists
2019-08-07T06:44:11.4518715Z    |
2019-08-07T06:44:11.4518715Z    |
2019-08-07T06:44:11.4518765Z LL |     let _: E = m::n::Z::Fn;
2019-08-07T06:44:11.4518855Z help: possible candidate is found in another module, you can import it into scope
2019-08-07T06:44:11.4518914Z    |
2019-08-07T06:44:11.4518914Z    |
2019-08-07T06:44:11.4518954Z LL | use m::n::Z;
2019-08-07T06:44:11.4519019Z 
2019-08-07T06:44:11.4519019Z 
2019-08-07T06:44:11.4519066Z error[E0412]: cannot find type `Z` in this scope
2019-08-07T06:44:11.4519358Z    |
2019-08-07T06:44:11.4519358Z    |
2019-08-07T06:44:11.4519405Z LL |     let _: Z = m::n::Z::Struct;
2019-08-07T06:44:11.4519491Z help: an enum with a similar name exists
2019-08-07T06:44:11.4519540Z    |
2019-08-07T06:44:11.4519540Z    |
2019-08-07T06:44:11.4519587Z LL |     let _: E = m::n::Z::Struct;
2019-08-07T06:44:11.4519677Z help: possible candidate is found in another module, you can import it into scope
2019-08-07T06:44:11.4519734Z    |
2019-08-07T06:44:11.4519734Z    |
2019-08-07T06:44:11.4519773Z LL | use m::n::Z;
2019-08-07T06:44:11.4519837Z 
2019-08-07T06:44:11.4519837Z 
2019-08-07T06:44:11.4519898Z error[E0423]: expected value, found struct variant `m::n::Z::Struct`
2019-08-07T06:44:11.4520195Z    |
2019-08-07T06:44:11.4520195Z    |
2019-08-07T06:44:11.4520250Z LL |     let _: Z = m::n::Z::Struct;
2019-08-07T06:44:11.4520300Z    |                ^^^^^^^^^^^^^^^ did you mean `m::n::Z::Struct { /* fields */ }`?
2019-08-07T06:44:11.4520332Z 
2019-08-07T06:44:11.4520374Z error[E0412]: cannot find type `Z` in this scope
2019-08-07T06:44:11.4520676Z    |
2019-08-07T06:44:11.4520676Z    |
2019-08-07T06:44:11.4520718Z LL |     let _: Z = m::n::Z::Unit {};
2019-08-07T06:44:11.4520817Z help: an enum with a similar name exists
2019-08-07T06:44:11.4520858Z    |
2019-08-07T06:44:11.4520858Z    |
2019-08-07T06:44:11.4520909Z LL |     let _: E = m::n::Z::Unit {};
2019-08-07T06:44:11.4520998Z help: possible candidate is found in another module, you can import it into scope
2019-08-07T06:44:11.4521050Z    |
2019-08-07T06:44:11.4521050Z    |
2019-08-07T06:44:11.4521095Z LL | use m::n::Z;
2019-08-07T06:44:11.4521160Z 
2019-08-07T06:44:11.4521160Z 
2019-08-07T06:44:11.4521209Z error[E0603]: enum `Z` is private
2019-08-07T06:44:11.4521495Z    |
2019-08-07T06:44:11.4521495Z    |
2019-08-07T06:44:11.4521536Z LL |     let _: Z = m::n::Z;
2019-08-07T06:44:11.4521614Z 
2019-08-07T06:44:11.4521614Z 
2019-08-07T06:44:11.4521655Z error[E0603]: enum `Z` is private
2019-08-07T06:44:11.4522067Z    |
2019-08-07T06:44:11.4522067Z    |
2019-08-07T06:44:11.4522108Z LL |     let _: Z = m::n::Z::Fn;
2019-08-07T06:44:11.4522192Z 
2019-08-07T06:44:11.4522192Z 
2019-08-07T06:44:11.4522233Z error[E0603]: enum `Z` is private
2019-08-07T06:44:11.4522567Z    |
2019-08-07T06:44:11.4522567Z    |
2019-08-07T06:44:11.4522608Z LL |     let _: Z = m::n::Z::Struct;
2019-08-07T06:44:11.4522867Z 
2019-08-07T06:44:11.4522867Z 
2019-08-07T06:44:11.4522917Z error[E0603]: enum `Z` is private
2019-08-07T06:44:11.4523219Z    |
2019-08-07T06:44:11.4523219Z    |
2019-08-07T06:44:11.4523259Z LL |     let _: Z = m::n::Z::Unit {};
2019-08-07T06:44:11.4523337Z 
2019-08-07T06:44:11.4523376Z error[E0308]: mismatched types
2019-08-07T06:44:11.4523621Z   --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:27:20
2019-08-07T06:44:11.4523667Z    |
2019-08-07T06:44:11.4523667Z    |
2019-08-07T06:44:11.4523706Z LL |             Fn(u8),
2019-08-07T06:44:11.4523940Z    |             ------ fn(u8) -> m::n::Z {m::n::Z::Fn} defined here
2019-08-07T06:44:11.4523995Z ...
2019-08-07T06:44:11.4524035Z LL |         let _: Z = Z::Fn;
2019-08-07T06:44:11.4524124Z    |                    |
2019-08-07T06:44:11.4524124Z    |                    |
2019-08-07T06:44:11.4524294Z    |                    expected enum `m::n::Z`, found fn item
2019-08-07T06:44:11.4524347Z    |                    help: use parentheses to call this function: `Z::Fn(...)`
2019-08-07T06:44:11.4524404Z    |
2019-08-07T06:44:11.4524446Z    = note: expected type `m::n::Z`
2019-08-07T06:44:11.4524682Z               found type `fn(u8) -> m::n::Z {m::n::Z::Fn}`
2019-08-07T06:44:11.4524714Z 
2019-08-07T06:44:11.4524773Z error[E0618]: expected function, found enum variant `Z::Unit`
2019-08-07T06:44:11.4525064Z    |
2019-08-07T06:44:11.4525117Z LL |             Unit,
2019-08-07T06:44:11.4525117Z LL |             Unit,
2019-08-07T06:44:11.4525335Z    |             ---- `Z::Unit` defined here
2019-08-07T06:44:11.4525380Z ...
2019-08-07T06:44:11.4525430Z LL |         let _ = Z::Unit();
2019-08-07T06:44:11.4525631Z    |                 ^^^^^^^--
2019-08-07T06:44:11.4525722Z    |                 call expression requires function
2019-08-07T06:44:11.4525722Z    |                 call expression requires function
2019-08-07T06:44:11.4525796Z help: `Z::Unit` is a unit variant, you need to write it without the parenthesis
2019-08-07T06:44:11.4525841Z    |
2019-08-07T06:44:11.4525882Z LL |         let _ = Z::Unit;
2019-08-07T06:44:11.4525962Z 
2019-08-07T06:44:11.4526002Z error[E0308]: mismatched types
2019-08-07T06:44:11.4526242Z   --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:43:16
2019-08-07T06:44:11.4526294Z    |
2019-08-07T06:44:11.4526294Z    |
2019-08-07T06:44:11.4526334Z LL |         Fn(u8),
2019-08-07T06:44:11.4526572Z    |         ------ fn(u8) -> m::E {m::E::Fn} defined here
2019-08-07T06:44:11.4526625Z ...
2019-08-07T06:44:11.4526666Z LL |     let _: E = m::E::Fn;
2019-08-07T06:44:11.4526759Z    |                |
2019-08-07T06:44:11.4526759Z    |                |
2019-08-07T06:44:11.4526805Z    |                expected enum `m::E`, found fn item
2019-08-07T06:44:11.4526857Z    |                help: use parentheses to call this function: `m::E::Fn(...)`
2019-08-07T06:44:11.4526916Z    |
2019-08-07T06:44:11.4526958Z    = note: expected type `m::E`
2019-08-07T06:44:11.4527189Z               found type `fn(u8) -> m::E {m::E::Fn}`
2019-08-07T06:44:11.4527221Z 
2019-08-07T06:44:11.4527276Z error[E0618]: expected function, found enum variant `m::E::Unit`
2019-08-07T06:44:11.4527558Z    |
2019-08-07T06:44:11.4527612Z LL |         Unit,
2019-08-07T06:44:11.4527612Z LL |         Unit,
2019-08-07T06:44:11.4528122Z    |         ---- `m::E::Unit` defined here
2019-08-07T06:44:11.4528272Z ...
2019-08-07T06:44:11.4528323Z LL |     let _: E = m::E::Unit();
2019-08-07T06:44:11.4528585Z    |                ^^^^^^^^^^--
2019-08-07T06:44:11.4528676Z    |                call expression requires function
2019-08-07T06:44:11.4528676Z    |                call expression requires function
2019-08-07T06:44:11.4528737Z help: `m::E::Unit` is a unit variant, you need to write it without the parenthesis
2019-08-07T06:44:11.4528782Z    |
2019-08-07T06:44:11.4528823Z LL |     let _: E = m::E::Unit;
2019-08-07T06:44:11.4528983Z 
2019-08-07T06:44:11.4529023Z error[E0308]: mismatched types
2019-08-07T06:44:11.4529288Z   --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:51:16
2019-08-07T06:44:11.4529343Z    |
2019-08-07T06:44:11.4529343Z    |
2019-08-07T06:44:11.4529382Z LL |         Fn(u8),
2019-08-07T06:44:11.4529614Z    |         ------ fn(u8) -> m::E {m::E::Fn} defined here
2019-08-07T06:44:11.4529666Z ...
2019-08-07T06:44:11.4529707Z LL |     let _: E = E::Fn;
2019-08-07T06:44:11.4529801Z    |                |
2019-08-07T06:44:11.4529801Z    |                |
2019-08-07T06:44:11.4529859Z    |                expected enum `m::E`, found fn item
2019-08-07T06:44:11.4529911Z    |                help: use parentheses to call this function: `E::Fn(...)`
2019-08-07T06:44:11.4529956Z    |
2019-08-07T06:44:11.4530009Z    = note: expected type `m::E`
2019-08-07T06:44:11.4530242Z               found type `fn(u8) -> m::E {m::E::Fn}`
2019-08-07T06:44:11.4530283Z 
2019-08-07T06:44:11.4530328Z error[E0618]: expected function, found enum variant `E::Unit`
2019-08-07T06:44:11.4530628Z    |
2019-08-07T06:44:11.4530669Z LL |         Unit,
2019-08-07T06:44:11.4530669Z LL |         Unit,
2019-08-07T06:44:11.4531015Z    |         ---- `E::Unit` defined here
2019-08-07T06:44:11.4531054Z ...
2019-08-07T06:44:11.4531089Z LL |     let _: E = E::Unit();
2019-08-07T06:44:11.4531276Z    |                ^^^^^^^--
2019-08-07T06:44:11.4531362Z    |                call expression requires function
2019-08-07T06:44:11.4531362Z    |                call expression requires function
2019-08-07T06:44:11.4531415Z help: `E::Unit` is a unit variant, you need to write it without the parenthesis
2019-08-07T06:44:11.4531455Z    |
2019-08-07T06:44:11.4531490Z LL |     let _: E = E::Unit;
2019-08-07T06:44:11.4531562Z 
2019-08-07T06:44:11.4531598Z error: aborting due to 23 previous errors
2019-08-07T06:44:11.4531622Z 
2019-08-07T06:44:11.4531669Z Some errors have detailed explanations: E0308, E0412, E0423, E0603, E0618.
---
2019-08-07T06:44:11.4532419Z 
2019-08-07T06:44:11.4532467Z 1 error[E0308]: mismatched types
2019-08-07T06:44:11.4532653Z 2   --> $DIR/substs-ppaux.rs:16:17
2019-08-07T06:44:11.4532691Z 3    |
2019-08-07T06:44:11.4532886Z + LL |     fn bar<'a, T>() where T: 'a {}
2019-08-07T06:44:11.4533138Z +    |     ------------------------------ fn() {<i8 as Foo<'static, 'static, u8>>::bar::<'static, char>} defined here
2019-08-07T06:44:11.4533183Z + ...
2019-08-07T06:44:11.4533421Z 4 LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>;
2019-08-07T06:44:11.4533517Z 6    |                 |
2019-08-07T06:44:11.4533541Z 
2019-08-07T06:44:11.4533586Z 13 error[E0308]: mismatched types
2019-08-07T06:44:11.4533773Z 14   --> $DIR/substs-ppaux.rs:25:17
2019-08-07T06:44:11.4533773Z 14   --> $DIR/substs-ppaux.rs:25:17
2019-08-07T06:44:11.4533812Z 15    |
2019-08-07T06:44:11.4534009Z + LL |     fn bar<'a, T>() where T: 'a {}
2019-08-07T06:44:11.4534327Z +    |     ------------------------------ fn() {<i8 as Foo<'static, 'static>>::bar::<'static, char>} defined here
2019-08-07T06:44:11.4534379Z + ...
2019-08-07T06:44:11.4534634Z 16 LL |     let x: () = <i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>;
2019-08-07T06:44:11.4534725Z 18    |                 |
2019-08-07T06:44:11.4534749Z 
2019-08-07T06:44:11.4534792Z 25 error[E0308]: mismatched types
2019-08-07T06:44:11.4534979Z 26   --> $DIR/substs-ppaux.rs:33:17
2019-08-07T06:44:11.4534979Z 26   --> $DIR/substs-ppaux.rs:33:17
2019-08-07T06:44:11.4535088Z 27    |
2019-08-07T06:44:11.4535132Z + LL |     fn baz() {}
2019-08-07T06:44:11.4535377Z +    |     ----------- fn() {<i8 as Foo<'static, 'static, u8>>::baz} defined here
2019-08-07T06:44:11.4535419Z + ...
2019-08-07T06:44:11.4535635Z 28 LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::baz;
2019-08-07T06:44:11.4535723Z 30    |                 |
2019-08-07T06:44:11.4535746Z 
2019-08-07T06:44:11.4535803Z 37 error[E0308]: mismatched types
2019-08-07T06:44:11.4535991Z 38   --> $DIR/substs-ppaux.rs:41:17
2019-08-07T06:44:11.4535991Z 38   --> $DIR/substs-ppaux.rs:41:17
2019-08-07T06:44:11.4536028Z 39    |
2019-08-07T06:44:11.4536221Z - LL |     let x: () = foo::<'static>;
2019-08-07T06:44:11.4536578Z -    |                 |
2019-08-07T06:44:11.4536578Z -    |                 |
2019-08-07T06:44:11.4536771Z -    |                 expected (), found fn item
2019-08-07T06:44:11.4537005Z -    |                 help: use parentheses to call this function: `foo::<'static>()`
2019-08-07T06:44:11.4537209Z + LL | / fn foo<'z>() where &'z (): Sized {
2019-08-07T06:44:11.4537548Z + LL | |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>;
2019-08-07T06:44:11.4537607Z + LL | |
2019-08-07T06:44:11.4537644Z + LL | |
2019-08-07T06:44:11.4537681Z + ...  |
2019-08-07T06:44:11.4538253Z + LL | |     let x: () = foo::<'static>;
2019-08-07T06:44:11.4538348Z +    | |                 |
2019-08-07T06:44:11.4538348Z +    | |                 |
2019-08-07T06:44:11.4538414Z +    | |                 expected (), found fn item
2019-08-07T06:44:11.4538680Z +    | |                 help: use parentheses to call this function: `foo::<'static>()`
2019-08-07T06:44:11.4538727Z + ...  |
2019-08-07T06:44:11.4538775Z + LL | |
2019-08-07T06:44:11.4538817Z + LL | | }
2019-08-07T06:44:11.4539034Z +    | |_- fn() {foo::<'static>} defined here
2019-08-07T06:44:11.4539128Z 46    = note: expected type `()`
2019-08-07T06:44:11.4539128Z 46    = note: expected type `()`
2019-08-07T06:44:11.4539364Z 47               found type `fn() {foo::<'static>}`
2019-08-07T06:44:11.4539420Z 
2019-08-07T06:44:11.4539474Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4539474Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4539768Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/substs-ppaux.normal/substs-ppaux.normal.stderr
2019-08-07T06:44:11.4540012Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4540303Z To only update this specific test, also pass `--test-args substs-ppaux.rs`
2019-08-07T06:44:11.4540340Z 
2019-08-07T06:44:11.4540388Z error in revision `normal`: 1 errors occurred comparing output.
2019-08-07T06:44:11.4540436Z status: exit code: 1
2019-08-07T06:44:11.4541409Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/substs-ppaux.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "normal" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/substs-ppaux.normal" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/substs-ppaux.normal/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4541715Z ------------------------------------------
2019-08-07T06:44:11.4541745Z 
2019-08-07T06:44:11.4541946Z ------------------------------------------
2019-08-07T06:44:11.4542088Z stderr:
2019-08-07T06:44:11.4542088Z stderr:
2019-08-07T06:44:11.4542324Z ------------------------------------------
2019-08-07T06:44:11.4542369Z error[E0308]: mismatched types
2019-08-07T06:44:11.4542708Z   --> /checkout/src/test/ui/substs-ppaux.rs:16:17
2019-08-07T06:44:11.4542751Z    |
2019-08-07T06:44:11.4542951Z LL |     fn bar<'a, T>() where T: 'a {}
2019-08-07T06:44:11.4543232Z    |     ------------------------------ fn() {<i8 as Foo<'static, 'static, u8>>::bar::<'static, char>} defined here
2019-08-07T06:44:11.4543371Z ...
2019-08-07T06:44:11.4543646Z LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>;
2019-08-07T06:44:11.4543759Z    |                 |
2019-08-07T06:44:11.4543759Z    |                 |
2019-08-07T06:44:11.4543910Z    |                 expected (), found fn item
2019-08-07T06:44:11.4544203Z    |                 help: use parentheses to call this function: `<i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>()`
2019-08-07T06:44:11.4544300Z    = note: expected type `()`
2019-08-07T06:44:11.4544300Z    = note: expected type `()`
2019-08-07T06:44:11.4544784Z               found type `fn() {<i8 as Foo<'static, 'static, u8>>::bar::<'static, char>}`
2019-08-07T06:44:11.4544864Z error[E0308]: mismatched types
2019-08-07T06:44:11.4545081Z   --> /checkout/src/test/ui/substs-ppaux.rs:25:17
2019-08-07T06:44:11.4545132Z    |
2019-08-07T06:44:11.4545132Z    |
2019-08-07T06:44:11.4545328Z LL |     fn bar<'a, T>() where T: 'a {}
2019-08-07T06:44:11.4545602Z    |     ------------------------------ fn() {<i8 as Foo<'static, 'static>>::bar::<'static, char>} defined here
2019-08-07T06:44:11.4545656Z ...
2019-08-07T06:44:11.4545999Z LL |     let x: () = <i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>;
2019-08-07T06:44:11.4546095Z    |                 |
2019-08-07T06:44:11.4546095Z    |                 |
2019-08-07T06:44:11.4546133Z    |                 expected (), found fn item
2019-08-07T06:44:11.4546401Z    |                 help: use parentheses to call this function: `<i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>()`
2019-08-07T06:44:11.4546491Z    = note: expected type `()`
2019-08-07T06:44:11.4546491Z    = note: expected type `()`
2019-08-07T06:44:11.4546715Z               found type `fn() {<i8 as Foo<'static, 'static>>::bar::<'static, char>}`
2019-08-07T06:44:11.4546787Z error[E0308]: mismatched types
2019-08-07T06:44:11.4546982Z   --> /checkout/src/test/ui/substs-ppaux.rs:33:17
2019-08-07T06:44:11.4547029Z    |
2019-08-07T06:44:11.4547073Z LL |     fn baz() {}
2019-08-07T06:44:11.4547073Z LL |     fn baz() {}
2019-08-07T06:44:11.4547293Z    |     ----------- fn() {<i8 as Foo<'static, 'static, u8>>::baz} defined here
2019-08-07T06:44:11.4547332Z ...
2019-08-07T06:44:11.4548158Z LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::baz;
2019-08-07T06:44:11.4548276Z    |                 |
2019-08-07T06:44:11.4548276Z    |                 |
2019-08-07T06:44:11.4548332Z    |                 expected (), found fn item
2019-08-07T06:44:11.4548664Z    |                 help: use parentheses to call this function: `<i8 as Foo<'static, 'static,  u8>>::baz()`
2019-08-07T06:44:11.4548755Z    = note: expected type `()`
2019-08-07T06:44:11.4548755Z    = note: expected type `()`
2019-08-07T06:44:11.4549015Z               found type `fn() {<i8 as Foo<'static, 'static, u8>>::baz}`
2019-08-07T06:44:11.4549088Z error[E0308]: mismatched types
2019-08-07T06:44:11.4549321Z   --> /checkout/src/test/ui/substs-ppaux.rs:41:17
2019-08-07T06:44:11.4549376Z    |
2019-08-07T06:44:11.4549376Z    |
2019-08-07T06:44:11.4549589Z LL | / fn foo<'z>() where &'z (): Sized {
2019-08-07T06:44:11.4549840Z LL | |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>;
2019-08-07T06:44:11.4549898Z LL | |     //[verbose]~^ ERROR mismatched types
2019-08-07T06:44:11.4549945Z LL | |     //[verbose]~| expected type `()`
2019-08-07T06:44:11.4549987Z ...  |
2019-08-07T06:44:11.4550203Z LL | |     let x: () = foo::<'static>;
2019-08-07T06:44:11.4550419Z    | |                 |
2019-08-07T06:44:11.4550419Z    | |                 |
2019-08-07T06:44:11.4550470Z    | |                 expected (), found fn item
2019-08-07T06:44:11.4550767Z    | |                 help: use parentheses to call this function: `foo::<'static>()`
2019-08-07T06:44:11.4550816Z ...  |
2019-08-07T06:44:11.4550860Z LL | |     //[normal]~^^ ERROR the size for values of type
2019-08-07T06:44:11.4550912Z LL | | }
2019-08-07T06:44:11.4551348Z    | |_- fn() {foo::<'static>} defined here
2019-08-07T06:44:11.4551552Z    = note: expected type `()`
2019-08-07T06:44:11.4551552Z    = note: expected type `()`
2019-08-07T06:44:11.4551746Z               found type `fn() {foo::<'static>}`
2019-08-07T06:44:11.4551815Z error[E0277]: the size for values of type `str` cannot be known at compilation time
2019-08-07T06:44:11.4552018Z   --> /checkout/src/test/ui/substs-ppaux.rs:49:5
2019-08-07T06:44:11.4552056Z    |
2019-08-07T06:44:11.4552056Z    |
2019-08-07T06:44:11.4552091Z LL |     <str as Foo<u8>>::bar;
2019-08-07T06:44:11.4552316Z    |     ^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-08-07T06:44:11.4552358Z    |
2019-08-07T06:44:11.4552398Z    = help: the trait `std::marker::Sized` is not implemented for `str`
2019-08-07T06:44:11.4552678Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-08-07T06:44:11.4552915Z    = note: required because of the requirements on the impl of `Foo<'_, '_, u8>` for `str`
2019-08-07T06:44:11.4552966Z note: required by `Foo::bar`
2019-08-07T06:44:11.4553203Z    |
2019-08-07T06:44:11.4553203Z    |
2019-08-07T06:44:11.4553384Z LL |     fn bar<'a, T>() where T: 'a {}
2019-08-07T06:44:11.4553454Z 
2019-08-07T06:44:11.4553490Z error: aborting due to 5 previous errors
2019-08-07T06:44:11.4553514Z 
2019-08-07T06:44:11.4553558Z Some errors have detailed explanations: E0277, E0308.
---
2019-08-07T06:44:11.4554300Z 
2019-08-07T06:44:11.4554335Z 1 error[E0308]: mismatched types
2019-08-07T06:44:11.4554526Z 2   --> $DIR/substs-ppaux.rs:16:17
2019-08-07T06:44:11.4554563Z 3    |
2019-08-07T06:44:11.4554754Z + LL |     fn bar<'a, T>() where T: 'a {}
2019-08-07T06:44:11.4555004Z +    |     ------------------------------ fn() {<i8 as Foo<ReStatic, ReStatic, u8>>::bar::<ReStatic, char>} defined here
2019-08-07T06:44:11.4555047Z + ...
2019-08-07T06:44:11.4555272Z 4 LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>;
2019-08-07T06:44:11.4555368Z 6    |                 |
2019-08-07T06:44:11.4555397Z 
2019-08-07T06:44:11.4555434Z 13 error[E0308]: mismatched types
2019-08-07T06:44:11.4555620Z 14   --> $DIR/substs-ppaux.rs:25:17
2019-08-07T06:44:11.4555620Z 14   --> $DIR/substs-ppaux.rs:25:17
2019-08-07T06:44:11.4555658Z 15    |
2019-08-07T06:44:11.4555847Z + LL |     fn bar<'a, T>() where T: 'a {}
2019-08-07T06:44:11.4556093Z +    |     ------------------------------ fn() {<i8 as Foo<ReStatic, ReStatic>>::bar::<ReStatic, char>} defined here
2019-08-07T06:44:11.4556143Z + ...
2019-08-07T06:44:11.4556369Z 16 LL |     let x: () = <i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>;
2019-08-07T06:44:11.4556458Z 18    |                 |
2019-08-07T06:44:11.4556486Z 
2019-08-07T06:44:11.4556523Z 25 error[E0308]: mismatched types
2019-08-07T06:44:11.4556707Z 26   --> $DIR/substs-ppaux.rs:33:17
2019-08-07T06:44:11.4556707Z 26   --> $DIR/substs-ppaux.rs:33:17
2019-08-07T06:44:11.4556745Z 27    |
2019-08-07T06:44:11.4556855Z + LL |     fn baz() {}
2019-08-07T06:44:11.4557108Z +    |     ----------- fn() {<i8 as Foo<ReStatic, ReStatic, u8>>::baz} defined here
2019-08-07T06:44:11.4557149Z + ...
2019-08-07T06:44:11.4557360Z 28 LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::baz;
2019-08-07T06:44:11.4557445Z 30    |                 |
2019-08-07T06:44:11.4557469Z 
2019-08-07T06:44:11.4557582Z 37 error[E0308]: mismatched types
2019-08-07T06:44:11.4558161Z 38   --> $DIR/substs-ppaux.rs:41:17
2019-08-07T06:44:11.4558161Z 38   --> $DIR/substs-ppaux.rs:41:17
2019-08-07T06:44:11.4558211Z 39    |
2019-08-07T06:44:11.4558433Z - LL |     let x: () = foo::<'static>;
2019-08-07T06:44:11.4558841Z -    |                 |
2019-08-07T06:44:11.4558841Z -    |                 |
2019-08-07T06:44:11.4559065Z -    |                 expected (), found fn item
2019-08-07T06:44:11.4559324Z -    |                 help: use parentheses to call this function: `foo::<'static>()`
2019-08-07T06:44:11.4559554Z + LL | / fn foo<'z>() where &'z (): Sized {
2019-08-07T06:44:11.4559815Z + LL | |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>;
2019-08-07T06:44:11.4559865Z + LL | |
2019-08-07T06:44:11.4559904Z + LL | |
2019-08-07T06:44:11.4559943Z + ...  |
2019-08-07T06:44:11.4560161Z + LL | |     let x: () = foo::<'static>;
2019-08-07T06:44:11.4560251Z +    | |                 |
2019-08-07T06:44:11.4560251Z +    | |                 |
2019-08-07T06:44:11.4560310Z +    | |                 expected (), found fn item
2019-08-07T06:44:11.4560571Z +    | |                 help: use parentheses to call this function: `foo::<'static>()`
2019-08-07T06:44:11.4560619Z + ...  |
2019-08-07T06:44:11.4560662Z + LL | |
2019-08-07T06:44:11.4560703Z + LL | | }
2019-08-07T06:44:11.4560919Z +    | |_- fn() {foo::<ReStatic>} defined here
2019-08-07T06:44:11.4561011Z 46    = note: expected type `()`
2019-08-07T06:44:11.4561011Z 46    = note: expected type `()`
2019-08-07T06:44:11.4561187Z 47               found type `fn() {foo::<ReStatic>}`
2019-08-07T06:44:11.4561235Z 
2019-08-07T06:44:11.4561278Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4561278Z The actual stderr differed from the expected stderr.
2019-08-07T06:44:11.4561535Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/substs-ppaux.verbose/substs-ppaux.verbose.stderr
2019-08-07T06:44:11.4561747Z To update references, rerun the tests and pass the `--bless` flag
2019-08-07T06:44:11.4561968Z To only update this specific test, also pass `--test-args substs-ppaux.rs`
2019-08-07T06:44:11.4562004Z 
2019-08-07T06:44:11.4562043Z error in revision `verbose`: 1 errors occurred comparing output.
2019-08-07T06:44:11.4562087Z status: exit code: 1
2019-08-07T06:44:11.4562726Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/substs-ppaux.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "verbose" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/substs-ppaux.verbose" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "verbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/substs-ppaux.verbose/auxiliary" "-A" "unused"
2019-08-07T06:44:11.4562999Z ------------------------------------------
2019-08-07T06:44:11.4563028Z 
2019-08-07T06:44:11.4563220Z ------------------------------------------
2019-08-07T06:44:11.4563268Z stderr:
2019-08-07T06:44:11.4563268Z stderr:
2019-08-07T06:44:11.4563453Z ------------------------------------------
2019-08-07T06:44:11.4563493Z error[E0308]: mismatched types
2019-08-07T06:44:11.4563692Z   --> /checkout/src/test/ui/substs-ppaux.rs:16:17
2019-08-07T06:44:11.4563730Z    |
2019-08-07T06:44:11.4563910Z LL |     fn bar<'a, T>() where T: 'a {}
2019-08-07T06:44:11.4564164Z    |     ------------------------------ fn() {<i8 as Foo<ReStatic, ReStatic, u8>>::bar::<ReStatic, char>} defined here
2019-08-07T06:44:11.4564209Z ...
2019-08-07T06:44:11.4564524Z LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>;
2019-08-07T06:44:11.4564630Z    |                 |
2019-08-07T06:44:11.4564630Z    |                 |
2019-08-07T06:44:11.4564669Z    |                 expected (), found fn item
2019-08-07T06:44:11.4564965Z    |                 help: use parentheses to call this function: `<i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>()`
2019-08-07T06:44:11.4565122Z    = note: expected type `()`
2019-08-07T06:44:11.4565122Z    = note: expected type `()`
2019-08-07T06:44:11.4565170Z               found type `fn() {<i8 as Foo<ReStatic, ReStatic, u8>>::bar::<ReStatic, char>}`
2019-08-07T06:44:11.4565235Z error[E0308]: mismatched types
2019-08-07T06:44:11.4565458Z   --> /checkout/src/test/ui/substs-ppaux.rs:25:17
2019-08-07T06:44:11.4565503Z    |
2019-08-07T06:44:11.4565503Z    |
2019-08-07T06:44:11.4565686Z LL |     fn bar<'a, T>() where T: 'a {}
2019-08-07T06:44:11.4565940Z    |     ------------------------------ fn() {<i8 as Foo<ReStatic, ReStatic>>::bar::<ReStatic, char>} defined here
2019-08-07T06:44:11.4565989Z ...
2019-08-07T06:44:11.4566208Z LL |     let x: () = <i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>;
2019-08-07T06:44:11.4566300Z    |                 |
2019-08-07T06:44:11.4566300Z    |                 |
2019-08-07T06:44:11.4566340Z    |                 expected (), found fn item
2019-08-07T06:44:11.4566606Z    |                 help: use parentheses to call this function: `<i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>()`
2019-08-07T06:44:11.4566691Z    = note: expected type `()`
2019-08-07T06:44:11.4566691Z    = note: expected type `()`
2019-08-07T06:44:11.4566735Z               found type `fn() {<i8 as Foo<ReStatic, ReStatic>>::bar::<ReStatic, char>}`
2019-08-07T06:44:11.4566802Z error[E0308]: mismatched types
2019-08-07T06:44:11.4566999Z   --> /checkout/src/test/ui/substs-ppaux.rs:33:17
2019-08-07T06:44:11.4567046Z    |
2019-08-07T06:44:11.4567086Z LL |     fn baz() {}
2019-08-07T06:44:11.4567086Z LL |     fn baz() {}
2019-08-07T06:44:11.4567306Z    |     ----------- fn() {<i8 as Foo<ReStatic, ReStatic, u8>>::baz} defined here
2019-08-07T06:44:11.4567346Z ...
2019-08-07T06:44:11.4567983Z LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::baz;
2019-08-07T06:44:11.4568099Z    |                 |
2019-08-07T06:44:11.4568099Z    |                 |
2019-08-07T06:44:11.4568156Z    |                 expected (), found fn item
2019-08-07T06:44:11.4568502Z    |                 help: use parentheses to call this function: `<i8 as Foo<'static, 'static,  u8>>::baz()`
2019-08-07T06:44:11.4568593Z    = note: expected type `()`
2019-08-07T06:44:11.4568593Z    = note: expected type `()`
2019-08-07T06:44:11.4568646Z               found type `fn() {<i8 as Foo<ReStatic, ReStatic, u8>>::baz}`
2019-08-07T06:44:11.4568718Z error[E0308]: mismatched types
2019-08-07T06:44:11.4568957Z   --> /checkout/src/test/ui/substs-ppaux.rs:41:17
2019-08-07T06:44:11.4569003Z    |
2019-08-07T06:44:11.4569003Z    |
2019-08-07T06:44:11.4569216Z LL | / fn foo<'z>() where &'z (): Sized {
2019-08-07T06:44:11.4569465Z LL | |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>;
2019-08-07T06:44:11.4569528Z LL | |     //[verbose]~^ ERROR mismatched types
2019-08-07T06:44:11.4569574Z LL | |     //[verbose]~| expected type `()`
2019-08-07T06:44:11.4569616Z ...  |
2019-08-07T06:44:11.4569831Z LL | |     let x: () = foo::<'static>;
2019-08-07T06:44:11.4569929Z    | |                 |
2019-08-07T06:44:11.4569929Z    | |                 |
2019-08-07T06:44:11.4569980Z    | |                 expected (), found fn item
2019-08-07T06:44:11.4570238Z    | |                 help: use parentheses to call this function: `foo::<'static>()`
2019-08-07T06:44:11.4570285Z ...  |
2019-08-07T06:44:11.4570336Z LL | |     //[normal]~^^ ERROR the size for values of type
2019-08-07T06:44:11.4570379Z LL | | }
2019-08-07T06:44:11.4570702Z    | |_- fn() {foo::<ReStatic>} defined here
2019-08-07T06:44:11.4570804Z    = note: expected type `()`
2019-08-07T06:44:11.4570804Z    = note: expected type `()`
2019-08-07T06:44:11.4570850Z               found type `fn() {foo::<ReStatic>}`
2019-08-07T06:44:11.4570932Z error[E0277]: the size for values of type `str` cannot be known at compilation time
2019-08-07T06:44:11.4571195Z   --> /checkout/src/test/ui/substs-ppaux.rs:49:5
2019-08-07T06:44:11.4571241Z    |
2019-08-07T06:44:11.4571241Z    |
2019-08-07T06:44:11.4571282Z LL |     <str as Foo<u8>>::bar;
2019-08-07T06:44:11.4571857Z    |     ^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2019-08-07T06:44:11.4571901Z    |
2019-08-07T06:44:11.4571944Z    = help: the trait `std::marker::Sized` is not implemented for `str`
2019-08-07T06:44:11.4572242Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-08-07T06:44:11.4572627Z    = note: required because of the requirements on the impl of `Foo<'_#0r, '_#1r, u8>` for `str`
2019-08-07T06:44:11.4572684Z note: required by `Foo::bar`
2019-08-07T06:44:11.4572947Z    |
2019-08-07T06:44:11.4572947Z    |
2019-08-07T06:44:11.4573323Z LL |     fn bar<'a, T>() where T: 'a {}
2019-08-07T06:44:11.4573520Z 
2019-08-07T06:44:11.4573556Z error: aborting due to 5 previous errors
2019-08-07T06:44:11.4573580Z 
2019-08-07T06:44:11.4573621Z Some errors have detailed explanations: E0277, E0308.
---
2019-08-07T06:44:11.4582341Z test result: FAILED. 8782 passed; 28 failed; 31 ignored; 0 measured; 0 filtered out
2019-08-07T06:44:11.4582382Z 
2019-08-07T06:44:11.4582526Z 
2019-08-07T06:44:11.4582558Z 
2019-08-07T06:44:11.4584011Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-07T06:44:11.4584325Z 
2019-08-07T06:44:11.4584355Z 
2019-08-07T06:44:11.4585160Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-07T06:44:11.4585224Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-07T06:44:11.4585224Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-07T06:44:11.4585335Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-07T06:44:11.4585390Z Build completed unsuccessfully in 1:07:09
2019-08-07T06:44:12.2047461Z ##[error]Bash exited with code '1'.
2019-08-07T06:44:12.2083933Z ##[section]Starting: Checkout
2019-08-07T06:44:12.2085592Z ==============================================================================
2019-08-07T06:44:12.2085639Z Task         : Get sources
2019-08-07T06:44:12.2085727Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
