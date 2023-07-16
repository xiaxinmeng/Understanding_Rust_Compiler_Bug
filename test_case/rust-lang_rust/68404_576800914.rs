plain
2020-01-21T16:46:23.1938052Z ========================== Starting Command Output ===========================
2020-01-21T16:46:23.1939645Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3c82bdaf-ec17-4ffa-8426-e01fd62067c3.sh
2020-01-21T16:46:23.1939743Z 
2020-01-21T16:46:23.1942247Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-21T16:46:23.1948583Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68404/merge to s
2020-01-21T16:46:23.1950291Z Task         : Get sources
2020-01-21T16:46:23.1950399Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-21T16:46:23.1950432Z Version      : 1.0.0
2020-01-21T16:46:23.1950465Z Author       : Microsoft
---
2020-01-21T16:46:24.1748030Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-21T16:46:24.1757907Z ##[command]git config gc.auto 0
2020-01-21T16:46:24.1760115Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-21T16:46:24.1761964Z ##[command]git config --get-all http.proxy
2020-01-21T16:46:24.1767988Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68404/merge:refs/remotes/pull/68404/merge
---
2020-01-21T17:40:54.6407086Z .................................................................................................... 1700/9542
2020-01-21T17:41:00.8058592Z .................................................................................................... 1800/9542
2020-01-21T17:41:11.9954406Z ....................i............................................................................... 1900/9542
2020-01-21T17:41:18.8572551Z .................................................................................................... 2000/9542
2020-01-21T17:41:33.6716757Z ..........iiiii..................................................................................... 2100/9542
2020-01-21T17:41:42.9630785Z .................................................................................................... 2300/9542
2020-01-21T17:41:45.3863556Z .................................................................................................... 2400/9542
2020-01-21T17:41:50.7161705Z .................F.................................................................................. 2500/9542
2020-01-21T17:42:10.8794621Z .................................................................................................... 2600/9542
---
2020-01-21T17:44:47.6710666Z ......................................................i...............i............................. 4900/9542
2020-01-21T17:44:55.7107568Z .................................................................................................... 5000/9542
2020-01-21T17:45:03.3583660Z .................................................................................................i.. 5100/9542
2020-01-21T17:45:08.3449413Z .................................................................................................... 5200/9542
2020-01-21T17:45:18.8001422Z .....................................................................ii.ii...........i.............. 5300/9542
2020-01-21T17:45:27.8836362Z ......i............................................................................................. 5500/9542
2020-01-21T17:45:38.1038062Z .................................................................................................... 5600/9542
2020-01-21T17:45:44.8554482Z .......................................................i............................................ 5700/9542
2020-01-21T17:45:51.9774268Z .................................................................................................... 5800/9542
2020-01-21T17:45:51.9774268Z .................................................................................................... 5800/9542
2020-01-21T17:46:01.8799530Z .................................................................................................... 5900/9542
2020-01-21T17:46:08.9238472Z ..............................................ii...i..ii...........i................................ 6000/9542
2020-01-21T17:46:31.2262605Z .................................................................................................... 6200/9542
2020-01-21T17:46:39.4138912Z .................................................................................................... 6300/9542
2020-01-21T17:46:39.4138912Z .................................................................................................... 6300/9542
2020-01-21T17:46:47.8401544Z ..........................................................................i..ii..................... 6400/9542
2020-01-21T17:47:15.3946534Z .................................................................................................... 6600/9542
2020-01-21T17:47:19.6586089Z ..................................................i................................................. 6700/9542
2020-01-21T17:47:22.0438350Z .................................................................................................... 6800/9542
2020-01-21T17:47:24.4521072Z .................................................i.................................................. 6900/9542
---
2020-01-21T17:49:06.0442833Z .................................................................................................... 7600/9542
2020-01-21T17:49:11.6806965Z .................................................................................................... 7700/9542
2020-01-21T17:49:18.2544996Z .................................................................................................... 7800/9542
2020-01-21T17:49:28.9021606Z .................................................................................................... 7900/9542
2020-01-21T17:49:34.8076687Z ..iiiiiii........................................................................................... 8000/9542
2020-01-21T17:49:50.2368476Z .................................................................................................... 8200/9542
2020-01-21T17:50:01.7039224Z .................................................................................................... 8300/9542
2020-01-21T17:50:14.6464160Z .................................................................................................... 8400/9542
2020-01-21T17:50:20.8224461Z .................................................................................................... 8500/9542
---
2020-01-21T17:52:17.4389044Z 
2020-01-21T17:52:17.4389074Z 
2020-01-21T17:52:17.4389145Z The actual stderr differed from the expected stderr.
2020-01-21T17:52:17.4389708Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/asm-parse-errors/asm-parse-errors.stderr
2020-01-21T17:52:17.4389995Z To update references, rerun the tests and pass the `--bless` flag
2020-01-21T17:52:17.4390320Z To only update this specific test, also pass `--test-args asm/asm-parse-errors.rs`
2020-01-21T17:52:17.4390423Z error: 1 errors occurred comparing output.
2020-01-21T17:52:17.4390474Z status: exit code: 1
2020-01-21T17:52:17.4390474Z status: exit code: 1
2020-01-21T17:52:17.4391384Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/asm-parse-errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/asm-parse-errors" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/asm-parse-errors/auxiliary" "-A" "unused"
2020-01-21T17:52:17.4391758Z ------------------------------------------
2020-01-21T17:52:17.4391796Z 
2020-01-21T17:52:17.4392057Z ------------------------------------------
2020-01-21T17:52:17.4392128Z stderr:
2020-01-21T17:52:17.4392128Z stderr:
2020-01-21T17:52:17.4392372Z ------------------------------------------
2020-01-21T17:52:17.4392439Z error: macro requires a string literal as an argument
2020-01-21T17:52:17.4392719Z   --> /checkout/src/test/ui/asm/asm-parse-errors.rs:4:5
2020-01-21T17:52:17.4392774Z    |
2020-01-21T17:52:17.4392833Z LL |     asm!(); //~ ERROR requires a string literal as an argument
2020-01-21T17:52:17.4392952Z    |
2020-01-21T17:52:17.4393316Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2020-01-21T17:52:17.4393364Z 
2020-01-21T17:52:17.4393439Z error: expected string literal
2020-01-21T17:52:17.4393439Z error: expected string literal
2020-01-21T17:52:17.4393710Z   --> /checkout/src/test/ui/asm/asm-parse-errors.rs:5:18
2020-01-21T17:52:17.4393762Z    |
2020-01-21T17:52:17.4393834Z LL |     asm!("nop" : struct); //~ ERROR expected string literal
2020-01-21T17:52:17.4393941Z 
2020-01-21T17:52:17.4394005Z error: expected string literal
2020-01-21T17:52:17.4394271Z   --> /checkout/src/test/ui/asm/asm-parse-errors.rs:6:30
2020-01-21T17:52:17.4394322Z    |
2020-01-21T17:52:17.4394322Z    |
2020-01-21T17:52:17.4394377Z LL |     asm!("mov %eax, $$0x2" : struct); //~ ERROR expected string literal
2020-01-21T17:52:17.4394489Z 
2020-01-21T17:52:17.4394489Z 
2020-01-21T17:52:17.4394539Z error: expected `(`, found keyword `struct`
2020-01-21T17:52:17.4394873Z    |
2020-01-21T17:52:17.4394873Z    |
2020-01-21T17:52:17.4394926Z LL |     asm!("mov %eax, $$0x2" : "={eax}" struct); //~ ERROR expected `(`
2020-01-21T17:52:17.4394999Z    |                                       ^^^^^^ expected `(`
2020-01-21T17:52:17.4395086Z error: expected expression, found keyword `struct`
2020-01-21T17:52:17.4395469Z   --> /checkout/src/test/ui/asm/asm-parse-errors.rs:8:39
2020-01-21T17:52:17.4395613Z    |
2020-01-21T17:52:17.4395613Z    |
2020-01-21T17:52:17.4395667Z LL |     asm!("mov %eax, $$0x2" : "={eax}"(struct)); //~ ERROR expected expression
2020-01-21T17:52:17.4395765Z 
2020-01-21T17:52:17.4395827Z error: expected string literal
2020-01-21T17:52:17.4396133Z   --> /checkout/src/test/ui/asm/asm-parse-errors.rs:9:44
2020-01-21T17:52:17.4396186Z    |
2020-01-21T17:52:17.4396186Z    |
2020-01-21T17:52:17.4396259Z LL |     asm!("in %dx, %al" : "={al}"(result) : struct); //~ ERROR expected string literal
2020-01-21T17:52:17.4396357Z 
2020-01-21T17:52:17.4396357Z 
2020-01-21T17:52:17.4396421Z error: expected `(`, found keyword `struct`
2020-01-21T17:52:17.4396742Z    |
2020-01-21T17:52:17.4396742Z    |
2020-01-21T17:52:17.4396806Z LL |     asm!("in %dx, %al" : "={al}"(result) : "{dx}" struct); //~ ERROR expected `(`
2020-01-21T17:52:17.4396885Z    |                                                   ^^^^^^ expected `(`
2020-01-21T17:52:17.4396979Z error: expected expression, found keyword `struct`
2020-01-21T17:52:17.4397270Z   --> /checkout/src/test/ui/asm/asm-parse-errors.rs:11:51
2020-01-21T17:52:17.4397322Z    |
2020-01-21T17:52:17.4397322Z    |
2020-01-21T17:52:17.4397377Z LL |     asm!("in %dx, %al" : "={al}"(result) : "{dx}"(struct)); //~ ERROR expected expression
2020-01-21T17:52:17.4397494Z 
2020-01-21T17:52:17.4397541Z error: expected string literal
2020-01-21T17:52:17.4397805Z   --> /checkout/src/test/ui/asm/asm-parse-errors.rs:12:36
2020-01-21T17:52:17.4397873Z    |
2020-01-21T17:52:17.4397873Z    |
2020-01-21T17:52:17.4397927Z LL |     asm!("mov $$0x200, %eax" : : : struct); //~ ERROR expected string literal
2020-01-21T17:52:17.4398048Z 
2020-01-21T17:52:17.4398095Z error: expected string literal
2020-01-21T17:52:17.4398374Z   --> /checkout/src/test/ui/asm/asm-parse-errors.rs:13:45
2020-01-21T17:52:17.4398424Z    |
2020-01-21T17:52:17.4398424Z    |
2020-01-21T17:52:17.4398497Z LL |     asm!("mov eax, 2" : "={eax}"(foo) : : : struct); //~ ERROR expected string literal
2020-01-21T17:52:17.4398595Z 
2020-01-21T17:52:17.4398661Z error: inline assembly must be a string literal
2020-01-21T17:52:17.4398928Z   --> /checkout/src/test/ui/asm/asm-parse-errors.rs:14:10
2020-01-21T17:52:17.4398978Z    |
2020-01-21T17:52:17.4398978Z    |
2020-01-21T17:52:17.4399047Z LL |     asm!(123); //~ ERROR inline assembly must be a string literal
2020-01-21T17:52:17.4399133Z 
2020-01-21T17:52:17.4399182Z error: aborting due to 11 previous errors
2020-01-21T17:52:17.4399230Z 
2020-01-21T17:52:17.4399261Z 
2020-01-21T17:52:17.4399261Z 
2020-01-21T17:52:17.4399513Z ------------------------------------------
2020-01-21T17:52:17.4399550Z 
2020-01-21T17:52:17.4399588Z 
2020-01-21T17:52:17.4399851Z ---- [ui] ui/error-codes/E0660.rs stdout ----
2020-01-21T17:52:17.4399905Z diff of stderr:
2020-01-21T17:52:17.4399937Z 
2020-01-21T17:52:17.4399981Z 3    |
2020-01-21T17:52:17.4400047Z 4 LL |     asm!("nop" "nop");
2020-01-21T17:52:17.4400143Z +    |
2020-01-21T17:52:17.4400520Z +    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2020-01-21T17:52:17.4400580Z 6 
2020-01-21T17:52:17.4400629Z 7 error[E0660]: malformed inline assembly
2020-01-21T17:52:17.4400629Z 7 error[E0660]: malformed inline assembly
2020-01-21T17:52:17.4400877Z 8   --> $DIR/E0660.rs:7:5
2020-01-21T17:52:17.4400911Z 
2020-01-21T17:52:17.4400955Z 9    |
2020-01-21T17:52:17.4401004Z 10 LL |     asm!("nop" "nop" : "=r"(a));
2020-01-21T17:52:17.4401220Z +    |
2020-01-21T17:52:17.4401621Z +    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2020-01-21T17:52:17.4401783Z 12 
2020-01-21T17:52:17.4401835Z 13 error: aborting due to 2 previous errors
2020-01-21T17:52:17.4401835Z 13 error: aborting due to 2 previous errors
2020-01-21T17:52:17.4401883Z 14 
2020-01-21T17:52:17.4401914Z 
2020-01-21T17:52:17.4401959Z 
2020-01-21T17:52:17.4402011Z The actual stderr differed from the expected stderr.
2020-01-21T17:52:17.4402366Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0660/E0660.stderr
2020-01-21T17:52:17.4402644Z To update references, rerun the tests and pass the `--bless` flag
2020-01-21T17:52:17.4402955Z To only update this specific test, also pass `--test-args error-codes/E0660.rs`
2020-01-21T17:52:17.4403044Z error: 1 errors occurred comparing output.
2020-01-21T17:52:17.4403111Z status: exit code: 1
2020-01-21T17:52:17.4403111Z status: exit code: 1
2020-01-21T17:52:17.4403997Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0660.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0660" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0660/auxiliary" "-A" "unused"
2020-01-21T17:52:17.4404380Z ------------------------------------------
2020-01-21T17:52:17.4404417Z 
2020-01-21T17:52:17.4404683Z ------------------------------------------
2020-01-21T17:52:17.4404734Z stderr:
2020-01-21T17:52:17.4404734Z stderr:
2020-01-21T17:52:17.4404974Z ------------------------------------------
2020-01-21T17:52:17.4405046Z error[E0660]: malformed inline assembly
2020-01-21T17:52:17.4405316Z   --> /checkout/src/test/ui/error-codes/E0660.rs:5:5
2020-01-21T17:52:17.4405368Z    |
2020-01-21T17:52:17.4405415Z LL |     asm!("nop" "nop");
2020-01-21T17:52:17.4405537Z    |
2020-01-21T17:52:17.4405897Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2020-01-21T17:52:17.4405963Z 
2020-01-21T17:52:17.4406015Z error[E0660]: malformed inline assembly
2020-01-21T17:52:17.4406015Z error[E0660]: malformed inline assembly
2020-01-21T17:52:17.4406780Z   --> /checkout/src/test/ui/error-codes/E0660.rs:7:5
2020-01-21T17:52:17.4406857Z    |
2020-01-21T17:52:17.4406906Z LL |     asm!("nop" "nop" : "=r"(a));
2020-01-21T17:52:17.4407003Z    |
2020-01-21T17:52:17.4407384Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2020-01-21T17:52:17.4407430Z 
2020-01-21T17:52:17.4407480Z error: aborting due to 2 previous errors
---
2020-01-21T17:52:17.4408456Z ---- [ui] ui/inline-asm-bad-constraint.rs stdout ----
2020-01-21T17:52:17.4408510Z diff of stderr:
2020-01-21T17:52:17.4408541Z 
2020-01-21T17:52:17.4408605Z 3    |
2020-01-21T17:52:17.4408654Z 4 LL |         asm!("" :"={rax"(rax))
2020-01-21T17:52:17.4408767Z +    |
2020-01-21T17:52:17.4409126Z +    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2020-01-21T17:52:17.4409189Z 6 
2020-01-21T17:52:17.4409260Z 7 error[E0668]: malformed inline assembly
2020-01-21T17:52:17.4409260Z 7 error[E0668]: malformed inline assembly
2020-01-21T17:52:17.4409521Z 8   --> $DIR/inline-asm-bad-constraint.rs:30:9
2020-01-21T17:52:17.4409683Z 
2020-01-21T17:52:17.4409738Z 9    |
2020-01-21T17:52:17.4409806Z 10 LL |         asm!("callq $0" : : "0"(foo))
2020-01-21T17:52:17.4409976Z +    |
2020-01-21T17:52:17.4410393Z +    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2020-01-21T17:52:17.4410454Z 12 
2020-01-21T17:52:17.4410503Z 13 error[E0668]: malformed inline assembly
2020-01-21T17:52:17.4410503Z 13 error[E0668]: malformed inline assembly
2020-01-21T17:52:17.4410779Z 14   --> $DIR/inline-asm-bad-constraint.rs:37:9
2020-01-21T17:52:17.4410816Z 
2020-01-21T17:52:17.4410861Z 15    |
2020-01-21T17:52:17.4410913Z 16 LL |         asm!("addb $1, $0" : "={rax}"((0i32, rax)));
2020-01-21T17:52:17.4411036Z +    |
2020-01-21T17:52:17.4411406Z +    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2020-01-21T17:52:17.4411484Z 18 
2020-01-21T17:52:17.4411534Z 19 error: aborting due to 3 previous errors
2020-01-21T17:52:17.4411534Z 19 error: aborting due to 3 previous errors
2020-01-21T17:52:17.4411590Z 20 
2020-01-21T17:52:17.4411619Z 
2020-01-21T17:52:17.4411666Z 
2020-01-21T17:52:17.4411715Z The actual stderr differed from the expected stderr.
2020-01-21T17:52:17.4412073Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inline-asm-bad-constraint/inline-asm-bad-constraint.stderr
2020-01-21T17:52:17.4412372Z To update references, rerun the tests and pass the `--bless` flag
2020-01-21T17:52:17.4412667Z To only update this specific test, also pass `--test-args inline-asm-bad-constraint.rs`
2020-01-21T17:52:17.4412756Z error: 1 errors occurred comparing output.
2020-01-21T17:52:17.4412824Z status: exit code: 1
2020-01-21T17:52:17.4412824Z status: exit code: 1
2020-01-21T17:52:17.4413705Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inline-asm-bad-constraint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inline-asm-bad-constraint" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inline-asm-bad-constraint/auxiliary" "-A" "unused"
2020-01-21T17:52:17.4414090Z ------------------------------------------
2020-01-21T17:52:17.4414127Z 
2020-01-21T17:52:17.4414395Z ------------------------------------------
2020-01-21T17:52:17.4414446Z stderr:
2020-01-21T17:52:17.4414446Z stderr:
2020-01-21T17:52:17.4414685Z ------------------------------------------
2020-01-21T17:52:17.4414757Z error[E0668]: malformed inline assembly
2020-01-21T17:52:17.4415024Z   --> /checkout/src/test/ui/inline-asm-bad-constraint.rs:22:9
2020-01-21T17:52:17.4415079Z    |
2020-01-21T17:52:17.4415160Z LL |         asm!("" :"={rax"(rax)) //~ ERROR E0668
2020-01-21T17:52:17.4415269Z    |
2020-01-21T17:52:17.4415651Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2020-01-21T17:52:17.4415699Z 
2020-01-21T17:52:17.4415751Z error[E0668]: malformed inline assembly
2020-01-21T17:52:17.4415751Z error[E0668]: malformed inline assembly
2020-01-21T17:52:17.4416040Z   --> /checkout/src/test/ui/inline-asm-bad-constraint.rs:30:9
2020-01-21T17:52:17.4416093Z    |
2020-01-21T17:52:17.4416147Z LL |         asm!("callq $0" : : "0"(foo)) //~ ERROR E0668
2020-01-21T17:52:17.4416263Z    |
2020-01-21T17:52:17.4416618Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2020-01-21T17:52:17.4416663Z 
2020-01-21T17:52:17.4416729Z error[E0668]: malformed inline assembly
2020-01-21T17:52:17.4416729Z error[E0668]: malformed inline assembly
2020-01-21T17:52:17.4417082Z   --> /checkout/src/test/ui/inline-asm-bad-constraint.rs:37:9
2020-01-21T17:52:17.4417141Z    |
2020-01-21T17:52:17.4417274Z LL |         asm!("addb $1, $0" : "={rax}"((0i32, rax))); //~ ERROR E0668
2020-01-21T17:52:17.4417383Z    |
2020-01-21T17:52:17.4417770Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2020-01-21T17:52:17.4417834Z 
2020-01-21T17:52:17.4417884Z error: aborting due to 3 previous errors
---
2020-01-21T17:52:17.4418821Z ---- [ui] ui/issues/issue-23458.rs stdout ----
2020-01-21T17:52:17.4418873Z diff of stderr:
2020-01-21T17:52:17.4418906Z 
2020-01-21T17:52:17.4418961Z 3    |
2020-01-21T17:52:17.4419027Z 4 LL |         asm!("int $3");
2020-01-21T17:52:17.4419131Z +    |
2020-01-21T17:52:17.4419511Z +    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2020-01-21T17:52:17.4419575Z 6 
2020-01-21T17:52:17.4419575Z 6 
2020-01-21T17:52:17.4419628Z 7 error: <inline asm>:1:2: error: too few operands for instruction
2020-01-21T17:52:17.4419727Z 
2020-01-21T17:52:17.4419772Z 12    |
2020-01-21T17:52:17.4419772Z 12    |
2020-01-21T17:52:17.4419820Z 13 LL |         asm!("int $3");
2020-01-21T17:52:17.4419932Z +    |
2020-01-21T17:52:17.4420292Z +    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2020-01-21T17:52:17.4420369Z 15 
2020-01-21T17:52:17.4420469Z 16 error: aborting due to 2 previous errors
2020-01-21T17:52:17.4420469Z 16 error: aborting due to 2 previous errors
2020-01-21T17:52:17.4420517Z 17 
2020-01-21T17:52:17.4420546Z 
2020-01-21T17:52:17.4420601Z 
2020-01-21T17:52:17.4420653Z The actual stderr differed from the expected stderr.
2020-01-21T17:52:17.4420994Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23458/issue-23458.stderr
2020-01-21T17:52:17.4421287Z To update references, rerun the tests and pass the `--bless` flag
2020-01-21T17:52:17.4421579Z To only update this specific test, also pass `--test-args issues/issue-23458.rs`
2020-01-21T17:52:17.4421667Z error: 1 errors occurred comparing output.
2020-01-21T17:52:17.4421735Z status: exit code: 1
2020-01-21T17:52:17.4421735Z status: exit code: 1
2020-01-21T17:52:17.4422595Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23458.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23458" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23458/auxiliary" "-A" "unused"
2020-01-21T17:52:17.4422966Z ------------------------------------------
2020-01-21T17:52:17.4423003Z 
2020-01-21T17:52:17.4423270Z ------------------------------------------
2020-01-21T17:52:17.4423320Z stderr:
2020-01-21T17:52:17.4423320Z stderr:
2020-01-21T17:52:17.4423558Z ------------------------------------------
2020-01-21T17:52:17.4423822Z error: invalid operand in inline asm: 'int $3'
2020-01-21T17:52:17.4424085Z   --> /checkout/src/test/ui/issues/issue-23458.rs:8:9
2020-01-21T17:52:17.4424136Z    |
2020-01-21T17:52:17.4424190Z LL |         asm!("int $3"); //~ ERROR too few operands for instruction
2020-01-21T17:52:17.4424312Z    |
2020-01-21T17:52:17.4424762Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2020-01-21T17:52:17.4424892Z 
2020-01-21T17:52:17.4424892Z 
2020-01-21T17:52:17.4424952Z error: <inline asm>:1:2: error: too few operands for instruction
2020-01-21T17:52:17.4425072Z         ^
2020-01-21T17:52:17.4425103Z 
2020-01-21T17:52:17.4425400Z   --> /checkout/src/test/ui/issues/issue-23458.rs:8:9
2020-01-21T17:52:17.4425453Z    |
2020-01-21T17:52:17.4425453Z    |
2020-01-21T17:52:17.4425524Z LL |         asm!("int $3"); //~ ERROR too few operands for instruction
2020-01-21T17:52:17.4425624Z    |
2020-01-21T17:52:17.4425998Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2020-01-21T17:52:17.4426043Z 
2020-01-21T17:52:17.4426092Z error: aborting due to 2 previous errors
---
2020-01-21T17:52:17.4440243Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-21T17:52:17.4440331Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-21T17:52:17.4440396Z 
2020-01-21T17:52:17.4440428Z 
2020-01-21T17:52:17.4447767Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-21T17:52:17.4448095Z 
2020-01-21T17:52:17.4448132Z 
2020-01-21T17:52:17.4459796Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-21T17:52:17.4459903Z Build completed unsuccessfully in 1:00:24
2020-01-21T17:52:17.4459903Z Build completed unsuccessfully in 1:00:24
2020-01-21T17:52:17.4512396Z == clock drift check ==
2020-01-21T17:52:17.4528782Z   local time: Tue Jan 21 17:52:17 UTC 2020
2020-01-21T17:52:17.4977018Z   network time: Tue, 21 Jan 2020 17:52:17 GMT
2020-01-21T17:52:17.4981499Z == end clock drift check ==
2020-01-21T17:52:17.9703487Z 
2020-01-21T17:52:17.9810273Z ##[error]Bash exited with code '1'.
2020-01-21T17:52:17.9822961Z ##[section]Finishing: Run build
2020-01-21T17:52:17.9853016Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68404/merge to s
2020-01-21T17:52:17.9855092Z Task         : Get sources
2020-01-21T17:52:17.9855166Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-21T17:52:17.9855218Z Version      : 1.0.0
2020-01-21T17:52:17.9855264Z Author       : Microsoft
2020-01-21T17:52:17.9855264Z Author       : Microsoft
2020-01-21T17:52:17.9855335Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-21T17:52:17.9855391Z ==============================================================================
2020-01-21T17:52:18.4221474Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-21T17:52:18.4262763Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68404/merge to s
2020-01-21T17:52:18.4371406Z Cleaning up task key
2020-01-21T17:52:18.4372152Z Start cleaning up orphan processes.
2020-01-21T17:52:18.4487288Z Terminate orphan process: pid (4431) (python)
2020-01-21T17:52:18.4742479Z ##[section]Finishing: Finalize Job
